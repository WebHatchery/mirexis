//! Application state for the derived Memorial Register.

use super::Game;
use crate::ui::UiAction;

impl Game {
    pub(super) fn apply_memorial_action(&mut self, action: &UiAction) -> bool {
        match action {
            UiAction::ToggleMemorial => {
                self.show_memorial = !self.show_memorial;
                if self.show_memorial {
                    self.show_settings = false;
                    self.show_field_notes = false;
                    self.facility_upgrade_open = false;
                    self.salvage_open = false;
                    self.colony_operations_open = true;
                    self.memorial_page = 0;
                    self.clear_colony_explorer_motion();
                }
                true
            }
            UiAction::PreviousMemorialPage if self.show_memorial => {
                self.memorial_page = crate::memorial_ui::previous_page(self.memorial_page);
                true
            }
            UiAction::NextMemorialPage if self.show_memorial => {
                self.memorial_page = crate::memorial_ui::next_page(
                    self.memorial_page,
                    crate::memorial_ui::page_count(&self.campaign),
                );
                true
            }
            _ => false,
        }
    }
}
