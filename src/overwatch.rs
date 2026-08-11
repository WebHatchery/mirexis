//! Prepaid reaction fire resolved during hostile movement.

use crate::data::Team;
use crate::state::GameSession;
use crate::tactical::{manhattan, BattleEvent, CommandCost, RuleError};

const REACTION_ACCURACY_PENALTY: i32 = 15;

impl GameSession {
    pub fn set_selected_overwatch(&mut self) -> Result<Vec<BattleEvent>, RuleError> {
        let unit_id = self
            .tactical
            .selected_unit
            .clone()
            .ok_or(RuleError::UnknownUnit)?;
        self.execute(crate::tactical::Command::SetOverwatch { unit_id })
    }

    pub fn can_set_selected_overwatch(&self) -> bool {
        self.tactical.selected_unit.as_ref().is_some_and(|unit_id| {
            self.validate(&crate::tactical::Command::SetOverwatch {
                unit_id: unit_id.clone(),
            })
            .is_ok()
        })
    }
}

pub(crate) fn validate(session: &GameSession, unit_id: &str) -> Result<CommandCost, RuleError> {
    let unit = session.active_unit_for_phase(unit_id)?;
    if unit.team != Team::Colony || unit.overwatching {
        return Err(RuleError::OverwatchUnavailable);
    }
    if unit.action_points < unit.weapon_ap_cost {
        return Err(RuleError::InsufficientActionPoints);
    }
    Ok(CommandCost {
        action_points: unit.weapon_ap_cost,
    })
}

pub(crate) fn execute(session: &mut GameSession, unit_id: &str) -> Vec<BattleEvent> {
    let unit = session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == unit_id)
        .unwrap();
    unit.action_points -= unit.weapon_ap_cost;
    unit.overwatching = true;
    vec![BattleEvent::OverwatchSet {
        unit_id: unit_id.to_owned(),
    }]
}

pub(crate) fn resolve_after_hostile_move(
    session: &mut GameSession,
    target_id: &str,
) -> Vec<BattleEvent> {
    let Some(target) = session.unit(target_id) else {
        return Vec::new();
    };
    if target.team != Team::Hostile || target.incapacitated {
        return Vec::new();
    }
    let mut reactors = session
        .tactical
        .units
        .iter()
        .filter(|unit| unit.team == Team::Colony && unit.overwatching && !unit.incapacitated)
        .filter(|unit| {
            manhattan(unit.position, target.position) <= i32::from(unit.weapon_range)
                && session.has_line_of_fire(unit.position, target.position)
        })
        .map(|unit| unit.id.clone())
        .collect::<Vec<_>>();
    reactors.sort();

    let mut events = Vec::new();
    for attacker_id in reactors {
        if session
            .unit(target_id)
            .is_none_or(|target| target.incapacitated)
        {
            break;
        }
        events.extend(resolve_reaction(session, &attacker_id, target_id));
    }
    session.check_outcome(&mut events);
    events
}

fn resolve_reaction(
    session: &mut GameSession,
    attacker_id: &str,
    target_id: &str,
) -> Vec<BattleEvent> {
    let attacker = session.unit(attacker_id).unwrap().clone();
    let target = session.unit(target_id).unwrap().clone();
    let hit_chance = (i32::from(session.hit_chance(&attacker, &target)) - REACTION_ACCURACY_PENALTY)
        .clamp(5, 95) as u8;
    let roll = session.tactical.rng.range_i32(1, 101) as u8;
    session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == attacker_id)
        .unwrap()
        .overwatching = false;

    let mut events = vec![
        BattleEvent::ReactionTriggered {
            attacker_id: attacker_id.to_owned(),
            target_id: target_id.to_owned(),
        },
        BattleEvent::AttackRolled {
            attacker_id: attacker_id.to_owned(),
            target_id: target_id.to_owned(),
            roll,
            hit_chance,
        },
    ];
    if roll > hit_chance {
        return events;
    }

    let critical = roll <= 10;
    let damage = (attacker.effective_weapon_damage() + i32::from(critical) * 2
        - target.effective_armour())
    .max(1);
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
    events
}

#[cfg(test)]
mod tests;
