//! Deterministic UI-reference scene construction.

use super::{AppState, Game, TacticalTargeting};
use crate::data::{ObjectiveKind, Team};
use crate::state::{GameSession, ObjectiveState};
use macroquad_toolkit::grid::TilePos;

impl Game {
    pub fn begin_capture_scene(&mut self, scene: &str) {
        self.targeting = None;
        match scene {
            "title" => self.state = AppState::Title,
            "colony" => self.state = AppState::Colony,
            "research" => self.capture_research(),
            "legacy" => self.capture_legacy(),
            "roster" => self.state = AppState::Roster,
            "briefing" => self.state = AppState::MissionBriefing,
            "extraction" => self.capture_extraction(),
            "variant" => self.capture_map_variant(),
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

    fn capture_research(&mut self) {
        for research in &mut self.campaign.strategy.research {
            research.completed = true;
        }
        self.state = AppState::Colony;
    }

    fn capture_legacy(&mut self) {
        self.campaign
            .resolve_first_character_event(&self.data)
            .expect("first capture event resolves");
        self.campaign
            .resolve_first_character_event(&self.data)
            .expect("second capture event resolves");
        self.campaign.selected_character_id = "mara_venn".to_owned();
        self.state = AppState::Roster;
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

    fn capture_extraction(&mut self) {
        self.configure_extraction(2, "ISOLATION: LAST TRANSMISSION");
    }

    fn capture_map_variant(&mut self) {
        self.configure_extraction(3, "LAST TRANSMISSION // SOUTHERN APPROACH");
    }

    fn configure_extraction(&mut self, seed: u64, name: &str) {
        self.active_mission.name = name.to_owned();
        self.active_mission.objective =
            "Get any colonist carrying the stolen coordinates to extraction.".to_owned();
        self.active_mission.objective_kind = ObjectiveKind::Extraction;
        self.active_mission.hostile_faction = "directorate".to_owned();
        self.active_mission.round_limit = 6;
        let recipe = self
            .data
            .campaign
            .map_recipes
            .iter()
            .find(|recipe| recipe.id == "directorate_checkpoint")
            .expect("extraction capture recipe exists");
        let layout = crate::map_variants::materialize(recipe, &self.data, seed);
        self.active_mission.seed = seed;
        self.active_mission.blocked_tiles = layout.blocked_tiles;
        self.active_mission.objective_tile = layout.objective_tile;
        self.active_mission.terrain_costs = layout.terrain_costs;
        self.active_mission.cover_edges = layout.cover_edges;
        self.reset_capture_session(AppState::Tactical);
        let evac = self.session.tactical.objective_tile;
        let colonist = self
            .session
            .tactical
            .selected_unit
            .clone()
            .expect("capture session selects a colonist");
        let unit = self
            .session
            .tactical
            .units
            .iter_mut()
            .find(|unit| unit.id == colonist)
            .expect("selected capture colonist exists");
        unit.position = TilePos::new(evac.x - 1, evac.y);
        self.session.tactical.selected_tile = unit.position;
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
