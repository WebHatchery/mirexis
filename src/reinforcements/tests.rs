use super::*;

#[test]
fn briefing_forecast_matches_the_created_wave_schedule_and_roles() {
    let data = GameData::load().unwrap();
    let mut mission = data.mission.clone();
    mission.objective_kind = ObjectiveKind::Holdout;
    mission.round_limit = 5;
    mission.hostile_faction = "brood".to_owned();
    mission.hostile_unit_ids.clear();
    let roster = data
        .roster
        .iter()
        .map(|unit| UnitState::from_def(unit, data.config.max_action_points))
        .collect::<Vec<_>>();
    let waves = create_waves(&data.config, &mission, &roster);
    let forecast = briefing_forecast(&data, &mission).unwrap();

    assert_eq!(
        waves.iter().map(|wave| wave.round).collect::<Vec<_>>(),
        vec![3, 5]
    );
    assert!(forecast.contains("R3/R5"));
    assert!(forecast.contains("HUNTER+ARTILLERY"));
    assert!(forecast.contains("EAST"));
}

#[test]
fn entry_tiles_telegraph_only_during_the_round_before_arrival() {
    let data = GameData::load().unwrap();
    let mut mission = data.mission.clone();
    mission.objective_kind = ObjectiveKind::Holdout;
    mission.round_limit = 5;
    let mut session = GameSession::new(&data.config, &mission, &data.roster);

    assert!(telegraphed_wave(&session).is_none());
    session.tactical.round = 2;
    let warning = telegraphed_wave(&session).unwrap();
    assert_eq!(warning.round, 3);
    assert!(warning
        .units
        .iter()
        .all(|unit| unit.position.x == data.config.world_width as i32 - 1));
    session.tactical.round = 3;
    assert!(telegraphed_wave(&session).is_none());
}
