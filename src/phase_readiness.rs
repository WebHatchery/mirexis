//! Read-only remaining-colonist readiness used by the end-phase guard.

use crate::data::Team;
use crate::state::GameSession;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct ReadinessCounts {
    pub(crate) ready: usize,
    pub(crate) spent: usize,
    pub(crate) incapacitated: usize,
}

pub(crate) fn counts(session: &GameSession) -> ReadinessCounts {
    let mut counts = ReadinessCounts::default();
    for unit in session
        .tactical
        .units
        .iter()
        .filter(|unit| unit.team == Team::Colony)
    {
        if unit.incapacitated {
            counts.incapacitated += 1;
        } else if unit.action_points > 0 {
            counts.ready += 1;
        } else {
            counts.spent += 1;
        }
    }
    counts
}

pub(crate) fn ready_count(session: &GameSession) -> usize {
    counts(session).ready
}

pub(crate) fn select_next(session: &mut GameSession) -> Option<String> {
    let start = session
        .tactical
        .selected_unit
        .as_ref()
        .and_then(|selected| {
            session
                .tactical
                .units
                .iter()
                .position(|unit| &unit.id == selected)
        })
        .unwrap_or_else(|| session.tactical.units.len().saturating_sub(1));
    let candidate = (1..=session.tactical.units.len())
        .map(|offset| (start + offset) % session.tactical.units.len())
        .find_map(|index| {
            let unit = &session.tactical.units[index];
            (unit.team == Team::Colony && !unit.incapacitated && unit.action_points > 0)
                .then(|| (unit.id.clone(), unit.position))
        });
    if let Some((unit_id, position)) = candidate {
        session.tactical.selected_unit = Some(unit_id.clone());
        session.tactical.selected_tile = position;
        Some(unit_id)
    } else {
        None
    }
}

#[cfg(test)]
mod tests;
