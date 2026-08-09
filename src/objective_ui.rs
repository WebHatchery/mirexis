//! Compact tactical objective progress labels.

use crate::data::{MissionDef, ObjectiveKind, Team};
use crate::state::GameSession;
use crate::tactical::ObjectiveState;

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
