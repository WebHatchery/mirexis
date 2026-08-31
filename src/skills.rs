//! Data-backed base-class techniques and their deterministic tactical effects.

use crate::data::{CoverEdgeDef, EdgeDirection, HazardKind, Team, TechniqueTarget};
use crate::state::{
    BattleEvent, Command, CommandCost, GameSession, ObscuringField, RuleError, UnitState,
};
use crate::tactical::{manhattan, path_cost, StatusKind, UnitAnimationState, UnitFacing};
use macroquad_toolkit::grid::TilePos;

pub(crate) fn skill_name(skill_id: &str) -> Option<&'static str> {
    match skill_id {
        "controlled_burst" => Some("CONTROLLED BURST"),
        "armour_drill" => Some("ARMOUR DRILL"),
        "interpose" => Some("INTERPOSE"),
        "anchor_point" => Some("ANCHOR POINT"),
        "slipstep" => Some("SLIPSTEP"),
        "spotters_mark" => Some("SPOTTER'S MARK"),
        "stabilise" => Some("STABILISE"),
        "combat_stimulant" => Some("COMBAT STIMULANT"),
        "portable_cover" => Some("PORTABLE COVER"),
        "overcharge" => Some("OVERCHARGE"),
        "kinetic_draw" => Some("KINETIC DRAW"),
        "premonition" => Some("PREMONITION"),
        "adaptive_secretion" => Some("ADAPTIVE SECRETION"),
        "spore_veil" => Some("SPORE VEIL"),
        _ => None,
    }
}

pub(crate) fn target_kind(skill_id: &str) -> Option<TechniqueTarget> {
    match skill_id {
        "controlled_burst" | "spotters_mark" | "kinetic_draw" | "premonition" => {
            Some(TechniqueTarget::Hostile)
        }
        "interpose" | "stabilise" | "combat_stimulant" | "adaptive_secretion" => {
            Some(TechniqueTarget::Ally)
        }
        "slipstep" | "portable_cover" | "spore_veil" => Some(TechniqueTarget::Tile),
        "armour_drill" | "anchor_point" | "overcharge" => Some(TechniqueTarget::SelfTarget),
        _ => None,
    }
}

pub(crate) fn requires_target(skill_id: &str) -> bool {
    !matches!(target_kind(skill_id), Some(TechniqueTarget::SelfTarget))
}

pub(crate) fn has_valid_target(session: &GameSession, unit_id: &str, skill_id: &str) -> bool {
    match target_kind(skill_id) {
        Some(TechniqueTarget::Tile) => session
            .tactical
            .fog
            .iter_with_pos()
            .any(|(tile, _)| can_target_tile(session, unit_id, skill_id, tile)),
        Some(TechniqueTarget::SelfTarget) => session
            .validate(&Command::ActivateSkill {
                unit_id: unit_id.to_owned(),
                skill_id: skill_id.to_owned(),
                target_id: None,
                target_tile: None,
            })
            .is_ok(),
        Some(TechniqueTarget::Ally | TechniqueTarget::Hostile) => session
            .tactical
            .units
            .iter()
            .any(|target| can_target_unit(session, unit_id, skill_id, &target.id)),
        None => false,
    }
}

pub(crate) fn can_target_unit(
    session: &GameSession,
    unit_id: &str,
    skill_id: &str,
    target_id: &str,
) -> bool {
    session
        .validate(&Command::ActivateSkill {
            unit_id: unit_id.to_owned(),
            skill_id: skill_id.to_owned(),
            target_id: Some(target_id.to_owned()),
            target_tile: None,
        })
        .is_ok()
}

pub(crate) fn can_target_tile(
    session: &GameSession,
    unit_id: &str,
    skill_id: &str,
    target_tile: TilePos,
) -> bool {
    session
        .validate(&Command::ActivateSkill {
            unit_id: unit_id.to_owned(),
            skill_id: skill_id.to_owned(),
            target_id: None,
            target_tile: Some(target_tile),
        })
        .is_ok()
}

