//! Colony resources, construction placement, facilities, and defense-map derivation.

use macroquad_toolkit::grid::TilePos;
use serde::{Deserialize, Serialize};

pub const COLONY_WIDTH: i32 = 20;
pub const COLONY_HEIGHT: i32 = 20;
pub const SETTLEMENT_CENTER: [i32; 2] = [10, 10];

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuildingKind {
    CommandCentre,
    Barracks,
    Infirmary,
    Workshop,
    #[default]
    Barricade,
    Hydroponics,
    PowerPlant,
    GeneLab,
}

impl BuildingKind {
    pub fn name(self) -> &'static str {
        match self {
            Self::CommandCentre => "Command Centre",
            Self::Barracks => "Barracks",
            Self::Infirmary => "Infirmary",
            Self::Workshop => "Workshop",
            Self::Barricade => "Barricade",
            Self::Hydroponics => "Hydroponics",
            Self::PowerPlant => "Power Plant",
            Self::GeneLab => "Gene Lab",
        }
    }

    pub fn material_cost(self) -> i32 {
        match self {
            Self::Barricade => 20,
            Self::PowerPlant => 45,
            Self::GeneLab => 50,
            _ => 0,
        }
    }

    pub fn power_demand(self) -> i32 {
        match self {
            Self::CommandCentre | Self::Barracks | Self::Infirmary => 1,
            Self::Workshop | Self::Hydroponics => 2,
            Self::GeneLab => 3,
            Self::Barricade | Self::PowerPlant => 0,
        }
    }

    pub fn power_output(self) -> i32 {
        i32::from(self == Self::PowerPlant) * 4
    }

    pub fn repair_cost(self) -> i32 {
        match self {
            Self::Barricade => 10,
            Self::CommandCentre => 35,
            Self::GeneLab => 30,
            _ => 25,
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
    #[serde(default)]
    pub planned_construction: BuildingKind,
    next_building_serial: u32,
}

impl ColonyState {
    pub fn new() -> Self {
        Self {
            resources: Resources {
                materials: 120,
                power: 4,
                food: 24,
                biomass: 12,
                alien_components: 0,
            },
            buildings: vec![
                building(
                    "command_centre",
                    BuildingKind::CommandCentre,
                    SETTLEMENT_CENTER,
                ),
                building("barracks", BuildingKind::Barracks, [6, 8]),
                building("infirmary", BuildingKind::Infirmary, [10, 6]),
                building("workshop", BuildingKind::Workshop, [14, 8]),
                building("hydroponics", BuildingKind::Hydroponics, [7, 13]),
                building("power_plant", BuildingKind::PowerPlant, [13, 13]),
            ],
            construction_queue: Vec::new(),
            planned_construction: BuildingKind::Barricade,
            next_building_serial: 1,
        }
    }

    pub fn has_facility(&self, kind: BuildingKind) -> bool {
        self.buildings.iter().any(|building| {
            building.kind == kind && !building.damaged && self.building_is_powered(&building.id)
        })
    }

    pub fn power_supply(&self) -> i32 {
        self.resources.power
            + self
                .buildings
                .iter()
                .filter(|building| !building.damaged)
                .map(|building| building.kind.power_output())
                .sum::<i32>()
    }

    pub fn power_demand(&self) -> i32 {
        self.buildings
            .iter()
            .filter(|building| !building.damaged)
            .map(|building| building.kind.power_demand())
            .sum()
    }

    pub fn building_is_powered(&self, building_id: &str) -> bool {
        let mut remaining = self.power_supply();
        for building in self.buildings.iter().filter(|building| !building.damaged) {
            let demand = building.kind.power_demand();
            let powered = demand <= remaining;
            if powered {
                remaining -= demand;
            }
            if building.id == building_id {
                return powered;
            }
        }
        false
    }

    pub fn select_construction(&mut self, kind: BuildingKind) -> Result<(), String> {
        if !matches!(
            kind,
            BuildingKind::Barricade | BuildingKind::PowerPlant | BuildingKind::GeneLab
        ) {
            return Err(format!("{} cannot be planned here", kind.name()));
        }
        self.planned_construction = kind;
        Ok(())
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
        if kind == BuildingKind::GeneLab
            && (self
                .buildings
                .iter()
                .any(|building| building.kind == BuildingKind::GeneLab)
                || self
                    .construction_queue
                    .iter()
                    .any(|project| project.kind == BuildingKind::GeneLab))
        {
            return Err("The colony can support only one Gene Lab".to_owned());
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
        self.resources.food += if self.has_facility(BuildingKind::Hydroponics) {
            3
        } else if self.has_facility(BuildingKind::CommandCentre) {
            1
        } else {
            0
        };
    }

    pub fn damage_for_failed_defense(&mut self, seed: u64) -> Option<String> {
        let mut candidates = self
            .buildings
            .iter()
            .enumerate()
            .filter(|(_, building)| {
                !building.damaged
                    && !matches!(
                        building.kind,
                        BuildingKind::CommandCentre | BuildingKind::Barricade
                    )
            })
            .map(|(index, _)| index)
            .collect::<Vec<_>>();
        if candidates.is_empty() {
            candidates = self
                .buildings
                .iter()
                .enumerate()
                .filter(|(_, building)| !building.damaged)
                .map(|(index, _)| index)
                .collect();
        }
        let index = *candidates.get(seed as usize % candidates.len().max(1))?;
        let building = &mut self.buildings[index];
        building.damaged = true;
        Some(building.kind.name().to_owned())
    }

    pub fn repair_building(&mut self, building_id: &str) -> Result<(String, i32), String> {
        let building = self
            .buildings
            .iter_mut()
            .find(|building| building.id == building_id)
            .ok_or_else(|| format!("Unknown colony building: {}", building_id))?;
        if !building.damaged {
            return Err(format!("{} does not need repair", building.kind.name()));
        }
        let cost = building.kind.repair_cost();
        if self.resources.materials < cost {
            return Err(format!("Repair requires {} materials", cost));
        }
        self.resources.materials -= cost;
        building.damaged = false;
        Ok((building.kind.name().to_owned(), cost))
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
                | BuildingKind::Hydroponics
                | BuildingKind::PowerPlant
                | BuildingKind::GeneLab => critical_objectives.push(position),
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

    pub fn ensure_phase_one_infrastructure(&mut self, migrate_power: bool) {
        if !self
            .buildings
            .iter()
            .any(|building| building.kind == BuildingKind::Hydroponics)
        {
            let position = self.first_open_plot([7, 13]);
            self.buildings
                .push(building("hydroponics", BuildingKind::Hydroponics, position));
        }
        if !self
            .buildings
            .iter()
            .any(|building| building.kind == BuildingKind::PowerPlant)
        {
            let position = self.first_open_plot([13, 13]);
            self.buildings
                .push(building("power_plant", BuildingKind::PowerPlant, position));
            if migrate_power {
                self.resources.power = (self.resources.power - 4).max(0);
            }
        }
    }

    pub fn ensure_gene_lab(&mut self) {
        if self
            .buildings
            .iter()
            .any(|building| building.kind == BuildingKind::GeneLab)
        {
            return;
        }
        let position = self.first_open_plot([10, 14]);
        self.buildings
            .push(building("gene_lab", BuildingKind::GeneLab, position));
    }

    fn first_open_plot(&self, preferred: [i32; 2]) -> [i32; 2] {
        if !self.is_occupied(preferred) {
            return preferred;
        }
        (0..COLONY_HEIGHT)
            .flat_map(|y| (0..COLONY_WIDTH).map(move |x| [x, y]))
            .find(|position| !self.is_occupied(*position))
            .expect("the colony has room for required Phase One infrastructure")
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
mod tests;
