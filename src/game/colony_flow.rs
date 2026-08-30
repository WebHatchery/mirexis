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

    pub(super) fn handle_relay_scan(&mut self) {
        match self.campaign.run_relay_scan(&self.data) {
            Ok(summary) => {
                self.notifications.warning(summary);
                self.autosave_campaign_only("Relay scan autosaved");
            }
            Err(err) => self.notifications.warning(err),
        }
    }

    pub(super) fn handle_identity_stewardship(&mut self) {
        match self.campaign.run_identity_stewardship() {
            Ok(summary) => {
                self.notifications.success(summary);
                self.autosave_campaign_only("Identity stewardship autosaved");
            }
            Err(err) => self.notifications.warning(err),
        }
    }
}
