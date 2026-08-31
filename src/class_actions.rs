//! Class-specific tactical actions and their status effects.

use crate::data::{CoverEdgeDef, EdgeDirection, Team, TechniqueTarget};
use crate::state::{BattleEvent, Command, CommandCost, GameSession, RuleError};
use crate::tactical::{manhattan, path_cost, StatusEffect, StatusKind};
use macroquad_toolkit::grid::TilePos;

mod advanced;

pub(crate) fn action_name(class_id: &str) -> Option<&'static str> {
    match class_id {
        "soldier" => Some("STEADY AIM"),
        "defender" => Some("BRACE"),
        "scout" => Some("SURGE"),
        "medic" => Some("FIELD DRESSING"),
        "engineer" => Some("SHOCK DRONE"),
        "psionic" => Some("NEURAL DISRUPT"),
        "biotech" => Some("SPORE WARD"),
        "vanguard" => Some("HOLD THE LINE"),
        "pathfinder" => Some("GHOST VECTOR"),
        "lifewright" => Some("VITAL CASCADE"),
        "null_adept" => Some("NULL LANCE"),
        "breacher" => Some("MAKE AN ENTRANCE"),
        "fortifier" => Some("RAISE BASTION"),
        "rescue_specialist" => Some("CARRY THROUGH"),
        "chorus_warden" => Some("BORROWED WEATHER"),
        _ => None,
    }
}

pub(crate) fn target_kind(class_id: &str) -> Option<TechniqueTarget> {
    match class_id {
        "medic" | "lifewright" => Some(TechniqueTarget::Ally),
        "engineer" | "psionic" | "null_adept" => Some(TechniqueTarget::Hostile),
        "breacher" | "fortifier" => Some(TechniqueTarget::Tile),
        "chorus_warden" => Some(TechniqueTarget::Tile),
        "rescue_specialist" => Some(TechniqueTarget::Ally),
        _ => None,
    }
}

pub(crate) fn requires_target(class_id: &str) -> bool {
    target_kind(class_id).is_some()
}

impl GameSession {
    pub fn activate_selected_class_action(&mut self) -> Result<Vec<BattleEvent>, RuleError> {
        let unit_id = self
            .tactical
            .selected_unit
            .clone()
            .ok_or(RuleError::UnknownUnit)?;
        self.execute(Command::ActivateClassAction {
            unit_id,
            target_id: None,
            target_tile: None,
        })
    }

    pub fn can_activate_selected_class_action(&self) -> bool {
        self.tactical.selected_unit.as_ref().is_some_and(|unit_id| {
            self.validate(&Command::ActivateClassAction {
                unit_id: unit_id.clone(),
                target_id: None,
                target_tile: None,
            })
            .is_ok()
        })
    }

    pub fn can_target_class_action(&self, unit_id: &str, target_id: &str) -> bool {
        self.validate(&Command::ActivateClassAction {
            unit_id: unit_id.to_owned(),
            target_id: Some(target_id.to_owned()),
            target_tile: None,
        })
        .is_ok()
    }

    pub fn activate_class_action_on(
        &mut self,
        unit_id: &str,
        target_id: &str,
    ) -> Result<Vec<BattleEvent>, RuleError> {
        self.execute(Command::ActivateClassAction {
            unit_id: unit_id.to_owned(),
            target_id: Some(target_id.to_owned()),
            target_tile: None,
        })
    }

    pub fn can_target_class_action_tile(&self, unit_id: &str, tile: TilePos) -> bool {
        self.validate(&Command::ActivateClassAction {
            unit_id: unit_id.to_owned(),
            target_id: None,
            target_tile: Some(tile),
        })
        .is_ok()
    }

    pub fn activate_class_action_on_tile(
        &mut self,
        unit_id: &str,
        tile: TilePos,
    ) -> Result<Vec<BattleEvent>, RuleError> {
        self.execute(Command::ActivateClassAction {
            unit_id: unit_id.to_owned(),
            target_id: None,
            target_tile: Some(tile),
        })
    }
}

