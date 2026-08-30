//! Deterministic UI-reference scene construction.

use super::{AppState, Game, TacticalTargeting};
use crate::data::{ObjectiveKind, OperationModifier, Team};
use crate::state::{BattleEvent, GameSession, ObjectiveState, TacticalPhase};
use macroquad_toolkit::grid::TilePos;

impl Game {
    pub fn begin_capture_scene(&mut self, scene: &str) {
        self.reset_capture_world();
        match scene {
            "title" => self.state = AppState::Title,
            "title_controller" => {
                self.state = AppState::Title;
                self.save_exists = true;
                self.title_focus_continue = true;
                self.title_focus_active = true;
            }
            "title_hover" => {
                self.state = AppState::Title;
                self.title_hover_preview = true;
            }
            "colony" => {
                self.campaign.first_hour.stage = crate::first_hour::FirstHourStage::MeetCoordinator;
                self.state = AppState::Colony;
            }
            "contact" => self.capture_contact(),
            "contact_gear" => self.capture_contact_gear(),
            "contact_event" => self.capture_contact_event(),
            "adaptation" => self.capture_adaptation(),
            "gene_lab" => self.capture_gene_lab(),
            "evolution" => self.capture_evolution(),
            "mara_evolution" => self.capture_mara_evolution(),
            "ilya_evolution" => self.capture_ilya_evolution(),
            "sol_evolution" => self.capture_sol_evolution(),
            "nadi_evolution" => self.capture_nadi_evolution(),
            "escalation" => self.capture_escalation(),
            "escalation_operation" => self.capture_escalation_operation(),
            "escalation_response" => self.capture_escalation_response(),
            "mirexis" => self.capture_mirexis(),
            "mirexis_path" => self.capture_mirexis_path(),
            "redoubt_end" => self.capture_mirexis_end("human_redoubt"),
            "commonwealth_end" => self.capture_mirexis_end("living_commonwealth"),
            "threshold_end" => self.capture_mirexis_end("open_threshold"),
            "finale_debrief" => self.capture_finale_debrief(),
            "adaptation_operation" => self.capture_adaptation_operation(),
            "glass_nerve" => self.capture_template_operation(
                "adaptation_glass_nerve",
                14,
                OperationModifier::AscendantInterference,
            ),
            "three_knives" => self.capture_template_operation(
                "escalation_three_knives",
                16,
                OperationModifier::EscalationCrossfire,
            ),
            "reinforcement_warning" => {
                self.capture_template_operation(
                    "escalation_three_knives",
                    16,
                    OperationModifier::EscalationCrossfire,
                );
                self.session.tactical.round = 2;
                self.session.tactical.event_log = vec![BattleEvent::PhaseStarted {
                    phase: TacticalPhase::Player,
                    round: 2,
                }];
            }
            "thin_shelter" => self.capture_defense_asset(),
            "brood_ability" => self.capture_enemy_ability("nest_suppression", "brood", 7),
            "directorate_ability" => {
                self.capture_enemy_ability("supply_recovery", "directorate", 9)
            }
            "ascendant_ability" => self.capture_enemy_ability("vault_purge", "ascendants", 11),
            "hazard" => self.capture_hazard(),
            "intent" => self.capture_enemy_intent(),
            "action_preview" => self.capture_player_action_preview(),
            "help" => {
                self.reset_capture_session(AppState::Tactical);
                self.show_tactical_help = true;
            }
            "first_hour_guide" => {
                self.state = AppState::Colony;
                self.campaign.first_hour.stage = crate::first_hour::FirstHourStage::SecondOperation;
                self.campaign.first_hour.help_open = true;
                let metrics = &mut self.campaign.first_hour.metrics;
                metrics.elapsed_millis = 2_846_000;
                metrics.first_city_move_millis = Some(18_000);
                metrics.first_city_interaction_millis = Some(46_000);
                metrics.first_tactical_attack_millis = Some(612_000);
                metrics.operation_one_duration_millis = Some(934_000);
                metrics.operation_one_rounds = Some(5);
                metrics.invalid_commands = 3;
                metrics.guide_opens = 2;
            }
            "battle_log" => {
                self.reset_capture_session(AppState::Tactical);
                self.session.end_player_phase(&self.data.config);
                self.show_battle_log = true;
            }
            "combat_feedback" => self.capture_combat_feedback(),
            "phase_replay" => self.capture_phase_replay(),
            "end_phase_guard" => {
                self.reset_capture_session(AppState::Tactical);
                self.end_phase_armed = true;
            }
            "readiness_markers" => self.capture_readiness_markers(),
            "vitality_markers" => self.capture_vitality_markers(),
            "movement_route" => self.capture_movement_route(),
            "cover_edges" => self.capture_cover_edges(),
            "invalid_command" => self.capture_invalid_command(),
            "valid_shot" => self.capture_valid_shot(),
            "threat_range" => self.capture_threat_range(),
            "danger_reach" => self.capture_danger_reach(),
            "line_formation" => self.capture_line_formation(),
            "breakwater" => self.capture_template_operation(
                "escalation_bastion_breakwater",
                18,
                OperationModifier::EscalationCrossfire,
            ),
            "false_heart" => self.capture_template_operation(
                "escalation_living_false_heart",
                19,
                OperationModifier::EscalationCrossfire,
            ),
            "live_wire" => self.capture_template_operation(
                "escalation_lattice_live_wire",
                20,
                OperationModifier::EscalationCrossfire,
            ),
            "last_wall" => self.capture_template_operation(
                "mirexis_redoubt_last_wall",
                22,
                OperationModifier::DirectorateFireControl,
            ),
            "root_choir" => self.capture_template_operation(
                "mirexis_commonwealth_root_choir",
                23,
                OperationModifier::BroodFrenzy,
            ),
            "door_of_light" => self.capture_template_operation(
                "mirexis_threshold_door_of_light",
                24,
                OperationModifier::AscendantInterference,
            ),
            "research" => self.capture_research(),
            "legacy" => self.capture_legacy(),
            "roster" => self.state = AppState::Roster,
            "advanced_roster" => self.capture_advanced_roster(),
            "relationships" => self.capture_relationships(AppState::Roster),
            "trauma" => self.capture_trauma(),
            "bonded_briefing" => self.capture_relationships(AppState::MissionBriefing),
            "briefing" => {
                self.campaign.first_hour.stage = crate::first_hour::FirstHourStage::FirstBriefing;
                self.state = AppState::MissionBriefing;
            }
            "threat_briefing" => {
                self.capture_template_operation(
                    "sporefield_extraction",
                    4,
                    OperationModifier::BroodFrenzy,
                );
                self.state = AppState::MissionBriefing;
            }
            "loadout_briefing" => {
                self.capture_trauma();
                self.state = AppState::MissionBriefing;
            }
            "gameplay" => self.reset_capture_session(AppState::Tactical),
            "first_hour_tactical" => {
                self.reset_capture_session(AppState::Tactical);
                self.campaign.first_hour.stage = crate::first_hour::FirstHourStage::FirstOperation;
                self.campaign.first_hour.lesson = crate::first_hour::TacticalLesson::MoveToCover;
                self.session
                    .tactical
                    .cover_edges
                    .push(crate::data::CoverEdgeDef {
                        position: [6, 18],
                        direction: crate::data::EdgeDirection::North,
                        strength: 25,
                    });
            }
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
            "repair" => self.capture_colony_repair(),
            "power" => self.capture_power_outage(),
            "construction" => self.capture_power_construction(),
            "extraction" => self.capture_extraction(),
            "variant" => self.capture_map_variant(),
            "equipment" => self.capture_equipment_target(),
            "weapon_profile" => self.capture_weapon_profile(),
            "overwatch" => self.capture_overwatch(),
            "class_target" => self.capture_class_target(),
            "breach" => self.capture_breach(),
            "debrief" => self.capture_debrief(),
            "trauma_debrief" => self.capture_trauma_debrief(),
            _ => panic!("unsupported Mirexis capture scene: {scene}"),
        }
    }

