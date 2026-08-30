//! The Salvage Yard turns recovered objects into one disclosed strategic choice.

use super::CampaignState;
use crate::colony::BuildingKind;

pub(crate) const SALVAGE_MATERIALS_REWARD: i32 = 24;
pub(crate) const SALVAGE_RESEARCH_INSIGHT: i32 = 12;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SalvageChoice {
    Materials,
    ResearchInsight,
    Prototype,
}

impl CampaignState {
    pub(crate) fn salvage_available(&self) -> bool {
        self.colony.has_facility(BuildingKind::SalvageYard)
            && self.salvage_cache_count > 0
            && self.salvage_yard_operation != Some(self.operations_completed)
    }

    pub(crate) fn process_salvage(&mut self, choice: SalvageChoice) -> Result<String, String> {
        if !self.colony.has_facility(BuildingKind::SalvageYard) {
            return Err("An operational Salvage Yard is required".to_owned());
        }
        if self.salvage_cache_count == 0 {
            return Err("No recovered object is waiting at the Salvage Yard".to_owned());
        }
        if self.salvage_yard_operation == Some(self.operations_completed) {
            return Err("The Salvage Yard has already sorted this operation's recovery".to_owned());
        }
        match choice {
            SalvageChoice::Materials => {
                self.colony.resources.materials += SALVAGE_MATERIALS_REWARD;
            }
            SalvageChoice::ResearchInsight => {
                if self.research_insight > 0 {
                    return Err("Research insight is already waiting for a doctrine".to_owned());
                }
                if !self
                    .strategy
                    .research
                    .iter()
                    .any(|research| !research.completed)
                {
                    return Err("Every doctrine is already complete".to_owned());
                }
                self.research_insight = SALVAGE_RESEARCH_INSIGHT;
            }
            SalvageChoice::Prototype => {
                self.salvage_prototypes = self.salvage_prototypes.saturating_add(1);
            }
        }
        self.salvage_cache_count = self.salvage_cache_count.saturating_sub(1);
        self.salvage_yard_operation = Some(self.operations_completed);
        Ok(match choice {
            SalvageChoice::Materials => {
                format!("SALVAGE SORTED // +{} MATERIALS", SALVAGE_MATERIALS_REWARD)
            }
            SalvageChoice::ResearchInsight => format!(
                "SALVAGE STUDIED // NEXT DOCTRINE -{} MATERIALS",
                SALVAGE_RESEARCH_INSIGHT
            ),
            SalvageChoice::Prototype => {
                "SALVAGE FABRICATED // ONE FREE STANDARD EQUIPMENT PROTOTYPE READY".to_owned()
            }
        })
    }
}
