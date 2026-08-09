//! Safe deterministic colony entry formations applied immediately before deployment.

use crate::data::{GameConfig, MissionDef, Team, UnitDef};
use std::collections::HashSet;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FormationKind {
    #[default]
    Wedge,
    Line,
    Column,
}

impl FormationKind {
    pub fn label(self) -> &'static str {
        match self {
            Self::Wedge => "WEDGE",
            Self::Line => "LINE",
            Self::Column => "COLUMN",
        }
    }

    pub fn next(self) -> Self {
        match self {
            Self::Wedge => Self::Line,
            Self::Line => Self::Column,
            Self::Column => Self::Wedge,
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
        FormationKind::Wedge => [[1, 3], [2, 2], [2, 4]],
        FormationKind::Line => [[1, 2], [1, 4], [1, 6]],
        FormationKind::Column => [[1, 2], [1, 3], [1, 4]],
    };
    let mut occupied = roster
        .iter()
        .filter(|unit| unit.team != Team::Colony)
        .map(|unit| unit.position)
        .collect::<HashSet<_>>();
    let mut fallback = (0..config.world_width.min(3) as i32)
        .flat_map(|x| (1..config.world_height as i32).map(move |y| [x, y]));
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
mod tests {
    use super::*;
    use crate::data::GameData;

    #[test]
    fn formations_are_distinct_and_fall_back_around_unsafe_entry_cells() {
        let data = GameData::load().unwrap();
        let mut mission = data.mission.clone();
        mission.blocked_tiles.extend([[1, 3], [1, 4]]);
        let mut shapes = Vec::new();
        for formation in [
            FormationKind::Wedge,
            FormationKind::Line,
            FormationKind::Column,
        ] {
            let mut roster = data.roster.clone();
            apply(&mut roster, &mission, &data.config, formation);
            let positions = roster
                .iter()
                .filter(|unit| unit.team == Team::Colony)
                .take(3)
                .map(|unit| unit.position)
                .collect::<Vec<_>>();
            assert_eq!(positions.iter().collect::<HashSet<_>>().len(), 3);
            assert!(positions.iter().all(|position| is_safe(
                *position,
                &mission,
                &data.config,
                &HashSet::new()
            )));
            shapes.push(positions);
        }
        assert_ne!(shapes[0], shapes[1]);
        assert_ne!(shapes[1], shapes[2]);
    }
}
