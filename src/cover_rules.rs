//! Directional cover-facing rules shared by simulation and tactical previews.

use crate::data::{CoverEdgeDef, EdgeDirection};
use macroquad_toolkit::grid::TilePos;

pub(crate) fn penalty(edges: &[CoverEdgeDef], target: TilePos, attacker: TilePos) -> i32 {
    let direction = if (attacker.x - target.x).abs() >= (attacker.y - target.y).abs() {
        if attacker.x < target.x {
            EdgeDirection::West
        } else {
            EdgeDirection::East
        }
    } else if attacker.y < target.y {
        EdgeDirection::North
    } else {
        EdgeDirection::South
    };
    edges
        .iter()
        .find(|edge| {
            TilePos::new(edge.position[0], edge.position[1]) == target
                && edge.direction == direction
        })
        .map_or(0, |edge| i32::from(edge.strength))
}
