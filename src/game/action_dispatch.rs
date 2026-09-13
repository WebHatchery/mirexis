//! Central routing for actions emitted by the game UI.

use super::*;

impl Game {
    pub fn apply_action(&mut self, action: UiAction) {
        if self.apply_audio_action(&action) {
            return;
        }
        if self.apply_first_hour_action(&action) {
            return;
        }
        if self.apply_field_notes_action(&action) {
            return;
        }
        if self.apply_memorial_action(&action) {
            return;
        }
        if self.apply_colony_action(&action) {
            return;
        }
        if self.guard_destructive_action(&action) {
            return;
        }
        let audio_event_count = self.session.tactical.event_log.len();
        let audio_action = action.clone();
        if !matches!(&action, UiAction::EndPhase) {
            self.end_phase_armed = false;
        }
        if self.apply_skill_action(&action, audio_event_count) {
            return;
        }
        if self.apply_class_action(&action, audio_event_count) {
            return;
        }
        if self.apply_facility_upgrade_action(&action) {
            return;
        }
        if self.apply_salvage_action(&action) {
            return;
        }
        let handled = action_campaign::apply_campaign_action(self, action.clone())
            || action_navigation::apply_navigation_action(self, action.clone())
            || action_colony::apply_colony_management_action(self, action.clone())
            || action_tactical::apply_tactical_action(self, action.clone())
            || action_phase::apply_phase_action(self, action.clone());
        if !handled {
            unreachable!("unhandled application action reached the game state match");
        }
        self.finish_action_audio(&audio_action, audio_event_count);
        self.enter_debrief_if_finished();
    }
}
