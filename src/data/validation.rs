//! Registry validation and cross-reference checks.

use super::*;

impl GameData {
    pub fn validate_registry(&self) -> Result<(), String> {
        self.validate_identity_references()?;
        self.validate_campaign_options()?;
        self.validate_content_catalog()?;
        self.validate_character_references()?;
        self.validate_mission_references()?;
        self.validate_contact_references()?;
        self.validate_map_references()?;
        crate::relationships::validate_event_definitions(self)?;
        Ok(())
    }

    fn validate_identity_references(&self) -> Result<(), String> {
        ensure_unique(
            "character",
            self.characters.iter().map(|entry| entry.id.as_str()),
        )?;
        ensure_unique(
            "unit",
            self.roster
                .iter()
                .chain(self.recruitable_roster.iter())
                .map(|entry| entry.id.as_str()),
        )?;
        for template in &self.campaign.mission_templates {
            if !template.required_response.is_empty()
                && !self
                    .campaign
                    .escalation_responses
                    .iter()
                    .any(|response| response.id == template.required_response)
            {
                return Err(format!(
                    "Mission template {} references missing Escalation response {}",
                    template.id, template.required_response
                ));
            }
            for unit_id in &template.hostile_unit_ids {
                if !self
                    .roster
                    .iter()
                    .any(|unit| unit.id == *unit_id && unit.team == Team::Hostile)
                {
                    return Err(format!(
                        "Mission template {} references missing hostile unit {}",
                        template.id, unit_id
                    ));
                }
            }
            if !template.required_mirexis_path.is_empty()
                && !self
                    .campaign
                    .mirexis_paths
                    .iter()
                    .any(|path| path.id == template.required_mirexis_path)
            {
                return Err(format!(
                    "Mission template {} references missing Mirexis path {}",
                    template.id, template.required_mirexis_path
                ));
            }
            if template.post_campaign && template.required_mirexis_path.is_empty() {
                return Err(format!(
                    "Post-campaign mission template {} must reference a Mirexis path",
                    template.id
                ));
            }
        }
        for equipment in &self.equipment {
            if !equipment.required_protocol.is_empty()
                && !self
                    .campaign
                    .contact_protocols
                    .iter()
                    .any(|protocol| protocol.id == equipment.required_protocol)
            {
                return Err(format!(
                    "Equipment {} references missing Contact protocol {}",
                    equipment.id, equipment.required_protocol
                ));
            }
        }
        Ok(())
    }

    fn validate_campaign_options(&self) -> Result<(), String> {
        ensure_unique(
            "faction",
            self.campaign.factions.iter().map(|entry| entry.id.as_str()),
        )?;
        ensure_unique(
            "research",
            self.campaign.research.iter().map(|entry| entry.id.as_str()),
        )?;
        ensure_unique(
            "contact protocol",
            self.campaign
                .contact_protocols
                .iter()
                .map(|entry| entry.id.as_str()),
        )?;
        ensure_unique(
            "escalation response",
            self.campaign
                .escalation_responses
                .iter()
                .map(|entry| entry.id.as_str()),
        )?;
        if self.campaign.escalation_responses.iter().any(|response| {
            response.materials_cost < 0 || response.biomass_cost < 0 || response.power_cost < 0
        }) {
            return Err("Escalation responses cannot refund their selection cost".to_owned());
        }
        ensure_unique(
            "Mirexis path",
            self.campaign
                .mirexis_paths
                .iter()
                .map(|entry| entry.id.as_str()),
        )?;
        if self
            .campaign
            .mirexis_paths
            .iter()
            .any(|path| path.materials_cost < 0 || path.biomass_cost < 0 || path.power_cost < 0)
        {
            return Err("Mirexis paths cannot refund their selection cost".to_owned());
        }
        ensure_unique(
            "campaign event",
            self.campaign.events.iter().map(|entry| entry.id.as_str()),
        )?;
        ensure_unique(
            "mission template",
            self.campaign
                .mission_templates
                .iter()
                .map(|entry| entry.id.as_str()),
        )?;
        ensure_unique(
            "map recipe",
            self.campaign
                .map_recipes
                .iter()
                .map(|entry| entry.id.as_str()),
        )?;
        Ok(())
    }

    fn validate_content_catalog(&self) -> Result<(), String> {
        ensure_unique("class", self.classes.iter().map(|entry| entry.id.as_str()))?;
        crate::class_training::validate_definitions(&self.classes)?;
        crate::skill_training::validate_definitions(&self.classes)?;
        ensure_unique(
            "mutation",
            self.mutations.iter().map(|entry| entry.id.as_str()),
        )?;
        for mutation in &self.mutations {
            ensure_unique(
                "mutation evolution",
                mutation.evolutions.iter().map(|entry| entry.id.as_str()),
            )?;
            if mutation
                .evolutions
                .iter()
                .any(|evolution| evolution.biomass_cost <= 0)
            {
                return Err(format!("Mutation {} has a free evolution", mutation.id));
            }
        }
        ensure_unique(
            "equipment",
            self.equipment.iter().map(|entry| entry.id.as_str()),
        )?;
        crate::equipment_catalog::validate_definitions(&self.equipment)?;
        Ok(())
    }

