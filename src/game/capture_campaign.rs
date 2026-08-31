//! Deterministic strategic and progression capture-scene construction.

use super::{AppState, Game};
use crate::data::{OperationModifier, Team};
use crate::state::ObjectiveState;

impl Game {
    pub(super) fn capture_research(&mut self) {
        for (index, research) in self.campaign.strategy.research.iter_mut().enumerate() {
            research.completed = index < 2;
        }
        for event in &mut self.campaign.strategy.character_events {
            event.resolved = true;
        }
        self.colony_operations_open = true;
        self.state = AppState::Colony;
    }

    pub(super) fn capture_field_notes(&mut self) {
        let notes = [
            (
                "mara_venn",
                "Mara Venn",
                crate::colony_story::current_beat("mara_venn", 0, None, None)
                    .expect("field notes capture has Mara's arrival note"),
            ),
            (
                "ilya_reed",
                "Ilya Reed",
                crate::colony_story::current_beat("ilya_reed", 1, Some(true), None)
                    .expect("field notes capture has Ilya's victory note"),
            ),
            (
                "kira_voss",
                "Kira Voss",
                crate::colony_story::commons_meal_beat("kira_voss")
                    .expect("field notes capture has Kira's commons note"),
            ),
            (
                "sol_cairn",
                "Sol Cairn",
                crate::colony_story::phase_beat("adaptation", "sol_cairn")
                    .expect("field notes capture has Sol's phase note"),
            ),
        ];
        for (character_id, speaker, beat) in notes {
            self.campaign
                .colony_story
                .acknowledge_note(character_id, speaker, beat);
        }
        self.colony_operations_open = true;
        self.selected_field_note = 3;
        self.show_field_notes = true;
        self.state = AppState::Colony;
    }

    pub(super) fn capture_memorial(&mut self) {
        let kira = self
            .campaign
            .roster
            .iter_mut()
            .find(|character| character.id == "kira_voss")
            .expect("memorial capture includes Kira");
        crate::trauma::record_incapacitation(kira, 1);

        let mara = self
            .campaign
            .roster
            .iter_mut()
            .find(|character| character.id == "mara_venn")
            .expect("memorial capture includes Mara");
        mara.injuries.push(crate::campaign::InjuryRecord {
            id: "capture_mireline_fracture".to_owned(),
            name: "Mireline fracture".to_owned(),
            recovery_operations: 1,
        });

        let sol = self
            .campaign
            .roster
            .iter_mut()
            .find(|character| character.id == "sol_cairn")
            .expect("memorial capture includes Sol");
        sol.event_legacies.push(crate::campaign::CharacterLegacy {
            id: "capture_route_held".to_owned(),
            name: "The Route Held".to_owned(),
            stat: "movement".to_owned(),
            amount: 1,
        });

        self.campaign.operations_completed = 3;
        self.campaign.colony.resources.materials = 84;
        self.colony_operations_open = true;
        self.show_memorial = true;
        self.state = AppState::Colony;
    }

    pub(super) fn capture_advanced_roster(&mut self) {
        self.campaign.strategy.phase_id = "adaptation".to_owned();
        self.campaign.colony.resources.materials = 480;
        self.campaign.selected_character_id = "mara_venn".to_owned();
        let mara = self
            .campaign
            .roster
            .iter_mut()
            .find(|character| character.id == "mara_venn")
            .expect("advanced roster capture includes Mara");
        mara.level = 3;
        if !mara.class_history.contains(&"soldier".to_owned()) {
            mara.class_history.push("soldier".to_owned());
        }
        self.state = AppState::Roster;
    }

    pub(super) fn capture_relationships(&mut self, state: AppState) {
        let kira_mara = vec!["kira_voss".to_owned(), "mara_venn".to_owned()];
        for _ in 0..3 {
            self.campaign.strengthen_shared_victory(&kira_mara);
        }
        self.campaign
            .strengthen_event_participants(&["kira_voss".to_owned(), "sol_cairn".to_owned()]);
        self.campaign.selected_character_id = "kira_voss".to_owned();
        self.state = state;
    }

