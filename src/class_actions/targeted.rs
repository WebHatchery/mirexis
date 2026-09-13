//! Target validation and execution helpers for class actions.

use super::*;

pub fn valid_breacher_tile(
    session: &GameSession,
    unit: &crate::state::UnitState,
    tile: TilePos,
) -> bool {
    let hostile_target = session.tactical.units.iter().any(|target| {
        target.position == tile && target.team == Team::Hostile && !target.incapacitated
    });
    let cover_target = session
        .tactical
        .destructible_cover
        .iter()
        .any(|cover| cover.position == tile && cover.health > 0);
    (hostile_target || cover_target) && breacher_route(session, unit, tile).is_some()
}

pub fn valid_fortifier_tile(
    session: &GameSession,
    unit: &crate::state::UnitState,
    tile: TilePos,
) -> bool {
    manhattan(unit.position, tile) > 0
        && manhattan(unit.position, tile) <= 3
        && session.tactical.fog.is_valid(tile)
        && !session.tactical.blocked.contains(&tile)
        && !session
            .tactical
            .units
            .iter()
            .any(|other| other.position == tile)
        && !session
            .tactical
            .hazards
            .iter()
            .any(|hazard| hazard.position == tile)
        && tile != session.tactical.objective_tile
        && !session
            .tactical
            .destructible_cover
            .iter()
            .any(|cover| cover.position == tile)
        && !session
            .tactical
            .cover_edges
            .iter()
            .any(|edge| edge.position == [tile.x, tile.y])
        && session.has_line_of_fire(unit.position, tile)
}

pub fn breacher_route(
    session: &GameSession,
    unit: &crate::state::UnitState,
    target: TilePos,
) -> Option<Vec<TilePos>> {
    let candidates = [
        TilePos::new(target.x - 1, target.y),
        TilePos::new(target.x + 1, target.y),
        TilePos::new(target.x, target.y - 1),
        TilePos::new(target.x, target.y + 1),
    ];
    candidates
        .into_iter()
        .filter(|candidate| {
            session.tactical.fog.is_valid(*candidate)
                && !session.tactical.blocked.contains(candidate)
                && !session
                    .tactical
                    .units
                    .iter()
                    .any(|other| !other.incapacitated && other.position == *candidate)
        })
        .filter_map(|candidate| {
            let path = session.movement_path(&unit.id, candidate)?;
            let cost = path_cost(&path, &session.tactical.terrain_costs);
            (cost <= 2 && session.has_line_of_fire(candidate, target)).then_some((path, cost))
        })
        .min_by_key(|(path, cost)| (*cost, path.last().unwrap().y, path.last().unwrap().x))
        .map(|(path, _)| path)
}

pub fn execute_breacher(
    session: &mut GameSession,
    unit_id: &str,
    target: TilePos,
    events: &mut Vec<BattleEvent>,
) {
    let Some(path) = breacher_route(
        session,
        session.unit(unit_id).expect("validated breacher user"),
        target,
    ) else {
        return;
    };
    if path.last().copied() != path.first().copied() {
        events.push(reposition_without_cost(session, unit_id, &path));
        events.extend(crate::hazards::resolve_after_move(session, unit_id));
    }
    if let Some(target_id) = session
        .tactical
        .units
        .iter()
        .find(|unit| unit.position == target && unit.team == Team::Hostile && !unit.incapacitated)
        .map(|unit| unit.id.clone())
    {
        damage_target(session, &target_id, 2, events);
        if !session.unit(&target_id).unwrap().incapacitated {
            apply_status(session, &target_id, StatusKind::Marked, 1, events);
        }
    } else {
        events.extend(crate::cover_actions::damage_cover(session, target, 4));
    }
}

pub fn execute_fortifier(session: &mut GameSession, unit_id: &str, tile: TilePos) {
    let origin = session
        .unit(unit_id)
        .expect("validated fortifier user")
        .position;
    session.tactical.blocked.insert(tile);
    session
        .tactical
        .destructible_cover
        .push(crate::state::DestructibleCover {
            position: tile,
            health: 8,
            max_health: 8,
        });
    session.tactical.cover_edges.push(CoverEdgeDef {
        position: [tile.x, tile.y],
        direction: cover_direction(origin, tile),
        strength: 25,
    });
}

