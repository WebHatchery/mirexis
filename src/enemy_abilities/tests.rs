use super::*;
use crate::data::GameData;
use crate::tactical::TacticalPhase;
use macroquad_toolkit::grid::TilePos;

fn faction_session(faction: &str) -> (GameSession, String) {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data.config, &data.mission, &data.roster);
    session.tactical.phase = TacticalPhase::Enemy;
    let unit = session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.faction.as_deref() == Some(faction))
        .unwrap();
    unit.position = TilePos::new(5, 4);
    let id = unit.id.clone();
    (session, id)
}

#[test]
fn brood_surge_trades_an_action_for_immediate_mobility() {
    let (mut session, unit_id) = faction_session("brood");
    let before = session.unit(&unit_id).unwrap().action_points;
    session
        .execute(Command::ActivateEnemyAbility {
            unit_id: unit_id.clone(),
            target_id: None,
        })
        .unwrap();

    let unit = session.unit(&unit_id).unwrap();
    assert_eq!(unit.action_points, before - 1);
    assert!(unit.has_status(StatusKind::Quickened));
}

#[test]
fn directorate_suppression_disrupts_a_visible_colonist() {
    let (mut session, unit_id) = faction_session("directorate");
    let target = session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.team == Team::Colony)
        .unwrap();
    target.position = TilePos::new(5, 3);
    let target_id = target.id.clone();
    session
        .execute(Command::ActivateEnemyAbility {
            unit_id,
            target_id: Some(target_id.clone()),
        })
        .unwrap();

    assert!(session
        .unit(&target_id)
        .unwrap()
        .has_status(StatusKind::Disrupted));
}

#[test]
fn ascendant_phase_ward_persists_into_the_player_response() {
    let (mut session, unit_id) = faction_session("ascendants");
    session
        .execute(Command::ActivateEnemyAbility {
            unit_id: unit_id.clone(),
            target_id: None,
        })
        .unwrap();

    let unit = session.unit(&unit_id).unwrap();
    assert!(unit.has_status(StatusKind::Guarded));
    assert_eq!(unit.effective_armour(), unit.armour + 2);
}
