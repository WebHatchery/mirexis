use super::*;
use crate::colony::ColonyState;
use crate::strategy::StrategyState;

#[test]
fn three_power_crossfire_rates_above_isolation_recovery() {
    let data = GameData::load().unwrap();
    let mut strategy = StrategyState::new(&data);
    let routine = strategy.selected_mission().unwrap().clone();
    let mut crossfire = routine.clone();
    crossfire.objective_kind = ObjectiveKind::Holdout;
    crossfire.round_limit = 6;
    crossfire.map_recipe = "contested_rift".to_owned();
    crossfire.hostile_unit_ids = vec![
        "directorate_rifle_a".to_owned(),
        "brood_stalker_b".to_owned(),
        "ascendant_warden".to_owned(),
    ];
    crossfire.operation_modifier = OperationModifier::EscalationCrossfire;
    strategy.mission_offers = vec![crossfire];

    assert!(
        for_instance(&strategy.mission_offers[0], &data).score
            > for_instance(&routine, &data).score
    );
    assert_eq!(
        for_instance(&strategy.mission_offers[0], &data).level,
        DangerLevel::Extreme
    );

    for modifier in [
        OperationModifier::MirexisRedoubt,
        OperationModifier::MirexisCommonwealth,
        OperationModifier::MirexisThreshold,
    ] {
        let mut engine_effect = routine.clone();
        engine_effect.operation_modifier = modifier;
        assert_eq!(
            for_instance(&engine_effect, &data).score,
            for_instance(&routine, &data).score
        );
    }
}

#[test]
fn offer_and_materialized_mission_share_the_same_rating() {
    let data = GameData::load().unwrap();
    let strategy = StrategyState::new(&data);
    let offer = strategy.selected_mission().unwrap();
    let mission = strategy.materialize_selected(&data, &ColonyState::new());

    assert_eq!(for_instance(offer, &data), for_mission(&mission, &data));
}
