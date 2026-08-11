use super::*;
use crate::campaign::CampaignState;
use crate::data::GameData;

#[test]
fn destroying_cover_opens_the_blocked_tile() {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let roster = campaign.deployment_roster(&data, &data.mission);
    let mut session = GameSession::new(&data.config, &data.mission, &roster);
    session
        .tactical
        .units
        .iter_mut()
        .find(|unit| Some(&unit.id) == session.tactical.selected_unit.as_ref())
        .unwrap()
        .position = TilePos::new(18, 17);
    let attacker_position = session.selected_unit().unwrap().position;
    let position = TilePos::new(20, 17);
    assert!(session.can_attack_selected_cover(position));
    let tile_beyond = TilePos::new(position.x + 1, position.y);
    assert!(!session.has_line_of_fire(attacker_position, tile_beyond));
    while session.tactical.blocked.contains(&position) {
        session
            .tactical
            .units
            .iter_mut()
            .find(|unit| Some(&unit.id) == session.tactical.selected_unit.as_ref())
            .unwrap()
            .action_points = data.config.max_action_points;
        session.attack_selected_cover(position).unwrap();
    }
    session
        .tactical
        .units
        .iter_mut()
        .find(|unit| Some(&unit.id) == session.tactical.selected_unit.as_ref())
        .unwrap()
        .action_points = 20;
    assert!(!session.tactical.blocked.contains(&position));
    assert!(session.has_line_of_fire(attacker_position, tile_beyond));
    let move_result = session.validate(&Command::Move {
        unit_id: session.tactical.selected_unit.clone().unwrap(),
        to: position,
    });
    assert!(
        move_result.is_ok(),
        "opened tile was not pathable: {move_result:?}"
    );
    assert!(!session
        .tactical
        .destructible_cover
        .iter()
        .any(|cover| cover.position == position));
    assert!(matches!(
        session.tactical.event_log.last(),
        Some(BattleEvent::CoverDestroyed { position: destroyed }) if *destroyed == position
    ));
}
