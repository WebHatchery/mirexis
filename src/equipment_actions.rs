//! Targeted tactical actions granted by carried field equipment.

use crate::data::Team;
use crate::state::{
    BattleEvent, Command, CommandCost, GameSession, RuleError, StatusKind, TacticalPhase,
};
use crate::tactical::manhattan;

pub(crate) fn action_name(equipment_id: &str) -> Option<&'static str> {
    match equipment_id {
        "field_medkit" => Some("FIELD PATCH"),
        "field_toolkit" => Some("FIELD FORTIFY"),
        "survey_harness" => Some("MARK HOSTILE"),
        "directorate_cipher" => Some("BREAK TARGETING NET"),
        "mireborn_sense" => Some("MAP HAZARD"),
        "severed_resonance" => Some("BORROW RESONANCE"),
        _ => None,
    }
}

impl GameSession {
    pub fn can_use_equipment(&self, unit_id: &str, equipment_id: &str, target_id: &str) -> bool {
        self.validate(&Command::UseEquipment {
            unit_id: unit_id.to_owned(),
            equipment_id: equipment_id.to_owned(),
            target_id: target_id.to_owned(),
        })
        .is_ok()
    }

    pub fn use_equipment(
        &mut self,
        unit_id: &str,
        equipment_id: &str,
        target_id: &str,
    ) -> Result<Vec<BattleEvent>, RuleError> {
        self.execute(Command::UseEquipment {
            unit_id: unit_id.to_owned(),
            equipment_id: equipment_id.to_owned(),
            target_id: target_id.to_owned(),
        })
    }
}

pub(crate) fn available_action(session: &GameSession, unit_id: &str) -> Option<String> {
    let unit = session.unit(unit_id)?;
    unit.equipment_ids
        .iter()
        .find(|equipment_id| {
            action_name(equipment_id).is_some() && !unit.used_equipment_ids.contains(equipment_id)
        })
        .cloned()
}

pub(crate) fn has_valid_target(session: &GameSession, unit_id: &str, equipment_id: &str) -> bool {
    session
        .tactical
        .units
        .iter()
        .any(|target| validate(session, unit_id, equipment_id, &target.id).is_ok())
}

pub(crate) fn validate(
    session: &GameSession,
    unit_id: &str,
    equipment_id: &str,
    target_id: &str,
) -> Result<CommandCost, RuleError> {
    let unit = session.unit(unit_id).ok_or(RuleError::UnknownUnit)?;
    let target = session.unit(target_id).ok_or(RuleError::UnknownUnit)?;
    if session.tactical.phase != TacticalPhase::Player {
        return Err(RuleError::WrongPhase);
    }
    if unit.team != Team::Colony {
        return Err(RuleError::WrongTeam);
    }
    if unit.incapacitated || target.incapacitated {
        return Err(RuleError::Incapacitated);
    }
    if unit.action_points < 1 {
        return Err(RuleError::InsufficientActionPoints);
    }
    if !unit.equipment_ids.iter().any(|id| id == equipment_id)
        || unit.used_equipment_ids.iter().any(|id| id == equipment_id)
        || action_name(equipment_id).is_none()
    {
        return Err(RuleError::EquipmentUnavailable);
    }
    let distance = manhattan(unit.position, target.position);
    let target_result = match equipment_id {
        "field_medkit" => validate_target(
            target,
            Team::Colony,
            distance,
            3,
            target.health < target.max_health,
        ),
        "field_toolkit" => validate_target(
            target,
            Team::Colony,
            distance,
            3,
            !target.has_status(StatusKind::Guarded),
        ),
        "survey_harness" => validate_target(
            target,
            Team::Hostile,
            distance,
            6,
            !target.has_status(StatusKind::Disrupted),
        ),
        "directorate_cipher" => validate_target(
            target,
            Team::Hostile,
            distance,
            6,
            !target.has_status(StatusKind::Hindered),
        ),
        "mireborn_sense" => validate_target(
            target,
            Team::Hostile,
            distance,
            6,
            !target.has_status(StatusKind::Disrupted) && !unit.has_status(StatusKind::Guarded),
        ),
        "severed_resonance" => validate_target(
            target,
            Team::Hostile,
            distance,
            6,
            !target.has_status(StatusKind::Hindered),
        ),
        _ => Err(RuleError::InvalidTarget),
    };
    target_result.map(|()| CommandCost { action_points: 1 })
}

