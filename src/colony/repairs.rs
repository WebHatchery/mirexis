//! Effective facility repair costs and material affordances.

use super::{BuildingKind, ColonyState};

impl ColonyState {
    pub(crate) fn repair_cost_for(&self, building_id: &str) -> Option<i32> {
        let building = self
            .buildings
            .iter()
            .find(|building| building.id == building_id)?;
        if !building.damaged {
            return None;
        }
        let discount = if self.has_active_upgrade(BuildingKind::Workshop, super::DRONE_BAY_UPGRADE)
        {
            10
        } else {
            0
        };
        Some((building.kind.repair_cost() - discount).max(5))
    }

    pub(crate) fn can_repair_building(&self, building_id: &str) -> bool {
        self.repair_cost_for(building_id)
            .is_some_and(|cost| self.resources.materials >= cost)
    }
}