    pub(super) fn capture_trauma(&mut self) {
        let kira = self
            .campaign
            .roster
            .iter_mut()
            .find(|character| character.id == "kira_voss")
            .expect("capture roster includes Kira");
        crate::trauma::record_incapacitation(kira, 1);
        self.campaign.selected_character_id = "kira_voss".to_owned();
        self.state = AppState::Roster;
    }

    pub(super) fn capture_contact(&mut self) {
        self.campaign.strategy.isolation_victories = 3;
        self.campaign.strategy.first_assault_repulsed = true;
        self.campaign.strategy.research[0].completed = true;
        self.campaign
            .strategy
            .refresh_isolation_completion(&mut self.campaign.colony);
        self.colony_operations_open = true;
        self.state = AppState::Colony;
    }

    pub(super) fn capture_contact_gear(&mut self) {
        self.capture_contact();
        self.campaign
            .strategy
            .choose_contact_protocol("ascendant_capacitor", &mut self.campaign.colony, &self.data)
            .expect("capture Contact protocol is available");
        self.campaign.strategy.contact_trace_completed = true;
        self.state = AppState::Roster;
    }

    pub(super) fn capture_contact_event(&mut self) {
        self.capture_contact_gear();
        self.campaign
            .resolve_first_character_event(&self.data)
            .expect("first Isolation capture event resolves");
        self.campaign
            .resolve_first_character_event(&self.data)
            .expect("second Isolation capture event resolves");
        self.colony_operations_open = true;
        self.state = AppState::Colony;
    }

    pub(super) fn capture_adaptation(&mut self) {
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
        self.colony_operations_open = true;
        self.state = AppState::Colony;
    }

    pub(super) fn capture_gene_lab(&mut self) {
        self.capture_adaptation();
        self.campaign
            .colony
            .place_construction(crate::colony::BuildingKind::GeneLab, [1, 1])
            .expect("Adaptation capture can construct the Gene Lab");
        self.campaign
            .colony
            .place_construction(crate::colony::BuildingKind::PowerPlant, [1, 4])
            .expect("Adaptation capture can power the Gene Lab");
        self.campaign.colony.advance_operation();
        self.campaign.selected_character_id = "kira_voss".to_owned();
        self.state = AppState::GeneLab;
    }

    pub(super) fn capture_evolution(&mut self) {
        self.capture_gene_lab();
        self.campaign
            .choose_mutation_evolution("kira_voss", "expanded_cortex", &self.data)
            .expect("Adaptation capture evolution is available");
        self.campaign.selected_character_id = "kira_voss".to_owned();
        self.state = AppState::Roster;
    }

    pub(super) fn capture_mara_evolution(&mut self) {
        self.capture_gene_lab();
        self.campaign
            .choose_mutation_evolution("mara_venn", "razor_plating", &self.data)
            .expect("Mara's Adaptation capture evolution is available");
        self.campaign.selected_character_id = "mara_venn".to_owned();
        self.state = AppState::GeneLab;
    }

    pub(super) fn capture_ilya_evolution(&mut self) {
        self.capture_gene_lab();
        self.campaign
            .choose_mutation_evolution("ilya_reed", "clean_marrow", &self.data)
            .expect("Ilya's Adaptation capture evolution is available");
        self.campaign.selected_character_id = "ilya_reed".to_owned();
        self.state = AppState::GeneLab;
    }

    pub(super) fn capture_sol_evolution(&mut self) {
        self.capture_gene_lab();
        self.campaign
            .choose_mutation_evolution("sol_cairn", "lattice_tendons", &self.data)
            .expect("Sol's Adaptation capture evolution is available");
        self.campaign.selected_character_id = "sol_cairn".to_owned();
        self.state = AppState::GeneLab;
    }

    pub(super) fn capture_nadi_evolution(&mut self) {
        self.capture_gene_lab();
        self.campaign.colony.resources.biomass += 4;
        self.campaign
            .choose_mutation_evolution("nadi_vale", "predatory_symbiote", &self.data)
            .expect("Nadi's Adaptation capture evolution is available");
        self.campaign.selected_character_id = "nadi_vale".to_owned();
        self.state = AppState::GeneLab;
    }

