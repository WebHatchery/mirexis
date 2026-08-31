//! Compact tactical objective progress labels.

use crate::data::{MissionDef, ObjectiveKind, Team};
use crate::state::{GameSession, UnitState};
use crate::tactical::ObjectiveState;
use macroquad::prelude::*;
use macroquad_toolkit::prelude::{dark, draw_text_block, TextStyle};

pub(crate) fn action_label(kind: ObjectiveKind, state: ObjectiveState) -> &'static str {
    match kind {
        ObjectiveKind::SecureAndClear => "SECURE OBJECTIVE",
        ObjectiveKind::EliminateAll => "ELIMINATE ALL HOSTILES",
        ObjectiveKind::Holdout => "HOLD THE PERIMETER",
        ObjectiveKind::Extraction => "EXTRACT COLONIST",
        ObjectiveKind::SignalTrace if state != ObjectiveState::Active => "SIGNAL RELAY ACTIVE",
        ObjectiveKind::SignalTrace => "ACTIVATE SIGNAL RELAY",
        ObjectiveKind::DefendAsset => "PROTECT FIELD ASSET",
    }
}

pub(crate) fn action_button_bounds(panel: Rect) -> Rect {
    Rect::new(panel.x + 18.0, panel.bottom() - 232.0, panel.w - 36.0, 34.0)
}

pub(crate) fn interaction_label(
    kind: ObjectiveKind,
    state: ObjectiveState,
    selected: Option<&UnitState>,
    at_objective: bool,
    enabled: bool,
) -> &'static str {
    let action = action_label(kind, state);
    if enabled {
        return action;
    }
    let Some(unit) = selected else {
        return "SELECT UNIT";
    };
    if unit.incapacitated {
        return "INCAPACITATED";
    }
    if state != ObjectiveState::Active {
        return match kind {
            ObjectiveKind::SignalTrace => "RELAY ACTIVE",
            ObjectiveKind::SecureAndClear => "OBJECTIVE SECURED",
            ObjectiveKind::Extraction => "EVACUATION COMPLETE",
            _ => action,
        };
    }
    if unit.action_points == 0 {
        return "NO AP";
    }
    match kind {
        ObjectiveKind::SecureAndClear | ObjectiveKind::Extraction | ObjectiveKind::SignalTrace
            if !at_objective =>
        {
            "MOVE TO OBJECTIVE"
        }
        ObjectiveKind::EliminateAll => "CLEAR HOSTILES",
        ObjectiveKind::Holdout => "WAIT FOR DEADLINE",
        ObjectiveKind::DefendAsset => "PROTECT ASSET",
        _ => action,
    }
}

pub(crate) fn draw_summary(session: &GameSession, mission: &MissionDef, x: f32, panel: Rect) {
    draw_text_ex(
        format!("OBJECTIVE // {}", progress(session, mission)),
        x,
        panel.y + 78.0,
        TextStyle::new(15.0, Color::new(0.43, 0.83, 0.69, 1.0)).params(),
    );
    if let Some(forecast) = crate::reinforcements::next_wave_forecast(session) {
        draw_text_ex(
            forecast,
            x,
            panel.y + 98.0,
            TextStyle::new(12.5, Color::new(0.96, 0.55, 0.35, 1.0)).params(),
        );
    }
    draw_text_block(
        &mission.objective,
        x,
        panel.y + 110.0,
        panel.w - 36.0,
        42.0,
        17.0,
        4.0,
        dark::TEXT_DIM,
    );
}

pub(crate) fn progress(session: &GameSession, mission: &MissionDef) -> String {
    let hostiles = session
        .tactical
        .units
        .iter()
        .filter(|unit| unit.team == Team::Hostile && !unit.incapacitated)
        .count();
    match mission.objective_kind {
        ObjectiveKind::SecureAndClear => match session.tactical.objective_state {
            ObjectiveState::Active => format!("UNSECURED · {} HOSTILES", hostiles),
            ObjectiveState::Secured => format!("SECURED · {} HOSTILES", hostiles),
            ObjectiveState::Victory => "SECURED · AREA CLEAR".to_owned(),
            ObjectiveState::Failed => "FAILED".to_owned(),
        },
        ObjectiveKind::EliminateAll => format!("{} HOSTILES", hostiles),
        ObjectiveKind::Holdout => format!(
            "{} ROUNDS · {} HOSTILES · {} WAVES",
            rounds_remaining(session, mission),
            hostiles,
            session.tactical.reinforcement_waves.len()
        ),
        ObjectiveKind::Extraction => match session.tactical.objective_state {
            ObjectiveState::Active => format!("REACH EVAC · {} HOSTILES", hostiles),
            ObjectiveState::Victory => "COLONIST EVACUATED".to_owned(),
            ObjectiveState::Failed => "EVACUATION FAILED".to_owned(),
            ObjectiveState::Secured => "EVACUATION CONFIRMED".to_owned(),
        },
        ObjectiveKind::SignalTrace => match session.tactical.objective_state {
            ObjectiveState::Active => format!("RELAY OFFLINE · {} HOSTILES", hostiles),
            ObjectiveState::Secured => format!(
                "TRACE {} ROUNDS · {} HOSTILES · {} WAVES",
                rounds_remaining(session, mission),
                hostiles,
                session.tactical.reinforcement_waves.len()
            ),
            ObjectiveState::Victory => "CONTACT TRACE COMPLETE".to_owned(),
            ObjectiveState::Failed => "CONTACT TRACE LOST".to_owned(),
        },
        ObjectiveKind::DefendAsset => match session.tactical.objective_state {
            ObjectiveState::Active => format!(
                "ASSET {}/{} · {} ROUNDS · {} HOSTILES",
                session.tactical.objective_integrity,
                session.tactical.objective_max_integrity,
                rounds_remaining(session, mission),
                hostiles
            ),
            ObjectiveState::Victory => "ASSET SURVIVED".to_owned(),
            ObjectiveState::Failed => "ASSET DESTROYED".to_owned(),
            ObjectiveState::Secured => "ASSET SECURED".to_owned(),
        },
    }
}

fn rounds_remaining(session: &GameSession, mission: &MissionDef) -> u32 {
    mission.round_limit.saturating_sub(session.tactical.round) + 1
}

#[cfg(test)]
mod tests;
