//! Application state machine, persistence, and toolkit integration.

pub mod action_dispatch;
pub mod audio_flow;
pub mod bootstrap;
pub mod capture_campaign;
pub mod capture_colony;
pub mod capture_debrief;
pub mod capture_first_hour;
pub mod capture_recruited_roster;
pub mod capture_reset;
pub mod capture_scene_groups;
pub mod capture_scenes;
pub mod capture_tactical;
pub mod class_action_flow;
pub mod colony_flow;
pub mod debrief_flow;
pub mod destructive_flow;
pub mod facility_upgrade_flow;
pub mod field_notes_flow;
pub mod first_hour_flow;
pub mod formation_flow;
pub mod input;
pub mod memorial_flow;
pub mod persistence_io;
pub mod playtest_flow;
pub mod salvage_flow;
pub mod session_flow;
pub mod skill_flow;
pub mod types;

use crate::campaign::CampaignState;
use crate::colony_ui;
use crate::combat_feedback::CombatFeedback;
use crate::data::{GameData, MissionDef};
use crate::formation::FormationKind;
use crate::grid_ui::WorldCamera;
use crate::phase_replay::PhaseReplay;
use crate::state::{GameSession, MissionOutcome};
use crate::ui::{self, TargetingView, UiAction, UiContext};
use crate::ui_action::BattleLogFilter;
use crate::visual_assets::VisualCatalog;
use macroquad::prelude::*;
use macroquad_toolkit::assets::AssetManager;
use macroquad_toolkit::events::EventBus;
use macroquad_toolkit::notifications::{
    NotificationAnchor, NotificationManager, NotificationRenderConfig,
};
use macroquad_toolkit::persistence::AutoSaveManager;
use macroquad_toolkit::prelude::{
    begin_virtual_ui_frame, dark, end_virtual_ui_frame, GamepadInput,
};

pub use types::{AppState, TacticalTargeting};

pub struct Game {
    data: GameData,
    session: GameSession,
    campaign: CampaignState,
    active_mission: MissionDef,
    state: AppState,
    assets: AssetManager,
    visuals: VisualCatalog,
    notifications: NotificationManager,
    events: EventBus<UiAction>,
    save_exists: bool,
    last_outcome: Option<MissionOutcome>,
    autosave: AutoSaveManager,
    targeting: Option<TacticalTargeting>,
    show_tactical_help: bool,
    show_battle_log: bool,
    battle_log_filter: BattleLogFilter,
    tactical_panel_open: bool,
    combat_feedback: CombatFeedback,
    observed_event_count: usize,
    played_audio_event_count: usize,
    phase_replay: PhaseReplay,
    deployment_formation: FormationKind,
    end_phase_armed: bool,
    gamepad: GamepadInput,
    gamepad_connected: bool,
    title_focus_continue: bool,
    title_focus_active: bool,
    title_hover_preview: bool,
    capture_pointer: Option<Vec2>,
    new_campaign_armed: bool,
    delete_save_armed: bool,
    tactical_camera: WorldCamera,
    colony_camera: WorldCamera,
    colony_explorer: crate::colony_exploration::ColonyExplorer,
    colony_suppress_map_release: bool,
    colony_operations_open: bool,
    facility_upgrade_open: bool,
    salvage_open: bool,
    audio: crate::audio::AudioSystem,
    show_settings: bool,
    show_field_notes: bool,
    show_memorial: bool,
    memorial_page: usize,
    selected_field_note: usize,
    roster_inspection_id: Option<String>,
}

impl Game {
    pub fn update(&mut self, dt: f32) {
        if self.colony_explorer_can_update() {
            self.colony_explorer.update(dt, &self.campaign.colony);
            self.colony_explorer
                .update_approach(&self.campaign, &self.data);
        }
        self.update_playtest_metrics(dt);
        self.update_motion(dt);
        self.notifications.update(dt);
        if self.capture_input() {
            return;
        }

        let actions: Vec<_> = self.events.drain().collect();
        for action in actions {
            self.apply_action(action);
        }
        self.sync_audio();
        let impact = self.combat_feedback.sync(
            (self.state == AppState::Tactical).then_some(&self.session.tactical.event_log),
            &mut self.observed_event_count,
        );
        if impact {
            self.gamepad.rumble(140, 0.48, 0.76);
        }
    }