pub(crate) fn has_valid_target(session: &GameSession, unit_id: &str) -> bool {
    let Some(unit) = session.unit(unit_id) else {
        return false;
    };
    match target_kind(&unit.class_id) {
        Some(TechniqueTarget::Tile) => session
            .tactical
            .fog
            .iter_with_pos()
            .any(|(tile, _)| session.can_target_class_action_tile(unit_id, tile)),
        Some(TechniqueTarget::Ally | TechniqueTarget::Hostile) => session
            .tactical
            .units
            .iter()
            .any(|target| session.can_target_class_action(unit_id, &target.id)),
        _ => false,
    }
}

pub(crate) fn validate(
    session: &GameSession,
    unit_id: &str,
    target_id: Option<&str>,
    target_tile: Option<TilePos>,
) -> Result<CommandCost, RuleError> {
    let unit = session.unit(unit_id).ok_or(RuleError::UnknownUnit)?;
    if session.tactical.phase != crate::state::TacticalPhase::Player {
        return Err(RuleError::WrongPhase);
    }
    if unit.team != Team::Colony {
        return Err(RuleError::WrongTeam);
    }
    if unit.incapacitated {
        return Err(RuleError::Incapacitated);
    }
    if unit.class_action_used || action_name(&unit.class_id).is_none() {
        return Err(RuleError::ClassActionUnavailable);
    }
    if unit.action_points < 1 {
        return Err(RuleError::InsufficientActionPoints);
    }
    let target_result = match unit.class_id.as_str() {
        "medic" => {
            if target_tile.is_some() {
                Err(RuleError::InvalidTarget)
            } else {
                target_matches(session, unit_id, target_id, Team::Colony, 4, |target| {
                    target.health < target.max_health
                })
            }
        }
        "engineer" => {
            if target_tile.is_some() {
                Err(RuleError::InvalidTarget)
            } else {
                target_matches(session, unit_id, target_id, Team::Hostile, 4, |_| true)
            }
        }
        "psionic" => {
            if target_tile.is_some() {
                Err(RuleError::InvalidTarget)
            } else {
                target_matches(session, unit_id, target_id, Team::Hostile, 5, |_| true)
            }
        }
        "lifewright" => {
            if target_tile.is_some() {
                Err(RuleError::InvalidTarget)
            } else {
                target_matches(session, unit_id, target_id, Team::Colony, 5, |_| true)
            }
        }
        "null_adept" => {
            if target_tile.is_some() {
                Err(RuleError::InvalidTarget)
            } else {
                target_matches(session, unit_id, target_id, Team::Hostile, 6, |_| true)
            }
        }
        "breacher" => (target_id.is_none()
            && target_tile.is_some_and(|tile| valid_breacher_tile(session, unit, tile)))
        .then_some(())
        .ok_or(RuleError::InvalidTarget),
        "fortifier" => (target_id.is_none()
            && target_tile.is_some_and(|tile| valid_fortifier_tile(session, unit, tile)))
        .then_some(())
        .ok_or(RuleError::InvalidTarget),
        "rescue_specialist" => (target_tile.is_none()
            && advanced::valid_rescue_target(session, unit, target_id))
        .then_some(())
        .ok_or(RuleError::InvalidTarget),
        "chorus_warden" => (target_id.is_none()
            && target_tile
                .is_some_and(|tile| advanced::valid_chorus_warden_tile(session, unit, tile)))
        .then_some(())
        .ok_or(RuleError::InvalidTarget),
        _ => (target_id.is_none() && target_tile.is_none())
            .then_some(())
            .ok_or(RuleError::InvalidTarget),
    };
    target_result.map(|()| CommandCost { action_points: 1 })
}

