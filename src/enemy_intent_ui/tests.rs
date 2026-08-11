use super::*;
use crate::data::{GameData, Team};

#[test]
fn weapon_range_respects_distance_and_blocked_line_of_fire() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data.config, &data.mission, &data.roster);
    let hostile = session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.team == Team::Hostile)
        .unwrap();
    hostile.position = TilePos::new(6, 3);
    hostile.weapon_range = 3;
    let hostile_id = hostile.id.clone();
    session.tactical.blocked.clear();
    session.tactical.blocked.insert(TilePos::new(6, 2));

    let threatened = threatened_tiles(&session, &hostile_id);

    assert!(threatened.contains(&TilePos::new(9, 3)));
    assert!(!threatened.contains(&TilePos::new(6, 1)));
    assert!(!threatened.contains(&TilePos::new(10, 3)));
}

#[test]
fn danger_reach_adds_one_legal_move_before_the_attack_envelope() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data.config, &data.mission, &data.roster);
    session.tactical.blocked.clear();
    let hostile = session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.team == Team::Hostile)
        .unwrap();
    hostile.position = TilePos::new(6, 3);
    hostile.weapon_range = 2;
    hostile.weapon_ap_cost = 2;
    let hostile_id = hostile.id.clone();

    assert!(!threatened_tiles(&session, &hostile_id).contains(&TilePos::new(9, 3)));
    assert!(danger_reach_tiles(&session, &hostile_id, 5).contains(&TilePos::new(9, 3)));
}
