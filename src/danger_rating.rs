//! Deterministic operation danger ratings shared by offers and briefings.

use crate::data::{GameData, MissionDef, ObjectiveKind, OperationModifier, Team};
use crate::strategy::MissionInstance;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum DangerLevel {
    Routine,
    Contested,
    Severe,
    Extreme,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct DangerRating {
    pub score: u8,
    pub level: DangerLevel,
}

impl DangerRating {
    pub fn label(self) -> &'static str {
        match self.level {
            DangerLevel::Routine => "ROUTINE",
            DangerLevel::Contested => "CONTESTED",
            DangerLevel::Severe => "SEVERE",
            DangerLevel::Extreme => "EXTREME",
        }
    }
}

pub(crate) fn for_instance(instance: &MissionInstance, data: &GameData) -> DangerRating {
    let (hostile_count, hostile_powers) =
        hostile_profile(&instance.hostile_unit_ids, &instance.faction_id, data);
    let hazard_count = data
        .campaign
        .map_recipes
        .iter()
        .find(|recipe| recipe.id == instance.map_recipe)
        .map_or(0, |recipe| recipe.hazards.len());
    rate(
        hostile_count,
        hostile_powers,
        instance.objective_kind,
        instance.round_limit,
        hazard_count,
        instance.operation_modifier,
    )
}

pub(crate) fn for_mission(mission: &MissionDef, data: &GameData) -> DangerRating {
    let (hostile_count, hostile_powers) =
        hostile_profile(&mission.hostile_unit_ids, &mission.hostile_faction, data);
    rate(
        hostile_count,
        hostile_powers,
        mission.objective_kind,
        mission.round_limit,
        mission.hazards.len(),
        mission.operation_modifier,
    )
}

fn hostile_profile(ids: &[String], faction: &str, data: &GameData) -> (usize, usize) {
    let hostiles = data.roster.iter().filter(|unit| {
        unit.team == Team::Hostile
            && if ids.is_empty() {
                unit.faction.as_deref() == Some(faction)
            } else {
                ids.contains(&unit.id)
            }
    });
    let mut powers = HashSet::new();
    let count = hostiles
        .inspect(|unit| {
            if let Some(faction) = &unit.faction {
                powers.insert(faction.as_str());
            }
        })
        .count();
    (count, powers.len())
}

fn rate(
    hostile_count: usize,
    hostile_powers: usize,
    objective: ObjectiveKind,
    round_limit: u32,
    hazard_count: usize,
    modifier: OperationModifier,
) -> DangerRating {
    let objective_pressure = match objective {
        ObjectiveKind::SecureAndClear | ObjectiveKind::EliminateAll => 1,
        ObjectiveKind::Extraction => 2,
        ObjectiveKind::Holdout | ObjectiveKind::SignalTrace | ObjectiveKind::DefendAsset => 4,
    };
    let reinforcement_pressure = matches!(
        objective,
        ObjectiveKind::Holdout | ObjectiveKind::SignalTrace
    ) as u8
        * 2;
    let deadline_pressure = match round_limit {
        0..=5 => 3,
        6 => 2,
        7 => 1,
        _ => 0,
    };
    let modifier_pressure = match modifier {
        OperationModifier::None => 0,
        OperationModifier::EscalationCrossfire => 4,
        _ => 2,
    };
    let mixed_power_pressure = hostile_powers.saturating_sub(1).min(3) as u8 * 2;
    let score = (hostile_count.min(8) as u8 * 2)
        + objective_pressure
        + reinforcement_pressure
        + deadline_pressure
        + hazard_count.min(4) as u8
        + modifier_pressure
        + mixed_power_pressure;
    let level = match score {
        0..=7 => DangerLevel::Routine,
        8..=12 => DangerLevel::Contested,
        13..=17 => DangerLevel::Severe,
        _ => DangerLevel::Extreme,
    };
    DangerRating { score, level }
}

#[cfg(test)]
mod tests;
