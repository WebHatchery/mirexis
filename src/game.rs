//! Application state machine, persistence, and toolkit integration.

use crate::campaign::CampaignState;
use crate::data::GameData;
use crate::persistence::migrate_save_value;
use crate::state::{GameSession, MissionOutcome, SaveData};
use crate::ui::{self, UiAction, UiContext};
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
    MissionBriefing,
    Tactical,
    Debrief,
}

pub struct Game {
    data: GameData,
    session: GameSession,
    campaign: CampaignState,
    state: AppState,
    assets: AssetManager,
    notifications: NotificationManager,
    events: EventBus<UiAction>,
    save_exists: bool,
    last_outcome: Option<MissionOutcome>,
    autosave: AutoSaveManager,
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
        let session = GameSession::new(
            &data.config,
            &data.mission,
            &campaign.deployment_roster(&data),
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
            state: AppState::Title,
            assets,
            notifications,
            events: EventBus::new(),
            save_exists,
            last_outcome: None,
            autosave: AutoSaveManager::default(),
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
            AppState::MissionBriefing => {
                if input.escape_pressed {
                    self.events.push(UiAction::ReturnToTitle);
                }
                if input.space_pressed || is_key_pressed(KeyCode::Enter) {
                    self.events.push(UiAction::DeployMission);
                }
            }
            AppState::Tactical => {
                if input.escape_pressed {
                    self.events.push(UiAction::ReturnToTitle);
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
                if let Some((dx, dy)) = ui::tile_move_from_keys() {
                    self.session.move_selection(dx, dy);
                }
            }
            AppState::Debrief => {
                if input.space_pressed || is_key_pressed(KeyCode::Enter) {
                    self.events.push(UiAction::ReturnToTitle);
                }
            }
        }

        let actions: Vec<_> = self.events.drain().collect();
        for action in actions {
            self.apply_action(action);
        }
    }

    pub fn begin_capture_scene(&mut self, scene: &str) {
        match scene {
            "title" => self.state = AppState::Title,
            "briefing" => self.state = AppState::MissionBriefing,
            "debrief" => {
                self.session = GameSession::new(
                    &self.data.config,
                    &self.data.mission,
                    &self.campaign.deployment_roster(&self.data),
                );
                self.session.tactical.objective_state = crate::state::ObjectiveState::Victory;
                for unit in &mut self.session.tactical.units {
                    if unit.team == crate::data::Team::Hostile {
                        unit.incapacitated = true;
                        unit.health = 0;
                    }
                }
                self.last_outcome = self.session.mission_outcome(&self.data.mission);
                self.state = AppState::Debrief;
            }
            _ => {
                self.session = GameSession::new(
                    &self.data.config,
                    &self.data.mission,
                    &self.campaign.deployment_roster(&self.data),
                );
                self.state = AppState::Tactical;
            }
        }
    }

    pub fn draw(&mut self) {
        clear_background(dark::BACKGROUND);
        let virtual_ui = begin_virtual_ui_frame(ui::LOGICAL_WIDTH, ui::LOGICAL_HEIGHT);
        let actions = match self.state {
            AppState::Title => ui::draw_title(&self.data, self.save_exists, &virtual_ui),
            AppState::MissionBriefing => {
                ui::draw_mission_briefing(&self.data, &self.campaign, &virtual_ui)
            }
            AppState::Tactical => ui::draw_tactical(UiContext {
                data: &self.data,
                session: &self.session,
                save_exists: self.save_exists,
                loaded_assets: self.assets.len(),
                ui: &virtual_ui,
            }),
            AppState::Debrief => ui::draw_debrief(
                &self.data,
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
                self.state = AppState::MissionBriefing;
                self.last_outcome = None;
            }
            UiAction::DeployMission => {
                self.session = GameSession::new(
                    &self.data.config,
                    &self.data.mission,
                    &self.campaign.deployment_roster(&self.data),
                );
                self.state = AppState::Tactical;
                self.autosave_current("Deployment autosaved");
                self.notifications.success("Operation Glassroot deployed");
            }
            UiAction::Continue => self.load_game(),
            UiAction::ReturnToTitle => self.state = AppState::Title,
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
            UiAction::InteractObjective => match self.session.interact_selected() {
                Ok(_) => self.notifications.success("Survey refuge stabilised"),
                Err(_) => self
                    .notifications
                    .warning("A colonist must reach the beacon"),
            },
            UiAction::EndPhase => {
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
        if let Some(outcome) = self.session.mission_outcome(&self.data.mission) {
            self.last_outcome = Some(outcome);
            self.campaign.advance_recovery();
            self.campaign.apply_mission_outcome(
                self.last_outcome.as_ref().expect("outcome was just stored"),
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
                self.campaign = save.campaign;
                let tactical = save.tactical.unwrap_or_else(|| {
                    GameSession::new(
                        &self.data.config,
                        &self.data.mission,
                        &self.campaign.deployment_roster(&self.data),
                    )
                    .tactical
                });
                self.session = GameSession::from_tactical(tactical);
                self.state = AppState::Tactical;
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