fn validate_target(
    target: &crate::state::UnitState,
    team: Team,
    distance: i32,
    range: i32,
    extra: bool,
) -> Result<(), RuleError> {
    if target.team != team {
        return Err(RuleError::WrongTeam);
    }
    if distance > range {
        return Err(RuleError::OutOfRange);
    }
    extra.then_some(()).ok_or(RuleError::InvalidTarget)
}

pub(crate) fn execute(
    session: &mut GameSession,
    unit_id: &str,
    equipment_id: &str,
    target_id: &str,
) -> Vec<BattleEvent> {
    let unit = session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == unit_id)
        .expect("validated equipment user exists");
    unit.action_points -= 1;
    unit.used_equipment_ids.push(equipment_id.to_owned());
    let overcharged = unit.next_equipment_overcharged;
    unit.next_equipment_overcharged = false;
    let mut events = vec![BattleEvent::EquipmentUsed {
        unit_id: unit_id.to_owned(),
        equipment_id: equipment_id.to_owned(),
        target_id: target_id.to_owned(),
    }];
    match equipment_id {
        "field_medkit" => {
            let target = session
                .tactical
                .units
                .iter_mut()
                .find(|unit| unit.id == target_id)
                .expect("validated equipment target exists");
            let before = target.health;
            let amount = if overcharged { 8 } else { 5 };
            target.health = (target.health + amount).min(target.max_health);
            events.push(BattleEvent::UnitHealed {
                unit_id: target_id.to_owned(),
                amount: target.health - before,
                remaining: target.health,
            });
        }
        "field_toolkit" => crate::class_actions::apply_status(
            session,
            target_id,
            StatusKind::Guarded,
            if overcharged { 3 } else { 2 },
            &mut events,
        ),
        "survey_harness" => crate::class_actions::apply_status(
            session,
            target_id,
            StatusKind::Disrupted,
            if overcharged { 3 } else { 2 },
            &mut events,
        ),
        "directorate_cipher" => crate::class_actions::apply_status(
            session,
            target_id,
            StatusKind::Hindered,
            if overcharged { 3 } else { 2 },
            &mut events,
        ),
        "mireborn_sense" => {
            crate::class_actions::apply_status(
                session,
                unit_id,
                StatusKind::Guarded,
                if overcharged { 3 } else { 2 },
                &mut events,
            );
            crate::class_actions::apply_status(
                session,
                target_id,
                StatusKind::Disrupted,
                if overcharged { 3 } else { 2 },
                &mut events,
            );
        }
        "severed_resonance" => {
            let target_position = session
                .unit(target_id)
                .expect("validated equipment target exists")
                .position;
            let allies = session
                .tactical
                .units
                .iter()
                .filter(|ally| {
                    ally.team == Team::Colony
                        && !ally.incapacitated
                        && manhattan(ally.position, target_position) <= 2
                })
                .map(|ally| ally.id.clone())
                .collect::<Vec<_>>();
            crate::class_actions::apply_status(
                session,
                target_id,
                StatusKind::Hindered,
                if overcharged { 3 } else { 2 },
                &mut events,
            );
            for ally_id in allies {
                crate::class_actions::apply_status(
                    session,
                    &ally_id,
                    StatusKind::Guarded,
                    if overcharged { 3 } else { 2 },
                    &mut events,
                );
            }
        }
        _ => unreachable!("validated equipment has an action"),
    }
    session.check_outcome(&mut events);
    events
}

#[cfg(test)]
mod tests;
