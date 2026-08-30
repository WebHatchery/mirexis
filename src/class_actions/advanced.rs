//! Advanced class actions that combine movement, rescue, and hazard control.

use super::{apply_status, reposition_without_cost};
use crate::data::Team;
use crate::state::{BattleEvent, GameSession, UnitState};
use crate::tactical::{manhattan, path_cost, StatusKind};
use macroquad_toolkit::grid::TilePos;

const CARDINAL: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

pub(super) fn valid_rescue_target(
    session: &GameSession,
    rescuer: &UnitState,
    target_id: Option<&str>,
) -> bool {
    let Some(ally) = target_id.and_then(|id| session.unit(id)) else {
        return false;
    };
    ally.id != rescuer.id
        && ally.team == Team::Colony
        && manhattan(rescuer.position, ally.position) <= 6
        && (ally.incapacitated || ally.health < ally.max_health)
        && rescue_plan(session, rescuer, ally).is_some()
}

pub(super) fn valid_chorus_warden_tile(
    session: &GameSession,
    unit: &UnitState,
    tile: TilePos,
) -> bool {
    session
        .tactical
        .hazards
        .iter()
        .any(|hazard| hazard.position == tile)
        && session.tactical.fog.is_valid(tile)
        && manhattan(unit.position, tile) <= 4
        && session.has_line_of_fire(unit.position, tile)
        && !session
            .tactical
            .obscuring_fields
            .iter()
            .any(|field| field.center == tile)
}

pub(super) fn execute_rescue_specialist(
    session: &mut GameSession,
    unit_id: &str,
    target_id: &str,
    events: &mut Vec<BattleEvent>,
) {
    let Some((rescuer, ally)) = session
        .unit(unit_id)
        .zip(session.unit(target_id))
        .map(|(rescuer, ally)| (rescuer.clone(), ally.clone()))
    else {
        return;
    };
    let Some(plan) = rescue_plan(session, &rescuer, &ally) else {
        return;
    };
    if plan.route.last() != plan.route.first() {
        events.push(reposition_without_cost(session, unit_id, &plan.route));
    }
    let rescue_position = *plan.route.last().expect("rescue route has a destination");
    if rescue_position != plan.rescuer_destination {
        events.push(reposition_without_cost(
            session,
            unit_id,
            &[rescue_position, plan.rescuer_destination],
        ));
    }
    if ally.position != plan.ally_destination {
        events.push(reposition_without_cost(
            session,
            target_id,
            &[ally.position, plan.ally_destination],
        ));
    }
    stabilise_target(session, target_id, events);
    apply_status(session, unit_id, StatusKind::Guarded, 1, events);
    apply_status(session, target_id, StatusKind::Guarded, 1, events);
}

pub(super) fn execute_chorus_warden(
    session: &mut GameSession,
    unit_id: &str,
    tile: TilePos,
    events: &mut Vec<BattleEvent>,
) {
    let Some(hazard) = session
        .tactical
        .hazards
        .iter()
        .find(|hazard| hazard.position == tile)
        .copied()
    else {
        return;
    };
    session
        .tactical
        .hazards
        .retain(|entry| entry.position != tile);
    session
        .tactical
        .obscuring_fields
        .push(crate::state::ObscuringField {
            center: tile,
            radius: 1,
            remaining_phases: 1,
        });
    events.push(BattleEvent::HazardConverted {
        unit_id: unit_id.to_owned(),
        position: tile,
        kind: hazard.kind,
    });
    let allies = session
        .tactical
        .units
        .iter()
        .filter(|ally| {
            ally.team == Team::Colony && !ally.incapacitated && manhattan(ally.position, tile) <= 1
        })
        .map(|ally| ally.id.clone())
        .collect::<Vec<_>>();
    for ally_id in allies {
        apply_status(session, &ally_id, StatusKind::Regenerating, 1, events);
    }
    apply_status(session, unit_id, StatusKind::Hindered, 1, events);
}

#[derive(Debug, Clone)]
struct RescuePlan {
    route: Vec<TilePos>,
    ally_destination: TilePos,
    rescuer_destination: TilePos,
}

fn rescue_plan(session: &GameSession, rescuer: &UnitState, ally: &UnitState) -> Option<RescuePlan> {
    let nearest_hostile = session
        .tactical
        .units
        .iter()
        .filter(|unit| unit.team == Team::Hostile && !unit.incapacitated)
        .min_by_key(|unit| {
            (
                manhattan(unit.position, ally.position),
                unit.position.y,
                unit.position.x,
            )
        })
        .map(|unit| unit.position);
    CARDINAL
        .into_iter()
        .filter_map(|(dx, dy)| {
            let rescue_position = offset(ally.position, dx, dy);
            let rescuer_destination = offset(rescue_position, dx, dy);
            if !open_pair_tile(session, rescue_position, &rescuer.id, &ally.id)
                || !open_pair_tile(session, rescuer_destination, &rescuer.id, &ally.id)
                || rescuer_destination == rescuer.position
            {
                return None;
            }
            let route = session.movement_path(&rescuer.id, rescue_position)?;
            let cost = path_cost(&route, &session.tactical.terrain_costs);
            if cost > rescuer.effective_move_range() {
                return None;
            }
            let danger_distance =
                nearest_hostile.map_or(0, |hostile| manhattan(hostile, rescue_position));
            let threat_count = session
                .tactical
                .units
                .iter()
                .filter(|hostile| {
                    hostile.team == Team::Hostile
                        && !hostile.incapacitated
                        && (session.has_line_of_fire(hostile.position, rescue_position)
                            || session.has_line_of_fire(hostile.position, rescuer_destination))
                })
                .count() as i32;
            Some((
                RescuePlan {
                    route,
                    ally_destination: rescue_position,
                    rescuer_destination,
                },
                (
                    danger_distance,
                    -threat_count,
                    -i32::from(cost),
                    -rescue_position.y,
                    -rescue_position.x,
                ),
            ))
        })
        .max_by_key(|(_, score)| *score)
        .map(|(plan, _)| plan)
}

fn open_pair_tile(session: &GameSession, tile: TilePos, rescuer_id: &str, ally_id: &str) -> bool {
    session.tactical.fog.is_valid(tile)
        && !session.tactical.blocked.contains(&tile)
        && tile != session.tactical.objective_tile
        && !session
            .tactical
            .hazards
            .iter()
            .any(|hazard| hazard.position == tile)
        && !session
            .tactical
            .units
            .iter()
            .any(|unit| unit.position == tile && unit.id != rescuer_id && unit.id != ally_id)
}

fn offset(origin: TilePos, dx: i32, dy: i32) -> TilePos {
    TilePos::new(origin.x + dx, origin.y + dy)
}

fn stabilise_target(session: &mut GameSession, target_id: &str, events: &mut Vec<BattleEvent>) {
    let target = session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == target_id)
        .expect("validated rescue target");
    let before = target.health;
    if target.incapacitated {
        target.health = 1.min(target.max_health);
        target.incapacitated = false;
    } else {
        target.health = (target.health + 2).min(target.max_health);
    }
    target.presentation_state = crate::tactical::UnitAnimationState::Idle;
    target.presentation_seconds = 0.0;
    events.push(BattleEvent::UnitHealed {
        unit_id: target_id.to_owned(),
        amount: target.health - before,
        remaining: target.health,
    });
}
