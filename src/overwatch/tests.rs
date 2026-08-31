use super::*;
use crate::data::GameData;
use crate::state::Command;
use macroquad_toolkit::grid::TilePos;

#[test]
fn overwatch_button_label_names_unavailable_states() {
    let data = GameData::load().unwrap();
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let mut unit = session.unit("kira_voss").unwrap().clone();

    assert_eq!(overwatch_button_label(None, false), "SELECT UNIT");
    assert_eq!(overwatch_button_label(Some(&unit), true), "OVERWATCH");

    unit.incapacitated = true;
    assert_eq!(overwatch_button_label(Some(&unit), false), "INCAPACITATED");
    unit.incapacitated = false;
    unit.overwatching = true;
    assert_eq!(overwatch_button_label(Some(&unit), false), "ARMED");
    unit.overwatching = false;
    unit.action_points = 0;
    assert_eq!(overwatch_button_label(Some(&unit), false), "NO AP");
    unit.action_points = 1;
    unit.weapon_ap_cost = 2;
    assert_eq!(overwatch_button_label(Some(&unit), false), "NEED MORE AP");
}

#[test]
fn hostile_movement_triggers_one_prepaid_reaction() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data.config, &data.mission, &data.roster);
    let kira = session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == "kira_voss")
        .unwrap();
    kira.position = TilePos::new(6, 3);
    kira.accuracy = 100;
    let hostile = session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.team == Team::Hostile)
        .unwrap();
    hostile.position = TilePos::new(8, 4);
    let hostile_id = hostile.id.clone();

    session
        .execute(Command::SetOverwatch {
            unit_id: "kira_voss".to_owned(),
        })
        .unwrap();
    session.tactical.phase = crate::tactical::TacticalPhase::Enemy;
    let events = session
        .execute(Command::Move {
            unit_id: hostile_id.clone(),
            to: TilePos::new(8, 3),
        })
        .unwrap();

    assert!(events.iter().any(|event| matches!(
        event,
        BattleEvent::ReactionTriggered { target_id, .. } if target_id == &hostile_id
    )));
    assert!(!session.unit("kira_voss").unwrap().overwatching);
}

#[test]
fn reaction_waits_until_a_hostile_enters_the_weapon_envelope() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data.config, &data.mission, &data.roster);
    let kira = session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == "kira_voss")
        .unwrap();
    kira.position = TilePos::new(1, 1);
    kira.weapon_range = 2;
    session
        .execute(Command::SetOverwatch {
            unit_id: "kira_voss".to_owned(),
        })
        .unwrap();
    session.tactical.phase = crate::tactical::TacticalPhase::Enemy;
    let hostile = session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.team == Team::Hostile)
        .unwrap();
    hostile.position = TilePos::new(7, 6);
    let hostile_id = hostile.id.clone();
    let events = session
        .execute(Command::Move {
            unit_id: hostile_id,
            to: TilePos::new(6, 6),
        })
        .unwrap();

    assert!(!events
        .iter()
        .any(|event| matches!(event, BattleEvent::ReactionTriggered { .. })));
    assert!(session.unit("kira_voss").unwrap().overwatching);
}

#[test]
fn untriggered_overwatch_expires_at_the_next_player_phase() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data.config, &data.mission, &data.roster);
    let kira = session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == "kira_voss")
        .unwrap();
    kira.position = TilePos::new(0, 0);
    kira.weapon_range = 1;
    let attack_cost = kira.weapon_ap_cost;
    let starting_ap = kira.action_points;

    session.set_selected_overwatch().unwrap();
    assert_eq!(
        session.unit("kira_voss").unwrap().action_points,
        starting_ap - attack_cost
    );
    session.end_player_phase(&data.config);

    assert_eq!(
        session.tactical.phase,
        crate::tactical::TacticalPhase::Player
    );
    assert!(!session.unit("kira_voss").unwrap().overwatching);
}
