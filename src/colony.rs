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
    Waystation,
    Commons,
    RelayMast,
    Watchtower,
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
            Self::Waystation => "Waystation",
            Self::Commons => "Commons",
            Self::RelayMast => "Relay Mast",
            Self::Watchtower => "Watchtower",
        }
    }

    pub fn material_cost(self) -> i32 {
        match self {
            Self::Barricade => 20,
            Self::PowerPlant => 45,
            Self::GeneLab => 50,
            Self::Waystation => 55,
            Self::Commons => 40,
            Self::RelayMast => 45,
            Self::Watchtower => 35,
            _ => 0,
        }
    }

    pub fn power_demand(self) -> i32 {
        match self {
            Self::CommandCentre | Self::Barracks | Self::Infirmary => 1,
            Self::Workshop | Self::Hydroponics => 2,
            Self::GeneLab => 3,
            Self::Waystation => 1,
            Self::Commons => 1,
            Self::RelayMast => 1,
            Self::Watchtower => 1,
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
            Self::GeneLab | Self::Waystation => 30,
            _ => 25,
        }
    }

    pub fn footprint(self) -> &'static [[i32; 2]] {
        &[[0, 0]]
    }

    fn occupies(self, anchor: [i32; 2], position: [i32; 2]) -> bool {
        self.footprint()
            .iter()
            .any(|offset| [anchor[0] + offset[0], anchor[1] + offset[1]] == position)
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
    pub watchtower_tiles: Vec<TilePos>,
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

    pub(crate) fn migrate_legacy_spatial_layout(&mut self) -> bool {
        let legacy = self
            .buildings
            .iter()
            .any(|building| building.id == "command_centre" && building.position == [3, 2]);
        if !legacy {
            return false;
        }

        let mut occupied = std::collections::HashSet::new();
        for building in &mut self.buildings {
            if let Some(position) = founding_building_position(&building.id) {
                building.position = position;
                reserve_anchor(&mut occupied, position);
            }
        }
        for building in &mut self.buildings {
            if founding_building_position(&building.id).is_some() {
                continue;
            }
            building.position = migrated_open_plot(building.position, &occupied);
            reserve_anchor(&mut occupied, building.position);
        }
        for project in &mut self.construction_queue {
            project.position = migrated_open_plot(project.position, &occupied);
            reserve_anchor(&mut occupied, project.position);
        }
        true
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
            BuildingKind::Barricade
                | BuildingKind::PowerPlant
                | BuildingKind::GeneLab
                | BuildingKind::Waystation
                | BuildingKind::Commons
                | BuildingKind::RelayMast
                | BuildingKind::Watchtower
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
        self.validate_construction_site(position)?;
        if matches!(
            kind,
            BuildingKind::GeneLab
                | BuildingKind::Waystation
                | BuildingKind::Commons
                | BuildingKind::RelayMast
                | BuildingKind::Watchtower
        ) && (self.buildings.iter().any(|building| building.kind == kind)
            || self
                .construction_queue
                .iter()
                .any(|project| project.kind == kind))
        {
            return Err(format!("The colony can support only one {}", kind.name()));
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
        let mut watchtower_tiles = Vec::new();
        let mut critical_objectives = Vec::new();
        for building in &self.buildings {
            for offset in building.kind.footprint() {
                let position = TilePos::new(
                    building.position[0] + offset[0] + 2,
                    building.position[1] + offset[1] + 1,
                );
                blocked_tiles.push(position);
                match building.kind {
                    BuildingKind::Barricade | BuildingKind::Barracks | BuildingKind::Workshop => {
                        cover_tiles.push(position)
                    }
                    BuildingKind::Watchtower
                        if !building.damaged && self.building_is_powered(&building.id) =>
                    {
                        cover_tiles.push(position);
                        watchtower_tiles.push(position);
                    }
                    BuildingKind::Watchtower => {}
                    BuildingKind::CommandCentre
                    | BuildingKind::Infirmary
                    | BuildingKind::Hydroponics
                    | BuildingKind::PowerPlant
                    | BuildingKind::GeneLab
                    | BuildingKind::Waystation
                    | BuildingKind::Commons
                    | BuildingKind::RelayMast => critical_objectives.push(position),
                }
            }
        }
        ColonyDefenseMap {
            blocked_tiles,
            cover_tiles,
            watchtower_tiles,
            critical_objectives,
        }
    }

    #[cfg(test)]
    fn is_occupied(&self, position: [i32; 2]) -> bool {
        self.buildings
            .iter()
            .any(|building| building.kind.occupies(building.position, position))
            || self
                .construction_queue
                .iter()
                .any(|project| project.kind.occupies(project.position, position))
    }

    pub(crate) fn validate_construction_site(&self, position: [i32; 2]) -> Result<(), String> {
        if !clearance_in_bounds(position) {
            return Err("Site needs one in-bounds clearance tile on every side".to_owned());
        }
        if self
            .buildings
            .iter()
            .any(|building| clearance_zones_overlap(position, building.position))
            || self
                .construction_queue
                .iter()
                .any(|project| clearance_zones_overlap(position, project.position))
        {
            return Err("Site's 3x3 clearance zone overlaps another structure".to_owned());
        }
        Ok(())
    }

    pub(crate) fn building_at(&self, position: [i32; 2]) -> Option<&BuildingState> {
        self.buildings
            .iter()
            .find(|building| building.kind.occupies(building.position, position))
    }

    pub(crate) fn project_at(&self, position: [i32; 2]) -> Option<&ConstructionProject> {
        self.construction_queue
            .iter()
            .find(|project| project.kind.occupies(project.position, position))
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
        let open = |position: [i32; 2]| self.validate_construction_site(position).is_ok();
        if open(preferred) {
            return preferred;
        }
        (0..COLONY_HEIGHT)
            .flat_map(|y| (0..COLONY_WIDTH).map(move |x| [x, y]))
            .find(|position| open(*position))
            .expect("the colony has room for required Phase One infrastructure")
    }
}

fn founding_building_position(id: &str) -> Option<[i32; 2]> {
    match id {
        "command_centre" => Some(SETTLEMENT_CENTER),
        "barracks" => Some([6, 8]),
        "infirmary" => Some([10, 6]),
        "workshop" => Some([14, 8]),
        "hydroponics" => Some([7, 13]),
        "power_plant" => Some([13, 13]),
        _ => None,
    }
}

fn migrated_open_plot(
    legacy: [i32; 2],
    occupied: &std::collections::HashSet<[i32; 2]>,
) -> [i32; 2] {
    let preferred = [
        (legacy[0] * 2 + 3).clamp(0, COLONY_WIDTH - 1),
        (legacy[1] * 2 + 4).clamp(0, COLONY_HEIGHT - 1),
    ];
    (0..COLONY_HEIGHT)
        .flat_map(|y| (0..COLONY_WIDTH).map(move |x| [x, y]))
        .filter(|position| {
            clearance_in_bounds(*position)
                && occupied
                    .iter()
                    .all(|anchor| !clearance_zones_overlap(*position, *anchor))
        })
        .min_by_key(|position| {
            (
                (position[0] - preferred[0]).abs() + (position[1] - preferred[1]).abs(),
                position[1],
                position[0],
            )
        })
        .expect("the expanded colony has room for migrated construction")
}

fn clearance_in_bounds(anchor: [i32; 2]) -> bool {
    clearance_positions(anchor).all(|position| {
        position[0] >= 0
            && position[1] >= 0
            && position[0] < COLONY_WIDTH
            && position[1] < COLONY_HEIGHT
    })
}

fn clearance_positions(anchor: [i32; 2]) -> impl Iterator<Item = [i32; 2]> {
    (-1..=1).flat_map(move |dy| (-1..=1).map(move |dx| [anchor[0] + dx, anchor[1] + dy]))
}

fn clearance_zones_overlap(left: [i32; 2], right: [i32; 2]) -> bool {
    (left[0] - right[0]).abs() <= 2 && (left[1] - right[1]).abs() <= 2
}

fn reserve_anchor(occupied: &mut std::collections::HashSet<[i32; 2]>, anchor: [i32; 2]) {
    occupied.insert(anchor);
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