impl GameSession {
    pub fn activate_selected_skill(
        &mut self,
        skill_id: &str,
    ) -> Result<Vec<BattleEvent>, RuleError> {
        let unit_id = self
            .tactical
            .selected_unit
            .clone()
            .ok_or(RuleError::UnknownUnit)?;
        self.execute(Command::ActivateSkill {
            unit_id,
            skill_id: skill_id.to_owned(),
            target_id: None,
            target_tile: None,
        })
    }

    pub fn activate_skill_on(
        &mut self,
        unit_id: &str,
        skill_id: &str,
        target_id: &str,
    ) -> Result<Vec<BattleEvent>, RuleError> {
        self.execute(Command::ActivateSkill {
            unit_id: unit_id.to_owned(),
            skill_id: skill_id.to_owned(),
            target_id: Some(target_id.to_owned()),
            target_tile: None,
        })
    }

    pub fn activate_skill_on_tile(
        &mut self,
        unit_id: &str,
        skill_id: &str,
        target_tile: TilePos,
    ) -> Result<Vec<BattleEvent>, RuleError> {
        self.execute(Command::ActivateSkill {
            unit_id: unit_id.to_owned(),
            skill_id: skill_id.to_owned(),
            target_id: None,
            target_tile: Some(target_tile),
        })
    }

    pub fn can_activate_selected_skill(&self, skill_id: &str) -> bool {
        self.tactical.selected_unit.as_ref().is_some_and(|unit_id| {
            self.validate(&Command::ActivateSkill {
                unit_id: unit_id.clone(),
                skill_id: skill_id.to_owned(),
                target_id: None,
                target_tile: None,
            })
            .is_ok()
        })
    }
}

pub(crate) fn validate(
    session: &GameSession,
    unit_id: &str,
    skill_id: &str,
    target_id: Option<&str>,
    target_tile: Option<TilePos>,
) -> Result<CommandCost, RuleError> {
    let unit = session.active_unit_for_phase(unit_id)?;
    if skill_name(skill_id).is_none()
        || !unit.active_skills.iter().any(|skill| skill == skill_id)
        || unit.used_skill_ids.iter().any(|skill| skill == skill_id)
    {
        return Err(RuleError::SkillUnavailable);
    }
    let target_result = match target_kind(skill_id) {
        Some(TechniqueTarget::SelfTarget) => {
            (target_id.is_none() && target_tile.is_none() && valid_self_target(unit, skill_id))
                .then_some(())
                .ok_or(RuleError::InvalidTarget)
        }
        Some(TechniqueTarget::Hostile) => {
            if target_tile.is_some() {
                Err(RuleError::InvalidTarget)
            } else {
                validate_hostile_target(session, unit, skill_id, target_id)
            }
        }
        Some(TechniqueTarget::Ally) => {
            if target_tile.is_some() {
                Err(RuleError::InvalidTarget)
            } else {
                validate_ally_target(session, unit, skill_id, target_id)
            }
        }
        Some(TechniqueTarget::Tile) => (target_id.is_none()
            && target_tile.is_some_and(|tile| valid_tile_target(session, unit, skill_id, tile)))
        .then_some(())
        .ok_or(RuleError::InvalidTarget),
        None => Err(RuleError::InvalidTarget),
    };
    target_result?;
    let action_points = skill_cost(session, unit, skill_id, target_tile);
    (unit.action_points >= action_points)
        .then_some(CommandCost { action_points })
        .ok_or(RuleError::InsufficientActionPoints)
}

