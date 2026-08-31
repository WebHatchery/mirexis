//! Application state for the colony's persistent field-note archive.

use super::Game;
use crate::ui::UiAction;

impl Game {
    pub(super) fn apply_field_notes_action(&mut self, action: &UiAction) -> bool {
        match action {
            UiAction::ToggleFieldNotes => {
                self.show_field_notes = !self.show_field_notes;
                if self.show_field_notes {
                    self.selected_field_note = self
                        .campaign
                        .colony_story
                        .archived_notes()
                        .len()
                        .saturating_sub(1);
                    self.show_settings = false;
                    self.show_memorial = false;
                    self.facility_upgrade_open = false;
                    self.salvage_open = false;
                    self.clear_colony_explorer_motion();
                }
            }
            UiAction::SelectFieldNote(index) => {
                self.selected_field_note = (*index).min(
                    self.campaign
                        .colony_story
                        .archived_notes()
                        .len()
                        .saturating_sub(1),
                );
            }
            _ => return false,
        }
        true
    }
}
