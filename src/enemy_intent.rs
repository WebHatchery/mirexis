//! Read-only hostile forecasts derived from the same selectors used by enemy AI.

use crate::data::{ObjectiveKind, Team};
use crate::state::{GameSession, TacticalPhase};
use crate::tactical::{manhattan, ObjectiveState};
use macroquad_toolkit::grid::TilePos;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum IntentAction {
    AttackUnit(String),
    AttackObjective,
    Advance {
        destination: TilePos,
        target: String,
    },
    Hold,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct EnemyIntent {
    pub ability: Option<&'static str>,
    pub action: IntentAction,
}

pub(crate) fn preview(
    session: &GameSession,
    enemy_id: &str,
    max_action_points: u8,
) -> Option<EnemyIntent> {
    let enemy = session.unit(enemy_id)?;
    if enemy.team != Team::Hostile || enemy.incapacitated {
        return None;
    }
    let ability = crate::enemy_abilities::ability_name(enemy.faction.as_deref());
    let mut forecast = session.clone();
    forecast.tactical.phase = TacticalPhase::Enemy;
    let forecast_unit = forecast
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == enemy_id)
        .unwrap();
    forecast_unit.action_points = max_action_points;
    forecast_unit.enemy_ability_used = false;

    let action = if forecast.can_attack_defense_objective(enemy_id) {
        IntentAction::AttackObjective
    } else if let Some(target) = crate::tactical_ai::best_attack_target(&forecast, enemy_id) {
        IntentAction::AttackUnit(target)
    } else if let Some(destination) = crate::tactical_ai::best_enemy_move(&forecast, enemy_id) {
        IntentAction::Advance {
            destination,
            target: pressure_target(&forecast, enemy_id)?,
        }
    } else {
        IntentAction::Hold
    };
    Some(EnemyIntent { ability, action })
}

fn pressure_target(session: &GameSession, enemy_id: &str) -> Option<String> {
    if session.tactical.objective_kind == ObjectiveKind::DefendAsset
        && session.tactical.objective_state == ObjectiveState::Active
    {
        return Some("FIELD ASSET".to_owned());
    }
    let enemy = session.unit(enemy_id)?;
    session
        .tactical
        .units
        .iter()
        .filter(|unit| unit.team == Team::Colony && !unit.incapacitated)
        .min_by_key(|unit| (manhattan(enemy.position, unit.position), unit.id.clone()))
        .map(|unit| unit.name.to_uppercase())
}

#[cfg(test)]
mod tests;
