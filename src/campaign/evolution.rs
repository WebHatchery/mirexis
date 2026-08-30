//! Mutation evolution rules and Gene Lab progression effects.

use super::CampaignState;
use crate::colony::{BuildingKind, EVOLUTION_CHAMBER_UPGRADE};
use crate::data::{GameData, MutationEvolutionDef};

const EVOLUTION_CHAMBER_BIOMASS_DISCOUNT: i32 = 4;

impl CampaignState {
    pub fn mutation_evolution_cost(&self, evolution: &MutationEvolutionDef) -> i32 {
        let discount = if self
            .colony
            .has_active_upgrade(BuildingKind::GeneLab, EVOLUTION_CHAMBER_UPGRADE)
        {
            EVOLUTION_CHAMBER_BIOMASS_DISCOUNT
        } else {
            0
        };
        (evolution.biomass_cost - discount).max(1)
    }

    pub fn choose_mutation_evolution(
        &mut self,
        character_id: &str,
        evolution_id: &str,
        data: &GameData,
    ) -> Result<String, String> {
        if !self.strategy.contact_complete {
            return Err("Mutation evolution unlocks in Adaptation".to_owned());
        }
        if !self.colony.has_facility(BuildingKind::GeneLab) {
            return Err("A powered Gene Lab is required for mutation evolution".to_owned());
        }
        let character = self
            .roster
            .iter()
            .find(|character| character.id == character_id)
            .ok_or_else(|| format!("Unknown character: {}", character_id))?;
        if !character.mutation_evolution_id.is_empty() {
            return Err(format!("{}'s mutation has already evolved", character.name));
        }
        let mutation = data
            .mutations
            .iter()
            .find(|mutation| mutation.id == character.mutation_id)
            .ok_or_else(|| format!("Unknown mutation: {}", character.mutation_id))?;
        let evolution = mutation
            .evolutions
            .iter()
            .find(|evolution| evolution.id == evolution_id)
            .ok_or_else(|| format!("Unknown mutation evolution: {}", evolution_id))?;
        let biomass_cost = self.mutation_evolution_cost(evolution);
        if self.colony.resources.biomass < biomass_cost {
            return Err(format!(
                "{} requires {} biomass",
                evolution.name, biomass_cost
            ));
        }
        self.colony.resources.biomass -= biomass_cost;
        self.roster
            .iter_mut()
            .find(|character| character.id == character_id)
            .expect("evolution character was validated")
            .mutation_evolution_id = evolution.id.clone();
        if !self.refresh_adaptation_completion(data) {
            self.refresh_mission_offers(data);
        }
        Ok(evolution.name.clone())
    }

    pub fn adaptation_completion_progress(&self) -> (bool, usize, bool) {
        (
            self.strategy.adaptation_operation_completed,
            self.roster
                .iter()
                .filter(|character| !character.mutation_evolution_id.is_empty())
                .count(),
            self.colony.has_facility(BuildingKind::GeneLab),
        )
    }

    pub fn refresh_adaptation_completion(&mut self, data: &GameData) -> bool {
        let (_, evolved, lab) = self.adaptation_completion_progress();
        let changed = self
            .strategy
            .refresh_adaptation_completion(evolved >= 2, lab);
        if changed {
            self.refresh_mission_offers(data);
        }
        changed
    }
}
