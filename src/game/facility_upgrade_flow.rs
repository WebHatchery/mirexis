//! Strategic facility-upgrade actions emitted by the colony operations panel.

use super::Game;
use crate::ui::UiAction;

impl Game {
    pub(super) fn apply_facility_upgrade_action(&mut self, action: &UiAction) -> bool {
        match action {
            UiAction::OpenFacilityUpgrade => {
                let can_open = self.campaign.colony.buildings.iter().any(|building| {
                    !building.kind.upgrade_options().is_empty()
                        && building.level < 2
                        && !building.damaged
                        && self.campaign.colony.building_is_powered(&building.id)
                });
                if can_open {
                    self.facility_upgrade_open = true;
                    self.clear_colony_explorer_motion();
                } else {
                    self.notifications
                        .warning("No facility can accept an upgrade right now");
                }
                true
            }
            UiAction::QueueFacilityUpgrade(building_id, upgrade_id) => {
                match self
                    .campaign
                    .colony
                    .queue_facility_upgrade(building_id, upgrade_id)
                {
                    Ok(message) => {
                        self.facility_upgrade_open = false;
                        self.clear_colony_explorer_motion();
                        self.notifications.success(message);
                        self.autosave_campaign_only("Facility upgrade autosaved");
                    }
                    Err(err) => self.notifications.warning(err),
                }
                true
            }
            UiAction::CloseFacilityUpgrade => {
                self.facility_upgrade_open = false;
                self.clear_colony_explorer_motion();
                true
            }
            _ => false,
        }
    }
}