    pub(super) fn capture_escalation(&mut self) {
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
        self.colony_operations_open = true;
        self.state = AppState::Colony;
    }

    pub(super) fn capture_escalation_operation(&mut self) {
        self.capture_escalation();
        let mission_id = self
            .campaign
            .strategy
            .mission_offers
            .iter()
            .find(|mission| mission.template_id == "escalation_three_knives")
            .expect("Escalation capture operation is offered")
            .id
            .clone();
        self.campaign
            .strategy
            .select_mission(&mission_id)
            .expect("Escalation capture operation can be selected");
        self.active_mission = self
            .campaign
            .strategy
            .materialize_selected(&self.data, &self.campaign.colony);
        self.reset_capture_session(AppState::MissionBriefing);
    }

    pub(super) fn capture_escalation_response(&mut self) {
        self.capture_escalation();
        self.campaign.strategy.escalation_operation_completed = true;
        self.campaign.colony.resources.materials = 60;
        self.campaign.colony.resources.biomass = 20;
        self.campaign.colony.resources.power = 8;
        self.colony_operations_open = true;
        self.state = AppState::Colony;
    }

    pub(super) fn capture_mirexis(&mut self) {
        self.capture_escalation_response();
        self.campaign
            .strategy
            .choose_escalation_response("bastion_beacon", &mut self.campaign.colony, &self.data)
            .expect("Mirexis capture can commit its convergence response");
        self.campaign.strategy.escalation_branch_completed = true;
        assert!(self.campaign.refresh_escalation_completion(&self.data));
        self.colony_operations_open = true;
        self.state = AppState::Colony;
    }

    pub(super) fn capture_mirexis_path(&mut self) {
        self.capture_mirexis();
        self.campaign
            .strategy
            .choose_mirexis_path("open_threshold", &mut self.campaign.colony, &self.data)
            .expect("Mirexis capture can choose the open threshold");
        self.colony_operations_open = true;
        self.state = AppState::Colony;
    }

    pub(super) fn capture_mirexis_end(&mut self, path_id: &str) {
        self.capture_mirexis();
        self.campaign.colony.resources.materials = 100;
        self.campaign.colony.resources.biomass = 30;
        self.campaign.colony.resources.power = 20;
        self.campaign
            .strategy
            .choose_mirexis_path(path_id, &mut self.campaign.colony, &self.data)
            .expect("Mirexis ending capture can commit its path");
        self.campaign.strategy.mirexis_operation_completed = true;
        assert!(self
            .campaign
            .strategy
            .refresh_mirexis_completion(&self.data));
        self.campaign.strategy.regenerate_missions(&self.data);
        self.campaign.operations_completed = 1;
        self.colony_operations_open = true;
        self.state = AppState::Colony;
    }

    pub(super) fn capture_finale_debrief(&mut self) {
        self.capture_template_operation(
            "mirexis_threshold_door_of_light",
            24,
            OperationModifier::AscendantInterference,
        );
        self.campaign.strategy.escalation_complete = true;
        self.campaign.strategy.mirexis_path_id = "open_threshold".to_owned();
        self.campaign.strategy.mirexis_operation_completed = true;
        assert!(self
            .campaign
            .strategy
            .refresh_mirexis_completion(&self.data));
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

    pub(super) fn capture_adaptation_operation(&mut self) {
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

    pub(super) fn capture_legacy(&mut self) {
        self.campaign
            .resolve_first_character_event(&self.data)
            .expect("first capture event resolves");
        self.campaign
            .resolve_first_character_event(&self.data)
            .expect("second capture event resolves");
        self.campaign.selected_character_id = "mara_venn".to_owned();
        self.state = AppState::Roster;
    }

    pub(super) fn capture_pressure(&mut self) {
        self.active_mission.operation_modifier = OperationModifier::BroodFrenzy;
        self.state = AppState::MissionBriefing;
    }
}
