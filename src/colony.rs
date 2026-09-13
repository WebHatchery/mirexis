//! Colony resources, construction placement, facilities, and defense-map derivation.
pub mod state;

use macroquad_toolkit::grid::TilePos;
use serde::{Deserialize, Serialize};

pub mod construction;
pub mod defense;
pub mod identity;
pub mod repairs;
pub mod upgrades;

pub use upgrades::*;

pub const COLONY_WIDTH: i32 = 20;
pub const COLONY_HEIGHT: i32 = 20;
pub const SETTLEMENT_CENTER: [i32; 2] = [10, 10];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepairResult {
    pub building_name: String,
    pub materials_spent: i32,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuildingKind {
    CommandCentre,
    Barracks,
    Infirmary,
    ResearchAnnex,
    SalvageYard,
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
    RedoubtArsenal,
    ChoirGarden,
    ThresholdSpire,
}

impl BuildingKind {
    pub fn name(self) -> &'static str {
        match self {
            Self::CommandCentre => "Command Centre",
            Self::Barracks => "Barracks",
            Self::Infirmary => "Infirmary",
            Self::ResearchAnnex => "Research Annex",
            Self::SalvageYard => "Salvage Yard",
            Self::Workshop => "Workshop",
            Self::Barricade => "Barricade",
            Self::Hydroponics => "Hydroponics",
            Self::PowerPlant => "Power Plant",
            Self::GeneLab => "Gene Lab",
            Self::Waystation => "Waystation",
            Self::Commons => "Commons",
            Self::RelayMast => "Relay Mast",
            Self::Watchtower => "Watchtower",
            Self::RedoubtArsenal => "Redoubt Arsenal",
            Self::ChoirGarden => "Choir Garden",
            Self::ThresholdSpire => "Threshold Spire",
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
            Self::ResearchAnnex => 60,
            Self::SalvageYard => 55,
            _ => 0,
        }
    }

    pub fn power_demand(self) -> i32 {
        match self {
            Self::CommandCentre
            | Self::Barracks
            | Self::Infirmary
            | Self::ResearchAnnex
            | Self::SalvageYard => 1,
            Self::Workshop | Self::Hydroponics => 2,
            Self::GeneLab => 3,
            Self::Waystation => 1,
            Self::Commons => 1,
            Self::RelayMast => 1,
            Self::Watchtower => 1,
            Self::RedoubtArsenal | Self::ChoirGarden => 1,
            Self::ThresholdSpire => 2,
            Self::Barricade | Self::PowerPlant => 0,
        }
    }

    pub fn power_output(self) -> i32 {
        i32::from(self == Self::PowerPlant) * 4
    }

    pub fn repair_cost(self) -> i32 {
        match self {
            Self::Barricade => 10,
            Self::CommandCentre
            | Self::RedoubtArsenal
            | Self::ChoirGarden
            | Self::ThresholdSpire => 35,
            Self::GeneLab | Self::Waystation => 30,
            _ => 25,
        }
    }

    pub fn footprint(self) -> &'static [[i32; 2]] {
        &[[0, 0]]
    }

    pub fn occupies(self, anchor: [i32; 2], position: [i32; 2]) -> bool {
        self.footprint()
            .iter()
            .any(|offset| [anchor[0] + offset[0], anchor[1] + offset[1]] == position)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FacilityUpgradeOption {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FacilityUpgradeProject {
    pub building_id: String,
    pub upgrade_id: String,
    pub operations_remaining: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FacilityUpgradeState {
    pub building_id: String,
    pub upgrade_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColonyDefenseMap {
    pub blocked_tiles: Vec<TilePos>,
    pub cover_tiles: Vec<TilePos>,
    pub watchtower_tiles: Vec<TilePos>,
    pub shield_tiles: Vec<TilePos>,
    pub critical_objectives: Vec<TilePos>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColonyState {
    pub resources: Resources,
    pub buildings: Vec<BuildingState>,
    pub construction_queue: Vec<ConstructionProject>,
    #[serde(default)]
    pub facility_upgrade_queue: Vec<FacilityUpgradeProject>,
    #[serde(default)]
    pub facility_upgrades: Vec<FacilityUpgradeState>,
    #[serde(default)]
    pub planned_construction: BuildingKind,
    next_building_serial: u32,
}

pub fn founding_building_position(id: &str) -> Option<[i32; 2]> {
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

pub fn migrated_open_plot(
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

pub fn clearance_in_bounds(anchor: [i32; 2]) -> bool {
    clearance_positions(anchor).all(|position| {
        position[0] >= 0
            && position[1] >= 0
            && position[0] < COLONY_WIDTH
            && position[1] < COLONY_HEIGHT
    })
}

pub fn clearance_positions(anchor: [i32; 2]) -> impl Iterator<Item = [i32; 2]> {
    (-1..=1).flat_map(move |dy| (-1..=1).map(move |dx| [anchor[0] + dx, anchor[1] + dy]))
}

pub fn clearance_zones_overlap(left: [i32; 2], right: [i32; 2]) -> bool {
    (left[0] - right[0]).abs() <= 2 && (left[1] - right[1]).abs() <= 2
}

pub fn reserve_anchor(occupied: &mut std::collections::HashSet<[i32; 2]>, anchor: [i32; 2]) {
    occupied.insert(anchor);
}

pub fn building(id: &str, kind: BuildingKind, position: [i32; 2]) -> BuildingState {
    BuildingState {
        id: id.to_owned(),
        kind,
        position,
        level: 1,
        damaged: false,
    }
}
