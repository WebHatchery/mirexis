//! Application state machine, persistence, and toolkit integration.

mod capture_colony;
mod capture_debrief;
mod capture_reset;
mod capture_scenes;
mod capture_tactical;
mod input;
mod persistence_io;

use crate::campaign::CampaignState;
use crate::colony_ui;
use crate::combat_feedback::CombatFeedback;
use crate::data::{GameData, MissionDef};
use crate::formation::FormationKind;
use crate::grid_ui::WorldCamera;
use crate::phase_replay::PhaseReplay;
use crate::state::{GameSession, MissionOutcome};
use crate::ui::{self, TargetingView, UiAction, UiContext};
use crate::visual_assets::VisualCatalog;
use macroquad::prelude::*;
use macroquad_toolkit::assets::AssetManager;
use macroquad_toolkit::events::EventBus;
use macroquad_toolkit::notifications::{
    NotificationAnchor, NotificationManager, NotificationRenderConfig,
};
use macroquad_toolkit::persistence::{slot_exists, AutoSaveManager};
use macroquad_toolkit::prelude::{
    begin_virtual_ui_frame, dark, end_virtual_ui_frame, GamepadInput,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AppState {
    Title,
    Colony,
    Roster,
    GeneLab,
    MissionBriefing,
    Tactical,
    Debrief,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum TacticalTargeting {
    Equipment {
        unit_id: String,
        equipment_id: String,
    },
    ClassAction {
        unit_id: String,
    },
}

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
    tactical_camera: WorldCamera,
    colony_camera: WorldCamera,
}

impl Game {
    pub async fn new() -> Self {
        let data = GameData::load()
            .unwrap_or_else(|err| panic!("Mirexis embedded data failed to load: {}", err));
        let mut assets = AssetManager::new();
        let placeholder = crate::visual_assets::diagnostic_placeholder_image(32);
        assets.set_placeholder_texture_direct(Texture2D::from_image(&placeholder));
        if let Err(error) = assets.load_asset_pack("assets.zip").await {
            println!(
                "Mirexis runtime asset pack unavailable ({error}); loading loose assets for native development."
            );
        }
        let loaded_assets = assets.load_texture_configs(&data.texture_manifest).await;
        let visuals = VisualCatalog::load();
        let missing_visuals = visuals.validate_loaded(&assets);
        let visual_summary = visuals.diagnostic_summary(&assets);
        println!("Mirexis visual catalog: {visual_summary}");
        let campaign = CampaignState::new(&data);
        let active_mission = campaign
            .strategy
            .materialize_selected(&data, &campaign.colony);
        let session = GameSession::new(
            &data.config,
            &active_mission,
            &campaign.deployment_roster(&data, &active_mission),
        );
        let save_exists = slot_exists(&data.config.game_name, &data.config.save_slot);
        let mut notifications = NotificationManager::new();
        notifications.info(format!(
            "Visual pipeline online; {} textures; {} sprite definitions",
            loaded_assets,
            visuals.units.len()
        ));
        if !missing_visuals.is_empty() {
            notifications.danger(format!(
                "MISSING VISUAL ASSETS: {}",
                missing_visuals.join(", ")
            ));
        }

        let tactical_camera = WorldCamera::tactical_start(session.tactical.selected_tile);
        let colony_camera = WorldCamera::colony_start(crate::colony::SETTLEMENT_CENTER);
        Self {
            data,
            session,
            campaign,
            active_mission,
            state: AppState::Title,
            assets,
            visuals,
            notifications,
            events: EventBus::new(),
            save_exists,
            last_outcome: None,
            autosave: AutoSaveManager::default(),
            targeting: None,
            show_tactical_help: false,
            show_battle_log: false,
            combat_feedback: CombatFeedback::default(),
            observed_event_count: 0,
            phase_replay: PhaseReplay::default(),
            deployment_formation: FormationKind::default(),
            end_phase_armed: false,
            gamepad: GamepadInput::new(),
            gamepad_connected: false,
            title_focus_continue: false,
            title_focus_active: false,
            title_hover_preview: false,
            tactical_camera,
            colony_camera,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.session.tactical.update_presentation(dt);
        self.notifications.update(dt);
        self.combat_feedback.update(dt);
        self.phase_replay.update(dt);
        if self.capture_input() {
            return;
        }

        let actions: Vec<_> = self.events.drain().collect();
        for action in actions {
            self.apply_action(action);
        }
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
        let actions = match self.state {
            AppState::Title => ui::draw_title(
                &self.data,
                self.save_exists,
                &self.assets,
                &self.visuals,
                &virtual_ui,
                self.title_focus_active.then_some(self.title_focus_continue),
                self.title_hover_preview,
            ),
            AppState::Colony => colony_ui::draw_colony(
                &self.campaign,
                &self.data,
                &self.assets,
                &self.visuals,
                &virtual_ui,
                &mut self.colony_camera,
            ),
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
                    loaded_assets: self.assets.len(),
                    ui: &virtual_ui,
                    targeting: self.targeting.as_ref().map(|targeting| match targeting {
                        TacticalTargeting::Equipment {
                            unit_id,
                            equipment_id,
                        } => TargetingView::Equipment {
                            unit_id,
                            equipment_id,
                        },
                        TacticalTargeting::ClassAction { unit_id } => {
                            TargetingView::ClassAction { unit_id }
                        }
                    }),
                    show_help: self.show_tactical_help,
                    show_battle_log: self.show_battle_log,
                },
                &mut self.tactical_camera,
            ),
            AppState::Debrief => crate::ui_debrief::draw_debrief(
                &self.active_mission,
                self.last_outcome
                    .as_ref()
                    .expect("debrief requires an outcome"),
                &self.campaign,
                &self.assets,
                &self.visuals,
                &virtual_ui,
            ),
        };
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
        if !matches!(&action, UiAction::EndPhase) {
            self.end_phase_armed = false;
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
                self.state = AppState::Colony;
                self.last_outcome = None;
                self.autosave_campaign_only("New colony autosaved");
            }
            UiAction::OpenMissionBriefing => {
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
                match self
                    .campaign
                    .strategy
                    .complete_research(&research_id, &mut self.campaign.colony)
                {
                    Ok(name) => {
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
            UiAction::DeployMission => match self.campaign.prepare_deployment(&self.data) {
                Ok(food_cost) => {
                    self.targeting = None;
                    self.show_tactical_help = false;
                    self.show_battle_log = false;
                    let mut roster = self
                        .campaign
                        .deployment_roster(&self.data, &self.active_mission);
                    crate::formation::apply(
                        &mut roster,
                        &self.active_mission,
                        &self.data.config,
                        self.deployment_formation,
                    );
                    self.session =
                        GameSession::new(&self.data.config, &self.active_mission, &roster);
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
            UiAction::CycleFormation => {
                self.deployment_formation = self.deployment_formation.next();
                self.notifications.info(format!(
                    "Deployment formation: {}",
                    self.deployment_formation.label()
                ));
            }
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
                self.state = AppState::Title;
            }
            UiAction::ReturnToColony => {
                self.targeting = None;
                self.state = AppState::Colony;
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
                match self.campaign.colony.place_construction(kind, position) {
                    Ok(_) => {
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
                        self.notifications
                            .success(format!("Training complete · {} materials", cost));
                        self.autosave_campaign_only("Training autosaved");
                    }
                    Err(err) => self.notifications.warning(err),
                }
            }
            UiAction::TreatInjury => match self.campaign.treat_first_injury() {
                Ok(name) => {
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
                        self.notifications
                            .success(format!("Equipment issued · {} materials", cost));
                        self.autosave_campaign_only("Workshop change autosaved");
                    }
                    Err(err) => self.notifications.warning(err),
                }
            }
            UiAction::SelectTile(tile) => self.session.select_tile(tile),
            UiAction::SelectNextReady => {
                if crate::phase_readiness::select_next(&mut self.session).is_none() {
                    self.notifications.info("No colonist has actions remaining");
                }
            }
            UiAction::MoveSelected(tile) => {
                if self.session.move_selected_to(tile) {
                    self.notifications.info("Colonist repositioned");
                } else {
                    self.notifications
                        .warning("Tile is outside the valid movement envelope");
                }
            }
            UiAction::AttackSelected(target_id) => match self.session.attack_selected(&target_id) {
                Ok(events) => self.notifications.info(format!(
                    "Attack resolved — {} tactical events",
                    events.len()
                )),
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
            UiAction::InteractObjective => match self.session.interact_selected() {
                Ok(_) => self.notifications.success("Mission objective secured"),
                Err(_) => self
                    .notifications
                    .warning("A colonist must reach the objective"),
            },
            UiAction::ActivateMutation => match self.session.activate_selected_mutation() {
                Ok(events) => self.notifications.success(
                    events
                        .first()
                        .map(crate::ui_widgets::event_summary)
                        .unwrap_or_else(|| "Mutation gift activated".to_owned()),
                ),
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
            UiAction::ActivateClassAction => match self.session.activate_selected_class_action() {
                Ok(events) => self.notifications.success(
                    events
                        .first()
                        .map(crate::ui_widgets::event_summary)
                        .unwrap_or_else(|| "Class action activated".to_owned()),
                ),
                Err(_) => self.notifications.warning("Class action is unavailable"),
            },
            UiAction::ArmClassAction => {
                if let Some(unit_id) = self.session.tactical.selected_unit.clone() {
                    self.targeting = Some(TacticalTargeting::ClassAction { unit_id });
                    self.notifications
                        .info("Choose a highlighted class-action target");
                }
            }
            UiAction::UseClassActionOn(target_id) => {
                let result = match self.targeting.take() {
                    Some(TacticalTargeting::ClassAction { unit_id }) => self
                        .session
                        .activate_class_action_on(&unit_id, &target_id)
                        .map_err(|_| ()),
                    _ => Err(()),
                };
                match result {
                    Ok(events) => self.notifications.success(
                        events
                            .first()
                            .map(crate::ui_widgets::event_summary)
                            .unwrap_or_else(|| "Class action activated".to_owned()),
                    ),
                    Err(()) => self
                        .notifications
                        .warning("Class-action target is no longer valid"),
                }
            }
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
                    Ok(events) => self.notifications.success(
                        events
                            .first()
                            .map(crate::ui_widgets::event_summary)
                            .unwrap_or_else(|| "Field equipment used".to_owned()),
                    ),
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
            UiAction::Save => self.save_game(),
            UiAction::Load => self.load_game(),
            UiAction::DeleteSave => self.delete_save(),
        }
        self.enter_debrief_if_finished();
    }

    fn enter_debrief_if_finished(&mut self) {
        if self.state != AppState::Tactical {
            return;
        }
        if let Some(outcome) = self.session.mission_outcome(&self.active_mission) {
            self.last_outcome = Some(outcome);
            self.campaign.advance_recovery();
            let mission = self
                .campaign
                .strategy
                .selected_mission()
                .expect("a deployed mission remains selected")
                .clone();
            self.campaign.apply_mission_outcome(
                self.last_outcome.as_ref().expect("outcome was just stored"),
                &mission,
                &self.data,
            );
            self.state = AppState::Debrief;
            self.autosave_current("Debrief autosaved");
        }
    }
}