    pub(super) fn reset_capture_session(&mut self, state: AppState) {
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
        for (index, research) in self.campaign.strategy.research.iter_mut().enumerate() {
            research.completed = index < 2;
        }
        for event in &mut self.campaign.strategy.character_events {
            event.resolved = true;
        }
        self.state = AppState::Colony;
    }

    fn capture_advanced_roster(&mut self) {
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

    fn capture_relationships(&mut self, state: AppState) {
        let kira_mara = vec!["kira_voss".to_owned(), "mara_venn".to_owned()];
        for _ in 0..3 {
            self.campaign.strengthen_shared_victory(&kira_mara);
        }
        self.campaign
            .strengthen_event_participants(&["kira_voss".to_owned(), "sol_cairn".to_owned()]);
        self.campaign.selected_character_id = "kira_voss".to_owned();
        self.state = state;
    }

    fn capture_trauma(&mut self) {
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

    fn capture_nadi_evolution(&mut self) {
        self.capture_gene_lab();
        self.campaign.colony.resources.biomass += 4;
        self.campaign
            .choose_mutation_evolution("nadi_vale", "predatory_symbiote", &self.data)
            .expect("Nadi's Adaptation capture evolution is available");
        self.campaign.selected_character_id = "nadi_vale".to_owned();
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

    fn capture_escalation_operation(&mut self) {
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

    fn capture_escalation_response(&mut self) {
        self.capture_escalation();
        self.campaign.strategy.escalation_operation_completed = true;
        self.campaign.colony.resources.materials = 60;
        self.campaign.colony.resources.biomass = 20;
        self.campaign.colony.resources.power = 8;
        self.state = AppState::Colony;
    }

    fn capture_mirexis(&mut self) {
        self.capture_escalation_response();
        self.campaign
            .strategy
            .choose_escalation_response("bastion_beacon", &mut self.campaign.colony, &self.data)
            .expect("Mirexis capture can commit its convergence response");
        self.campaign.strategy.escalation_branch_completed = true;
        assert!(self.campaign.refresh_escalation_completion(&self.data));
        self.state = AppState::Colony;
    }

    fn capture_mirexis_path(&mut self) {
        self.capture_mirexis();
        self.campaign
            .strategy
            .choose_mirexis_path("open_threshold", &mut self.campaign.colony, &self.data)
            .expect("Mirexis capture can choose the open threshold");
        self.state = AppState::Colony;
    }

    fn capture_mirexis_end(&mut self, path_id: &str) {
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
        self.state = AppState::Colony;
    }

    fn capture_finale_debrief(&mut self) {
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

    pub(super) fn capture_template_operation(
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

    fn capture_defense_asset(&mut self) {
        self.capture_template_operation("shelter_signal", 6, OperationModifier::BroodFrenzy);
        self.session.tactical.objective_integrity = 7;
    }

    fn capture_equipment_target(&mut self) {
        self.reset_capture_session(AppState::Tactical);
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

    fn capture_weapon_profile(&mut self) {
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
    }

    fn capture_overwatch(&mut self) {
        self.reset_capture_session(AppState::Tactical);
        self.session
            .set_selected_overwatch()
            .expect("capture colonist can enter overwatch");
    }

    fn capture_extraction(&mut self) {
        self.configure_extraction(2, "ISOLATION: LAST TRANSMISSION");
    }

    fn capture_map_variant(&mut self) {
        self.configure_extraction(3, "LAST TRANSMISSION // SOUTHERN APPROACH");
        let selected = self.session.tactical.selected_tile;
        self.tactical_camera = crate::grid_ui::WorldCamera::tactical_view(
            TilePos::new(selected.x + 7, selected.y + 7),
            selected,
            0.78,
        );
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
        self.active_mission.hazards = layout.hazards;
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
            .unit("kira_voss")
            .expect("capture roster includes Kira")
            .position;
        self.targeting = Some(TacticalTargeting::ClassAction {
            unit_id: "ilya_reed".to_owned(),
            target_kind: crate::data::TechniqueTarget::Ally,
        });
    }
}

#[cfg(test)]
mod tests;
