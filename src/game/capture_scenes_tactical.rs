//! Capture builders for tactical, phase, and combat-reference screens.

use super::*;
use crate::data::OperationModifier;
use crate::state::{BattleEvent, TacticalPhase};

impl Game {
    pub(super) fn capture_tactical_scene(&mut self, scene: &str) -> bool {
        match scene {
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
                true
            }
            "thin_shelter" => {
                self.capture_defense_asset();
                true
            }
            "brood_ability" => {
                self.capture_enemy_ability("nest_suppression", "brood", 7);
                true
            }
            "directorate_ability" => {
                self.capture_enemy_ability("supply_recovery", "directorate", 9);
                true
            }
            "ascendant_ability" => {
                self.capture_enemy_ability("vault_purge", "ascendants", 11);
                true
            }
            "hazard" => {
                self.capture_hazard();
                true
            }
            "intent" => {
                self.capture_enemy_intent();
                true
            }
            "action_preview" => {
                self.capture_player_action_preview();
                true
            }
            "help" => {
                self.reset_capture_session(AppState::Tactical);
                self.show_tactical_help = true;
                true
            }
            "battle_log" => {
                self.capture_battle_log();
                true
            }
            "combat_feedback" => {
                self.capture_combat_feedback();
                true
            }
            "phase_replay" => {
                self.capture_phase_replay();
                true
            }
            "end_phase_guard" => {
                self.reset_capture_session(AppState::Tactical);
                self.end_phase_armed = true;
                true
            }
            "readiness_markers" => {
                self.capture_readiness_markers();
                true
            }
            "vitality_markers" => {
                self.capture_vitality_markers();
                true
            }
            "movement_route" => {
                self.capture_movement_route();
                true
            }
            "cover_edges" => {
                self.capture_cover_edges();
                true
            }
            "invalid_command" => {
                self.capture_invalid_command();
                true
            }
            "valid_shot" => {
                self.capture_valid_shot();
                true
            }
            "threat_range" => {
                self.capture_threat_range();
                true
            }
            "danger_reach" => {
                self.capture_danger_reach();
                true
            }
            "line_formation" => {
                self.capture_line_formation();
                true
            }
            "breakwater" => {
                self.capture_template_operation(
                    "escalation_bastion_breakwater",
                    18,
                    OperationModifier::EscalationCrossfire,
                );
                true
            }
            "false_heart" => {
                self.capture_template_operation(
                    "escalation_living_false_heart",
                    19,
                    OperationModifier::EscalationCrossfire,
                );
                true
            }
            "live_wire" => {
                self.capture_template_operation(
                    "escalation_lattice_live_wire",
                    20,
                    OperationModifier::EscalationCrossfire,
                );
                true
            }
            "last_wall" => {
                self.capture_template_operation(
                    "mirexis_redoubt_last_wall",
                    22,
                    OperationModifier::DirectorateFireControl,
                );
                true
            }
            "root_choir" => {
                self.capture_template_operation(
                    "mirexis_commonwealth_root_choir",
                    23,
                    OperationModifier::BroodFrenzy,
                );
                true
            }
            "gameplay" => {
                self.reset_capture_session(AppState::Tactical);
                true
            }
            "first_hour_tactical" => {
                self.reset_capture_session(AppState::Tactical);
                self.campaign.first_hour.stage = crate::first_hour::FirstHourStage::FirstOperation;
                self.campaign.first_hour.lesson = crate::first_hour::TacticalLesson::MoveToCover;
                super::first_hour_flow::prepare_first_hour_tactical_session(
                    &mut self.session,
                    &self.active_mission,
                );
                true
            }
            "first_hour_attack" => {
                self.capture_valid_shot();
                self.campaign.first_hour.stage = crate::first_hour::FirstHourStage::FirstOperation;
                self.campaign.first_hour.lesson = crate::first_hour::TacticalLesson::Attack;
                true
            }
            "first_hour_enemy_phase" => {
                self.capture_first_hour_enemy_phase();
                true
            }
            "first_hour_objective" => {
                self.reset_capture_session(AppState::Tactical);
                self.tactical_panel_open = true;
                self.campaign.first_hour.stage = crate::first_hour::FirstHourStage::FirstOperation;
                self.campaign.first_hour.lesson = crate::first_hour::TacticalLesson::Objective;
                let objective = self.session.tactical.objective_tile;
                let selected_id = self
                    .session
                    .tactical
                    .selected_unit
                    .clone()
                    .expect("objective capture selects a colonist");
                self.session
                    .tactical
                    .units
                    .iter_mut()
                    .find(|unit| unit.id == selected_id)
                    .expect("objective capture colonist exists")
                    .position = objective;
                self.session.tactical.selected_tile = objective;
                true
            }
            "first_hour_replay" => {
                self.capture_first_hour_replay();
                true
            }
            "first_hour_ability" => {
                self.reset_capture_session(AppState::Tactical);
                self.tactical_panel_open = true;
                self.campaign.first_hour.stage = crate::first_hour::FirstHourStage::FirstOperation;
                self.campaign.first_hour.lesson = crate::first_hour::TacticalLesson::Ability;
                super::first_hour_flow::prepare_first_hour_ability_selection(&mut self.session);
                true
            }
            "second_operation_tactical" => {
                self.capture_valid_shot();
                self.campaign.operations_completed = 1;
                self.campaign.first_hour.stage =
                    crate::first_hour::FirstHourStage::SecondOperationTactical;
                self.campaign.first_hour.lesson = crate::first_hour::TacticalLesson::ApplyLearning;
                self.campaign.first_hour.investment_name = "survey_uplink".to_owned();
                true
            }
            "pressure" => {
                self.capture_pressure();
                true
            }
            "sporefield" => {
                self.capture_template_operation(
                    "sporefield_extraction",
                    4,
                    OperationModifier::BroodFrenzy,
                );
                true
            }
            "vault" => {
                self.capture_template_operation(
                    "vault_purge",
                    5,
                    OperationModifier::AscendantInterference,
                );
                true
            }
            "black_channel" => {
                self.capture_template_operation(
                    "directorate_contact_trace",
                    8,
                    OperationModifier::DirectorateFireControl,
                );
                true
            }
            "living_chorus" => {
                self.capture_template_operation(
                    "brood_contact_trace",
                    10,
                    OperationModifier::BroodFrenzy,
                );
                true
            }
            "open_circuit" => {
                self.capture_template_operation(
                    "ascendant_contact_trace",
                    12,
                    OperationModifier::AscendantInterference,
                );
                true
            }
            "trace_active" => {
                self.capture_active_trace();
                true
            }
            "damage" => {
                self.capture_colony_damage();
                true
            }
            "repair" => {
                self.capture_colony_repair();
                true
            }
            "power" => {
                self.capture_power_outage();
                true
            }
            "construction" => {
                self.capture_power_construction();
                true
            }
            "extraction" => {
                self.capture_extraction();
                true
            }
            "variant" => {
                self.capture_map_variant();
                true
            }
            "equipment" => {
                self.capture_equipment_target();
                true
            }
            "ninth_resonance" => {
                self.capture_ninth_resonance();
                true
            }
            "weapon_profile" => {
                self.capture_weapon_profile();
                true
            }
            "overwatch" => {
                self.capture_overwatch();
                true
            }
            "class_target" => {
                self.capture_class_target();
                true
            }
            "breach" => {
                self.capture_breach();
                true
            }
            "debrief" => {
                self.capture_debrief();
                true
            }
            "trauma_debrief" => {
                self.capture_trauma_debrief();
                true
            }
            _ => false,
        }
    }
}
