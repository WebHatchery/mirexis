//! Escalation responses and deterministic map-variant regressions.

use super::*;

#[test]
fn escalation_operation_is_phase_gated_and_forces_crossfire() {
    let data = GameData::load().unwrap();
    let mut strategy = StrategyState::new(&data);
    strategy.phase_id = "escalation".to_owned();
    strategy.regenerate_missions(&data);
    assert_eq!(
        strategy.mission_offers[0].template_id,
        "escalation_three_knives"
    );
    assert_eq!(
        strategy.mission_offers[0].operation_modifier,
        OperationModifier::EscalationCrossfire
    );
    let mission = strategy.mission_offers[0].clone();
    let outcome = MissionOutcome {
        result: ObjectiveState::Victory,
        colonists_deployed: 3,
        colonists_incapacitated: Vec::new(),
        hostiles_neutralised: 3,
        materials_awarded: mission.materials_reward,
        biomass_awarded: mission.biomass_reward,
        power_awarded: mission.power_reward,
    };
    strategy.resolve_mission(&outcome, &mission, &data);
    assert!(strategy.escalation_operation_completed);
}

#[test]
fn escalation_responses_trade_distinct_resources_for_distinct_strategy_effects() {
    let data = GameData::load().unwrap();

    let mut bastion = StrategyState::new(&data);
    let mut colony = ColonyState::new();
    assert!(bastion
        .choose_escalation_response("bastion_beacon", &mut colony, &data)
        .is_err());
    bastion.escalation_operation_completed = true;
    bastion.phase_id = "escalation".to_owned();
    let materials_before = colony.resources.materials;
    let threat_before = bastion.threats[0].operations_until;
    bastion
        .choose_escalation_response("bastion_beacon", &mut colony, &data)
        .unwrap();
    assert_eq!(colony.resources.materials, materials_before - 30);
    assert_eq!(bastion.threats[0].operations_until, threat_before + 2);
    assert_eq!(
        bastion.mission_offers[0].template_id,
        "escalation_bastion_breakwater"
    );

    let mut decoy = StrategyState::new(&data);
    let mut colony = ColonyState::new();
    decoy.escalation_operation_completed = true;
    decoy.phase_id = "escalation".to_owned();
    colony.resources.biomass = 20;
    let attention_before = decoy.factions[0].attention;
    decoy
        .choose_escalation_response("living_decoy", &mut colony, &data)
        .unwrap();
    assert_eq!(colony.resources.biomass, 12);
    assert_eq!(decoy.factions[0].attention, attention_before - 8);
    assert_eq!(
        decoy.mission_offers[0].template_id,
        "escalation_living_false_heart"
    );

    let mut lattice = StrategyState::new(&data);
    let mut colony = ColonyState::new();
    lattice.escalation_operation_completed = true;
    lattice.phase_id = "escalation".to_owned();
    lattice.regenerate_missions(&data);
    let power_before = colony.resources.power;
    lattice
        .choose_escalation_response("weaponized_lattice", &mut colony, &data)
        .unwrap();
    assert_eq!(colony.resources.power, power_before - 4);
    assert_eq!(
        lattice.mission_offers[0].template_id,
        "escalation_lattice_live_wire"
    );
    let base_reward = lattice.selected_mission().unwrap().materials_reward;
    assert_eq!(
        lattice
            .materialize_selected(&data, &colony)
            .materials_reward,
        base_reward + 8
    );
    assert!(lattice
        .choose_escalation_response("living_decoy", &mut colony, &data)
        .is_err());
    let branch = lattice.selected_mission().unwrap().clone();
    let outcome = MissionOutcome {
        result: ObjectiveState::Victory,
        colonists_deployed: 3,
        colonists_incapacitated: Vec::new(),
        hostiles_neutralised: 3,
        materials_awarded: branch.materials_reward,
        biomass_awarded: branch.biomass_reward,
        power_awarded: branch.power_reward,
    };
    lattice.adaptation_complete = true;
    lattice.resolve_mission(&outcome, &branch, &data);
    assert!(lattice.escalation_branch_completed);
    assert!(lattice.refresh_escalation_completion());
    assert_eq!(lattice.phase_id, "mirexis");
}

#[test]
fn mission_seed_selects_a_repeatable_safe_map_variant() {
    let data = GameData::load().unwrap();
    let colony = ColonyState::new();
    let template = data
        .campaign
        .mission_templates
        .iter()
        .find(|template| template.id == "courier_extraction")
        .unwrap();
    let mut strategy = StrategyState::new(&data);
    let mut instance = strategy.instantiate(template);
    instance.seed = 2;
    strategy.selected_mission_id = instance.id.clone();
    strategy.mission_offers = vec![instance.clone()];
    let authored = strategy.materialize_selected(&data, &colony);
    instance.seed = 3;
    strategy.mission_offers = vec![instance.clone()];
    let variant = strategy.materialize_selected(&data, &colony);
    assert_ne!(authored.blocked_tiles, variant.blocked_tiles);
    assert_eq!(
        variant.blocked_tiles,
        strategy.materialize_selected(&data, &colony).blocked_tiles
    );
    assert!(!variant.blocked_tiles.contains(&variant.objective_tile));
}
