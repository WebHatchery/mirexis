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
mod tests {
    use super::*;
    use crate::data::GameData;

    fn session_and_hostile() -> (GameSession, String) {
        let data = GameData::load().unwrap();
        let session = GameSession::new(&data.config, &data.mission, &data.roster);
        let id = session
            .tactical
            .units
            .iter()
            .find(|unit| unit.team == Team::Hostile)
            .unwrap()
            .id
            .clone();
        (session, id)
    }

    #[test]
    fn preview_uses_enemy_phase_rules_without_mutating_the_live_session() {
        let (mut session, hostile_id) = session_and_hostile();
        let colonist_position = session
            .tactical
            .units
            .iter()
            .find(|unit| unit.team == Team::Colony)
            .unwrap()
            .position;
        session
            .tactical
            .units
            .iter_mut()
            .find(|unit| unit.id == hostile_id)
            .unwrap()
            .position = TilePos::new(colonist_position.x + 1, colonist_position.y);
        let original_phase = session.tactical.phase;

        let intent = preview(&session, &hostile_id, 6).unwrap();

        assert!(matches!(intent.action, IntentAction::AttackUnit(_)));
        assert_eq!(session.tactical.phase, original_phase);
    }

    #[test]
    fn distant_hostile_forecast_names_ability_target_and_next_step() {
        let (mut session, hostile_id) = session_and_hostile();
        session.tactical.blocked.clear();
        for unit in &mut session.tactical.units {
            if unit.id == hostile_id {
                unit.position = TilePos::new(11, 7);
            } else if unit.team == Team::Hostile {
                unit.incapacitated = true;
            } else {
                unit.position = TilePos::new(0, unit.position.y.min(3));
            }
        }
        let intent = preview(&session, &hostile_id, 6).unwrap();

        assert!(intent.ability.is_some());
        assert!(matches!(intent.action, IntentAction::Advance { .. }));
    }
}
