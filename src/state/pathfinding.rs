//! Occupancy-aware tactical route calculation.

use super::GameSession;
use crate::tactical::terrain_cost;
use macroquad_toolkit::grid::TilePos;
use macroquad_toolkit::pathfinding::{find_path_with, Heuristic, Pos};
use std::collections::HashSet;

impl GameSession {
    pub(super) fn path_for(
        &self,
        start: TilePos,
        goal: TilePos,
        mover: Option<&str>,
    ) -> Option<Vec<TilePos>> {
        let occupied = self
            .tactical
            .units
            .iter()
            .filter(|unit| !unit.incapacitated && Some(unit.id.as_str()) != mover)
            .map(|unit| unit.position)
            .collect::<HashSet<_>>();
        let path = find_path_with(
            Pos::new(start.x, start.y),
            Pos::new(goal.x, goal.y),
            self.tactical.fog.width,
            self.tactical.fog.height,
            |pos| {
                let pos = TilePos::new(pos.x, pos.y);
                !self.tactical.blocked.contains(&pos) && !occupied.contains(&pos)
            },
            |pos| {
                f32::from(terrain_cost(
                    TilePos::new(pos.x, pos.y),
                    &self.tactical.terrain_costs,
                ))
            },
            Heuristic::Manhattan,
            false,
        )?;
        Some(
            path.waypoints
                .into_iter()
                .map(|pos| TilePos::new(pos.x, pos.y))
                .collect(),
        )
    }

    pub(crate) fn movement_path(&self, unit_id: &str, goal: TilePos) -> Option<Vec<TilePos>> {
        let unit = self.unit(unit_id)?;
        self.path_for(unit.position, goal, Some(unit_id))
    }
}