pub fn reposition_without_cost(
    session: &mut GameSession,
    unit_id: &str,
    path: &[TilePos],
) -> BattleEvent {
    let from = path[0];
    let to = *path.last().expect("breacher route has a destination");
    let unit = session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == unit_id)
        .expect("validated breacher user");
    unit.position = to;
    unit.facing = crate::tactical::UnitFacing::toward(from, to);
    unit.presentation_state = crate::tactical::UnitAnimationState::Move;
    unit.presentation_seconds = 0.38;
    if session.tactical.selected_unit.as_deref() == Some(unit_id) {
        session.tactical.selected_tile = to;
    }
    BattleEvent::UnitMoved {
        unit_id: unit_id.to_owned(),
        path: path.to_vec(),
        cost: 0,
    }
}

pub fn cover_direction(origin: TilePos, position: TilePos) -> EdgeDirection {
    if origin.x < position.x {
        EdgeDirection::West
    } else if origin.x > position.x {
        EdgeDirection::East
    } else if origin.y < position.y {
        EdgeDirection::North
    } else {
        EdgeDirection::South
    }
}

pub fn target_matches(
    session: &GameSession,
    unit_id: &str,
    target_id: Option<&str>,
    team: Team,
    range: i32,
    extra: impl FnOnce(&crate::state::UnitState) -> bool,
) -> Result<(), RuleError> {
    let Some((unit, target)) = session
        .unit(unit_id)
        .zip(target_id.and_then(|target_id| session.unit(target_id)))
    else {
        return Err(RuleError::InvalidTarget);
    };
    if target.team != team {
        return Err(RuleError::WrongTeam);
    }
    if target.incapacitated {
        return Err(RuleError::Incapacitated);
    }
    if manhattan(unit.position, target.position) > range {
        return Err(RuleError::OutOfRange);
    }
    extra(target).then_some(()).ok_or(RuleError::InvalidTarget)
}

pub fn apply_status(
    session: &mut GameSession,
    unit_id: &str,
    kind: StatusKind,
    phases: u8,
    events: &mut Vec<BattleEvent>,
) {
    let unit = session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == unit_id)
        .unwrap();
    if let Some(status) = unit.statuses.iter_mut().find(|status| status.kind == kind) {
        status.remaining_phases = status.remaining_phases.max(phases);
    } else {
        unit.statuses.push(StatusEffect {
            kind,
            remaining_phases: phases,
        });
    }
    events.push(BattleEvent::StatusApplied {
        unit_id: unit_id.to_owned(),
        status: kind,
    });
}

pub fn heal_target(session: &mut GameSession, target_id: &str, events: &mut Vec<BattleEvent>) {
    heal_target_amount(session, target_id, 4, events);
}

pub fn heal_target_amount(
    session: &mut GameSession,
    target_id: &str,
    amount: i32,
    events: &mut Vec<BattleEvent>,
) {
    let target = session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == target_id)
        .unwrap();
    let before = target.health;
    target.health = (target.health + amount).min(target.max_health);
    events.push(BattleEvent::UnitHealed {
        unit_id: target_id.to_owned(),
        amount: target.health - before,
        remaining: target.health,
    });
}

pub fn damage_target(
    session: &mut GameSession,
    target_id: &str,
    damage: i32,
    events: &mut Vec<BattleEvent>,
) {
    let target = session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == target_id)
        .unwrap();
    target.health = (target.health - damage).max(0);
    events.push(BattleEvent::DamageApplied {
        target_id: target_id.to_owned(),
        amount: damage,
        remaining: target.health,
    });
    if target.health == 0 {
        target.incapacitated = true;
        events.push(BattleEvent::UnitIncapacitated {
            unit_id: target_id.to_owned(),
        });
    }
}