fn validate_hostile_target(
    session: &GameSession,
    unit: &UnitState,
    skill_id: &str,
    target_id: Option<&str>,
) -> Result<(), RuleError> {
    let target = target_id
        .and_then(|target_id| session.unit(target_id))
        .ok_or(RuleError::InvalidTarget)?;
    if target.team != Team::Hostile {
        return Err(RuleError::WrongTeam);
    }
    if target.incapacitated {
        return Err(RuleError::Incapacitated);
    }
    let range = if skill_id == "spotters_mark" {
        6
    } else {
        i32::from(unit.weapon_range)
    };
    if manhattan(unit.position, target.position) > range {
        return Err(RuleError::OutOfRange);
    }
    if !matches!(skill_id, "spotters_mark")
        && !session.has_line_of_fire(unit.position, target.position)
    {
        return Err(RuleError::NoLineOfFire);
    }
    if skill_id == "kinetic_draw"
        && kinetic_draw_destination(session, unit.position, target.position).is_none()
    {
        return Err(RuleError::InvalidTarget);
    }
    Ok(())
}

fn validate_ally_target(
    session: &GameSession,
    unit: &UnitState,
    skill_id: &str,
    target_id: Option<&str>,
) -> Result<(), RuleError> {
    let target_id = target_id.ok_or(RuleError::InvalidTarget)?;
    let target = session.unit(target_id).ok_or(RuleError::InvalidTarget)?;
    if target.team != Team::Colony {
        return Err(RuleError::WrongTeam);
    }
    if target.id == unit.id {
        return Err(RuleError::InvalidTarget);
    }
    if target.incapacitated && skill_id != "stabilise" {
        return Err(RuleError::Incapacitated);
    }
    let range = if skill_id == "interpose" { 2 } else { 4 };
    if manhattan(unit.position, target.position) > range {
        return Err(RuleError::OutOfRange);
    }
    if skill_id == "adaptive_secretion" && adaptive_hazard(session, target_id).is_none() {
        return Err(RuleError::InvalidTarget);
    }
    Ok(())
}

fn valid_tile_target(
    session: &GameSession,
    unit: &UnitState,
    skill_id: &str,
    tile: TilePos,
) -> bool {
    match skill_id {
        "slipstep" => {
            session
                .tactical
                .hazards
                .iter()
                .any(|hazard| hazard.position == tile)
                && session.movement_path(&unit.id, tile).is_some_and(|path| {
                    let cost = path_cost(&path, &session.tactical.terrain_costs);
                    cost > 0 && cost <= unit.effective_move_range()
                })
        }
        "portable_cover" => valid_cover_tile(session, unit, tile),
        "spore_veil" => valid_veil_tile(session, unit, tile),
        _ => false,
    }
}

fn valid_cover_tile(session: &GameSession, unit: &UnitState, tile: TilePos) -> bool {
    manhattan(unit.position, tile) == 1
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
}

fn valid_veil_tile(session: &GameSession, unit: &UnitState, tile: TilePos) -> bool {
    manhattan(unit.position, tile) > 0
        && manhattan(unit.position, tile) <= 3
        && session.tactical.fog.is_valid(tile)
        && !session.tactical.blocked.contains(&tile)
        && !session
            .tactical
            .units
            .iter()
            .any(|other| other.position == tile)
        && tile != session.tactical.objective_tile
        && session.has_line_of_fire(unit.position, tile)
        && !session
            .tactical
            .obscuring_fields
            .iter()
            .any(|field| field.center == tile)
}

fn valid_self_target(unit: &UnitState, skill_id: &str) -> bool {
    skill_id != "overcharge"
        || unit.equipment_ids.iter().any(|equipment_id| {
            crate::equipment_actions::action_name(equipment_id).is_some()
                && !unit.used_equipment_ids.contains(equipment_id)
        })
}

pub(crate) fn adaptive_hazard(session: &GameSession, target_id: &str) -> Option<HazardKind> {
    let target = session.unit(target_id)?;
    session
        .tactical
        .hazards
        .iter()
        .filter(|hazard| manhattan(target.position, hazard.position) <= 6)
        .min_by_key(|hazard| {
            (
                manhattan(target.position, hazard.position),
                hazard.position.y,
                hazard.position.x,
            )
        })
        .map(|hazard| hazard.kind)
}

