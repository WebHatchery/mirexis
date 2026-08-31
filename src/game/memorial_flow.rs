//! Application state for the derived Memorial Register.

use super::Game;
use crate::ui::UiAction;

impl Game {
    pub(super) fn apply_memorial_action(&mut self, action: &UiAction) -> bool {
        if !matches!(action, UiAction::ToggleMemorial) {
            return false;
        }

        self.show_memorial = !self.show_memorial;
        if self.show_memorial {
            self.show_settings = false;
            self.show_field_notes = false;
            self.facility_upgrade_open = false;
            self.salvage_open = false;
            self.colony_operations_open = true;
            self.clear_colony_explorer_motion();
        }
        true
    }
}
