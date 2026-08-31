use super::{AppState, Game};

impl Game {
    pub(super) fn enter_debrief_if_finished(&mut self) {
        if self.state != AppState::Tactical {
            return;
        }
        if let Some(outcome) = self.session.mission_outcome(&self.active_mission) {
            self.last_outcome = Some(outcome);
            self.campaign.advance_recovery();
            let mission = self
                .campaign
                .strategy
                .selected_mission()
                .expect("a deployed mission remains selected")
                .clone();
            let learned_techniques = self.campaign.apply_mission_outcome(
                self.last_outcome.as_ref().expect("outcome was just stored"),
                &mission,
                &self.data,
            );
            self.record_first_hour_outcome();
            self.state = AppState::Debrief;
            self.autosave_current("Debrief autosaved");
            for technique in learned_techniques {
                self.notifications
                    .success(format!("Technique learned · {technique}"));
            }
        }
    }
}
