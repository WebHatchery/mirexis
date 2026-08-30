//! Level-two facility branches and their shared construction cost.

use super::{BuildingKind, FacilityUpgradeOption};

pub const REDUNDANT_GRID_UPGRADE: &str = "redundant_grid";
pub const HOT_CORE_UPGRADE: &str = "hot_core";
pub const COMMUNITY_KITCHEN_UPGRADE: &str = "community_kitchen";
pub const CULTURE_BEDS_UPGRADE: &str = "culture_beds";
pub const PRECISION_BENCH_UPGRADE: &str = "precision_bench";
pub const DRONE_BAY_UPGRADE: &str = "drone_bay";
pub const STABILISATION_WING_UPGRADE: &str = "stabilisation_wing";
pub const EVOLUTION_CHAMBER_UPGRADE: &str = "evolution_chamber";
pub const SIGNAL_CARTOGRAPHY_UPGRADE: &str = "signal_cartography";
pub const COUNTERINTELLIGENCE_CELL_UPGRADE: &str = "counterintelligence_cell";

impl BuildingKind {
    pub fn upgrade_options(self) -> &'static [FacilityUpgradeOption] {
        match self {
            Self::PowerPlant => &[
                FacilityUpgradeOption {
                    id: REDUNDANT_GRID_UPGRADE,
                    name: "Redundant Grid",
                    description: "A damaged plant still routes two power through the colony.",
                },
                FacilityUpgradeOption {
                    id: HOT_CORE_UPGRADE,
                    name: "Hot Core",
                    description: "The plant feeds three extra power, but draws more attention.",
                },
            ],
            Self::Hydroponics => &[
                FacilityUpgradeOption {
                    id: COMMUNITY_KITCHEN_UPGRADE,
                    name: "Community Kitchen",
                    description: "The harvest feeds more mouths and makes Commons meals cheaper.",
                },
                FacilityUpgradeOption {
                    id: CULTURE_BEDS_UPGRADE,
                    name: "Culture Beds",
                    description: "The beds cultivate two extra biomass after each operation.",
                },
            ],
            Self::Workshop => &[
                FacilityUpgradeOption {
                    id: PRECISION_BENCH_UPGRADE,
                    name: "Precision Bench",
                    description: "Weapon and armour fabrication costs five fewer materials.",
                },
                FacilityUpgradeOption {
                    id: DRONE_BAY_UPGRADE,
                    name: "Drone Bay",
                    description: "Automated repair crews reduce facility repair costs by ten.",
                },
            ],
            Self::GeneLab => &[
                FacilityUpgradeOption {
                    id: STABILISATION_WING_UPGRADE,
                    name: "Stabilisation Wing",
                    description:
                        "Suppresses chosen evolution complications while the lab is online.",
                },
                FacilityUpgradeOption {
                    id: EVOLUTION_CHAMBER_UPGRADE,
                    name: "Evolution Chamber",
                    description: "Reduces each mutation evolution's biomass cost by four.",
                },
            ],
            Self::CommandCentre => &[
                FacilityUpgradeOption {
                    id: SIGNAL_CARTOGRAPHY_UPGRADE,
                    name: "Signal Cartography",
                    description: "Reveals a third mission route and clearer wave intelligence.",
                },
                FacilityUpgradeOption {
                    id: COUNTERINTELLIGENCE_CELL_UPGRADE,
                    name: "Counterintelligence Cell",
                    description: "Cuts mission pressure and keeps hostile attention in check.",
                },
            ],
            _ => &[],
        }
    }

    pub fn upgrade_cost(self) -> i32 {
        if self.upgrade_options().is_empty() {
            0
        } else {
            55
        }
    }
}
