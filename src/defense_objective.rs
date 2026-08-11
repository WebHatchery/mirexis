//! Hostile attacks against vulnerable mission assets.

use crate::data::{ObjectiveKind, Team};
use crate::state::GameSession;
use crate::tactical::{manhattan, BattleEvent, Command, CommandCost, RuleError};

const OBJECTIVE_TARGET_ID: &str = "field_asset";
const STARTING_INTEGRITY: i32 = 12;

pub(crate) fn initial_integrity(kind: ObjectiveKind) -> i32 {
    i32::from(kind == ObjectiveKind::DefendAsset) * STARTING_INTEGRITY
}

pub(crate) fn is_destroyed(session: &GameSession) -> bool {
    session.tactical.objective_kind == ObjectiveKind::DefendAsset
        && session.tactical.objective_integrity <= 0
}

pub(crate) fn survives_deadline(session: &GameSession) -> bool {
    session.tactical.objective_kind == ObjectiveKind::DefendAsset
        && session.tactical.objective_integrity > 0
}

impl GameSession {
    pub(crate) fn can_attack_defense_objective(&self, attacker_id: &str) -> bool {
        self.validate(&Command::AttackObjective {
            attacker_id: attacker_id.to_owned(),
        })
        .is_ok()
    }
}

pub(crate) fn validate(session: &GameSession, attacker_id: &str) -> Result<CommandCost, RuleError> {
    let attacker = session.active_unit_for_phase(attacker_id)?;
    if attacker.team != Team::Hostile
        || session.tactical.objective_kind != ObjectiveKind::DefendAsset
        || session.tactical.objective_integrity <= 0
    {
        return Err(RuleError::ObjectiveUnavailable);
    }
    if manhattan(attacker.position, session.tactical.objective_tile)
        > i32::from(attacker.weapon_range)
    {
        return Err(RuleError::OutOfRange);
    }
    if !session.has_line_of_fire(attacker.position, session.tactical.objective_tile) {
        return Err(RuleError::NoLineOfFire);
    }
    if attacker.action_points < attacker.weapon_ap_cost {
        return Err(RuleError::InsufficientActionPoints);
    }
    Ok(CommandCost {
        action_points: attacker.weapon_ap_cost,
    })
}

pub(crate) fn execute(session: &mut GameSession, attacker_id: &str) -> Vec<BattleEvent> {
    let attacker = session.unit(attacker_id).unwrap().clone();
    let hit_chance = attacker.effective_accuracy().clamp(5, 95) as u8;
    let roll = session.tactical.rng.range_i32(1, 101) as u8;
    session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == attacker_id)
        .unwrap()
        .action_points -= attacker.weapon_ap_cost;
    let mut events = vec![BattleEvent::AttackRolled {
        attacker_id: attacker_id.to_owned(),
        target_id: OBJECTIVE_TARGET_ID.to_owned(),
        roll,
        hit_chance,
    }];
    if roll <= hit_chance {
        let damage = attacker.effective_weapon_damage().max(1);
        session.tactical.objective_integrity =
            (session.tactical.objective_integrity - damage).max(0);
        events.push(BattleEvent::ObjectiveDamaged {
            amount: damage,
            remaining: session.tactical.objective_integrity,
        });
        if session.tactical.objective_integrity == 0 {
            events.push(BattleEvent::ObjectiveDestroyed);
        }
    }
    session.check_outcome(&mut events);
    events
}

#[cfg(test)]
mod tests;
