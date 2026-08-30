//! Application state machine, persistence, and toolkit integration.

mod audio_flow;
mod bootstrap;
mod capture_colony;
mod capture_debrief;
mod capture_first_hour;
mod capture_recruited_roster;
mod capture_reset;
mod capture_scenes;
mod capture_tactical;
mod class_action_flow;
mod colony_flow;
mod debrief_flow;
mod destructive_flow;
mod facility_upgrade_flow;
mod first_hour_flow;
mod formation_flow;
mod input;
mod persistence_io;
mod playtest_flow;
mod salvage_flow;
mod skill_flow;
mod types;

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

use types::{AppState, TacticalTargeting};

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
    combat_feedback: CombatFeedback,
    observed_event_count: usize,
    phase_replay: PhaseReplay,
    deployment_formation: FormationKind,
    end_phase_armed: bool,
    gamepad: GamepadInput,
    gamepad_connected: bool,
    title_focus_continue: bool,
    title_focus_active: bool,
    title_hover_preview: bool,
    new_campaign_armed: bool,
    delete_save_armed: bool,
    tactical_camera: WorldCamera,
    colony_camera: WorldCamera,
    colony_explorer: crate::colony_exploration::ColonyExplorer,
    colony_operations_open: bool,
    facility_upgrade_open: bool,
    salvage_open: bool,
    audio: crate::audio::AudioSystem,
    show_settings: bool,
}

