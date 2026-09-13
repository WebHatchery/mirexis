//! Route-specific recruitment rules and Waystation outsider arcs.
use super::{CampaignState, CharacterLegacy, CharacterRecord, OutsiderArcState};
use crate::colony::BuildingKind;
use crate::data::{CharacterDef, GameData, OutsiderBeatDef, OutsiderChoiceDef};

pub const OUTSIDER_IDS: [&str; 3] = ["veya_orn", "sedge", "ninth_voice_apart"];

#[derive(Debug, Clone)]
pub struct OutsiderChoice {
    pub id: String,
    pub label: String,
    pub description: String,
    pub materials_cost: i32,
    pub food_cost: i32,
    pub power_cost: i32,
    pub biomass_cost: i32,
    pub attention_change: i32,
    pub relationship_partner: String,
    pub disagreement: bool,
    pub legacy_name: String,
    pub legacy_stat: String,
    pub legacy_amount: i32,
}

impl From<&OutsiderChoiceDef> for OutsiderChoice {
    fn from(definition: &OutsiderChoiceDef) -> Self {
        Self {
            id: definition.id.clone(),
            label: definition.label.clone(),
            description: definition.description.clone(),
            materials_cost: definition.materials_cost,
            food_cost: definition.food_cost,
            power_cost: definition.power_cost,
            biomass_cost: definition.biomass_cost,
            attention_change: definition.attention_change,
            relationship_partner: definition.relationship_partner.clone(),
            disagreement: definition.disagreement,
            legacy_name: definition.legacy_name.clone(),
            legacy_stat: definition.legacy_stat.clone(),
            legacy_amount: definition.legacy_amount,
        }
    }
}

pub fn recruitment_resource(definition: &CharacterDef) -> &str {
    if definition.recruitment_resource.is_empty() {
        "materials"
    } else {
        &definition.recruitment_resource
    }
}

#[derive(Debug, Clone)]
pub struct OutsiderBeat {
    pub outsider_id: String,
    pub outsider_name: String,
    pub attention_faction: String,
    pub stage: u8,
    pub title: String,
    pub description: String,
    pub choices: Vec<OutsiderChoice>,
}

impl From<&OutsiderBeatDef> for OutsiderBeat {
    fn from(definition: &OutsiderBeatDef) -> Self {
        Self {
            outsider_id: definition.outsider_id.clone(),
            outsider_name: definition.outsider_name.clone(),
            attention_faction: definition.attention_faction.clone(),
            stage: definition.stage,
            title: definition.title.clone(),
            description: definition.description.clone(),
            choices: definition
                .choices
                .iter()
                .map(OutsiderChoice::from)
                .collect(),
        }
    }
}

pub fn outsider_beat(data: &GameData, outsider_id: &str, stage: u8) -> Option<OutsiderBeat> {
    data.campaign
        .outsider_beats
        .iter()
        .find(|beat| beat.outsider_id == outsider_id && beat.stage == stage)
        .map(OutsiderBeat::from)
}

impl CampaignState {
    pub fn waystation_unlocked(&self) -> bool {
        self.strategy.contact_protocol_id == "directorate_requisition"
            || self.strategy.contact_protocol_id == "brood_cultivation"
            || (self.strategy.phase_id == "adaptation" && self.strategy.contact_complete)
    }

