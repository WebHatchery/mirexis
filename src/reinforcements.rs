//! Deterministic construction and deployment of tactical reinforcement waves.

use crate::data::{GameConfig, MissionDef, ObjectiveKind, Team};
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