pub(crate) fn execute(
    session: &mut GameSession,
    unit_id: &str,
    target_id: Option<&str>,
    target_tile: Option<TilePos>,
) -> Vec<BattleEvent> {
    let class_id = session.unit(unit_id).unwrap().class_id.clone();
    let action = action_name(&class_id).unwrap().to_owned();
    let unit = session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == unit_id)
        .unwrap();
    unit.action_points -= 1;
    unit.class_action_used = true;
    let mut events = vec![BattleEvent::ClassActionActivated {
        unit_id: unit_id.to_owned(),
        action,
    }];
    match class_id.as_str() {
        "soldier" => apply_status(session, unit_id, StatusKind::Focused, 1, &mut events),
        "defender" => apply_status(session, unit_id, StatusKind::Guarded, 2, &mut events),
        "scout" => {
            let unit = session
                .tactical
                .units
                .iter_mut()
                .find(|unit| unit.id == unit_id)
                .unwrap();
            unit.action_points = unit.action_points.saturating_add(3);
            apply_status(session, unit_id, StatusKind::Quickened, 1, &mut events);
        }
        "medic" => heal_target(session, target_id.unwrap(), &mut events),
        "engineer" => damage_target(session, target_id.unwrap(), 3, &mut events),
        "psionic" => {
            apply_status(
                session,
                target_id.unwrap(),
                StatusKind::Disrupted,
                1,
                &mut events,
            );
        }
        "biotech" => {
            let allies = session
                .tactical
                .units
                .iter()
                .filter(|unit| unit.team == Team::Colony && !unit.incapacitated)
                .map(|unit| unit.id.clone())
                .collect::<Vec<_>>();
            for ally in allies {
                apply_status(session, &ally, StatusKind::Regenerating, 2, &mut events);
            }
        }
        "vanguard" => {
            let origin = session.unit(unit_id).unwrap().position;
            let allies = session
                .tactical
                .units
                .iter()
                .filter(|unit| {
                    unit.team == Team::Colony
                        && !unit.incapacitated
                        && manhattan(origin, unit.position) <= 2
                })
                .map(|unit| unit.id.clone())
                .collect::<Vec<_>>();
            for ally in allies {
                apply_status(session, &ally, StatusKind::Guarded, 2, &mut events);
            }
        }
        "pathfinder" => {
            let unit = session
                .tactical
                .units
                .iter_mut()
                .find(|unit| unit.id == unit_id)
                .unwrap();
            unit.action_points = unit.action_points.saturating_add(2);
            apply_status(session, unit_id, StatusKind::Quickened, 2, &mut events);
            apply_status(session, unit_id, StatusKind::Focused, 1, &mut events);
        }
        "lifewright" => {
            let target_id = target_id.unwrap();
            heal_target_amount(session, target_id, 3, &mut events);
            apply_status(session, target_id, StatusKind::Regenerating, 2, &mut events);
        }
        "null_adept" => {
            let target_id = target_id.unwrap();
            damage_target(session, target_id, 2, &mut events);
            if !session.unit(target_id).unwrap().incapacitated {
                apply_status(session, target_id, StatusKind::Disrupted, 2, &mut events);
            }
        }
        "breacher" => execute_breacher(session, unit_id, target_tile.unwrap(), &mut events),
        "fortifier" => execute_fortifier(session, unit_id, target_tile.unwrap()),
        "rescue_specialist" => {
            advanced::execute_rescue_specialist(session, unit_id, target_id.unwrap(), &mut events)
        }
        "chorus_warden" => {
            advanced::execute_chorus_warden(session, unit_id, target_tile.unwrap(), &mut events)
        }
        _ => unreachable!("validated class has an action"),
    }
    session.check_outcome(&mut events);
    events
}

fn valid_breacher_tile(
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

fn valid_fortifier_tile(
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

fn breacher_route(
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

fn execute_breacher(
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

fn execute_fortifier(session: &mut GameSession, unit_id: &str, tile: TilePos) {
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

fn reposition_without_cost(
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

fn cover_direction(origin: TilePos, position: TilePos) -> EdgeDirection {
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

fn target_matches(
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

pub(crate) fn apply_status(
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

fn heal_target(session: &mut GameSession, target_id: &str, events: &mut Vec<BattleEvent>) {
    heal_target_amount(session, target_id, 4, events);
}

fn heal_target_amount(
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

fn damage_target(
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

#[cfg(test)]
mod tests;
