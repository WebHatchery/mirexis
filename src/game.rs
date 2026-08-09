//! Application state machine, persistence, and toolkit integration.

mod capture_scenes;

use crate::campaign::CampaignState;
use crate::colony_ui;
use crate::data::{GameData, MissionDef};
use crate::persistence::migrate_save_value;
use crate::state::{GameSession, MissionOutcome, SaveData};
use crate::ui::{self, TargetingView, UiAction, UiContext};
use macroquad::prelude::*;
use macroquad_toolkit::assets::AssetManager;
use macroquad_toolkit::events::EventBus;
use macroquad_toolkit::notifications::{
    NotificationAnchor, NotificationManager, NotificationRenderConfig,
};
use macroquad_toolkit::persistence::{
    delete_slot, load_from_slot_with_migration, save_to_slot_with_version, slot_exists,
    AutoSaveManager,
};
use macroquad_toolkit::prelude::{begin_virtual_ui_frame, dark, end_virtual_ui_frame, InputState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AppState {
    Title,
    Colony,
    Roster,
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
    notifications: NotificationManager,
    events: EventBus<UiAction>,
    save_exists: bool,
    last_outcome: Option<MissionOutcome>,
    autosave: AutoSaveManager,
    targeting: Option<TacticalTargeting>,
}

impl Game {
    pub async fn new() -> Self {
        let data = GameData::load()
            .unwrap_or_else(|err| panic!("Mirexis embedded data failed to load: {}", err));
        let mut assets = AssetManager::new();
        let placeholder = Image::gen_image_color(16, 16, Color::new(0.20, 0.78, 0.58, 1.0));
        assets.set_placeholder_texture_direct(Texture2D::from_image(&placeholder));
        let _ = assets.load_asset_pack("assets.zip").await;
        let loaded_assets = assets.load_texture_configs(&data.texture_manifest).await;
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
            "Mission systems online; {} manifest textures loaded",
            loaded_assets
        ));

        Self {
            data,
            session,
            campaign,
            active_mission,
            state: AppState::Title,
            assets,
            notifications,
            events: EventBus::new(),
            save_exists,
            last_outcome: None,
            autosave: AutoSaveManager::default(),
            targeting: None,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.notifications.update(dt);
        let input = InputState::capture();
        match self.state {
            AppState::Title => {
                if input.space_pressed || is_key_pressed(KeyCode::Enter) {
                    self.events.push(UiAction::StartMission);
                }
            }
            AppState::Colony => {
                if input.escape_pressed {
                    self.events.push(UiAction::ReturnToTitle);
                }
            }
            AppState::Roster => {
                if input.escape_pressed {
                    self.events.push(UiAction::ReturnToColony);
                }
            }
            AppState::MissionBriefing => {
                if input.escape_pressed {
                    self.events.push(UiAction::ReturnToColony);
                }
                if input.space_pressed || is_key_pressed(KeyCode::Enter) {
                    self.events.push(UiAction::DeployMission);
                }
            }
            AppState::Tactical => {
                if input.escape_pressed {
                    self.events.push(if self.targeting.is_some() {
                        UiAction::CancelTargeting
                    } else {
                        UiAction::ReturnToTitle
                    });
                }
                if is_key_pressed(KeyCode::S) {
                    self.events.push(UiAction::Save);
                }
                if is_key_pressed(KeyCode::L) {
                    self.events.push(UiAction::Load);
                }
                if is_key_pressed(KeyCode::Enter) {
                    self.events.push(UiAction::EndPhase);
                }
                if self.targeting.is_none() {
                    if let Some((dx, dy)) = ui::tile_move_from_keys() {
                        self.session.move_selection(dx, dy);
                    }
                }
            }
            AppState::Debrief => {
                if input.space_pressed || is_key_pressed(KeyCode::Enter) {
                    self.events.push(UiAction::ReturnToColony);
                }
            }
        }

        let actions: Vec<_> = self.events.drain().collect();
        for action in actions {
            self.apply_action(action);
        }
    }

    pub fn draw(&mut self) {
        clear_background(dark::BACKGROUND);
        let virtual_ui = begin_virtual_ui_frame(ui::LOGICAL_WIDTH, ui::LOGICAL_HEIGHT);
        let actions = match self.state {
            AppState::Title => ui::draw_title(&self.data, self.save_exists, &virtual_ui),
            AppState::Colony => colony_ui::draw_colony(&self.campaign, &self.data, &virtual_ui),
            AppState::Roster => {
                crate::roster_ui::draw_roster(&self.campaign, &self.data, &virtual_ui)
            }
            AppState::MissionBriefing => ui::draw_mission_briefing(
                &self.data,
                &self.campaign,
                &self.active_mission,
                &virtual_ui,
            ),
            AppState::Tactical => ui::draw_tactical(UiContext {
                data: &self.data,
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
            }),
            AppState::Debrief => ui::draw_debrief(
                &self.active_mission,
                self.last_outcome
                    .as_ref()
                    .expect("debrief requires an outcome"),
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
        match action {
            UiAction::StartMission => {
                self.targeting = None;
                self.campaign = CampaignState::new(&self.data);
                self.active_mission = self
                    .campaign
                    .strategy
                    .materialize_selected(&self.data, &self.campaign.colony);
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
            UiAction::ResolveCharacterEvent => {
                match self.campaign.resolve_first_character_event(&self.data) {
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
                    self.session = GameSession::new(
                        &self.data.config,
                        &self.active_mission,
                        &self
                            .campaign
                            .deployment_roster(&self.data, &self.active_mission),
                    );
                    self.state = AppState::Tactical;
                    self.autosave_current("Deployment autosaved");
                    self.notifications.success(format!(
                        "{} deployed · {} food committed",
                        self.active_mission.name, food_cost
                    ));
                }
                Err(err) => self.notifications.warning(err),
            },
            UiAction::ToggleDeployment(character_id) => {
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
                self.state = AppState::Title;
            }
            UiAction::ReturnToColony => {
                self.targeting = None;
                self.state = AppState::Colony;
                self.autosave_campaign_only("Colony entry autosaved");
            }
            UiAction::OpenRoster => self.state = AppState::Roster,
            UiAction::SelectColonist(character_id) => {
                match self.campaign.select_character(&character_id) {
                    Ok(()) => self.autosave_campaign_only("Roster selection autosaved"),
                    Err(err) => self.notifications.warning(err),
                }
            }
            UiAction::SelectConstruction(kind) => {
                match self.campaign.colony.select_construction(kind) {
                    Ok(()) => self.notifications.info(format!("Planning {}", kind.name())),
                    Err(err) => self.notifications.warning(err),
                }
            }
            UiAction::ConstructBuilding(kind, position) => {
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
                self.targeting = None;
                self.session.end_player_phase(&self.data.config);
                self.notifications.info(format!(
                    "Enemy activity resolved — round {}",
                    self.session.tactical.round
                ));
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

    fn autosave_current(&mut self, success_message: &str) {
        let save = self
            .session
            .to_save(&self.data.config.version, &self.campaign);
        let game_name = self.data.config.game_name.clone();
        let slot = self.data.config.save_slot.clone();
        let version = self.data.config.version.clone();
        match self
            .autosave
            .force(move || save_to_slot_with_version(&game_name, &slot, &save, &version))
        {
            Ok(()) => {
                self.save_exists = true;
                self.notifications.info(success_message);
            }
            Err(err) => self
                .notifications
                .danger(format!("Autosave failed: {}", err)),
        }
    }

    fn autosave_campaign_only(&mut self, success_message: &str) {
        let save = SaveData::campaign_only(&self.data.config.version, &self.campaign);
        let game_name = self.data.config.game_name.clone();
        let slot = self.data.config.save_slot.clone();
        let version = self.data.config.version.clone();
        match self
            .autosave
            .force(move || save_to_slot_with_version(&game_name, &slot, &save, &version))
        {
            Ok(()) => {
                self.save_exists = true;
                self.notifications.info(success_message);
            }
            Err(err) => self
                .notifications
                .danger(format!("Autosave failed: {}", err)),
        }
    }

    fn save_game(&mut self) {
        let save = self
            .session
            .to_save(&self.data.config.version, &self.campaign);
        match save_to_slot_with_version(
            &self.data.config.game_name,
            &self.data.config.save_slot,
            &save,
            &self.data.config.version,
        ) {
            Ok(()) => {
                self.save_exists = true;
                self.notifications.success("Tactical state saved");
            }
            Err(err) => self.notifications.danger(format!("Save failed: {}", err)),
        }
    }

    fn load_game(&mut self) {
        let loaded: Result<SaveData, String> = load_from_slot_with_migration(
            &self.data.config.game_name,
            &self.data.config.save_slot,
            &self.data.config.version,
            |version, value| migrate_save_value(version, value, &self.data),
        );
        match loaded {
            Ok(save) => {
                self.targeting = None;
                self.campaign = save.campaign;
                self.active_mission = self
                    .campaign
                    .strategy
                    .materialize_selected(&self.data, &self.campaign.colony);
                if let Some(tactical) = save.tactical {
                    self.session = GameSession::from_tactical(tactical);
                    self.state = if self.session.battle_is_over() {
                        AppState::Colony
                    } else {
                        AppState::Tactical
                    };
                } else {
                    self.session = GameSession::new(
                        &self.data.config,
                        &self.active_mission,
                        &self
                            .campaign
                            .deployment_roster(&self.data, &self.active_mission),
                    );
                    self.state = AppState::Colony;
                }
                self.last_outcome = None;
                self.notifications.success("Tactical state restored");
            }
            Err(err) => self.notifications.warning(format!("Load failed: {}", err)),
        }
    }

    fn delete_save(&mut self) {
        match delete_slot(&self.data.config.game_name, &self.data.config.save_slot) {
            Ok(()) => {
                self.save_exists = false;
                self.notifications.info("Save slot cleared");
            }
            Err(err) => self.notifications.danger(format!("Delete failed: {}", err)),
        }
    }
}
