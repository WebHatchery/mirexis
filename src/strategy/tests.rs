use super::*;

#[test]
fn isolation_starts_with_pressure_threats_and_content() {
    let data = GameData::load().unwrap();
    let strategy = StrategyState::new(&data);
    assert_eq!(strategy.phase_id, "isolation");
    assert_eq!(strategy.factions.len(), 3);
    assert_eq!(strategy.active_threat().unwrap().operations_until, 3);
    assert!(!strategy.research.is_empty());
    assert!(!strategy.character_events.is_empty());
}

#[test]
fn mission_resolution_advances_pressure_and_generates_seeded_offers() {
    let data = GameData::load().unwrap();
    let mut a = StrategyState::new(&data);
    let mut b = a.clone();
    let mission = a.selected_mission().unwrap().clone();
    let outcome = MissionOutcome {
        result: ObjectiveState::Victory,
        colonists_deployed: 4,
        colonists_incapacitated: Vec::new(),
        hostiles_neutralised: 3,
        materials_awarded: 30,
        biomass_awarded: 0,
        power_awarded: 0,
    };
    a.resolve_mission(&outcome, &mission, &data);
    b.resolve_mission(&outcome, &mission, &data);
    assert_eq!(a.mission_offers, b.mission_offers);
    assert_eq!(a.active_threat().unwrap().operations_until, 2);
    assert_eq!(a.mission_offers.len(), 2);
}

#[test]
fn isolation_completion_requires_victories_research_and_a_repulsed_assault() {
    let data = GameData::load().unwrap();
    let mut strategy = StrategyState::new(&data);
    strategy.isolation_victories = 2;
    let mut defense = strategy.selected_mission().unwrap().clone();
    defense.map_recipe = "colony_defense".to_owned();
    let outcome = MissionOutcome {
        result: ObjectiveState::Victory,
        colonists_deployed: 3,
        colonists_incapacitated: Vec::new(),
        hostiles_neutralised: 3,
        materials_awarded: 0,
        biomass_awarded: 0,
        power_awarded: 0,
    };
    strategy.resolve_mission(&outcome, &defense, &data);
    assert_eq!(strategy.isolation_victories, 3);
    assert!(strategy.first_assault_repulsed);

    let mut colony = ColonyState::new();
    assert!(!strategy.refresh_isolation_completion(&mut colony));
    strategy.research[0].completed = true;
    assert!(strategy.refresh_isolation_completion(&mut colony));
    assert_eq!(strategy.phase_id, "contact");
    assert_eq!(colony.resources.alien_components, 2);
    assert!(!strategy.refresh_isolation_completion(&mut colony));
    assert_eq!(colony.resources.alien_components, 2);
}

#[test]
fn contact_protocols_spend_the_reward_and_change_future_recovery() {
    let data = GameData::load().unwrap();
    for (protocol_id, expected_bonus) in [
        ("directorate_requisition", (10, 0, 0)),
        ("brood_cultivation", (0, 4, 0)),
        ("ascendant_capacitor", (0, 0, 2)),
    ] {
        let mut strategy = StrategyState::new(&data);
        let mut colony = ColonyState::new();
        assert!(strategy
            .choose_contact_protocol(protocol_id, &mut colony, &data)
            .is_err());
        strategy.isolation_victories = 3;
        strategy.first_assault_repulsed = true;
        strategy.research[0].completed = true;
        assert!(strategy.refresh_isolation_completion(&mut colony));
        strategy
            .choose_contact_protocol(protocol_id, &mut colony, &data)
            .unwrap();
        assert_eq!(colony.resources.alien_components, 0);
        let base = strategy.selected_mission().unwrap().clone();
        assert_eq!(
            base.template_id,
            format!("{}_contact_trace", protocol_id.split('_').next().unwrap())
        );
        assert!(strategy.mission_offers.iter().all(|offer| {
            data.campaign
                .mission_templates
                .iter()
                .find(|template| template.id == offer.template_id)
                .is_none_or(|template| {
                    template.required_protocol.is_empty()
                        || template.required_protocol == protocol_id
                })
        }));
        assert!(strategy
            .choose_contact_protocol(protocol_id, &mut colony, &data)
            .is_err());
        let mission = strategy.materialize_selected(&data, &colony);
        assert_eq!(
            (
                mission.materials_reward - base.materials_reward,
                mission.biomass_reward - base.biomass_reward,
                mission.power_reward - base.power_reward,
            ),
            expected_bonus
        );
    }
}

