//! Data-backed base-class techniques and their deterministic tactical effects.

use crate::data::{Team, TechniqueTarget};
use crate::state::{BattleEvent, Command, CommandCost, GameSession, RuleError, UnitState};
use crate::tactical::{manhattan, path_cost, StatusKind};
use macroquad_toolkit::grid::TilePos;

pub(crate) fn skill_name(skill_id: &str) -> Option<&'static str> {
    match skill_id {
        "controlled_burst" => Some("CONTROLLED BURST"),
        "armour_drill" => Some("ARMOUR DRILL"),
        "interpose" => Some("INTERPOSE"),
        "anchor_point" => Some("ANCHOR POINT"),
        "slipstep" => Some("SLIPSTEP"),
        "spotters_mark" => Some("SPOTTER'S MARK"),
        _ => None,
    }
}

pub(crate) fn target_kind(skill_id: &str) -> Option<TechniqueTarget> {
    match skill_id {
        "controlled_burst" | "spotters_mark" => Some(TechniqueTarget::Hostile),
        "interpose" => Some(TechniqueTarget::Ally),
        "slipstep" => Some(TechniqueTarget::Tile),
        "armour_drill" | "anchor_point" => Some(TechniqueTarget::SelfTarget),
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
            .hazards
            .iter()
            .any(|hazard| can_target_tile(session, unit_id, skill_id, hazard.position)),
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
    let valid = match target_kind(skill_id) {
        Some(TechniqueTarget::SelfTarget) => target_id.is_none() && target_tile.is_none(),
        Some(TechniqueTarget::Hostile) => {
            target_id
                .is_some_and(|target_id| valid_hostile_target(session, unit, skill_id, target_id))
                && target_tile.is_none()
        }
        Some(TechniqueTarget::Ally) => {
            target_id.is_some_and(|target_id| valid_ally_target(session, unit, target_id))
                && target_tile.is_none()
        }
        Some(TechniqueTarget::Tile) => {
            target_id.is_none()
                && target_tile.is_some_and(|tile| valid_slipstep_tile(session, unit, tile))
        }
        None => false,
    };
    if !valid {
        return Err(RuleError::InvalidTarget);
    }
    let action_points = skill_cost(session, unit, skill_id, target_tile);
    (unit.action_points >= action_points)
        .then_some(CommandCost { action_points })
        .ok_or(RuleError::InsufficientActionPoints)
}

fn valid_hostile_target(
    session: &GameSession,
    unit: &UnitState,
    skill_id: &str,
    target_id: &str,
) -> bool {
    let Some(target) = session.unit(target_id) else {
        return false;
    };
    if target.team != Team::Hostile || target.incapacitated {
        return false;
    }
    let range = if skill_id == "spotters_mark" {
        6
    } else {
        i32::from(unit.weapon_range)
    };
    manhattan(unit.position, target.position) <= range
        && (skill_id == "spotters_mark" || session.has_line_of_fire(unit.position, target.position))
}

fn valid_ally_target(session: &GameSession, unit: &UnitState, target_id: &str) -> bool {
    session.unit(target_id).is_some_and(|target| {
        target.team == Team::Colony
            && target.id != unit.id
            && !target.incapacitated
            && manhattan(unit.position, target.position) <= 2
    })
}

fn valid_slipstep_tile(session: &GameSession, unit: &UnitState, tile: TilePos) -> bool {
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

fn unit_mut<'a>(session: &'a mut GameSession, unit_id: &str) -> Option<&'a mut UnitState> {
    session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == unit_id)
}

#[cfg(test)]
mod tests;
