//! Derive the physical colony layout used by settlement defence missions.

use super::{
    BuildingKind, ColonyDefenseMap, ColonyState, DOCTRINE_YARD_UPGRADE, TRAUMA_WARD_UPGRADE,
};
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
                    | BuildingKind::ResearchAnnex
                    | BuildingKind::Hydroponics
                    | BuildingKind::PowerPlant
                    | BuildingKind::GeneLab
                    | BuildingKind::Waystation
                    | BuildingKind::Commons
                    | BuildingKind::RelayMast => critical_objectives.push(position),
                }
            }
        }
        if let Some(barracks) = self.buildings.iter().find(|building| {
            building.kind == BuildingKind::Barracks
                && !building.damaged
                && self.has_active_upgrade(BuildingKind::Barracks, DOCTRINE_YARD_UPGRADE)
        }) {
            let rally_tile = TilePos::new(barracks.position[0] + 2, barracks.position[1] + 1);
            for position in [
                TilePos::new(rally_tile.x - 1, rally_tile.y + 1),
                TilePos::new(rally_tile.x + 1, rally_tile.y + 1),
            ] {
                if !blocked_tiles.contains(&position) {
                    blocked_tiles.push(position);
                    cover_tiles.push(position);
                }
            }
        }
        if let Some(infirmary) = self.buildings.iter().find(|building| {
            building.kind == BuildingKind::Infirmary
                && !building.damaged
                && self.has_active_upgrade(BuildingKind::Infirmary, TRAUMA_WARD_UPGRADE)
        }) {
            let stabilization_tile =
                TilePos::new(infirmary.position[0] + 3, infirmary.position[1] + 2);
            if !blocked_tiles.contains(&stabilization_tile) {
                blocked_tiles.push(stabilization_tile);
                cover_tiles.push(stabilization_tile);
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
