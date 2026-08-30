//! Safe deterministic colony entry formations applied immediately before deployment.

use crate::data::{GameConfig, MissionDef, Team, UnitDef};
use std::collections::HashSet;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FormationKind {
    #[default]
    Wedge,
    Line,
    Column,
    Rally,
}

impl FormationKind {
    pub fn label(self) -> &'static str {
        match self {
            Self::Wedge => "WEDGE",
            Self::Line => "LINE",
            Self::Column => "COLUMN",
            Self::Rally => "RALLY",
        }
    }

    pub fn next(self) -> Self {
        match self {
            Self::Wedge => Self::Line,
            Self::Line => Self::Column,
            Self::Column => Self::Wedge,
            Self::Rally => Self::Wedge,
        }
    }

    pub(crate) fn next_with_doctrine_yard(self, doctrine_yard_active: bool) -> Self {
        match self {
            Self::Column if doctrine_yard_active => Self::Rally,
            Self::Rally => Self::Wedge,
            other => other.next(),
        }
    }

    pub(crate) fn without_doctrine_yard(self) -> Self {
        if self == Self::Rally {
            Self::Wedge
        } else {
            self
        }
    }
}

pub(crate) fn apply(
    roster: &mut [UnitDef],
    mission: &MissionDef,
    config: &GameConfig,
    formation: FormationKind,
) {
    let candidates = match formation {
        FormationKind::Wedge => [[4, 19], [5, 18], [5, 20]],
        FormationKind::Line => [[4, 17], [4, 19], [4, 21]],
        FormationKind::Column => [[4, 18], [4, 19], [4, 20]],
        FormationKind::Rally => [[4, 19], [5, 19], [6, 19]],
    };
    let mut occupied = roster
        .iter()
        .filter(|unit| unit.team != Team::Colony)
        .map(|unit| unit.position)
        .collect::<HashSet<_>>();
    let mut fallback = (3..config.world_width.min(7) as i32)
        .flat_map(|x| (16..config.world_height.min(24) as i32).map(move |y| [x, y]));
    for (index, unit) in roster
        .iter_mut()
        .filter(|unit| unit.team == Team::Colony)
        .enumerate()
    {
        let preferred = candidates.get(index).copied();
        let position = preferred
            .filter(|position| is_safe(*position, mission, config, &occupied))
            .or_else(|| fallback.find(|position| is_safe(*position, mission, config, &occupied)));
        if let Some(position) = position {
            unit.position = position;
            occupied.insert(position);
        }
    }
}

fn is_safe(
    position: [i32; 2],
    mission: &MissionDef,
    config: &GameConfig,
    occupied: &HashSet<[i32; 2]>,
) -> bool {
    position[0] >= 0
        && position[1] >= 0
        && position[0] < config.world_width as i32
        && position[1] < config.world_height as i32
        && position != mission.objective_tile
        && !mission.blocked_tiles.contains(&position)
        && !mission
            .hazards
            .iter()
            .any(|hazard| hazard.position == position)
        && !occupied.contains(&position)
}

#[cfg(test)]
mod tests;