    pub fn available_outsider<'a>(&self, data: &'a GameData) -> Option<&'a CharacterDef> {
        if !self.colony.has_facility(BuildingKind::Waystation) {
            return None;
        }
        data.characters.iter().find(|definition| {
            !definition.recruitment_protocol.is_empty()
                && self.recruitment_route_matches(definition)
                && !self
                    .roster
                    .iter()
                    .any(|character| character.id == definition.id)
        })
    }

    pub fn outsider_recruit_available(&self, data: &GameData) -> bool {
        self.available_outsider(data).is_some()
    }

    pub fn can_recruit_outsider(&self, data: &GameData) -> bool {
        self.available_outsider(data).is_some_and(|definition| {
            self.recruitment_resource_amount(recruitment_resource(definition))
                >= definition.recruitment_cost
        })
    }

    pub fn recruit_outsider(&mut self, data: &GameData) -> Result<String, String> {
        let definition = self
            .available_outsider(data)
            .cloned()
            .ok_or_else(|| "No route-exclusive outsider is authored".to_owned())?;
        let resource = recruitment_resource(&definition);
        if self.recruitment_resource_amount(resource) < definition.recruitment_cost {
            return Err(format!(
                "Recruitment requires {} {}",
                definition.recruitment_cost, resource
            ));
        }
        self.spend_recruitment_resource(resource, definition.recruitment_cost);
        if definition.id == "sedge" || definition.id == "ninth_voice_apart" {
            if let Some(brood) = self
                .strategy
                .factions
                .iter_mut()
                .find(|faction| faction.id == "brood")
            {
                brood.attention = (brood.attention + 1).min(100);
            }
        }
        let mut character = CharacterRecord::from_def(&definition);
        character.deployment_selected = false;
        let name = character.name.clone();
        self.roster.push(character);
        Ok(name)
    }

    pub fn recruitment_resource_amount(&self, resource: &str) -> i32 {
        match resource {
            "materials" => self.colony.resources.materials,
            "power" => self.colony.resources.power,
            "food" => self.colony.resources.food,
            "biomass" => self.colony.resources.biomass,
            _ => 0,
        }
    }

    pub fn spend_recruitment_resource(&mut self, resource: &str, amount: i32) {
        match resource {
            "materials" => self.colony.resources.materials -= amount,
            "power" => self.colony.resources.power -= amount,
            "food" => self.colony.resources.food -= amount,
            "biomass" => self.colony.resources.biomass -= amount,
            _ => {}
        }
    }

    pub fn recruitment_route_matches(&self, definition: &CharacterDef) -> bool {
        if definition.recruitment_phase.is_empty() {
            self.strategy.contact_protocol_id == definition.recruitment_protocol
        } else {
            self.strategy.phase_id == definition.recruitment_phase && self.strategy.contact_complete
        }
    }

    pub fn outsider_arc_state(&self, outsider_id: &str) -> OutsiderArcState {
        self.outsider_arc_states
            .get(outsider_id)
            .cloned()
            .unwrap_or_else(|| {
                if outsider_id == "veya_orn" {
                    OutsiderArcState {
                        stage: self.outsider_arc_stage,
                        disagreements: self.outsider_disagreements,
                        final_choice: self.outsider_final_choice.clone(),
                    }
                } else {
                    OutsiderArcState::default()
                }
            })
    }

    pub fn set_outsider_arc_state(&mut self, outsider_id: &str, state: OutsiderArcState) {
        if outsider_id == "veya_orn" {
            self.outsider_arc_stage = state.stage;
            self.outsider_disagreements = state.disagreements;
            self.outsider_final_choice = state.final_choice.clone();
        }
        self.outsider_arc_states
            .insert(outsider_id.to_owned(), state);
    }

    pub fn outsider_arc_beat(&self, data: &GameData) -> Option<OutsiderBeat> {
        for outsider_id in OUTSIDER_IDS {
            if !self
                .roster
                .iter()
                .any(|character| character.id == outsider_id)
            {
                continue;
            }
            let state = self.outsider_arc_state(outsider_id);
            let Some(beat) = outsider_beat(data, outsider_id, state.stage) else {
                continue;
            };
            if self.operations_completed >= u32::from(state.stage) {
                return Some(beat);
            }
        }
        None
    }

    pub fn outsider_arc_disagreements(&self, data: &GameData) -> u8 {
        self.outsider_arc_beat(data)
            .map(|beat| self.outsider_arc_state(&beat.outsider_id).disagreements)
            .unwrap_or_default()
    }

    pub fn outsider_arc_available(&self, data: &GameData) -> bool {
        self.outsider_arc_beat(data).is_some()
    }

    pub fn resolve_outsider_beat(
        &mut self,
        data: &GameData,
        stage: u8,
        choice_id: &str,
    ) -> Result<String, String> {
        let beat = self
            .outsider_arc_beat(data)
            .ok_or_else(|| "No recruited outsider has a current conversation".to_owned())?;
        if beat.stage != stage {
            return Err("That outsider conversation is no longer current".to_owned());
        }
        let choice = beat
            .choices
            .iter()
            .find(|choice| choice.id == choice_id)
            .cloned()
            .ok_or_else(|| format!("Unknown outsider choice: {choice_id}"))?;
        if self.colony.resources.materials < choice.materials_cost {
            return Err(format!(
                "This choice requires {} materials",
                choice.materials_cost
            ));
        }
        if self.colony.resources.food < choice.food_cost {
            return Err(format!("This choice requires {} food", choice.food_cost));
        }
        if self.colony.resources.power < choice.power_cost {
            return Err(format!("This choice requires {} power", choice.power_cost));
        }
        if self.colony.resources.biomass < choice.biomass_cost {
            return Err(format!(
                "This choice requires {} biomass",
                choice.biomass_cost
            ));
        }
        self.colony.resources.materials -= choice.materials_cost;
        self.colony.resources.food -= choice.food_cost;
        self.colony.resources.power -= choice.power_cost;
        self.colony.resources.biomass -= choice.biomass_cost;
        if let Some(faction) = self
            .strategy
            .factions
            .iter_mut()
            .find(|faction| faction.id == beat.attention_faction)
        {
            faction.attention = (faction.attention + choice.attention_change).clamp(0, 100);
        }
        let mut arc_state = self.outsider_arc_state(&beat.outsider_id);
        if choice.disagreement {
            arc_state.disagreements = arc_state.disagreements.saturating_add(1);
        }
        let participants = [
            beat.outsider_id.to_owned(),
            choice.relationship_partner.to_owned(),
        ];
        self.strengthen_event_participants(&participants);
        if choice.legacy_amount != 0 {
            let character = self
                .roster
                .iter_mut()
                .find(|character| character.id == beat.outsider_id)
                .expect("outsider roster membership was validated before mutation");
            if !character
                .event_legacies
                .iter()
                .any(|legacy| legacy.id == choice.id)
            {
                character.event_legacies.push(CharacterLegacy {
                    id: choice.id.to_owned(),
                    name: choice.legacy_name.to_owned(),
                    stat: choice.legacy_stat.to_owned(),
                    amount: choice.legacy_amount,
                });
            }
        }
        if stage == 2 {
            arc_state.final_choice = choice.id.to_owned();
        }
        arc_state.stage = arc_state.stage.saturating_add(1);
        self.set_outsider_arc_state(&beat.outsider_id, arc_state);
        Ok(format!("{} // {}", beat.title, choice.label))
    }
}
