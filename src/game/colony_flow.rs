//! Strategic colony actions that need application-level persistence feedback.

use super::Game;

impl Game {
    pub(super) fn handle_commons_meal(&mut self) {
        match self.campaign.host_commons_meal() {
            Ok(summary) => {
                self.notifications.success(summary);
                self.autosave_campaign_only("Commons meal autosaved");
            }
            Err(err) => self.notifications.warning(err),
        }
    }
}
