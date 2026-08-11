//! Deterministic, safety-checked transforms for authored map families.

use crate::data::{CoverEdgeDef, EdgeDirection, GameData, HazardDef, MapRecipeDef, TerrainCostDef};

// Campaign recipes were authored inside the original 12x8 combat footprint.
// The runtime world may be much larger, but recipe variants must stay inside
// that authored footprint so mirroring never turns a local encounter into a
// battlefield-spanning deployment.
const AUTHORED_MAX_Y: i32 = 7;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MapLayout {
    pub blocked_tiles: Vec<[i32; 2]>,
    pub objective_tile: [i32; 2],
    pub terrain_costs: Vec<TerrainCostDef>,
    pub hazards: Vec<HazardDef>,
    pub cover_edges: Vec<CoverEdgeDef>,
}

pub(crate) fn materialize(recipe: &MapRecipeDef, data: &GameData, seed: u64) -> MapLayout {
    let authored = MapLayout {
        blocked_tiles: recipe.blocked_tiles.clone(),
        objective_tile: recipe.objective_tile,
        terrain_costs: recipe.terrain_costs.clone(),
        hazards: recipe.hazards.clone(),
        cover_edges: recipe.cover_edges.clone(),
    };
    if seed & 1 == 0 {
        return authored;
    }
    let candidate = MapLayout {
        blocked_tiles: authored
            .blocked_tiles
            .iter()
            .map(|position| mirror_position(*position, AUTHORED_MAX_Y))
            .collect(),
        objective_tile: mirror_position(authored.objective_tile, AUTHORED_MAX_Y),
        terrain_costs: authored
            .terrain_costs
            .iter()
            .map(|entry| TerrainCostDef {
                position: mirror_position(entry.position, AUTHORED_MAX_Y),
                cost: entry.cost,
            })
            .collect(),
        hazards: authored
            .hazards
            .iter()
            .map(|hazard| HazardDef {
                position: mirror_position(hazard.position, AUTHORED_MAX_Y),
                kind: hazard.kind,
            })
            .collect(),
        cover_edges: authored
            .cover_edges
            .iter()
            .map(|edge| CoverEdgeDef {
                position: mirror_position(edge.position, AUTHORED_MAX_Y),
                direction: match edge.direction {
                    EdgeDirection::North => EdgeDirection::South,
                    EdgeDirection::South => EdgeDirection::North,
                    other => other,
                },
                strength: edge.strength,
            })
            .collect(),
    };
    if layout_is_safe(&candidate, data) {
        candidate
    } else {
        authored
    }
}

fn mirror_position(position: [i32; 2], max_y: i32) -> [i32; 2] {
    [position[0], max_y - position[1]]
}

fn layout_is_safe(layout: &MapLayout, data: &GameData) -> bool {
    let in_bounds = |position: [i32; 2]| {
        position[0] >= 0
            && position[1] >= 0
            && position[0] < data.config.world_width as i32
            && position[1] < data.config.world_height as i32
    };
    in_bounds(layout.objective_tile)
        && !layout.blocked_tiles.contains(&layout.objective_tile)
        && layout
            .blocked_tiles
            .iter()
            .all(|position| in_bounds(*position))
        && layout.hazards.iter().all(|hazard| {
            in_bounds(hazard.position) && !layout.blocked_tiles.contains(&hazard.position)
        })
        && data
            .roster
            .iter()
            .all(|unit| !layout.blocked_tiles.contains(&unit.position))
}

#[cfg(test)]
mod tests;
