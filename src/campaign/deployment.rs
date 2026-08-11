//! Safe large-world hostile entry placement for materialized operations.

use crate::data::{MissionDef, Team, UnitDef};
use std::collections::HashSet;

pub(super) fn spread_hostile_deployment(
    roster: &mut [UnitDef],
    mission: &MissionDef,
    width: i32,
    height: i32,
) {
    let hostile_count = roster
        .iter()
        .filter(|unit| unit.team == Team::Hostile)
        .count() as i32;
    let mut index = 0;
    let mut occupied = roster
        .iter()
        .filter(|unit| unit.team == Team::Colony)
        .map(|unit| unit.position)
        .collect::<HashSet<_>>();
    for unit in roster.iter_mut().filter(|unit| unit.team == Team::Hostile) {
        index += 1;
        let x_offset = [5, 7, 4][(index as usize - 1) % 3];
        let preferred = [width - x_offset, index * height / (hostile_count + 1)];
        let position = (width / 2..width)
            .flat_map(|x| (0..height).map(move |y| [x, y]))
            .filter(|position| hostile_entry_is_safe(*position, mission, width, height, &occupied))
            .min_by_key(|position| {
                (
                    (position[0] - preferred[0]).abs() + (position[1] - preferred[1]).abs(),
                    position[1],
                    position[0],
                )
            })
            .expect("validated large battlefield has a safe eastern hostile entry");
        unit.position = position;
        occupied.insert(position);
    }
}

fn hostile_entry_is_safe(
    position: [i32; 2],
    mission: &MissionDef,
    width: i32,
    height: i32,
    occupied: &HashSet<[i32; 2]>,
) -> bool {
    position[0] >= 0
        && position[1] >= 0
        && position[0] < width
        && position[1] < height
        && position != mission.objective_tile
        && !mission.blocked_tiles.contains(&position)
        && !mission
            .hazards
            .iter()
            .any(|hazard| hazard.position == position)
        && !occupied.contains(&position)
}
