//! Campaign progression, contacts, and phase-gate regressions.

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
fn character_event_affordance_tracks_available_food() {
    let data = GameData::load().unwrap();
    let strategy = StrategyState::new(&data);
    let event = strategy.available_event().unwrap();

    assert!(strategy.can_resolve_first_event(event.food_cost));
    assert!(!strategy.can_resolve_first_event(event.food_cost.saturating_sub(1)));
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
        .place_construction(mirexis::colony::BuildingKind::Barricade, [1, 1])
        .unwrap();
    colony.advance_operation();
    let materialized = strategy.materialize_selected(&data, &colony);
    assert!(materialized.blocked_tiles.contains(&[3, 2]));
}

#[test]
fn powered_watchtower_materializes_as_stronger_directional_cover() {
    let data = GameData::load().unwrap();
    let mut strategy = StrategyState::new(&data);
    strategy.mission_offers[0].map_recipe = "colony_defense".to_owned();
    let mut colony = ColonyState::new();
    colony
        .place_construction(mirexis::colony::BuildingKind::Watchtower, [1, 1])
        .unwrap();
    colony.advance_operation();

    let mission = strategy.materialize_selected(&data, &colony);
    let tower_edge = mission
        .cover_edges
        .iter()
        .find(|edge| edge.position == [3, 2])
        .expect("the placed watchtower should map to a tactical cover edge");
    assert_eq!(tower_edge.strength, 45);
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