#[test]
fn victorious_matching_contact_trace_unlocks_its_aftermath() {
    let data = GameData::load().unwrap();
    let mut strategy = StrategyState::new(&data);
    let mut colony = ColonyState::new();
    strategy.isolation_victories = 3;
    strategy.first_assault_repulsed = true;
    strategy.research[0].completed = true;
    strategy.refresh_isolation_completion(&mut colony);
    strategy
        .choose_contact_protocol("ascendant_capacitor", &mut colony, &data)
        .unwrap();
    let mission = strategy.selected_mission().unwrap().clone();
    let outcome = MissionOutcome {
        result: ObjectiveState::Victory,
        colonists_deployed: 3,
        colonists_incapacitated: Vec::new(),
        hostiles_neutralised: 3,
        materials_awarded: 0,
        biomass_awarded: 0,
        power_awarded: 0,
    };
    assert!(!strategy.contact_trace_completed);
    strategy.resolve_mission(&outcome, &mission, &data);
    assert!(strategy.contact_trace_completed);
}

#[test]
fn contact_completion_requires_trace_aftermath_and_prototype() {
    let data = GameData::load().unwrap();
    let mut strategy = StrategyState::new(&data);
    let mut colony = ColonyState::new();
    strategy.isolation_victories = 3;
    strategy.first_assault_repulsed = true;
    strategy.research[0].completed = true;
    strategy.refresh_isolation_completion(&mut colony);
    strategy.contact_trace_completed = true;
    assert!(!strategy.refresh_contact_completion(false, false));
    assert!(!strategy.refresh_contact_completion(true, false));
    assert!(strategy.refresh_contact_completion(true, true));
    assert_eq!(strategy.phase_id, "adaptation");
    assert!(!strategy.refresh_contact_completion(true, true));
}

#[test]
fn expired_threat_generates_a_defense_mission_from_colony_placement() {
    let data = GameData::load().unwrap();
    let mut strategy = StrategyState::new(&data);
    let outcome = MissionOutcome {
        result: ObjectiveState::Victory,
        colonists_deployed: 4,
        colonists_incapacitated: Vec::new(),
        hostiles_neutralised: 3,
        materials_awarded: 30,
        biomass_awarded: 0,
        power_awarded: 0,
    };
    for _ in 0..3 {
        let mission = strategy.selected_mission().unwrap().clone();
        strategy.resolve_mission(&outcome, &mission, &data);
    }
    let defense = strategy.selected_mission().unwrap();
    assert_eq!(defense.map_recipe, "colony_defense");
    let mut colony = ColonyState::new();
    colony
        .place_construction(crate::colony::BuildingKind::Barricade, [1, 1])
        .unwrap();
    colony.advance_operation();
    let materialized = strategy.materialize_selected(&data, &colony);
    assert!(materialized.blocked_tiles.contains(&[3, 2]));
}

#[test]
fn completed_research_changes_future_mission_materialization() {
    let data = GameData::load().unwrap();
    let colony = ColonyState::new();
    let mut strategy = StrategyState::new(&data);
    let base_reward = strategy.selected_mission().unwrap().materials_reward;
    strategy
        .research
        .iter_mut()
        .find(|research| research.id == "salvage_doctrine")
        .unwrap()
        .completed = true;
    assert_eq!(
        strategy
            .materialize_selected(&data, &colony)
            .materials_reward,
        base_reward + 8
    );

    strategy
        .research
        .iter_mut()
        .find(|research| research.id == "field_fortifications")
        .unwrap()
        .completed = true;
    strategy.threats[0].operations_until = 0;
    strategy.generate_missions(&data);
    assert_eq!(
        strategy
            .materialize_selected(&data, &colony)
            .cover_integrity,
        10
    );
}

#[test]
fn high_faction_attention_adds_its_pressure_modifier() {
    let data = GameData::load().unwrap();
    for (faction_id, expected) in [
        ("directorate", OperationModifier::DirectorateFireControl),
        ("brood", OperationModifier::BroodFrenzy),
        ("ascendants", OperationModifier::AscendantInterference),
    ] {
        let mut strategy = StrategyState::new(&data);
        strategy
            .factions
            .iter_mut()
            .find(|faction| faction.id == faction_id)
            .unwrap()
            .attention = 20;
        let template = data
            .campaign
            .mission_templates
            .iter()
            .find(|template| template.faction == faction_id)
            .unwrap();
        assert_eq!(strategy.instantiate(template).operation_modifier, expected);
    }
}

