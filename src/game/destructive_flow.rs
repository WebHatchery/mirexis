//! Two-step confirmation for campaign replacement and save deletion.

use super::Game;
use crate::ui::UiAction;

impl Game {
    pub(super) fn guard_destructive_action(&mut self, action: &UiAction) -> bool {
        match action {
            UiAction::StartMission if self.save_exists && !self.new_campaign_armed => {
                self.new_campaign_armed = true;
                self.notifications
                    .warning("Tap CONFIRM NEW COLONY to replace the current campaign");
                true
            }
            UiAction::StartMission => {
                self.new_campaign_armed = false;
                false
            }
            UiAction::DeleteSave if self.delete_save_armed => {
                self.delete_save();
                self.delete_save_armed = false;
                true
            }
            UiAction::DeleteSave => {
                self.delete_save_armed = true;
                self.notifications
                    .warning("Tap CONFIRM DELETE to clear the campaign save");
                true
            }
            _ => false,
        }
    }
}
