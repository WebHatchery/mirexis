//! Deterministic victory and casualty debrief showcase scenes.

use super::{AppState, Game};
use crate::{data::Team, state::ObjectiveState};

impl Game {
    pub(super) fn capture_debrief(&mut self) {
        self.reset_capture_session(AppState::Tactical);
        self.session.tactical.objective_state = ObjectiveState::Victory;
        for unit in &mut self.session.tactical.units {
            if unit.team == Team::Hostile {
                unit.incapacitated = true;
                unit.health = 0;
            }
        }
        let deployed_ids = self.session.deployed_colonist_ids();
        for character in &mut self.campaign.roster {
            if deployed_ids.iter().any(|id| id == &character.id) {
                character.experience = 20;
            }
        }
        let learned_techniques = crate::skill_training::learn_after_operation(
            &mut self.campaign,
            &deployed_ids,
            &self.data,
        );
        self.last_outcome = self.session.mission_outcome(&self.active_mission);
        for technique in learned_techniques {
            self.notifications
                .success(format!("Technique learned · {technique}"));
        }
        self.campaign.first_hour.stage = crate::first_hour::FirstHourStage::FirstReturn;
        self.state = AppState::Debrief;
    }

    pub(super) fn capture_trauma_debrief(&mut self) {
        self.reset_capture_session(AppState::Tactical);
        self.session.tactical.objective_state = ObjectiveState::Victory;
        for unit in &mut self.session.tactical.units {
            if unit.team == Team::Hostile || unit.id == "kira_voss" {
                unit.incapacitated = true;
                unit.health = 0;
            }
        }
        self.last_outcome = self.session.mission_outcome(&self.active_mission);
        self.campaign.first_hour.stage = crate::first_hour::FirstHourStage::FirstReturn;
        self.state = AppState::Debrief;
    }
}
