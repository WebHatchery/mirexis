//! Deterministic UI-reference scene construction.

use super::{AppState, Game, TacticalTargeting};
use crate::data::Team;
use crate::state::{GameSession, ObjectiveState};
use macroquad_toolkit::grid::TilePos;

impl Game {
    pub fn begin_capture_scene(&mut self, scene: &str) {
        self.targeting = None;
        match scene {
            "title" => self.state = AppState::Title,
            "colony" => self.state = AppState::Colony,
            "roster" => self.state = AppState::Roster,
            "briefing" => self.state = AppState::MissionBriefing,
            "equipment" => self.capture_equipment_target(),
            "class_target" => self.capture_class_target(),
            "breach" => self.capture_breach(),
            "debrief" => self.capture_debrief(),
            _ => self.reset_capture_session(AppState::Tactical),
        }
    }

    fn reset_capture_session(&mut self, state: AppState) {
        self.session = GameSession::new(
            &self.data.config,
            &self.active_mission,
            &self
                .campaign
                .deployment_roster(&self.data, &self.active_mission),
        );
        self.state = state;
    }

    fn capture_equipment_target(&mut self) {
        self.reset_capture_session(AppState::Tactical);
        let kira_position = self
            .session
            .unit("kira_voss")
            .expect("capture roster includes Kira")
            .position;
        if let Some(hostile) = self
            .session
            .tactical
            .units
            .iter_mut()
            .find(|unit| unit.team == Team::Hostile)
        {
            hostile.position = TilePos::new(kira_position.x + 3, kira_position.y);
        }
        self.targeting = Some(TacticalTargeting::Equipment {
            unit_id: "kira_voss".to_owned(),
            equipment_id: "survey_harness".to_owned(),
        });
    }

    fn capture_class_target(&mut self) {
        self.reset_capture_session(AppState::Tactical);
        for unit in &mut self.session.tactical.units {
            if matches!(unit.id.as_str(), "kira_voss" | "mara_venn") {
                unit.health = (unit.health - 4).max(1);
            }
        }
        self.session.tactical.selected_unit = Some("ilya_reed".to_owned());
        self.session.tactical.selected_tile = self
            .session
            .unit("ilya_reed")
            .expect("capture roster includes Ilya")
            .position;
        self.targeting = Some(TacticalTargeting::ClassAction {
            unit_id: "ilya_reed".to_owned(),
        });
    }

    fn capture_breach(&mut self) {
        self.reset_capture_session(AppState::Tactical);
        let position = self
            .session
            .tactical
            .destructible_cover
            .iter()
            .find(|cover| self.session.can_attack_selected_cover(cover.position))
            .expect("capture map includes attackable cover")
            .position;
        while self.session.tactical.blocked.contains(&position) {
            self.refresh_capture_unit();
            self.session
                .attack_selected_cover(position)
                .expect("capture cover remains attackable");
        }
        self.refresh_capture_unit();
    }

    fn refresh_capture_unit(&mut self) {
        if let Some(unit) = self
            .session
            .tactical
            .units
            .iter_mut()
            .find(|unit| Some(&unit.id) == self.session.tactical.selected_unit.as_ref())
        {
            unit.action_points = self.data.config.max_action_points;
        }
    }

    fn capture_debrief(&mut self) {
        self.reset_capture_session(AppState::Tactical);
        self.session.tactical.objective_state = ObjectiveState::Victory;
        for unit in &mut self.session.tactical.units {
            if unit.team == Team::Hostile {
                unit.incapacitated = true;
                unit.health = 0;
            }
        }
        self.last_outcome = self.session.mission_outcome(&self.active_mission);
        self.state = AppState::Debrief;
    }
}
