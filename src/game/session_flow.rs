//! Transient tactical state cleanup when the application crosses a session boundary.

use super::Game;
use crate::combat_feedback::CombatFeedback;
use crate::phase_replay::PhaseReplay;
use crate::ui_action::BattleLogFilter;

impl Game {
    pub(super) fn reset_tactical_transients(&mut self, play_initial_phase_audio: bool) {
        self.targeting = None;
        self.show_tactical_help = false;
        self.show_battle_log = false;
        self.battle_log_filter = BattleLogFilter::default();
        self.tactical_panel_open = false;
        self.show_settings = false;
        self.show_field_notes = false;
        self.show_memorial = false;
        self.selected_field_note = 0;
        reset_tactical_presentation(
            &mut self.combat_feedback,
            &mut self.phase_replay,
            &mut self.observed_event_count,
            &mut self.played_audio_event_count,
            &mut self.end_phase_armed,
            self.session.tactical.event_log.len(),
            play_initial_phase_audio,
        );
    }
}

fn reset_tactical_presentation(
    combat_feedback: &mut CombatFeedback,
    phase_replay: &mut PhaseReplay,
    observed_event_count: &mut usize,
    played_audio_event_count: &mut usize,
    end_phase_armed: &mut bool,
    event_count: usize,
    play_initial_phase_audio: bool,
) {
    combat_feedback.clear();
    phase_replay.clear();
    let initial_event_count = if play_initial_phase_audio {
        0
    } else {
        event_count
    };
    *observed_event_count = initial_event_count;
    *played_audio_event_count = initial_event_count;
    *end_phase_armed = false;
}

#[cfg(test)]
mod tests;