    pub fn draw(&mut self) {
        clear_background(dark::BACKGROUND);
        let virtual_ui = begin_virtual_ui_frame(ui::LOGICAL_WIDTH, ui::LOGICAL_HEIGHT);
        let mut actions = match self.state {
            AppState::Title => ui::draw_title(ui::TitleDrawContext {
                data: &self.data,
                save_exists: self.save_exists,
                assets: &self.assets,
                visuals: &self.visuals,
                ui: &virtual_ui,
                controller_focus_continue: self
                    .title_focus_active
                    .then_some(self.title_focus_continue),
                hover_preview: self.title_hover_preview,
                new_campaign_armed: self.new_campaign_armed,
            }),
            AppState::Colony => {
                let result = colony_ui::draw_colony(colony_ui::ColonyDrawContext {
                    campaign: &self.campaign,
                    data: &self.data,
                    assets: &self.assets,
                    visuals: &self.visuals,
                    ui: &virtual_ui,
                    camera: &self.colony_camera,
                    explorer: &self.colony_explorer,
                    operations_open: self.colony_operations_open,
                    suppress_map_release: self.colony_suppress_map_release,
                    facility_upgrade_open: self.facility_upgrade_open,
                    salvage_open: self.salvage_open,
                    settings_open: self.show_settings,
                    field_notes_open: self.show_field_notes,
                    memorial_open: self.show_memorial,
                    memorial_page: self.memorial_page,
                    selected_field_note: self.selected_field_note,
                });
                result.actions
            }
            AppState::Roster => crate::roster_ui::draw_roster(
                &self.campaign,
                &self.data,
                &self.assets,
                &self.visuals,
                &virtual_ui,
                self.roster_inspection_id.as_deref(),
            ),
            AppState::GeneLab => crate::gene_lab_ui::draw_gene_lab(
                &self.campaign,
                &self.data,
                &self.assets,
                &self.visuals,
                &virtual_ui,
            ),
            AppState::MissionBriefing => ui::draw_mission_briefing(
                &self.data,
                &self.campaign,
                &self.active_mission,
                self.deployment_formation,
                &self.assets,
                &self.visuals,
                &virtual_ui,
            ),
            AppState::Tactical => ui::draw_tactical(
                UiContext {
                    feedback: &self.combat_feedback,
                    phase_replay: &self.phase_replay,
                    end_phase_armed: self.end_phase_armed,
                    data: &self.data,
                    assets: &self.assets,
                    visuals: &self.visuals,
                    mission: &self.active_mission,
                    session: &self.session,
                    save_exists: self.save_exists,
                    delete_save_armed: self.delete_save_armed,
                    first_hour: &self.campaign.first_hour,
                    ui: &virtual_ui,
                    pointer_override: self.capture_pointer,
                    targeting: self.targeting.as_ref().map(|targeting| match targeting {
                        TacticalTargeting::Equipment {
                            unit_id,
                            equipment_id,
                        } => TargetingView::Equipment {
                            unit_id,
                            equipment_id,
                        },
                        TacticalTargeting::ClassAction {
                            unit_id,
                            target_kind,
                        } => TargetingView::ClassAction {
                            unit_id,
                            target_kind: *target_kind,
                        },
                        TacticalTargeting::Skill { unit_id, skill_id } => {
                            TargetingView::Skill { unit_id, skill_id }
                        }
                    }),
                    show_help: self.show_tactical_help,
                    show_battle_log: self.show_battle_log,
                    battle_log_filter: self.battle_log_filter,
                    show_settings: self.show_settings,
                    tactical_panel_open: self.tactical_panel_open,
                },
                &mut self.tactical_camera,
            ),
            AppState::Debrief => crate::ui_debrief::draw_debrief(
                &self.active_mission,
                self.last_outcome
                    .as_ref()
                    .expect("debrief requires an outcome"),
                &self.session.deployed_colonist_ids(),
                &self.campaign,
                &self.assets,
                &self.visuals,
                &virtual_ui,
            ),
            AppState::DemoComplete => {
                crate::demo_ui::draw(&self.campaign, &self.assets, &self.visuals, &virtual_ui)
            }
        };
        self.draw_first_hour(&virtual_ui, &mut actions);
        self.draw_settings(&virtual_ui, &mut actions);
        end_virtual_ui_frame();
        for action in actions {
            self.events.push(action);
        }
        self.notifications
            .draw_with_config(&NotificationRenderConfig {
                anchor: NotificationAnchor::BottomRight,
                ..Default::default()
            });
    }
}
