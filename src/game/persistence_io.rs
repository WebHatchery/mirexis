//! Save-slot operations owned by the application state machine.

use super::{AppState, Game, TacticalTargeting};
use crate::persistence::migrate_save_value;
use crate::state::{GameSession, SaveData};
use macroquad_toolkit::persistence::{
    delete_slot, load_from_slot_with_migration, save_to_slot_with_version,
};

impl Game {
    pub(super) fn autosave_current(&mut self, success_message: &str) {
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
            Err(err) => self.notifications.danger(format!("Autosave failed: {err}")),
        }
    }

    pub(super) fn autosave_campaign_only(&mut self, success_message: &str) {
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
            Err(err) => self.notifications.danger(format!("Autosave failed: {err}")),
        }
    }

    pub(super) fn save_game(&mut self) {
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
            Err(err) => self.notifications.danger(format!("Save failed: {err}")),
        }
    }

    pub(super) fn load_game(&mut self) {
        let loaded: Result<SaveData, String> = load_from_slot_with_migration(
            &self.data.config.game_name,
            &self.data.config.save_slot,
            &self.data.config.version,
            |version, value| migrate_save_value(version, value, &self.data),
        );
        match loaded {
            Ok(save) => {
                self.targeting = None::<TacticalTargeting>;
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
            Err(err) => self.notifications.warning(format!("Load failed: {err}")),
        }
    }

    pub(super) fn delete_save(&mut self) {
        match delete_slot(&self.data.config.game_name, &self.data.config.save_slot) {
            Ok(()) => {
                self.save_exists = false;
                self.notifications.info("Save slot cleared");
            }
            Err(err) => self.notifications.danger(format!("Delete failed: {err}")),
        }
    }
}
