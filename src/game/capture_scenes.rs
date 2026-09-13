//! Deterministic UI-reference scene construction.

use super::{AppState, Game, TacticalTargeting};
use crate::data::{ObjectiveKind, OperationModifier, Team};
use crate::state::GameSession;
use macroquad_toolkit::grid::TilePos;

impl Game {
    pub fn begin_capture_scene(&mut self, scene: &str) {
        self.reset_capture_world();
        if self.capture_scene_group_a(scene) {
            return;
        }
        if self.capture_scene_group_b(scene) {
            return;
        }
        if self.capture_scene_group_c(scene) {
            return;
        }
        if self.capture_scene_group_d(scene) {
            return;
        }
        if self.capture_scene_group_e(scene) {
            return;
        }
        if self.capture_scene_group_f(scene) {
            return;
        }
        if self.capture_scene_group_g(scene) {
            return;
        }
        panic!("unsupported Mirexis capture scene: {scene}");
    }

    pub fn reset_capture_session(&mut self, state: AppState) {
        self.session = GameSession::new(
            &self.data.config,
            &self.active_mission,
            &self
                .campaign
                .deployment_roster(&self.data, &self.active_mission),
        );
        self.state = state;
    }

    pub fn capture_template_operation(
        &mut self,
        template_id: &str,
        seed: u64,
        modifier: OperationModifier,
    ) {
        let template = self
            .data
            .campaign
            .mission_templates
            .iter()
            .find(|template| template.id == template_id)
            .expect("capture mission template exists");
        let recipe = self
            .data
            .campaign
            .map_recipes
            .iter()
            .find(|recipe| recipe.id == template.map_recipe)
            .expect("capture map recipe exists");
        let layout = crate::map_variants::materialize(recipe, &self.data, seed);
        self.active_mission.id = format!("capture_{}", template.id);
        self.active_mission.name = template.name.clone();
        self.active_mission.briefing = if !template.required_mirexis_path.is_empty() {
            format!(
                "The committed {} path opens this battlefield.",
                template.required_mirexis_path.replace('_', " ")
            )
        } else if !template.required_response.is_empty() {
            format!(
                "The committed {} response opens this battlefield.",
                template.required_response.replace('_', " ")
            )
        } else if template.required_phase == "escalation" {
            "Escalation intelligence confirms three-power crossfire.".to_owned()
        } else if !template.required_phase.is_empty() {
            format!(
                "Adaptation intelligence confirms {} resistance.",
                template.faction
            )
        } else if template.required_protocol.is_empty() {
            format!(
                "Pressure intelligence confirms {} resistance.",
                template.faction
            )
        } else {
            format!(
                "Contact intelligence confirms {} resistance.",
                template.faction
            )
        };
        self.active_mission.objective = template.objective.clone();
        self.active_mission.objective_kind = template.objective_kind;
        self.active_mission.hostile_faction = template.faction.clone();
        self.active_mission.hostile_unit_ids = template.hostile_unit_ids.clone();
        self.active_mission.round_limit = template.round_limit;
        self.active_mission.materials_reward = template.materials_reward;
        self.active_mission.biomass_reward = template.biomass_reward;
        self.active_mission.power_reward = template.power_reward;
        self.active_mission.operation_modifier = modifier;
        self.active_mission.seed = seed;
        self.active_mission.blocked_tiles = layout.blocked_tiles;
        self.active_mission.objective_tile = layout.objective_tile;
        self.active_mission.terrain_costs = layout.terrain_costs;
        self.active_mission.hazards = layout.hazards;
        self.active_mission.cover_edges = layout.cover_edges;
        self.reset_capture_session(AppState::Tactical);
        self.tactical_panel_open = true;
    }

