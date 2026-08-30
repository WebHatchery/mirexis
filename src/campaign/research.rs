//! Research actions that depend on the physical colony's evidence facilities.

use super::CampaignState;
use crate::colony::BuildingKind;

const RESEARCH_ANNEX_DISCOUNT: i32 = 5;

impl CampaignState {
    pub fn research_material_cost(&self, base_cost: i32) -> i32 {
        let annex_discount = if self.colony.has_facility(BuildingKind::ResearchAnnex) {
            RESEARCH_ANNEX_DISCOUNT
        } else {
            0
        };
        (base_cost - annex_discount - self.research_insight.max(0)).max(5)
    }

    pub fn complete_research(&mut self, research_id: &str) -> Result<String, String> {
        let annex_discount = if self.colony.has_facility(BuildingKind::ResearchAnnex) {
            RESEARCH_ANNEX_DISCOUNT
        } else {
            0
        };
        let insight = self.research_insight.max(0);
        let result = self.strategy.complete_research_with_discount(
            research_id,
            &mut self.colony,
            annex_discount + insight,
        );
        if result.is_ok() {
            self.research_insight = 0;
        }
        result
    }
}
