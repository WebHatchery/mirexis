//! Strategic colony actions that need application-level persistence feedback.

use super::{AppState, Game};

impl Game {
    pub(super) fn colony_explorer_can_update(&self) -> bool {
        colony_explorer_can_update(
            self.state,
            self.campaign.first_hour.help_open,
            self.show_settings,
            self.show_field_notes,
            self.facility_upgrade_open,
            self.salvage_open,
        )
    }

    pub(super) fn clear_colony_explorer_motion(&mut self) {
        self.colony_explorer
            .set_keyboard_direction(macroquad::prelude::Vec2::ZERO);
        self.colony_explorer
            .set_touch_direction(macroquad::prelude::Vec2::ZERO);
    }

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

    pub(super) fn handle_identity_preparation(&mut self) {
        match self.campaign.prepare_identity_building() {
            Ok(summary) => {
                self.notifications.success(summary);
                self.autosave_campaign_only("Identity preparation autosaved");
            }
            Err(err) => self.notifications.warning(err),
        }
    }
}

fn colony_explorer_can_update(
    state: AppState,
    first_hour_help_open: bool,
    settings_open: bool,
    field_notes_open: bool,
    facility_upgrade_open: bool,
    salvage_open: bool,
) -> bool {
    state == AppState::Colony
        && !first_hour_help_open
        && !settings_open
        && !field_notes_open
        && !facility_upgrade_open
        && !salvage_open
}

#[cfg(test)]
mod tests;
