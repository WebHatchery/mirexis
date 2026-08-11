use super::*;
use crate::data::GameData;
use crate::tactical::{ObjectiveState, TacticalPhase};
use macroquad_toolkit::grid::TilePos;

#[test]
fn hostile_fire_can_destroy_and_fail_a_defense_asset() {
    let data = GameData::load().unwrap();
    let mut mission = data.mission.clone();
    mission.objective_kind = ObjectiveKind::DefendAsset;
    mission.objective_tile = [8, 4];
    let mut session = GameSession::new(&data.config, &mission, &data.roster);
    session.tactical.phase = TacticalPhase::Enemy;
    let hostile = session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.team == Team::Hostile)
        .unwrap();
    hostile.position = TilePos::new(8, 3);
    hostile.accuracy = 100;
    hostile.weapon_damage = 20;
    let hostile_id = hostile.id.clone();

    let events = session
        .execute(Command::AttackObjective {
            attacker_id: hostile_id,
        })
        .unwrap();

    assert!(events
        .iter()
        .any(|event| matches!(event, BattleEvent::ObjectiveDestroyed)));
    assert_eq!(session.tactical.objective_state, ObjectiveState::Failed);
}

#[test]
fn hostile_ai_prioritizes_an_exposed_defense_asset() {
    let data = GameData::load().unwrap();
    let mut mission = data.mission.clone();
    mission.objective_kind = ObjectiveKind::DefendAsset;
    mission.objective_tile = [8, 4];
    let mut session = GameSession::new(&data.config, &mission, &data.roster);
    session.tactical.phase = TacticalPhase::Enemy;
    for unit in &mut session.tactical.units {
        if unit.team == Team::Hostile {
            unit.position = TilePos::new(8, 3);
            unit.accuracy = 100;
        }
    }

    crate::tactical_ai::resolve_enemy_phase(&mut session);

    assert!(session.tactical.objective_integrity < STARTING_INTEGRITY);
    assert!(session.tactical.event_log.iter().any(|event| matches!(
        event,
        BattleEvent::AttackRolled { target_id, .. } if target_id == OBJECTIVE_TARGET_ID
    )));
}

#[test]
fn an_intact_asset_wins_when_its_defense_timer_expires() {
    let data = GameData::load().unwrap();
    let mut mission = data.mission.clone();
    mission.objective_kind = ObjectiveKind::DefendAsset;
    mission.round_limit = 1;
    let mut session = GameSession::new(&data.config, &mission, &data.roster);
    for unit in &mut session.tactical.units {
        if unit.team == Team::Hostile {
            unit.incapacitated = true;
        }
    }

    session.end_player_phase(&data.config);

    assert_eq!(session.tactical.objective_state, ObjectiveState::Victory);
    assert_eq!(session.tactical.objective_integrity, STARTING_INTEGRITY);
}
