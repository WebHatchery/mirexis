//! Infirmary treatment, injury recovery, and mutation-aware medical consequences.

use super::{derived_mutation_traits_with_options, CampaignState, InjuryRecord};
use crate::colony::{BuildingKind, ADAPTATION_CLINIC_UPGRADE, TRAUMA_WARD_UPGRADE};
use crate::data::GameData;
use crate::state::MissionOutcome;

impl CampaignState {
    pub(super) fn apply_injury_consequences(&mut self, outcome: &MissionOutcome, data: &GameData) {
        let stabilisation_wing_active = self.colony.has_active_upgrade(
            BuildingKind::GeneLab,
            crate::colony::STABILISATION_WING_UPGRADE,
        );
        let adaptation_clinic_active = self
            .colony
            .has_active_upgrade(BuildingKind::Infirmary, ADAPTATION_CLINIC_UPGRADE);
        let trauma_ward_active = self
            .colony
            .has_active_upgrade(BuildingKind::Infirmary, TRAUMA_WARD_UPGRADE);
        let recovery_reduction = u8::from(self.strategy.research_completed("xeno_triage"))
            + u8::from(trauma_ward_active);
        for consequence in &outcome.colonists_incapacitated {
            if let Some(character) = self
                .roster
                .iter_mut()
                .find(|record| record.id == consequence.id)
            {
                let traits = derived_mutation_traits_with_options(
                    character,
                    data,
                    stabilisation_wing_active,
                );
                let delayed_healing = traits.get("medical_healing").copied().unwrap_or(0) < 0
                    && !adaptation_clinic_active;
                let base_recovery: u8 = if delayed_healing { 3 } else { 2 };
                let recovery_operations = base_recovery.saturating_sub(recovery_reduction).max(1);
                character.injuries.push(InjuryRecord {
                    id: format!("operation_{}_trauma", self.operations_completed),
                    name: "Mire exposure trauma".to_owned(),
                    recovery_operations,
                });
                crate::trauma::record_incapacitation(character, self.operations_completed);
                character.availability = super::Availability::Recovering;
            }
        }
    }

    pub fn advance_recovery(&mut self) {
        for character in &mut self.roster {
            for injury in &mut character.injuries {
                injury.recovery_operations = injury.recovery_operations.saturating_sub(1);
            }
            character
                .injuries
                .retain(|injury| injury.recovery_operations > 0);
            character.availability = if character.injuries.is_empty() {
                super::Availability::Ready
            } else {
                super::Availability::Recovering
            };
        }
    }

    pub fn treat_first_injury(&mut self) -> Result<String, String> {
        if !self.colony.has_facility(BuildingKind::Infirmary) {
            return Err("An operational infirmary is required".to_owned());
        }
        let cost = if self
            .colony
            .has_active_upgrade(BuildingKind::Infirmary, ADAPTATION_CLINIC_UPGRADE)
        {
            3
        } else {
            5
        };
        if self.colony.resources.biomass < cost {
            return Err(format!("Treatment requires {} biomass", cost));
        }
        let character = self
            .roster
            .iter_mut()
            .find(|character| !character.injuries.is_empty())
            .ok_or_else(|| "No colonist currently needs treatment".to_owned())?;
        self.colony.resources.biomass -= cost;
        for injury in &mut character.injuries {
            injury.recovery_operations = injury.recovery_operations.saturating_sub(1);
        }
        character
            .injuries
            .retain(|injury| injury.recovery_operations > 0);
        if character.injuries.is_empty() {
            character.availability = super::Availability::Ready;
        }
        Ok(character.name.clone())
    }
}