fn kinetic_draw_destination(
    session: &GameSession,
    source: TilePos,
    target: TilePos,
) -> Option<TilePos> {
    let horizontal = (source.x - target.x).signum();
    let vertical = (source.y - target.y).signum();
    let candidates = [
        (horizontal != 0).then(|| TilePos::new(target.x + horizontal, target.y)),
        (vertical != 0).then(|| TilePos::new(target.x, target.y + vertical)),
    ];
    candidates.into_iter().flatten().find(|tile| {
        session.tactical.fog.is_valid(*tile)
            && !session.tactical.blocked.contains(tile)
            && !session
                .tactical
                .units
                .iter()
                .any(|unit| !unit.incapacitated && unit.position == *tile)
    })
}

fn skill_cost(
    session: &GameSession,
    unit: &UnitState,
    skill_id: &str,
    target_tile: Option<TilePos>,
) -> u8 {
    match skill_id {
        "controlled_burst" => unit.weapon_ap_cost.saturating_mul(2),
        "slipstep" => target_tile
            .and_then(|tile| session.movement_path(&unit.id, tile))
            .map(|path| path_cost(&path, &session.tactical.terrain_costs).saturating_add(1))
            .unwrap_or(u8::MAX),
        _ => 1,
    }
}

pub(crate) fn execute(
    session: &mut GameSession,
    unit_id: &str,
    skill_id: &str,
    target_id: Option<&str>,
    target_tile: Option<TilePos>,
) -> Vec<BattleEvent> {
    mark_used(session, unit_id, skill_id);
    let mut events = vec![BattleEvent::SkillActivated {
        unit_id: unit_id.to_owned(),
        skill_id: skill_id.to_owned(),
    }];
    match skill_id {
        "controlled_burst" => {
            let target_id = target_id.expect("validated burst target");
            events.extend(session.execute_attack(unit_id, target_id));
            if session
                .unit(target_id)
                .is_some_and(|target| !target.incapacitated)
            {
                events.extend(session.execute_attack(unit_id, target_id));
            }
        }
        "armour_drill" => {
            spend_one_action_point(session, unit_id);
            unit_mut(session, unit_id)
                .expect("validated skill user")
                .next_attack_ignores_armour = true;
        }
        "interpose" => {
            spend_one_action_point(session, unit_id);
            crate::class_actions::apply_status(
                session,
                target_id.expect("validated ally target"),
                StatusKind::Guarded,
                1,
                &mut events,
            );
            crate::class_actions::apply_status(
                session,
                unit_id,
                StatusKind::Guarded,
                1,
                &mut events,
            );
        }
        "anchor_point" => {
            spend_one_action_point(session, unit_id);
            crate::class_actions::apply_status(
                session,
                unit_id,
                StatusKind::Guarded,
                1,
                &mut events,
            );
        }
        "slipstep" => {
            spend_one_action_point(session, unit_id);
            events.extend(
                session.execute_move(unit_id, target_tile.expect("validated slipstep tile")),
            );
        }
        "spotters_mark" => {
            spend_one_action_point(session, unit_id);
            crate::class_actions::apply_status(
                session,
                target_id.expect("validated mark target"),
                StatusKind::Marked,
                1,
                &mut events,
            );
        }
        "kinetic_draw" => {
            spend_one_action_point(session, unit_id);
            let target_id = target_id.expect("validated kinetic draw target");
            if let Some(destination) = kinetic_draw_destination(
                session,
                session.unit(unit_id).expect("validated draw user").position,
                session
                    .unit(target_id)
                    .expect("validated draw target")
                    .position,
            ) {
                let target = unit_mut(session, target_id).expect("validated draw target");
                let from = target.position;
                target.position = destination;
                target.facing = UnitFacing::toward(from, destination);
                events.push(BattleEvent::UnitMoved {
                    unit_id: target_id.to_owned(),
                    path: vec![from, destination],
                    cost: 0,
                });
                events.extend(crate::hazards::resolve_after_move(session, target_id));
            }
        }
        "premonition" => {
            spend_one_action_point(session, unit_id);
            crate::class_actions::apply_status(
                session,
                target_id.expect("validated premonition target"),
                StatusKind::Disrupted,
                1,
                &mut events,
            );
        }
        "adaptive_secretion" => {
            spend_one_action_point(session, unit_id);
            let target_id = target_id.expect("validated adaptive secretion target");
            let hazard = adaptive_hazard(session, target_id).expect("validated visible hazard");
            unit_mut(session, target_id)
                .expect("validated adaptive secretion target")
                .hazard_resistance = Some(hazard);
            crate::class_actions::apply_status(
                session,
                target_id,
                StatusKind::Adapted,
                1,
                &mut events,
            );
        }
        "spore_veil" => {
            spend_one_action_point(session, unit_id);
            session.tactical.obscuring_fields.push(ObscuringField {
                center: target_tile.expect("validated spore veil tile"),
                radius: 1,
                remaining_phases: 1,
            });
        }
        "stabilise" => {
            spend_one_action_point(session, unit_id);
            stabilise_target(
                session,
                target_id.expect("validated stabilise target"),
                &mut events,
            );
        }
        "combat_stimulant" => {
            spend_one_action_point(session, unit_id);
            let target_id = target_id.expect("validated stimulant target");
            let target = unit_mut(session, target_id).expect("validated stimulant target");
            target.action_points = target.action_points.saturating_add(2);
            crate::class_actions::apply_status(
                session,
                target_id,
                StatusKind::Hindered,
                2,
                &mut events,
            );
        }
        "portable_cover" => {
            spend_one_action_point(session, unit_id);
            place_portable_cover(
                session,
                unit_id,
                target_tile.expect("validated portable cover tile"),
            );
        }
        "overcharge" => {
            spend_one_action_point(session, unit_id);
            unit_mut(session, unit_id)
                .expect("validated overcharge user")
                .next_equipment_overcharged = true;
        }
        _ => unreachable!("validated skill has an implementation"),
    }
    if skill_id != "controlled_burst" {
        session.check_outcome(&mut events);
    }
    events
}

