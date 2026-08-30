//! Derive the physical colony layout used by settlement defence missions.

use super::{BuildingKind, ColonyDefenseMap, ColonyState};
use macroquad_toolkit::grid::TilePos;

impl ColonyState {
    pub fn defense_map(&self) -> ColonyDefenseMap {
        let mut blocked_tiles = Vec::new();
        let mut cover_tiles = Vec::new();
        let mut watchtower_tiles = Vec::new();
        let mut shield_tiles = Vec::new();
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
                    BuildingKind::RedoubtArsenal => {
                        cover_tiles.push(position);
                        critical_objectives.insert(0, position);
                    }
                    BuildingKind::ChoirGarden => {
                        critical_objectives.insert(0, position);
                        if !building.damaged && self.building_is_powered(&building.id) {
                            cover_tiles.push(position);
                        }
                    }
                    BuildingKind::ThresholdSpire => {
                        critical_objectives.insert(0, position);
                        if !building.damaged && self.building_is_powered(&building.id) {
                            shield_tiles.push(position);
                        }
                    }
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
            shield_tiles,
            critical_objectives,
        }
    }
}
