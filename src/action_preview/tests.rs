use super::*;
use crate::data::{GameData, HazardKind, Team};
use crate::tactical::HazardTile;

#[test]
fn movement_preview_reports_validated_cost_and_landing_hazard() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data.config, &data.mission, &data.roster);
    let start = session.selected_unit().unwrap().position;
    let tile = TilePos::new(start.x + 1, start.y);
    session.tactical.hazards.push(HazardTile {
        position: tile,
        kind: HazardKind::FireLane,
    });

    assert!(matches!(
        for_tile(&session, tile),
        Some(ActionPreview::Move {
            cost: 1,
            hazard: Some(HazardKind::FireLane),
            path,
        }) if path.first() == Some(&start) && path.last() == Some(&tile)
    ));
}

#[test]
fn attack_preview_matches_live_hit_chance_and_armour_damage() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data.config, &data.mission, &data.roster);
    let selected = session.selected_unit().unwrap().position;
    let hostile = session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.team == Team::Hostile)
        .unwrap();
    hostile.position = TilePos::new(selected.x + 2, selected.y);
    let hostile_tile = hostile.position;

    assert!(matches!(
        for_tile(&session, hostile_tile),
        Some(ActionPreview::Attack { hit_chance, damage, .. }) if hit_chance > 0 && damage > 0
    ));
}

#[test]
fn invalid_preview_explains_blocked_shots_and_moves() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data.config, &data.mission, &data.roster);
    let selected_id = session.tactical.selected_unit.clone().unwrap();
    session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == selected_id)
        .unwrap()
        .position = TilePos::new(3, 3);
    let hostile = session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.team == Team::Hostile)
        .unwrap();
    hostile.position = TilePos::new(5, 3);
    let hostile_tile = hostile.position;
    session.tactical.blocked.insert(TilePos::new(4, 3));

    assert_eq!(
        for_tile(&session, hostile_tile),
        Some(ActionPreview::Invalid {
            action: "ATTACK",
            reason: RuleError::NoLineOfFire,
        })
    );
    assert_eq!(
        for_tile(&session, TilePos::new(4, 3)),
        Some(ActionPreview::Invalid {
            action: "MOVE",
            reason: RuleError::NoPath,
        })
    );
}
