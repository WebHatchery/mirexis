//! Deterministic, safety-checked transforms for authored map families.

use crate::data::{CoverEdgeDef, EdgeDirection, GameData, HazardDef, MapRecipeDef, TerrainCostDef};

// Campaign recipes retain their compact 12x8 authoring coordinates in JSON.
// Materialization projects that vocabulary into the 40x40 runtime battlefield.
const AUTHORED_MAX_Y: i32 = 7;
const BATTLEFIELD_X_OFFSET: i32 = 10;
const BATTLEFIELD_Y_OFFSET: i32 = 5;
const BATTLEFIELD_X_STRIDE: i32 = 2;
const BATTLEFIELD_Y_STRIDE: i32 = 4;

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
        blocked_tiles: recipe
            .blocked_tiles
            .iter()
            .map(|position| battlefield_position(*position))
            .collect(),
        objective_tile: battlefield_position(recipe.objective_tile),
        terrain_costs: recipe
            .terrain_costs
            .iter()
            .map(|entry| TerrainCostDef {
                position: battlefield_position(entry.position),
                cost: entry.cost,
            })
            .collect(),
        hazards: recipe
            .hazards
            .iter()
            .map(|hazard| HazardDef {
                position: battlefield_position(hazard.position),
                kind: hazard.kind,
            })
            .collect(),
        cover_edges: recipe
            .cover_edges
            .iter()
            .map(|edge| CoverEdgeDef {
                position: battlefield_position(edge.position),
                direction: edge.direction,
                strength: edge.strength,
            })
            .collect(),
    };
    if seed & 1 == 0 {
        return authored;
    }
    let candidate = MapLayout {
        blocked_tiles: authored
            .blocked_tiles
            .iter()
            .map(|position| mirror_battlefield_position(*position))
            .collect(),
        objective_tile: mirror_battlefield_position(authored.objective_tile),
        terrain_costs: authored
            .terrain_costs
            .iter()
            .map(|entry| TerrainCostDef {
                position: mirror_battlefield_position(entry.position),
                cost: entry.cost,
            })
            .collect(),
        hazards: authored
            .hazards
            .iter()
            .map(|hazard| HazardDef {
                position: mirror_battlefield_position(hazard.position),
                kind: hazard.kind,
            })
            .collect(),
        cover_edges: authored
            .cover_edges
            .iter()
            .map(|edge| CoverEdgeDef {
                position: mirror_battlefield_position(edge.position),
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

fn battlefield_position(position: [i32; 2]) -> [i32; 2] {
    [
        BATTLEFIELD_X_OFFSET + position[0] * BATTLEFIELD_X_STRIDE,
        BATTLEFIELD_Y_OFFSET + position[1] * BATTLEFIELD_Y_STRIDE,
    ]
}

fn mirror_battlefield_position(position: [i32; 2]) -> [i32; 2] {
    let authored_span = AUTHORED_MAX_Y * BATTLEFIELD_Y_STRIDE;
    [
        position[0],
        BATTLEFIELD_Y_OFFSET + authored_span - (position[1] - BATTLEFIELD_Y_OFFSET),
    ]
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
        && layout
            .terrain_costs
            .iter()
            .all(|entry| in_bounds(entry.position))
        && layout
            .cover_edges
            .iter()
            .all(|edge| in_bounds(edge.position))
        && data
            .roster
            .iter()
            .all(|unit| !layout.blocked_tiles.contains(&unit.position))
}

#[cfg(test)]
mod tests;
