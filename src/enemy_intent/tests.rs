use super::*;
use crate::data::GameData;

fn session_and_hostile() -> (GameSession, String) {
    let data = GameData::load().unwrap();
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let id = session
        .tactical
        .units
        .iter()
        .find(|unit| unit.team == Team::Hostile)
        .unwrap()
        .id
        .clone();
    (session, id)
}

#[test]
fn preview_uses_enemy_phase_rules_without_mutating_the_live_session() {
    let (mut session, hostile_id) = session_and_hostile();
    let colonist_position = session
        .tactical
        .units
        .iter()
        .find(|unit| unit.team == Team::Colony)
        .unwrap()
        .position;
    session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == hostile_id)
        .unwrap()
        .position = TilePos::new(colonist_position.x + 1, colonist_position.y);
    let original_phase = session.tactical.phase;

    let intent = preview(&session, &hostile_id, 6).unwrap();

    assert!(matches!(intent.action, IntentAction::AttackUnit(_)));
    assert_eq!(session.tactical.phase, original_phase);
}

#[test]
fn distant_hostile_forecast_names_ability_target_and_next_step() {
    let (mut session, hostile_id) = session_and_hostile();
    session.tactical.blocked.clear();
    for unit in &mut session.tactical.units {
        if unit.id == hostile_id {
            unit.position = TilePos::new(11, 7);
        } else if unit.team == Team::Hostile {
            unit.incapacitated = true;
        } else {
            unit.position = TilePos::new(0, unit.position.y.min(3));
        }
    }
    let intent = preview(&session, &hostile_id, 6).unwrap();

    assert!(intent.ability.is_some());
    assert!(matches!(intent.action, IntentAction::Advance { .. }));
}
