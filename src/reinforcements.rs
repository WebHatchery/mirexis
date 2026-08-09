//! Deterministic construction and deployment of tactical reinforcement waves.

use crate::data::{GameConfig, GameData, MissionDef, ObjectiveKind, Team};
use crate::state::{BattleEvent, GameSession, ReinforcementWave, UnitState};
use crate::tactical::manhattan;
use macroquad_toolkit::grid::TilePos;

pub(crate) fn create_waves(
    config: &GameConfig,
    mission: &MissionDef,
    units: &[UnitState],
) -> Vec<ReinforcementWave> {
    if !matches!(
        mission.objective_kind,
        ObjectiveKind::Holdout | ObjectiveKind::SignalTrace
    ) {
        return Vec::new();
    }
    let templates = units
        .iter()
        .filter(|unit| unit.team == Team::Hostile)
        .take(2)
        .cloned()
        .collect::<Vec<_>>();
    [3, 5]
        .into_iter()
        .filter(|round| *round <= mission.round_limit)
        .map(|round| ReinforcementWave {
            round,
            units: templates
                .iter()
                .enumerate()
                .map(|(index, template)| {
                    let mut unit = template.clone();
                    unit.id = format!("{}_reinforcement_{}", unit.id, round);
                    unit.position = TilePos::new(
                        config.world_width.saturating_sub(1) as i32,
                        1 + index as i32 * config.world_height.saturating_sub(3) as i32,
                    );
                    unit
                })
                .collect(),
        })
        .collect()
}

pub(crate) fn deploy(session: &mut GameSession, action_points: u8) {
    let round = session.tactical.round;
    let mut arriving = Vec::new();
    session.tactical.reinforcement_waves.retain(|wave| {
        if wave.round == round {
            arriving.extend(wave.units.iter().cloned());
            false
        } else {
            true
        }
    });
    let mut count = 0;
    for mut unit in arriving {
        if let Some(position) = reinforcement_position(session, unit.position) {
            unit.position = position;
            unit.action_points = action_points;
            session.tactical.units.push(unit);
            count += 1;
        }
    }
    if count > 0 {
        session
            .tactical
            .event_log
            .push(BattleEvent::ReinforcementsArrived { round, count });
    }
}

pub(crate) fn briefing_forecast(data: &GameData, mission: &MissionDef) -> Option<String> {
    let rounds = wave_rounds(mission)
        .map(|round| format!("R{}", round))
        .collect::<Vec<_>>();
    if rounds.is_empty() {
        return None;
    }
    let roles = data
        .roster
        .iter()
        .filter(|unit| {
            unit.team == Team::Hostile
                && if mission.hostile_unit_ids.is_empty() {
                    unit.faction.as_deref() == Some(&mission.hostile_faction)
                } else {
                    mission.hostile_unit_ids.contains(&unit.id)
                }
        })
        .take(2)
        .map(|unit| forecast_role(&unit.role))
        .collect::<Vec<_>>();
    (!roles.is_empty()).then(|| format!("{} · {} · EAST", rounds.join("/"), roles.join("+")))
}

pub(crate) fn next_wave_forecast(session: &GameSession) -> Option<String> {
    let wave = session
        .tactical
        .reinforcement_waves
        .iter()
        .min_by_key(|wave| wave.round)?;
    let roles = wave
        .units
        .iter()
        .map(|unit| forecast_role(&unit.role))
        .collect::<Vec<_>>();
    Some(format!(
        "INBOUND R{} // {} // EAST EDGE",
        wave.round,
        roles.join("+")
    ))
}

fn wave_rounds(mission: &MissionDef) -> impl Iterator<Item = u32> + '_ {
    [3, 5].into_iter().filter(|round| {
        matches!(
            mission.objective_kind,
            ObjectiveKind::Holdout | ObjectiveKind::SignalTrace
        ) && *round <= mission.round_limit
    })
}

fn forecast_role(role: &str) -> String {
    match role {
        "Line Infantry" => "INFANTRY".to_owned(),
        "Energy Construct" => "CONSTRUCT".to_owned(),
        "Battlefield Controller" => "CONTROLLER".to_owned(),
        "Combat Drone" => "DRONE".to_owned(),
        _ => role.to_uppercase(),
    }
}

fn reinforcement_position(session: &GameSession, preferred: TilePos) -> Option<TilePos> {
    let width = session.tactical.fog.width as i32;
    let height = session.tactical.fog.height as i32;
    (0..width)
        .flat_map(|offset| {
            (0..height).map(move |y| TilePos::new((preferred.x - offset).clamp(0, width - 1), y))
        })
        .filter(|position| {
            session.tactical.fog.is_valid(*position)
                && !session.tactical.blocked.contains(position)
                && !session
                    .tactical
                    .units
                    .iter()
                    .any(|unit| !unit.incapacitated && unit.position == *position)
        })
        .min_by_key(|position| manhattan(*position, preferred))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn briefing_forecast_matches_the_created_wave_schedule_and_roles() {
        let data = GameData::load().unwrap();
        let mut mission = data.mission.clone();
        mission.objective_kind = ObjectiveKind::Holdout;
        mission.round_limit = 5;
        mission.hostile_faction = "brood".to_owned();
        mission.hostile_unit_ids.clear();
        let roster = data
            .roster
            .iter()
            .map(|unit| UnitState::from_def(unit, data.config.max_action_points))
            .collect::<Vec<_>>();
        let waves = create_waves(&data.config, &mission, &roster);
        let forecast = briefing_forecast(&data, &mission).unwrap();

        assert_eq!(
            waves.iter().map(|wave| wave.round).collect::<Vec<_>>(),
            vec![3, 5]
        );
        assert!(forecast.contains("R3/R5"));
        assert!(forecast.contains("HUNTER+ARTILLERY"));
        assert!(forecast.contains("EAST"));
    }
}
