//! Deterministic once-per-phase abilities that distinguish hostile factions.

use crate::data::Team;
use crate::state::GameSession;
use crate::tactical::{manhattan, BattleEvent, Command, CommandCost, RuleError, StatusKind};

const ABILITY_COST: u8 = 1;

pub(crate) fn ability_name(faction: Option<&str>) -> Option<&'static str> {
    match faction {
        Some("brood") => Some("PREDATORY SURGE"),
        Some("directorate") => Some("SUPPRESSION LOCK"),
        Some("ascendants") => Some("PHASE WARD"),
        _ => None,
    }
}

pub(crate) fn try_activate(session: &mut GameSession, unit_id: &str) {
    let faction = session
        .unit(unit_id)
        .and_then(|unit| unit.faction.as_deref())
        .unwrap_or_default();
    let target_id = (faction == "directorate")
        .then(|| disruption_target(session, unit_id))
        .flatten();
    let command = Command::ActivateEnemyAbility {
        unit_id: unit_id.to_owned(),
        target_id,
    };
    if session.validate(&command).is_ok() {
        let _ = session.execute(command);
    }
}

pub(crate) fn validate(
    session: &GameSession,
    unit_id: &str,
    target_id: Option<&str>,
) -> Result<CommandCost, RuleError> {
    let unit = session.active_unit_for_phase(unit_id)?;
    if unit.team != Team::Hostile || unit.enemy_ability_used || unit.action_points < ABILITY_COST {
        return Err(RuleError::EnemyAbilityUnavailable);
    }
    match unit.faction.as_deref() {
        Some("brood" | "ascendants") if target_id.is_none() => {}
        Some("directorate") => {
            let target = session
                .unit(target_id.ok_or(RuleError::InvalidTarget)?)
                .ok_or(RuleError::UnknownUnit)?;
            if target.team != Team::Colony
                || target.incapacitated
                || manhattan(unit.position, target.position) > i32::from(unit.weapon_range)
                || !session.has_line_of_fire(unit.position, target.position)
            {
                return Err(RuleError::InvalidTarget);
            }
        }
        _ => return Err(RuleError::EnemyAbilityUnavailable),
    }
    Ok(CommandCost {
        action_points: ABILITY_COST,
    })
}

pub(crate) fn execute(
    session: &mut GameSession,
    unit_id: &str,
    target_id: Option<&str>,
) -> Vec<BattleEvent> {
    let faction = session
        .unit(unit_id)
        .and_then(|unit| unit.faction.as_deref())
        .unwrap()
        .to_owned();
    let (ability, affected_id, status, duration) = match faction.as_str() {
        "brood" => ("Predatory Surge", unit_id, StatusKind::Quickened, 1),
        "directorate" => (
            "Suppression Lock",
            target_id.unwrap(),
            StatusKind::Disrupted,
            1,
        ),
        "ascendants" => ("Phase Ward", unit_id, StatusKind::Guarded, 2),
        _ => unreachable!("enemy ability faction was validated"),
    };
    let unit = session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == unit_id)
        .unwrap();
    unit.action_points -= ABILITY_COST;
    unit.enemy_ability_used = true;
    let mut events = Vec::new();
    crate::class_actions::apply_status(session, affected_id, status, duration, &mut events);
    events.push(BattleEvent::EnemyAbilityActivated {
        unit_id: unit_id.to_owned(),
        ability: ability.to_owned(),
    });
    events
}

fn disruption_target(session: &GameSession, unit_id: &str) -> Option<String> {
    let unit = session.unit(unit_id)?;
    let mut targets = session
        .tactical
        .units
        .iter()
        .filter(|target| {
            target.team == Team::Colony
                && !target.incapacitated
                && manhattan(unit.position, target.position) <= i32::from(unit.weapon_range)
                && session.has_line_of_fire(unit.position, target.position)
        })
        .collect::<Vec<_>>();
    targets.sort_by_key(|target| (target.health, target.id.clone()));
    targets.first().map(|target| target.id.clone())
}

#[cfg(test)]
mod tests;
