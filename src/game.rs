//! Application state machine, persistence, and toolkit integration.

use crate::data::GameData;
use crate::state::{migrate_save_value, GameSession, SaveData};
use crate::ui::{self, UiAction, UiContext};
use macroquad::prelude::*;
use macroquad_toolkit::assets::AssetManager;
use macroquad_toolkit::events::EventBus;
use macroquad_toolkit::notifications::{
    NotificationAnchor, NotificationManager, NotificationRenderConfig,
};
use macroquad_toolkit::persistence::{
    delete_slot, load_from_slot_with_migration, save_to_slot_with_version, slot_exists,
};
use macroquad_toolkit::prelude::{begin_virtual_ui_frame, dark, end_virtual_ui_frame, InputState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AppState {
    Title,
    Tactical,
}

pub struct Game {
    data: GameData,
    session: GameSession,
    state: AppState,
    assets: AssetManager,
    notifications: NotificationManager,
    events: EventBus<UiAction>,
    save_exists: bool,
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
        let session = GameSession::new(&data.config, &data.mission, &data.roster);
        let save_exists = slot_exists(&data.config.game_name, &data.config.save_slot);
        let mut notifications = NotificationManager::new();
        notifications.info(format!(
            "Phase 0 systems online; {} manifest textures loaded",
            loaded_assets
        ));

        Self {
            data,
            session,
            state: AppState::Title,
            assets,
            notifications,
            events: EventBus::new(),
            save_exists,
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
            AppState::Tactical => ui::draw_tactical(UiContext {
                data: &self.data,
                session: &self.session,
                save_exists: self.save_exists,
                loaded_assets: self.assets.len(),
                ui: &virtual_ui,
            }),
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
                self.session =
                    GameSession::new(&self.data.config, &self.data.mission, &self.data.roster);
                self.state = AppState::Tactical;
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
    }

    fn save_game(&mut self) {
        let save = self.session.to_save(&self.data.config.version);
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
            |version, value| migrate_save_value(version, value, &self.data.config),
        );
        match loaded {
            Ok(save) => {
                self.session = GameSession::from_save(save);
                self.state = AppState::Tactical;
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