fn mark_used(session: &mut GameSession, unit_id: &str, skill_id: &str) {
    unit_mut(session, unit_id)
        .expect("validated skill user")
        .used_skill_ids
        .push(skill_id.to_owned());
}

fn spend_one_action_point(session: &mut GameSession, unit_id: &str) {
    unit_mut(session, unit_id)
        .expect("validated skill user")
        .action_points -= 1;
}

fn stabilise_target(session: &mut GameSession, target_id: &str, events: &mut Vec<BattleEvent>) {
    let target = unit_mut(session, target_id).expect("validated stabilise target");
    let before = target.health;
    target.health = 1.min(target.max_health);
    target.incapacitated = false;
    target.presentation_state = UnitAnimationState::Idle;
    target.presentation_seconds = 0.0;
    events.push(BattleEvent::UnitHealed {
        unit_id: target_id.to_owned(),
        amount: target.health - before,
        remaining: target.health,
    });
}

fn place_portable_cover(session: &mut GameSession, unit_id: &str, position: TilePos) {
    let origin = session
        .unit(unit_id)
        .expect("validated cover user")
        .position;
    session.tactical.blocked.insert(position);
    session
        .tactical
        .destructible_cover
        .push(crate::state::DestructibleCover {
            position,
            health: 6,
            max_health: 6,
        });
    session.tactical.cover_edges.push(CoverEdgeDef {
        position: [position.x, position.y],
        direction: cover_direction(origin, position),
        strength: 18,
    });
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

fn unit_mut<'a>(session: &'a mut GameSession, unit_id: &str) -> Option<&'a mut UnitState> {
    session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == unit_id)
}

#[cfg(test)]
mod tests;
