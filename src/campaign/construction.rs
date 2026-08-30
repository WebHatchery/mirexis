//! Campaign-level construction unlocks and placement affordances.

use super::CampaignState;
use crate::colony::BuildingKind;

impl CampaignState {
    pub(crate) fn construction_available(&self, kind: BuildingKind) -> bool {
        self.construction_unlocked(kind) && self.colony.can_start_construction(kind)
    }

    pub(crate) fn can_construct_building(&self, kind: BuildingKind, position: [i32; 2]) -> bool {
        self.construction_available(kind)
            && self.colony.validate_construction_site(position).is_ok()
    }

    fn construction_unlocked(&self, kind: BuildingKind) -> bool {
        match kind {
            BuildingKind::Barricade
            | BuildingKind::PowerPlant
            | BuildingKind::Commons
            | BuildingKind::RelayMast
            | BuildingKind::Watchtower
            | BuildingKind::ResearchAnnex
            | BuildingKind::SalvageYard => true,
            BuildingKind::GeneLab => self.strategy.contact_complete,
            BuildingKind::Waystation => self.waystation_unlocked(),
            _ => false,
        }
    }
}
