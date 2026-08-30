//! The chosen identity building's once-per-operation civic stewardship action.

use super::CampaignState;
use crate::colony::BuildingKind;

pub(crate) const IDENTITY_STEWARDSHIP_ATTENTION: i32 = 5;
pub(crate) const IDENTITY_PREPARATION_ATTENTION: i32 = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum StewardshipResource {
    Materials,
    Biomass,
    Power,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct StewardshipDefinition {
    pub(crate) building: BuildingKind,
    pub(crate) action: &'static str,
    pub(crate) resource: StewardshipResource,
    pub(crate) resource_name: &'static str,
    pub(crate) cost: i32,
    pub(crate) faction_id: &'static str,
    pub(crate) faction_name: &'static str,
    pub(crate) summary: &'static str,
}

impl StewardshipDefinition {
    pub(crate) fn for_path(path_id: &str) -> Option<Self> {
        match path_id {
            "human_redoubt" => Some(Self {
                building: BuildingKind::RedoubtArsenal,
                action: "FORTIFY WATCHLINE",
                resource: StewardshipResource::Materials,
                resource_name: "MAT",
                cost: 4,
                faction_id: "directorate",
                faction_name: "DIRECTORATE",
                summary: "WATCHLINE FORTIFIED",
            }),
            "living_commonwealth" => Some(Self {
                building: BuildingKind::ChoirGarden,
                action: "ANSWER THE CHORUS",
                resource: StewardshipResource::Biomass,
                resource_name: "BIO",
                cost: 2,
                faction_id: "brood",
                faction_name: "BROOD",
                summary: "CHORUS ANSWERED",
            }),
            "open_threshold" => Some(Self {
                building: BuildingKind::ThresholdSpire,
                action: "GUIDE A RETURN",
                resource: StewardshipResource::Power,
                resource_name: "PWR",
                cost: 2,
                faction_id: "ascendants",
                faction_name: "ASCENDANTS",
                summary: "RETURN ROUTE GUIDED",
            }),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PreparationDefinition {
    action: &'static str,
    summary: &'static str,
}

impl PreparationDefinition {
    fn for_path(path_id: &str) -> Option<Self> {
        match path_id {
            "human_redoubt" => Some(Self {
                action: "DRILL THE WATCH",
                summary: "WATCH DRILL COMPLETE",
            }),
            "living_commonwealth" => Some(Self {
                action: "TUNE THE CHORUS",
                summary: "CHORUS TUNED",
            }),
            "open_threshold" => Some(Self {
                action: "CALIBRATE THE GATE",
                summary: "GATE CALIBRATED",
            }),
            _ => None,
        }
    }
}

impl CampaignState {
    pub(crate) fn identity_preparation_available(&self) -> bool {
        let Some(definition) = self.identity_preparation_definition() else {
            return false;
        };
        self.identity_preparation_operation != Some(self.operations_completed)
            && self.colony.has_facility(definition.building)
            && self.identity_resource(definition.resource) >= definition.cost
    }

    pub(crate) fn identity_preparation_copy(&self) -> Option<String> {
        let definition = self.identity_preparation_definition()?;
        let preparation = PreparationDefinition::for_path(&self.strategy.mirexis_path_id)?;
        if self.identity_preparation_operation == Some(self.operations_completed) {
            return Some(format!(
                "{} // PREPARED THIS OPERATION // {} -{} ATT",
                definition.building.name().to_uppercase(),
                definition.faction_name,
                IDENTITY_PREPARATION_ATTENTION
            ));
        }
        if !self.colony.has_facility(definition.building) {
            return Some(format!(
                "{} // OFFLINE // PREPARATION NEEDS POWER + REPAIR",
                definition.building.name().to_uppercase()
            ));
        }
        if self.identity_resource(definition.resource) < definition.cost {
            return Some(format!(
                "{} // NEEDS {} {} TO PREPARE",
                definition.building.name().to_uppercase(),
                definition.cost,
                definition.resource_name
            ));
        }
        Some(format!(
            "{} // TAP TO {} // {} {} // {} -{} ATT",
            definition.building.name().to_uppercase(),
            preparation.action,
            definition.cost,
            definition.resource_name,
            definition.faction_name,
            IDENTITY_PREPARATION_ATTENTION
        ))
    }

    pub(crate) fn identity_preparation_definition(&self) -> Option<StewardshipDefinition> {
        (!self.strategy.campaign_complete)
            .then(|| StewardshipDefinition::for_path(&self.strategy.mirexis_path_id))
            .flatten()
    }

    pub(crate) fn prepare_identity_building(&mut self) -> Result<String, String> {
        let definition = self
            .identity_preparation_definition()
            .ok_or_else(|| "Identity preparation unlocks before the campaign finale".to_owned())?;
        let preparation = PreparationDefinition::for_path(&self.strategy.mirexis_path_id)
            .ok_or_else(|| "Unknown identity preparation path".to_owned())?;
        if !self.colony.has_facility(definition.building) {
            return Err(format!(
                "An operational {} is required",
                definition.building.name()
            ));
        }
        if self.identity_preparation_operation == Some(self.operations_completed) {
            return Err(
                "The identity building has already been prepared this operation".to_owned(),
            );
        }
        if self.identity_resource(definition.resource) < definition.cost {
            return Err(format!(
                "{} preparation requires {} {}",
                definition.building.name(),
                definition.cost,
                definition.resource_name
            ));
        }
        self.spend_identity_resource(definition.resource, definition.cost);
        if let Some(faction) = self
            .strategy
            .factions
            .iter_mut()
            .find(|faction| faction.id == definition.faction_id)
        {
            faction.attention = (faction.attention - IDENTITY_PREPARATION_ATTENTION).max(0);
        }
        self.identity_preparations_completed =
            self.identity_preparations_completed.saturating_add(1);
        self.identity_preparation_operation = Some(self.operations_completed);
        Ok(format!(
            "{} // {} // {} ATTENTION -{}",
            definition.building.name().to_uppercase(),
            preparation.summary,
            definition.faction_name,
            IDENTITY_PREPARATION_ATTENTION
        ))
    }

    pub(crate) fn identity_stewardship_definition(&self) -> Option<StewardshipDefinition> {
        self.strategy
            .campaign_complete
            .then(|| StewardshipDefinition::for_path(&self.strategy.mirexis_path_id))
            .flatten()
    }

    pub(crate) fn identity_stewardship_available(&self) -> bool {
        let Some(definition) = self.identity_stewardship_definition() else {
            return false;
        };
        self.identity_stewardship_operation != Some(self.operations_completed)
            && self.colony.has_facility(definition.building)
            && self.identity_resource(definition.resource) >= definition.cost
    }

    pub(crate) fn identity_stewardship_copy(&self) -> Option<String> {
        let definition = self.identity_stewardship_definition()?;
        if self.identity_stewardship_operation == Some(self.operations_completed) {
            return Some(format!(
                "{} // STEWARDED THIS OPERATION // {} -{} ATT",
                definition.building.name().to_uppercase(),
                definition.faction_name,
                IDENTITY_STEWARDSHIP_ATTENTION
            ));
        }
        if !self.colony.has_facility(definition.building) {
            return Some(format!(
                "{} // OFFLINE // STEWARDSHIP NEEDS POWER + REPAIR",
                definition.building.name().to_uppercase()
            ));
        }
        if self.identity_resource(definition.resource) < definition.cost {
            return Some(format!(
                "{} // NEEDS {} {} TO STEWARD",
                definition.building.name().to_uppercase(),
                definition.cost,
                definition.resource_name
            ));
        }
        Some(format!(
            "{} // TAP TO {} // {} {} // {} -{} ATT",
            definition.building.name().to_uppercase(),
            definition.action,
            definition.cost,
            definition.resource_name,
            definition.faction_name,
            IDENTITY_STEWARDSHIP_ATTENTION
        ))
    }

    pub(crate) fn run_identity_stewardship(&mut self) -> Result<String, String> {
        let definition = self
            .identity_stewardship_definition()
            .ok_or_else(|| "Identity stewardship unlocks after the campaign".to_owned())?;
        if !self.colony.has_facility(definition.building) {
            return Err(format!(
                "An operational {} is required",
                definition.building.name()
            ));
        }
        if self.identity_stewardship_operation == Some(self.operations_completed) {
            return Err(
                "The identity building has already been stewarded this operation".to_owned(),
            );
        }
        if self.identity_resource(definition.resource) < definition.cost {
            return Err(format!(
                "{} stewardship requires {} {}",
                definition.building.name(),
                definition.cost,
                definition.resource_name
            ));
        }
        self.spend_identity_resource(definition.resource, definition.cost);
        if let Some(faction) = self
            .strategy
            .factions
            .iter_mut()
            .find(|faction| faction.id == definition.faction_id)
        {
            faction.attention = (faction.attention - IDENTITY_STEWARDSHIP_ATTENTION).max(0);
        }
        self.identity_stewardship_completed = self.identity_stewardship_completed.saturating_add(1);
        self.identity_stewardship_operation = Some(self.operations_completed);
        Ok(format!(
            "{} // {} // {} ATTENTION -{}",
            definition.building.name().to_uppercase(),
            definition.summary,
            definition.faction_name,
            IDENTITY_STEWARDSHIP_ATTENTION
        ))
    }

    fn identity_resource(&self, resource: StewardshipResource) -> i32 {
        match resource {
            StewardshipResource::Materials => self.colony.resources.materials,
            StewardshipResource::Biomass => self.colony.resources.biomass,
            StewardshipResource::Power => self.colony.resources.power,
        }
    }

    fn spend_identity_resource(&mut self, resource: StewardshipResource, amount: i32) {
        match resource {
            StewardshipResource::Materials => self.colony.resources.materials -= amount,
            StewardshipResource::Biomass => self.colony.resources.biomass -= amount,
            StewardshipResource::Power => self.colony.resources.power -= amount,
        }
    }
}
