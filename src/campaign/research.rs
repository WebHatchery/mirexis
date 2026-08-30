//! Research actions that depend on the physical colony's evidence facilities.

use super::CampaignState;
use crate::colony::BuildingKind;

const RESEARCH_ANNEX_DISCOUNT: i32 = 5;

impl CampaignState {
    pub fn research_material_cost(&self, base_cost: i32) -> i32 {
        if self.colony.has_facility(BuildingKind::ResearchAnnex) {
            (base_cost - RESEARCH_ANNEX_DISCOUNT).max(5)
        } else {
            base_cost
        }
    }

    pub fn complete_research(&mut self, research_id: &str) -> Result<String, String> {
        let discount = if self.colony.has_facility(BuildingKind::ResearchAnnex) {
            RESEARCH_ANNEX_DISCOUNT
        } else {
            0
        };
        self.strategy
            .complete_research_with_discount(research_id, &mut self.colony, discount)
    }
}