#[test]
fn each_faction_template_materializes_its_own_battlefield() {
    let data = GameData::load().unwrap();
    let colony = ColonyState::new();
    let mut layouts = std::collections::HashSet::new();
    for template in &data.campaign.mission_templates {
        let mut strategy = StrategyState::new(&data);
        let instance = strategy.instantiate(template);
        strategy.selected_mission_id = instance.id.clone();
        strategy.mission_offers = vec![instance];
        let mission = strategy.materialize_selected(&data, &colony);
        assert_eq!(mission.hostile_faction, template.faction);
        assert!(layouts.insert(mission.blocked_tiles));
    }
    assert_eq!(layouts.len(), data.campaign.mission_templates.len());
}

#[test]
fn every_reinforcement_operation_enters_safely_across_the_large_world() {
    use crate::campaign::CampaignState;
    use crate::state::GameSession;
    use macroquad_toolkit::grid::TilePos;

    let data = GameData::load().unwrap();
    let colony = ColonyState::new();
    let campaign = CampaignState::new(&data);
    for template in data.campaign.mission_templates.iter().filter(|template| {
        matches!(
            template.objective_kind,
            ObjectiveKind::Holdout | ObjectiveKind::SignalTrace
        )
    }) {
        let mut strategy = StrategyState::new(&data);
        let instance = strategy.instantiate(template);
        strategy.selected_mission_id = instance.id.clone();
        strategy.mission_offers = vec![instance];
        let mission = strategy.materialize_selected(&data, &colony);
        let roster = campaign.deployment_roster(&data, &mission);
        let session = GameSession::new(&data.config, &mission, &roster);
        assert!(!session.tactical.reinforcement_waves.is_empty());

        for wave in session.tactical.reinforcement_waves.clone() {
            let mut arrival = session.clone();
            arrival.tactical.round = wave.round;
            for unit in &wave.units {
                arrival.tactical.blocked.insert(unit.position);
            }
            crate::reinforcements::deploy(&mut arrival, data.config.max_action_points);
            let arrived = arrival
                .tactical
                .units
                .iter()
                .filter(|unit| unit.id.ends_with(&format!("_reinforcement_{}", wave.round)))
                .collect::<Vec<_>>();
            assert_eq!(
                arrived.len(),
                wave.units.len(),
                "{} R{}",
                template.id,
                wave.round
            );
            let positions = arrived
                .iter()
                .map(|unit| unit.position)
                .collect::<std::collections::HashSet<_>>();
            assert_eq!(positions.len(), arrived.len());
            assert!(arrived.iter().all(|unit| {
                unit.position
                    .in_bounds(data.config.world_width, data.config.world_height)
                    && !arrival.tactical.blocked.contains(&unit.position)
                    && unit.position
                        != TilePos::new(mission.objective_tile[0], mission.objective_tile[1])
            }));
        }
    }
}

#[test]
fn new_faction_operations_have_distinct_objectives_and_recovery() {
    let data = GameData::load().unwrap();
    let sporefield = data
        .campaign
        .mission_templates
        .iter()
        .find(|template| template.id == "sporefield_extraction")
        .unwrap();
    assert_eq!(sporefield.objective_kind, ObjectiveKind::Extraction);
    assert_eq!(sporefield.biomass_reward, 8);
    let vault = data
        .campaign
        .mission_templates
        .iter()
        .find(|template| template.id == "vault_purge")
        .unwrap();
    assert_eq!(vault.objective_kind, ObjectiveKind::EliminateAll);
    assert_eq!(vault.power_reward, 3);
    assert_ne!(sporefield.map_recipe, vault.map_recipe);
}

#[test]
fn adaptation_operation_is_phase_gated_and_prioritized() {
    let data = GameData::load().unwrap();
    let mut strategy = StrategyState::new(&data);
    strategy.regenerate_missions(&data);
    assert!(strategy
        .mission_offers
        .iter()
        .all(|mission| mission.template_id != "adaptation_glass_nerve"));
    strategy.phase_id = "adaptation".to_owned();
    strategy.regenerate_missions(&data);
    assert_eq!(
        strategy.mission_offers[0].template_id,
        "adaptation_glass_nerve"
    );
}

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