impl Game {
    pub fn update(&mut self, dt: f32) {
        if self.colony_explorer_can_update() {
            self.colony_explorer.update(dt, &self.campaign.colony);
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
            AppState::Colony => colony_ui::draw_colony(colony_ui::ColonyDrawContext {
                campaign: &self.campaign,
                data: &self.data,
                assets: &self.assets,
                visuals: &self.visuals,
                ui: &virtual_ui,
                camera: &mut self.colony_camera,
                explorer: &mut self.colony_explorer,
                operations_open: &mut self.colony_operations_open,
                facility_upgrade_open: &mut self.facility_upgrade_open,
                salvage_open: &mut self.salvage_open,
                settings_open: self.show_settings,
            }),
            AppState::Roster => crate::roster_ui::draw_roster(
                &self.campaign,
                &self.data,
                &self.assets,
                &self.visuals,
                &virtual_ui,
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

    fn apply_action(&mut self, action: UiAction) {
        if self.apply_audio_action(&action) {
            return;
        }
        if self.apply_first_hour_action(&action) {
            return;
        }
        if self.guard_destructive_action(&action) {
            return;
        }
        let audio_event_count = self.session.tactical.event_log.len();
        let audio_action = action.clone();
        if !matches!(&action, UiAction::EndPhase) {
            self.end_phase_armed = false;
        }
        if self.apply_skill_action(&action, audio_event_count) {
            return;
        }
        if self.apply_class_action(&action, audio_event_count) {
            return;
        }
        if self.apply_facility_upgrade_action(&action) {
            return;
        }
        if self.apply_salvage_action(&action) {
            return;
        }
        match action {
            UiAction::StartMission => {
                self.targeting = None;
                self.campaign = CampaignState::new(&self.data);
                self.active_mission = self
                    .campaign
                    .strategy
                    .materialize_selected(&self.data, &self.campaign.colony);
                self.colony_camera = WorldCamera::colony_start(crate::colony::SETTLEMENT_CENTER);
                self.colony_explorer.reset();
                self.colony_operations_open = false;
                self.facility_upgrade_open = false;
                self.salvage_open = false;
                self.state = AppState::Colony;
                self.last_outcome = None;
                self.autosave_campaign_only("New colony autosaved");
            }
            UiAction::OpenMissionBriefing => {
                self.campaign
                    .first_hour
                    .opened_briefing(self.campaign.operations_completed);
                self.normalize_deployment_formation();
                self.active_mission = self
                    .campaign
                    .strategy
                    .materialize_selected(&self.data, &self.campaign.colony);
                self.state = AppState::MissionBriefing;
            }
            UiAction::SelectMission(mission_id) => {
                match self.campaign.strategy.select_mission(&mission_id) {
                    Ok(()) => {
                        self.active_mission = self
                            .campaign
                            .strategy
                            .materialize_selected(&self.data, &self.campaign.colony);
                        self.notifications.success("Mission selected");
                        self.autosave_campaign_only("Mission selection autosaved");
                    }
                    Err(err) => self.notifications.warning(err),
                }
            }
            UiAction::CompleteResearch(research_id) => {
                match self.campaign.complete_research(&research_id) {
                    Ok(name) => {
                        self.campaign.first_hour.invested(name.clone());
                        self.notifications
                            .success(format!("Research complete: {}", name));
                        self.autosave_campaign_only("Research autosaved");
                    }
                    Err(err) => self.notifications.warning(err),
                }
            }
            UiAction::ChooseContactProtocol(protocol_id) => {
                match self.campaign.strategy.choose_contact_protocol(
                    &protocol_id,
                    &mut self.campaign.colony,
                    &self.data,
                ) {
                    Ok(name) => {
                        self.notifications
                            .success(format!("Contact protocol active: {}", name));
                        self.autosave_campaign_only("Contact protocol autosaved");
                    }
                    Err(err) => self.notifications.warning(err),
                }
            }
            UiAction::ChooseEscalationResponse(response_id) => {
                match self.campaign.strategy.choose_escalation_response(
                    &response_id,
                    &mut self.campaign.colony,
                    &self.data,
                ) {
                    Ok(name) => {
                        self.notifications
                            .success(format!("Escalation response committed: {}", name));
                        self.autosave_campaign_only("Escalation response autosaved");
                    }
                    Err(err) => self.notifications.warning(err),
                }
            }
            UiAction::ChooseMirexisPath(path_id) => {
                match self.campaign.strategy.choose_mirexis_path(
                    &path_id,
                    &mut self.campaign.colony,
                    &self.data,
                ) {
                    Ok(name) => {
                        self.notifications
                            .success(format!("Mirexis path committed: {}", name));
                        self.autosave_campaign_only("Mirexis path autosaved");
                    }
                    Err(err) => self.notifications.warning(err),
                }
            }
            UiAction::ChooseMutationEvolution(character_id, evolution_id) => {
                match self.campaign.choose_mutation_evolution(
                    &character_id,
                    &evolution_id,
                    &self.data,
                ) {
                    Ok(name) => {
                        self.notifications
                            .success(format!("Mutation evolved: {}", name));
                        self.autosave_campaign_only("Mutation evolution autosaved");
                    }
                    Err(err) => self.notifications.warning(err),
                }
            }
            UiAction::ResolveCharacterEvent(character_id) => {
                match self
                    .campaign
                    .resolve_first_character_event_for(&character_id, &self.data)
                {
                    Ok(title) => {
                        self.notifications
                            .success(format!("Event resolved: {}", title));
                        self.autosave_campaign_only("Character event autosaved");
                    }
                    Err(err) => self.notifications.warning(err),
                }
            }
            UiAction::RecruitOutsider => match self.campaign.recruit_outsider(&self.data) {
                Ok(name) => {
                    self.notifications
                        .success(format!("{} joined the colony roster", name));
                    self.autosave_campaign_only("Outsider recruitment autosaved");
                }
                Err(err) => self.notifications.warning(err),
            },
            UiAction::ResolveOutsiderBeat(stage, choice_id) => {
                match self.campaign.resolve_outsider_beat(stage, &choice_id) {
                    Ok(summary) => {
                        self.notifications.success(summary);
                        self.autosave_campaign_only("Outsider arc autosaved");
                    }
                    Err(err) => self.notifications.warning(err),
                }
            }
            UiAction::HostCommonsMeal => self.handle_commons_meal(),
            UiAction::RunRelayScan => self.handle_relay_scan(),
            UiAction::RunIdentityStewardship => self.handle_identity_stewardship(),
            UiAction::PrepareIdentityBuilding => self.handle_identity_preparation(),
            UiAction::DeployMission => match self.campaign.prepare_deployment(&self.data) {
                Ok(food_cost) => {
                    self.campaign
                        .first_hour
                        .deployed(self.campaign.operations_completed);
                    self.targeting = None;
                    self.show_tactical_help = false;
                    self.show_battle_log = false;
                    self.normalize_deployment_formation();
                    let mut roster = self
                        .campaign
                        .deployment_roster(&self.data, &self.active_mission);
                    crate::formation::apply(
                        &mut roster,
                        &self.active_mission,
                        &self.data.config,
                        self.deployment_formation,
                    );
                    self.session = self.create_first_hour_session(&roster);
                    self.tactical_camera =
                        WorldCamera::tactical_start(self.session.tactical.selected_tile);
                    self.state = AppState::Tactical;
                    self.autosave_current("Deployment autosaved");
                    self.notifications.success(format!(
                        "{} deployed · {} food committed",
                        self.active_mission.name, food_cost
                    ));
                }
                Err(err) => self.notifications.warning(err),
            },
            UiAction::CycleFormation => self.cycle_deployment_formation(),
            UiAction::ToggleDeployment(character_id) => {
                let _ = self.campaign.select_character(&character_id);
                match self.campaign.toggle_deployment(&character_id) {
                    Ok(selected) => {
                        self.notifications.info(if selected {
                            "Colonist assigned to the deployment squad"
                        } else {
                            "Colonist moved to reserve"
                        });
                        self.autosave_campaign_only("Squad selection autosaved");
                    }
                    Err(err) => self.notifications.warning(err),
                }
            }
            UiAction::Continue => self.load_game(),
            UiAction::ReturnToTitle => {
                self.targeting = None;
                self.show_tactical_help = false;
                self.show_battle_log = false;
                self.facility_upgrade_open = false;
                self.salvage_open = false;
                self.state = AppState::Title;
            }
            UiAction::ReturnToColony => {
                self.targeting = None;
                self.facility_upgrade_open = false;
                self.salvage_open = false;
                self.state = AppState::Colony;
                self.ensure_first_hour_recovery_reserve();
                self.campaign.first_hour.returned_to_colony();
                self.autosave_campaign_only("Colony entry autosaved");
            }
            UiAction::OpenRoster => self.state = AppState::Roster,
            UiAction::OpenGeneLab => {
                if self
                    .campaign
                    .colony
                    .has_facility(crate::colony::BuildingKind::GeneLab)
                {
                    self.state = AppState::GeneLab;
                } else {
                    self.notifications.warning("The Gene Lab is offline");
                }
            }
            UiAction::SelectColonist(character_id) => {
                match self.campaign.select_character(&character_id) {
                    Ok(()) => self.autosave_campaign_only("Roster selection autosaved"),
                    Err(err) => self.notifications.warning(err),
                }
            }
            UiAction::SelectConstruction(kind) => {
                if kind == crate::colony::BuildingKind::GeneLab
                    && !self.campaign.strategy.contact_complete
                {
                    self.notifications
                        .warning("The Gene Lab unlocks in Adaptation");
                    return;
                }
                if kind == crate::colony::BuildingKind::Waystation
                    && !self.campaign.waystation_unlocked()
                {
                    self.notifications
                        .warning("The Waystation unlocks through Contact or Adaptation");
                    return;
                }
                match self.campaign.colony.select_construction(kind) {
                    Ok(()) => self.notifications.info(format!("Planning {}", kind.name())),
                    Err(err) => self.notifications.warning(err),
                }
            }
            UiAction::ConstructBuilding(kind, position) => {
                if kind == crate::colony::BuildingKind::GeneLab
                    && !self.campaign.strategy.contact_complete
                {
                    self.notifications
                        .warning("The Gene Lab unlocks in Adaptation");
                    return;
                }
                if kind == crate::colony::BuildingKind::Waystation
                    && !self.campaign.waystation_unlocked()
                {
                    self.notifications
                        .warning("The Waystation unlocks through Contact or Adaptation");
                    return;
                }
                match self.campaign.colony.place_construction(kind, position) {
                    Ok(_) => {
                        self.campaign.first_hour.invested(kind.name());
                        self.notifications
                            .success(format!("{} construction planned", kind.name()));
                        self.autosave_campaign_only("Construction plan autosaved");
                    }
                    Err(err) => self.notifications.warning(err),
                }
            }
            UiAction::RepairBuilding(building_id) => {
                match self.campaign.colony.repair_building(&building_id) {
                    Ok((name, cost)) => {
                        self.notifications
                            .success(format!("{} repaired for {} materials", name, cost));
                        self.autosave_campaign_only("Colony repair autosaved");
                    }
                    Err(err) => self.notifications.warning(err),
                }
            }
            UiAction::TrainSelected(class_id) => {
                let character_id = self.campaign.selected_character_id.clone();
                match self
                    .campaign
                    .train_character(&character_id, &class_id, &self.data)
                {
                    Ok(cost) => {
                        self.campaign.first_hour.invested("squad training");
                        self.notifications
                            .success(format!("Training complete · {} materials", cost));
                        self.autosave_campaign_only("Training autosaved");
                    }
                    Err(err) => self.notifications.warning(err),
                }
            }
            UiAction::TreatInjury => match self.campaign.treat_first_injury() {
                Ok(name) => {
                    self.campaign.first_hour.invested("priority treatment");
                    self.notifications
                        .success(format!("{} received priority treatment", name));
                    self.autosave_campaign_only("Treatment autosaved");
                }
                Err(err) => self.notifications.warning(err),
            },
            UiAction::CraftSelected(equipment_id) => {
                let character_id = self.campaign.selected_character_id.clone();
                match self
                    .campaign
                    .craft_equipment(&character_id, &equipment_id, &self.data)
                {
                    Ok(cost) => {
                        self.campaign.first_hour.invested("field equipment");
                        self.notifications.success(if cost == 0 {
                            "Equipment issued · salvage prototype consumed".to_owned()
                        } else {
                            format!("Equipment issued · {} materials", cost)
                        });
                        self.autosave_campaign_only("Workshop change autosaved");
                    }
                    Err(err) => self.notifications.warning(err),
                }
            }
            UiAction::SelectTile(tile) => {
                self.session.select_tile(tile);
                if self
                    .session
                    .selected_unit()
                    .is_some_and(|unit| unit.position == tile)
                {
                    self.campaign.first_hour.selected();
                }
            }
            UiAction::SelectNextReady => {
                if crate::phase_readiness::select_next(&mut self.session).is_none() {
                    self.notifications.info("No colonist has actions remaining");
                }
            }
            UiAction::MoveSelected(tile) => {
                if self.session.move_selected_to(tile) {
                    self.record_first_hour_move(tile);
                    self.notifications.info("Colonist repositioned");
                } else {
                    self.notifications
                        .warning("Tile is outside the valid movement envelope");
                }
            }
            UiAction::AttackSelected(target_id) => match self.session.attack_selected(&target_id) {
                Ok(events) => {
                    self.campaign.first_hour.attacked();
                    self.notifications.info(format!(
                        "Attack resolved — {} tactical events",
                        events.len()
                    ));
                }
                Err(_) => self.notifications.warning("No valid firing solution"),
            },
            UiAction::AttackCover(position) => match self.session.attack_selected_cover(position) {
                Ok(events) => self.notifications.info(
                    events
                        .last()
                        .map(crate::ui_widgets::event_summary)
                        .unwrap_or_else(|| "Cover struck".to_owned()),
                ),
                Err(_) => self
                    .notifications
                    .warning("Cover is outside the firing solution"),
            },
            UiAction::InteractObjective => self.handle_objective_interaction(),
            UiAction::ActivateMutation => match self.session.activate_selected_mutation() {
                Ok(events) => {
                    self.campaign.first_hour.used_ability();
                    self.notifications.success(
                        events
                            .first()
                            .map(crate::ui_widgets::event_summary)
                            .unwrap_or_else(|| "Mutation gift activated".to_owned()),
                    );
                }
                Err(_) => self.notifications.warning("Mutation gift is unavailable"),
            },
            UiAction::SetOverwatch => match self.session.set_selected_overwatch() {
                Ok(_) => self
                    .notifications
                    .success("Overwatch armed · first hostile movement in range draws fire"),
                Err(_) => self
                    .notifications
                    .warning("Selected colonist cannot enter overwatch"),
            },
            UiAction::ArmEquipment(equipment_id) => {
                if let Some(unit_id) = self.session.tactical.selected_unit.clone() {
                    self.targeting = Some(TacticalTargeting::Equipment {
                        unit_id,
                        equipment_id,
                    });
                    self.notifications
                        .info("Choose a highlighted equipment target");
                }
            }
            UiAction::CancelTargeting => {
                self.targeting = None;
                self.notifications.info("Tactical targeting cancelled");
            }
            UiAction::UseEquipmentOn(target_id) => {
                let result = match self.targeting.take() {
                    Some(TacticalTargeting::Equipment {
                        unit_id,
                        equipment_id,
                    }) => self
                        .session
                        .use_equipment(&unit_id, &equipment_id, &target_id)
                        .map_err(|_| ()),
                    _ => Err(()),
                };
                match result {
                    Ok(events) => self.first_hour_ability_success(events, "Field equipment used"),
                    Err(()) => self
                        .notifications
                        .warning("Equipment target is no longer valid"),
                }
            }
            UiAction::EndPhase => {
                let ready = crate::phase_readiness::ready_count(&self.session);
                if ready > 0 && !self.end_phase_armed {
                    self.end_phase_armed = true;
                    self.notifications.warning(format!(
                        "{} colonist{} still ready · end phase again to confirm",
                        ready,
                        if ready == 1 { "" } else { "s" }
                    ));
                    return;
                }
                self.end_phase_armed = false;
                self.targeting = None;
                let first_event = self.session.tactical.event_log.len();
                self.session.end_player_phase(&self.data.config);
                self.campaign.first_hour.ended_phase();
                self.phase_replay
                    .start(&self.session.tactical.event_log[first_event..]);
                self.notifications.info(format!(
                    "Enemy activity resolved — round {}",
                    self.session.tactical.round
                ));
            }
            UiAction::SkipPhaseReplay => self.phase_replay.clear(),
            UiAction::ToggleTacticalHelp => {
                self.show_tactical_help = !self.show_tactical_help;
                self.show_battle_log = false;
                self.targeting = None;
            }
            UiAction::ToggleBattleLog => {
                self.show_battle_log = !self.show_battle_log;
                self.show_tactical_help = false;
                self.targeting = None;
            }
            UiAction::SetBattleLogFilter(filter) => self.battle_log_filter = filter,
            UiAction::Save => self.save_game(),
            UiAction::Load => self.load_game(),
            _ => unreachable!("prehandled application action reached the game state match"),
        }
        self.finish_action_audio(&audio_action, audio_event_count);
        self.enter_debrief_if_finished();
    }
}