    fn validate_character_references(&self) -> Result<(), String> {
        for character in &self.characters {
            if !self
                .classes
                .iter()
                .any(|entry| entry.id == character.initial_class)
            {
                return Err(format!(
                    "Character {} references missing class {}",
                    character.id, character.initial_class
                ));
            }
            if !self
                .mutations
                .iter()
                .any(|entry| entry.id == character.mutation)
            {
                return Err(format!(
                    "Character {} references missing mutation {}",
                    character.id, character.mutation
                ));
            }
            for equipment_id in &character.equipment {
                if !self.equipment.iter().any(|entry| &entry.id == equipment_id) {
                    return Err(format!(
                        "Character {} references missing equipment {}",
                        character.id, equipment_id
                    ));
                }
            }
            if character
                .aptitudes
                .values()
                .any(|rating| !(1..=5).contains(rating))
            {
                return Err(format!(
                    "Character {} has an aptitude outside 1..=5",
                    character.id
                ));
            }
        }
        for unit in self
            .roster
            .iter()
            .chain(self.recruitable_roster.iter())
            .filter(|unit| unit.team == Team::Colony)
        {
            if !self
                .characters
                .iter()
                .any(|character| character.id == unit.id)
            {
                return Err(format!(
                    "Deployed colonist {} has no persistent character record",
                    unit.id
                ));
            }
        }
        Ok(())
    }

    fn validate_mission_references(&self) -> Result<(), String> {
        for template in &self.campaign.mission_templates {
            if !self
                .campaign
                .factions
                .iter()
                .any(|faction| faction.id == template.faction)
            {
                return Err(format!(
                    "Mission template {} references missing faction {}",
                    template.id, template.faction
                ));
            }
            if !template.required_protocol.is_empty()
                && !self
                    .campaign
                    .contact_protocols
                    .iter()
                    .any(|protocol| protocol.id == template.required_protocol)
            {
                return Err(format!(
                    "Mission template {} references missing Contact protocol {}",
                    template.id, template.required_protocol
                ));
            }
            if !template.required_phase.is_empty()
                && ![
                    "isolation",
                    "contact",
                    "adaptation",
                    "escalation",
                    "mirexis",
                ]
                .contains(&template.required_phase.as_str())
            {
                return Err(format!(
                    "Mission template {} references unknown phase {}",
                    template.id, template.required_phase
                ));
            }
            if !self
                .campaign
                .map_recipes
                .iter()
                .any(|recipe| recipe.id == template.map_recipe)
            {
                return Err(format!(
                    "Mission template {} references missing map recipe {}",
                    template.id, template.map_recipe
                ));
            }
        }
        Ok(())
    }

    fn validate_contact_references(&self) -> Result<(), String> {
        for protocol in &self.campaign.contact_protocols {
            if !self
                .campaign
                .factions
                .iter()
                .any(|faction| faction.id == protocol.faction)
            {
                return Err(format!(
                    "Contact protocol {} references missing faction {}",
                    protocol.id, protocol.faction
                ));
            }
            if protocol.alien_components_cost <= 0
                || [
                    protocol.materials_bonus,
                    protocol.biomass_bonus,
                    protocol.power_bonus,
                ]
                .into_iter()
                .filter(|bonus| *bonus > 0)
                .count()
                    != 1
            {
                return Err(format!(
                    "Contact protocol {} must have a cost and one positive reward bonus",
                    protocol.id
                ));
            }
        }
        Ok(())
    }

    fn validate_map_references(&self) -> Result<(), String> {
        for faction in &self.campaign.factions {
            if !self.roster.iter().any(|unit| {
                unit.team == Team::Hostile && unit.faction.as_deref() == Some(&faction.id)
            }) {
                return Err(format!("Faction {} has no hostile units", faction.id));
            }
        }
        for recipe in &self.campaign.map_recipes {
            let positions = recipe
                .blocked_tiles
                .iter()
                .chain(std::iter::once(&recipe.objective_tile))
                .chain(recipe.terrain_costs.iter().map(|entry| &entry.position))
                .chain(recipe.hazards.iter().map(|entry| &entry.position))
                .chain(recipe.cover_edges.iter().map(|entry| &entry.position));
            if positions.into_iter().any(|position| {
                position[0] < 0
                    || position[1] < 0
                    || position[0] >= self.config.world_width as i32
                    || position[1] >= self.config.world_height as i32
            }) {
                return Err(format!(
                    "Map recipe {} contains an out-of-bounds tile",
                    recipe.id
                ));
            }
            if recipe.blocked_tiles.contains(&recipe.objective_tile) {
                return Err(format!("Map recipe {} blocks its objective", recipe.id));
            }
            if recipe
                .hazards
                .iter()
                .any(|hazard| recipe.blocked_tiles.contains(&hazard.position))
            {
                return Err(format!("Map recipe {} blocks a hazard", recipe.id));
            }
        }
        for template in &self.campaign.mission_templates {
            let recipe = self
                .campaign
                .map_recipes
                .iter()
                .find(|recipe| recipe.id == template.map_recipe)
                .expect("map recipe references were validated above");
            if self.roster.iter().any(|unit| {
                unit.team == Team::Hostile
                    && unit.faction.as_deref() == Some(&template.faction)
                    && recipe.blocked_tiles.contains(&unit.position)
            }) {
                return Err(format!(
                    "Mission template {} blocks a hostile spawn",
                    template.id
                ));
            }
        }
        Ok(())
    }
}

pub fn ensure_unique<'a>(label: &str, ids: impl Iterator<Item = &'a str>) -> Result<(), String> {
    let mut seen = std::collections::HashSet::new();
    for id in ids {
        if !seen.insert(id) {
            return Err(format!("Duplicate {} id: {}", label, id));
        }
    }
    Ok(())
}