    pub fn capture_active_trace(&mut self) {
        self.capture_template_operation(
            "directorate_contact_trace",
            8,
            OperationModifier::DirectorateFireControl,
        );
        let relay = self.session.tactical.objective_tile;
        let selected = self
            .session
            .tactical
            .selected_unit
            .clone()
            .expect("capture trace has a selected colonist");
        self.session
            .tactical
            .units
            .iter_mut()
            .find(|unit| unit.id == selected)
            .expect("selected capture colonist exists")
            .position = TilePos::new(relay.x - 1, relay.y);
        self.session
            .interact_selected()
            .expect("capture colonist activates relay");
    }

    pub fn capture_defense_asset(&mut self) {
        self.capture_template_operation("shelter_signal", 6, OperationModifier::BroodFrenzy);
        self.session.tactical.objective_integrity = 7;
    }

    pub fn capture_equipment_target(&mut self) {
        self.reset_capture_session(AppState::Tactical);
        self.tactical_panel_open = true;
        let kira_position = self
            .session
            .unit("kira_voss")
            .expect("capture roster includes Kira")
            .position;
        let hostile_position = if let Some(hostile) = self
            .session
            .tactical
            .units
            .iter_mut()
            .find(|unit| unit.team == Team::Hostile)
        {
            hostile.position = TilePos::new(kira_position.x + 3, kira_position.y);
            Some(hostile.position)
        } else {
            None
        };
        if let Some(position) = hostile_position {
            self.session.tactical.selected_tile = position;
        }
        self.targeting = Some(TacticalTargeting::Equipment {
            unit_id: "kira_voss".to_owned(),
            equipment_id: "survey_harness".to_owned(),
        });
    }

    pub fn capture_weapon_profile(&mut self) {
        let kira = self
            .campaign
            .roster
            .iter_mut()
            .find(|character| character.id == "kira_voss")
            .expect("weapon capture roster includes Kira");
        kira.equipment_ids.retain(|equipment_id| {
            !matches!(
                equipment_id.as_str(),
                "frontier_rifle" | "mire_lmg" | "service_pistol" | "breach_scattergun"
            )
        });
        kira.equipment_ids.push("needle_carbine".to_owned());
        self.reset_capture_session(AppState::Tactical);
        self.tactical_panel_open = true;
    }

    pub fn capture_overwatch(&mut self) {
        self.reset_capture_session(AppState::Tactical);
        self.tactical_panel_open = true;
        self.session
            .set_selected_overwatch()
            .expect("capture colonist can enter overwatch");
    }

    pub fn capture_extraction(&mut self) {
        self.configure_extraction(2, "ISOLATION: LAST TRANSMISSION");
    }

    pub fn capture_map_variant(&mut self) {
        self.configure_extraction(3, "LAST TRANSMISSION // SOUTHERN APPROACH");
        let selected = self.session.tactical.selected_tile;
        self.tactical_camera = crate::grid_ui::WorldCamera::tactical_view(
            TilePos::new(selected.x + 7, selected.y + 7),
            selected,
            0.78,
        );
    }

    pub fn configure_extraction(&mut self, seed: u64, name: &str) {
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
        self.active_mission.hazards = layout.hazards;
        self.active_mission.cover_edges = layout.cover_edges;
        self.reset_capture_session(AppState::Tactical);
        self.tactical_panel_open = true;
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

    pub fn capture_class_target(&mut self) {
        self.reset_capture_session(AppState::Tactical);
        for unit in &mut self.session.tactical.units {
            if matches!(unit.id.as_str(), "kira_voss" | "mara_venn") {
                unit.health = (unit.health - 4).max(1);
            }
        }
        self.session.tactical.selected_unit = Some("ilya_reed".to_owned());
        self.session.tactical.selected_tile = self
            .session
            .unit("kira_voss")
            .expect("capture roster includes Kira")
            .position;
        self.targeting = Some(TacticalTargeting::ClassAction {
            unit_id: "ilya_reed".to_owned(),
            target_kind: crate::data::TechniqueTarget::Ally,
        });
        self.tactical_panel_open = true;
    }
}
