//! Colony resources, construction placement, facilities, and defense-map derivation.

use macroquad_toolkit::grid::TilePos;
use serde::{Deserialize, Serialize};

pub const COLONY_WIDTH: i32 = 8;
pub const COLONY_HEIGHT: i32 = 6;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuildingKind {
    CommandCentre,
    Barracks,
    Infirmary,
    Workshop,
    Barricade,
    PowerPlant,
}

impl BuildingKind {
    pub fn name(self) -> &'static str {
        match self {
            Self::CommandCentre => "Command Centre",
            Self::Barracks => "Barracks",
            Self::Infirmary => "Infirmary",
            Self::Workshop => "Workshop",
            Self::Barricade => "Barricade",
            Self::PowerPlant => "Power Plant",
        }
    }

    pub fn material_cost(self) -> i32 {
        match self {
            Self::Barricade => 20,
            Self::PowerPlant => 45,
            _ => 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Resources {
    pub materials: i32,
    pub power: i32,
    pub food: i32,
    pub biomass: i32,
    pub alien_components: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildingState {
    pub id: String,
    pub kind: BuildingKind,
    pub position: [i32; 2],
    pub level: u8,
    pub damaged: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstructionProject {
    pub id: String,
    pub kind: BuildingKind,
    pub position: [i32; 2],
    pub operations_remaining: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColonyDefenseMap {
    pub blocked_tiles: Vec<TilePos>,
    pub cover_tiles: Vec<TilePos>,
    pub critical_objectives: Vec<TilePos>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColonyState {
    pub resources: Resources,
    pub buildings: Vec<BuildingState>,
    pub construction_queue: Vec<ConstructionProject>,
    pub mission_offers: Vec<String>,
    pub selected_mission: Option<String>,
    next_building_serial: u32,
}

impl ColonyState {
    pub fn new() -> Self {
        Self {
            resources: Resources {
                materials: 120,
                power: 8,
                food: 24,
                biomass: 12,
                alien_components: 0,
            },
            buildings: vec![
                building("command_centre", BuildingKind::CommandCentre, [3, 2]),
                building("barracks", BuildingKind::Barracks, [2, 3]),
                building("infirmary", BuildingKind::Infirmary, [4, 3]),
                building("workshop", BuildingKind::Workshop, [3, 4]),
            ],
            construction_queue: Vec::new(),
            mission_offers: vec!["operation_glassroot".to_owned()],
            selected_mission: Some("operation_glassroot".to_owned()),
            next_building_serial: 1,
        }
    }

    pub fn has_facility(&self, kind: BuildingKind) -> bool {
        self.buildings
            .iter()
            .any(|building| building.kind == kind && !building.damaged)
    }

    pub fn place_construction(
        &mut self,
        kind: BuildingKind,
        position: [i32; 2],
    ) -> Result<String, String> {
        if position[0] < 0
            || position[1] < 0
            || position[0] >= COLONY_WIDTH
            || position[1] >= COLONY_HEIGHT
        {
            return Err("Plot is outside the colony footprint".to_owned());
        }
        if self.is_occupied(position) {
            return Err("Plot is already occupied or reserved".to_owned());
        }
        let cost = kind.material_cost();
        if self.resources.materials < cost {
            return Err(format!("Construction requires {} materials", cost));
        }
        self.resources.materials -= cost;
        let id = format!("{:?}_{}", kind, self.next_building_serial).to_lowercase();
        self.next_building_serial += 1;
        self.construction_queue.push(ConstructionProject {
            id: id.clone(),
            kind,
            position,
            operations_remaining: 1,
        });
        Ok(id)
    }

    pub fn advance_operation(&mut self) {
        for project in &mut self.construction_queue {
            project.operations_remaining = project.operations_remaining.saturating_sub(1);
        }
        let completed = self
            .construction_queue
            .iter()
            .filter(|project| project.operations_remaining == 0)
            .cloned()
            .collect::<Vec<_>>();
        self.construction_queue
            .retain(|project| project.operations_remaining > 0);
        self.buildings
            .extend(completed.into_iter().map(|project| BuildingState {
                id: project.id,
                kind: project.kind,
                position: project.position,
                level: 1,
                damaged: false,
            }));
    }

    pub fn defense_map(&self) -> ColonyDefenseMap {
        let mut blocked_tiles = Vec::new();
        let mut cover_tiles = Vec::new();
        let mut critical_objectives = Vec::new();
        for building in &self.buildings {
            let position = TilePos::new(building.position[0] + 2, building.position[1] + 1);
            blocked_tiles.push(position);
            match building.kind {
                BuildingKind::Barricade | BuildingKind::Barracks | BuildingKind::Workshop => {
                    cover_tiles.push(position)
                }
                BuildingKind::CommandCentre
                | BuildingKind::Infirmary
                | BuildingKind::PowerPlant => critical_objectives.push(position),
            }
        }
        ColonyDefenseMap {
            blocked_tiles,
            cover_tiles,
            critical_objectives,
        }
    }

    fn is_occupied(&self, position: [i32; 2]) -> bool {
        self.buildings
            .iter()
            .any(|building| building.position == position)
            || self
                .construction_queue
                .iter()
                .any(|project| project.position == position)
    }
}

fn building(id: &str, kind: BuildingKind, position: [i32; 2]) -> BuildingState {
    BuildingState {
        id: id.to_owned(),
        kind,
        position,
        level: 1,
        damaged: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construction_reserves_resources_and_completes_after_an_operation() {
        let mut colony = ColonyState::new();
        colony
            .place_construction(BuildingKind::Barricade, [1, 1])
            .unwrap();
        assert_eq!(colony.resources.materials, 100);
        assert!(colony
            .place_construction(BuildingKind::Barricade, [1, 1])
            .is_err());
        colony.advance_operation();
        assert!(colony.buildings.iter().any(|building| {
            building.kind == BuildingKind::Barricade && building.position == [1, 1]
        }));
    }

    #[test]
    fn physical_placement_generates_the_colony_defense_map() {
        let mut colony = ColonyState::new();
        colony
            .place_construction(BuildingKind::Barricade, [1, 1])
            .unwrap();
        colony.advance_operation();
        let map = colony.defense_map();
        assert!(map.cover_tiles.contains(&TilePos::new(3, 2)));
        assert!(map.critical_objectives.contains(&TilePos::new(5, 3)));
    }
}
