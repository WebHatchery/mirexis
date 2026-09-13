//! Strategic colony actions that need application-level persistence feedback.

use super::{AppState, Game};

impl Game {
    pub fn apply_colony_action(&mut self, action: &crate::ui::UiAction) -> bool {
        match action {
            crate::ui::UiAction::ConfirmColonyPlot(position) => {
                let Some(construction) = crate::colony_map_ui::interaction::plot_action(
                    &self.campaign,
                    &self.data,
                    *position,
                ) else {
                    self.colony_camera.clear_pending_colony_plot();
                    return true;
                };
                if self.colony_camera.confirm_colony_plot(*position) {
                    self.apply_action(construction);
                }
                true
            }
            crate::ui::UiAction::WalkColony(encoded) => {
                let destination = crate::colony_map_ui::decode_world_position(*encoded);
                self.colony_explorer
                    .request_walk(destination, &self.campaign.colony);
                true
            }
            crate::ui::UiAction::ApproachColonist(character_id) => {
                if let Some(position) =
                    crate::colony_exploration::npc_position(&self.campaign, character_id)
                {
                    self.colony_explorer.request_approach(
                        character_id,
                        position,
                        &self.campaign.colony,
                    );
                }
                true
            }
            crate::ui::UiAction::InteractColony => {
                self.colony_explorer.interact(&self.campaign, &self.data);
                true
            }
            crate::ui::UiAction::SetColonyBuildMode(enabled) => {
                self.colony_explorer.set_build_mode(*enabled);
                self.colony_camera.clear_pending_colony_plot();
                true
            }
            _ => false,
        }
    }

    pub fn colony_explorer_can_update(&self) -> bool {
        colony_explorer_can_update(
            self.state,
            self.campaign.first_hour.help_open,
            self.show_settings,
            self.show_field_notes,
            self.show_memorial,
            self.facility_upgrade_open,
            self.salvage_open,
        )
    }

    pub fn clear_colony_explorer_motion(&mut self) {
        self.colony_explorer
            .set_keyboard_direction(macroquad::prelude::Vec2::ZERO);
        self.colony_explorer
            .set_touch_direction(macroquad::prelude::Vec2::ZERO);
    }

    pub fn handle_commons_meal(&mut self) {
        match self.campaign.host_commons_meal() {
            Ok(summary) => {
                self.notifications.success(summary);
                self.autosave_campaign_only("Commons meal autosaved");
            }
            Err(err) => self.notifications.warning(err),
        }
    }

    pub fn handle_relay_scan(&mut self) {
        match self.campaign.run_relay_scan(&self.data) {
            Ok(summary) => {
                self.notifications.warning(summary);
                self.autosave_campaign_only("Relay scan autosaved");
            }
            Err(err) => self.notifications.warning(err),
        }
    }

    pub fn handle_identity_stewardship(&mut self) {
        match self.campaign.run_identity_stewardship() {
            Ok(summary) => {
                self.notifications.success(summary);
                self.autosave_campaign_only("Identity stewardship autosaved");
            }
            Err(err) => self.notifications.warning(err),
        }
    }

    pub fn handle_identity_preparation(&mut self) {
        match self.campaign.prepare_identity_building() {
            Ok(summary) => {
                self.notifications.success(summary);
                self.autosave_campaign_only("Identity preparation autosaved");
            }
            Err(err) => self.notifications.warning(err),
        }
    }
}

pub fn colony_explorer_can_update(
    state: AppState,
    first_hour_help_open: bool,
    settings_open: bool,
    field_notes_open: bool,
    memorial_open: bool,
    facility_upgrade_open: bool,
    salvage_open: bool,
) -> bool {
    state == AppState::Colony
        && !first_hour_help_open
        && !settings_open
        && !field_notes_open
        && !memorial_open
        && !facility_upgrade_open
        && !salvage_open
}
