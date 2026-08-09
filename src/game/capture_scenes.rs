//! Deterministic UI-reference scene construction.

use super::{AppState, Game, TacticalTargeting};
use crate::data::{ObjectiveKind, OperationModifier, Team};
use crate::state::{GameSession, ObjectiveState};
use macroquad_toolkit::grid::TilePos;

impl Game {
    pub fn begin_capture_scene(&mut self, scene: &str) {
        self.targeting = None;
        match scene {
            "title" => self.state = AppState::Title,
            "colony" => self.state = AppState::Colony,
            "contact" => self.capture_contact(),
            "contact_gear" => self.capture_contact_gear(),
            "contact_event" => self.capture_contact_event(),
            "adaptation" => self.capture_adaptation(),
            "gene_lab" => self.capture_gene_lab(),
            "evolution" => self.capture_evolution(),
            "mara_evolution" => self.capture_mara_evolution(),
            "ilya_evolution" => self.capture_ilya_evolution(),
            "sol_evolution" => self.capture_sol_evolution(),
            "escalation" => self.capture_escalation(),
            "adaptation_operation" => self.capture_adaptation_operation(),
            "glass_nerve" => self.capture_template_operation(
                "adaptation_glass_nerve",
                14,
                OperationModifier::AscendantInterference,
            ),
            "research" => self.capture_research(),
            "legacy" => self.capture_legacy(),
            "roster" => self.state = AppState::Roster,
            "briefing" => self.state = AppState::MissionBriefing,
            "pressure" => self.capture_pressure(),
            "sporefield" => self.capture_template_operation(
                "sporefield_extraction",
                4,
                OperationModifier::BroodFrenzy,
            ),
            "vault" => self.capture_template_operation(
                "vault_purge",
                5,
                OperationModifier::AscendantInterference,
            ),
            "black_channel" => self.capture_template_operation(
                "directorate_contact_trace",
                8,
                OperationModifier::DirectorateFireControl,
            ),
            "living_chorus" => self.capture_template_operation(
                "brood_contact_trace",
                10,
                OperationModifier::BroodFrenzy,
            ),
            "open_circuit" => self.capture_template_operation(
                "ascendant_contact_trace",
                12,
                OperationModifier::AscendantInterference,
            ),
            "trace_active" => self.capture_active_trace(),
            "damage" => self.capture_colony_damage(),
            "power" => self.capture_power_outage(),
            "construction" => self.capture_power_construction(),
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

    fn capture_contact(&mut self) {
        self.campaign.strategy.isolation_victories = 3;
        self.campaign.strategy.first_assault_repulsed = true;
        self.campaign.strategy.research[0].completed = true;
        self.campaign
            .strategy
            .refresh_isolation_completion(&mut self.campaign.colony);
        self.state = AppState::Colony;
    }

    fn capture_contact_gear(&mut self) {
        self.capture_contact();
        self.campaign
            .strategy
            .choose_contact_protocol("ascendant_capacitor", &mut self.campaign.colony, &self.data)
            .expect("capture Contact protocol is available");
        self.campaign.strategy.contact_trace_completed = true;
        self.state = AppState::Roster;
    }

    fn capture_contact_event(&mut self) {
        self.capture_contact_gear();
        self.campaign
            .resolve_first_character_event(&self.data)
            .expect("first Isolation capture event resolves");
        self.campaign
            .resolve_first_character_event(&self.data)
            .expect("second Isolation capture event resolves");
        self.state = AppState::Colony;
    }

    fn capture_adaptation(&mut self) {
        self.capture_contact_event();
        self.campaign
            .resolve_first_character_event(&self.data)
            .expect("Contact capture event resolves");
        self.campaign
            .craft_equipment("sol_cairn", "ascendant_phase_lens", &self.data)
            .expect("Contact capture prototype is crafted");
        self.campaign
            .colony
            .select_construction(crate::colony::BuildingKind::GeneLab)
            .expect("Adaptation capture can plan the Gene Lab");
        self.state = AppState::Colony;
    }

    fn capture_gene_lab(&mut self) {
        self.capture_adaptation();
        self.campaign
            .colony
            .place_construction(crate::colony::BuildingKind::GeneLab, [1, 1])
            .expect("Adaptation capture can construct the Gene Lab");
        self.campaign
            .colony
            .place_construction(crate::colony::BuildingKind::PowerPlant, [1, 2])
            .expect("Adaptation capture can power the Gene Lab");
        self.campaign.colony.advance_operation();
        self.campaign.selected_character_id = "kira_voss".to_owned();
        self.state = AppState::GeneLab;
    }

    fn capture_evolution(&mut self) {
        self.capture_gene_lab();
        self.campaign
            .choose_mutation_evolution("kira_voss", "expanded_cortex", &self.data)
            .expect("Adaptation capture evolution is available");
        self.campaign.selected_character_id = "kira_voss".to_owned();
        self.state = AppState::Roster;
    }

    fn capture_mara_evolution(&mut self) {
        self.capture_gene_lab();
        self.campaign
            .choose_mutation_evolution("mara_venn", "razor_plating", &self.data)
            .expect("Mara's Adaptation capture evolution is available");
        self.campaign.selected_character_id = "mara_venn".to_owned();
        self.state = AppState::GeneLab;
    }

    fn capture_ilya_evolution(&mut self) {
        self.capture_gene_lab();
        self.campaign
            .choose_mutation_evolution("ilya_reed", "clean_marrow", &self.data)
            .expect("Ilya's Adaptation capture evolution is available");
        self.campaign.selected_character_id = "ilya_reed".to_owned();
        self.state = AppState::GeneLab;
    }

    fn capture_sol_evolution(&mut self) {
        self.capture_gene_lab();
        self.campaign
            .choose_mutation_evolution("sol_cairn", "lattice_tendons", &self.data)
            .expect("Sol's Adaptation capture evolution is available");
        self.campaign.selected_character_id = "sol_cairn".to_owned();
        self.state = AppState::GeneLab;
    }

    fn capture_escalation(&mut self) {
        self.capture_gene_lab();
        self.campaign.colony.resources.biomass += 10;
        self.campaign
            .choose_mutation_evolution("kira_voss", "expanded_cortex", &self.data)
            .expect("Escalation capture can evolve Kira");
        self.campaign
            .choose_mutation_evolution("mara_venn", "fortress_carapace", &self.data)
            .expect("Escalation capture can evolve Mara");
        self.campaign.strategy.adaptation_operation_completed = true;
        assert!(self.campaign.refresh_adaptation_completion(&self.data));
        self.state = AppState::Colony;
    }

    fn capture_adaptation_operation(&mut self) {
        self.capture_evolution();
        let mission_id = self
            .campaign
            .strategy
            .mission_offers
            .iter()
            .find(|mission| mission.template_id == "adaptation_glass_nerve")
            .expect("Adaptation capture operation is offered")
            .id
            .clone();
        self.campaign
            .strategy
            .select_mission(&mission_id)
            .expect("Adaptation capture operation can be selected");
        self.active_mission = self
            .campaign
            .strategy
            .materialize_selected(&self.data, &self.campaign.colony);
        self.reset_capture_session(AppState::MissionBriefing);
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

    fn capture_pressure(&mut self) {
        self.active_mission.operation_modifier = OperationModifier::BroodFrenzy;
        self.state = AppState::MissionBriefing;
    }

    fn capture_template_operation(
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
        self.active_mission.briefing = if !template.required_phase.is_empty() {
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
        self.active_mission.round_limit = template.round_limit;
        self.active_mission.materials_reward = template.materials_reward;
        self.active_mission.biomass_reward = template.biomass_reward;
        self.active_mission.power_reward = template.power_reward;
        self.active_mission.operation_modifier = modifier;
        self.active_mission.seed = seed;
        self.active_mission.blocked_tiles = layout.blocked_tiles;
        self.active_mission.objective_tile = layout.objective_tile;
        self.active_mission.terrain_costs = layout.terrain_costs;
        self.active_mission.cover_edges = layout.cover_edges;
        self.reset_capture_session(AppState::Tactical);
    }

    fn capture_active_trace(&mut self) {
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

    fn capture_colony_damage(&mut self) {
        self.campaign
            .colony
            .buildings
            .iter_mut()
            .find(|building| building.id == "workshop")
            .expect("capture colony has a workshop")
            .damaged = true;
        self.state = AppState::Colony;
    }

    fn capture_power_outage(&mut self) {
        self.campaign
            .colony
            .buildings
            .iter_mut()
            .find(|building| building.id == "power_plant")
            .expect("capture colony has a power plant")
            .damaged = true;
        self.state = AppState::Colony;
    }

    fn capture_power_construction(&mut self) {
        self.campaign
            .colony
            .select_construction(crate::colony::BuildingKind::PowerPlant)
            .expect("power plants are constructible");
        self.campaign
            .colony
            .place_construction(crate::colony::BuildingKind::PowerPlant, [1, 1])
            .expect("capture plot is open");
        self.state = AppState::Colony;
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
