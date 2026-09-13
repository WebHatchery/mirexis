//! Focused tests integration tests.

use super::*;

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
    let campaign = mirexis::campaign::CampaignState::new(&data);
    let mut layouts = std::collections::HashSet::new();
    for template in &data.campaign.mission_templates {
        let mut strategy = StrategyState::new(&data);
        let instance = strategy.instantiate(template);
        strategy.selected_mission_id = instance.id.clone();
        strategy.mission_offers = vec![instance];
        let mission = strategy.materialize_selected(&data, &colony);
        assert_eq!(mission.hostile_faction, template.faction);
        let deployment = campaign.deployment_roster(&data, &mission);
        let positions = deployment
            .iter()
            .map(|unit| unit.position)
            .collect::<std::collections::HashSet<_>>();
        assert_eq!(
            positions.len(),
            deployment.len(),
            "{} overlaps",
            template.id
        );
        assert!(deployment.iter().all(|unit| {
            unit.position[0] >= 0
                && unit.position[1] >= 0
                && unit.position[0] < data.config.world_width as i32
                && unit.position[1] < data.config.world_height as i32
                && unit.position != mission.objective_tile
                && !mission.blocked_tiles.contains(&unit.position)
                && !mission
                    .hazards
                    .iter()
                    .any(|hazard| hazard.position == unit.position)
        }));
        assert!(layouts.insert(mission.blocked_tiles));
    }
    assert_eq!(layouts.len(), data.campaign.mission_templates.len());
}

#[test]
fn every_reinforcement_operation_enters_safely_across_the_large_world() {
    use macroquad_toolkit::grid::TilePos;
    use mirexis::campaign::CampaignState;
    use mirexis::state::GameSession;

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
        let allied_position = session
            .tactical
            .units
            .iter()
            .find(|unit| unit.team == mirexis::data::Team::Colony)
            .unwrap()
            .position;

        for wave in session.tactical.reinforcement_waves.clone() {
            let mut arrival = session.clone();
            arrival.tactical.round = wave.round;
            for unit in &wave.units {
                arrival.tactical.blocked.insert(unit.position);
            }
            mirexis::reinforcements::deploy(&mut arrival, data.config.max_action_points);
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
            assert!(
                arrived.iter().all(|unit| {
                    positions_are_connected(
                        [unit.position.x, unit.position.y],
                        [allied_position.x, allied_position.y],
                        &arrival
                            .tactical
                            .blocked
                            .iter()
                            .map(|position| [position.x, position.y])
                            .collect(),
                        data.config.world_width as i32,
                        data.config.world_height as i32,
                    )
                }),
                "{} R{} strands a reinforcement fallback",
                template.id,
                wave.round
            );
        }
    }
}

#[test]
fn every_formation_and_hostile_front_can_cross_each_materialized_battlefield() {
    use macroquad_toolkit::grid::TilePos;
    use mirexis::campaign::CampaignState;
    use mirexis::data::Team;
    use mirexis::formation::{self, FormationKind};
    use std::collections::HashSet;

    let data = GameData::load().unwrap();
    let colony = ColonyState::new();
    let campaign = CampaignState::new(&data);
    for template in &data.campaign.mission_templates {
        let mut strategy = StrategyState::new(&data);
        let instance = strategy.instantiate(template);
        strategy.selected_mission_id = instance.id.clone();
        strategy.mission_offers = vec![instance];
        let mission = strategy.materialize_selected(&data, &colony);
        let blocked = mission
            .blocked_tiles
            .iter()
            .copied()
            .collect::<HashSet<_>>();
        for formation in [
            FormationKind::Wedge,
            FormationKind::Line,
            FormationKind::Column,
        ] {
            let mut deployment = campaign.deployment_roster(&data, &mission);
            formation::apply(&mut deployment, &mission, &data.config, formation);
            let first_colonist = deployment
                .iter()
                .find(|unit| unit.team == Team::Colony)
                .unwrap()
                .position;
            for unit in &deployment {
                let destination = if unit.team == Team::Colony {
                    mission.objective_tile
                } else {
                    first_colonist
                };
                assert!(
                    positions_are_connected(
                        unit.position,
                        destination,
                        &blocked,
                        data.config.world_width as i32,
                        data.config.world_height as i32,
                    ),
                    "{} {:?} strands {} at {:?}",
                    template.id,
                    formation,
                    unit.id,
                    TilePos::new(unit.position[0], unit.position[1])
                );
            }
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
