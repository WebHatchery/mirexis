//! Persistent character identity, progression, and deployment derivation.

use crate::data::{CharacterDef, ClassDef, GameData, MutationDef, Team, UnitDef};
use crate::state::{MissionOutcome, ObjectiveState};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Availability {
    Ready,
    Recovering,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InjuryRecord {
    pub id: String,
    pub name: String,
    pub recovery_operations: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterRecord {
    pub id: String,
    pub name: String,
    pub biography: String,
    pub aptitudes: BTreeMap<String, u8>,
    pub experience: u32,
    pub level: u8,
    pub active_class: String,
    pub class_history: Vec<String>,
    pub learned_skills: Vec<String>,
    pub active_skills: Vec<String>,
    pub mutation_id: String,
    pub injuries: Vec<InjuryRecord>,
    pub availability: Availability,
    pub equipment_ids: Vec<String>,
}

impl CharacterRecord {
    fn from_def(def: &CharacterDef) -> Self {
        let starter_skill = format!("{}_fundamentals", def.initial_class);
        Self {
            id: def.id.clone(),
            name: def.name.clone(),
            biography: def.biography.clone(),
            aptitudes: def.aptitudes.clone(),
            experience: 0,
            level: 1,
            active_class: def.initial_class.clone(),
            class_history: vec![def.initial_class.clone()],
            learned_skills: vec![starter_skill.clone()],
            active_skills: vec![starter_skill],
            mutation_id: def.mutation.clone(),
            injuries: Vec::new(),
            availability: Availability::Ready,
            equipment_ids: def.equipment.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignState {
    pub roster: Vec<CharacterRecord>,
    pub operations_completed: u32,
}

impl CampaignState {
    pub fn new(data: &GameData) -> Self {
        Self {
            roster: data
                .characters
                .iter()
                .map(CharacterRecord::from_def)
                .collect(),
            operations_completed: 0,
        }
    }

    pub fn deployment_roster(&self, data: &GameData) -> Vec<UnitDef> {
        data.roster
            .iter()
            .filter_map(|base| {
                if base.team == Team::Hostile {
                    return Some(base.clone());
                }
                let character = self.roster.iter().find(|record| record.id == base.id)?;
                (character.availability == Availability::Ready)
                    .then(|| derive_unit(base, character, data))
            })
            .collect()
    }

    pub fn apply_mission_outcome(&mut self, outcome: &MissionOutcome, data: &GameData) {
        self.operations_completed += 1;
        let xp = if outcome.result == ObjectiveState::Victory {
            20
        } else {
            8
        };
        for character in &mut self.roster {
            character.experience += xp;
            character.level = 1 + (character.experience / 100).min(9) as u8;
        }
        for consequence in &outcome.colonists_incapacitated {
            if let Some(character) = self
                .roster
                .iter_mut()
                .find(|record| record.id == consequence.id)
            {
                let traits = derived_mutation_traits(character, data);
                let delayed_healing = traits.get("medical_healing").copied().unwrap_or(0) < 0;
                character.injuries.push(InjuryRecord {
                    id: format!("operation_{}_trauma", self.operations_completed),
                    name: "Mire exposure trauma".to_owned(),
                    recovery_operations: if delayed_healing { 3 } else { 2 },
                });
                character.availability = Availability::Recovering;
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
                Availability::Ready
            } else {
                Availability::Recovering
            };
        }
    }

    #[allow(dead_code)]
    pub fn training_cost(&self, character_id: &str, class: &ClassDef) -> Option<u32> {
        let character = self
            .roster
            .iter()
            .find(|record| record.id == character_id)?;
        let aptitude = character
            .aptitudes
            .get(&class.primary_aptitude)
            .copied()
            .unwrap_or(1);
        Some(
            100u32
                .saturating_sub(u32::from(aptitude.saturating_sub(1)) * 10)
                .max(60),
        )
    }

    #[allow(dead_code)]
    pub fn switch_class(
        &mut self,
        character_id: &str,
        class_id: &str,
        data: &GameData,
    ) -> Result<(), String> {
        let class = data
            .classes
            .iter()
            .find(|class| class.id == class_id)
            .ok_or_else(|| format!("Unknown class: {}", class_id))?;
        let character = self
            .roster
            .iter_mut()
            .find(|record| record.id == character_id)
            .ok_or_else(|| format!("Unknown character: {}", character_id))?;
        character.active_class = class.id.clone();
        if !character.class_history.contains(&class.id) {
            character.class_history.push(class.id.clone());
        }
        let skill = format!("{}_fundamentals", class.id);
        if !character.learned_skills.contains(&skill) {
            character.learned_skills.push(skill.clone());
        }
        if !character.active_skills.contains(&skill) {
            character.active_skills.push(skill);
        }
        character.active_skills.truncate(class.skill_slots as usize);
        Ok(())
    }
}

pub fn derived_mutation_traits(
    character: &CharacterRecord,
    data: &GameData,
) -> BTreeMap<String, i32> {
    let mut traits = BTreeMap::new();
    if let Some(mutation) = data
        .mutations
        .iter()
        .find(|entry| entry.id == character.mutation_id)
    {
        apply_mutation(mutation, &mut traits);
    }
    traits
}

fn apply_mutation(mutation: &MutationDef, traits: &mut BTreeMap<String, i32>) {
    for modifier in mutation.gift.iter().chain(&mutation.complication) {
        *traits.entry(modifier.stat.clone()).or_default() += modifier.amount;
    }
}

fn derive_unit(base: &UnitDef, character: &CharacterRecord, data: &GameData) -> UnitDef {
    let mut unit = base.clone();
    let class = data
        .classes
        .iter()
        .find(|entry| entry.id == character.active_class);
    if let Some(class) = class {
        unit.role = class.name.clone();
        unit.max_health += class.health_bonus;
        unit.accuracy += class.accuracy_bonus;
        unit.move_range = add_signed(unit.move_range, class.move_bonus);
    }
    let traits = derived_mutation_traits(character, data);
    unit.armour += traits.get("armour").copied().unwrap_or(0);
    unit.move_range = add_signed(
        unit.move_range,
        traits.get("movement").copied().unwrap_or(0) as i8,
    );
    unit.round_regeneration = traits.get("round_regeneration").copied().unwrap_or(0);
    if let Some(mutation) = data
        .mutations
        .iter()
        .find(|entry| entry.id == character.mutation_id)
    {
        unit.mutation = mutation.name.clone();
    }
    for equipment_id in &character.equipment_ids {
        if let Some(item) = data
            .equipment
            .iter()
            .find(|entry| &entry.id == equipment_id)
        {
            unit.accuracy += item.accuracy;
            unit.armour += item.armour;
            unit.max_health += item.health;
            unit.weapon_damage += item.damage;
        }
    }
    unit
}

fn add_signed(value: u8, change: i8) -> u8 {
    (i16::from(value) + i16::from(change)).clamp(1, i16::from(u8::MAX)) as u8
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::CharacterConsequence;

    #[test]
    fn all_five_mutations_produce_gift_and_complication_traits() {
        let data = GameData::load().unwrap();
        assert!(data.mutations.len() >= 5);
        for mutation in &data.mutations {
            assert!(!mutation.gift.is_empty());
            assert!(!mutation.complication.is_empty());
            let mut traits = BTreeMap::new();
            apply_mutation(mutation, &mut traits);
            assert!(traits.len() >= 2);
        }
    }

    #[test]
    fn aptitude_changes_training_cost_but_not_class_eligibility() {
        let data = GameData::load().unwrap();
        let mut campaign = CampaignState::new(&data);
        let psionic = data
            .classes
            .iter()
            .find(|class| class.id == "psionic")
            .unwrap();
        let cost = campaign.training_cost("mara_venn", psionic).unwrap();
        assert!(cost > campaign.training_cost("kira_voss", psionic).unwrap());
        campaign
            .switch_class("mara_venn", "psionic", &data)
            .unwrap();
        assert_eq!(campaign.roster[1].active_class, "psionic");
    }

    #[test]
    fn deployment_applies_class_mutation_and_equipment() {
        let data = GameData::load().unwrap();
        let campaign = CampaignState::new(&data);
        let roster = campaign.deployment_roster(&data);
        let mara = roster.iter().find(|unit| unit.id == "mara_venn").unwrap();
        assert!(mara.armour >= 5);
        assert_eq!(mara.role, "Defender");
    }

    fn consequence(id: &str, name: &str) -> CharacterConsequence {
        CharacterConsequence {
            id: id.to_owned(),
            name: name.to_owned(),
        }
    }

    #[test]
    fn injury_blocks_deployment_until_operations_recover_it() {
        let data = GameData::load().unwrap();
        let mut campaign = CampaignState::new(&data);
        let outcome = MissionOutcome {
            result: ObjectiveState::Failed,
            colonists_deployed: 4,
            colonists_incapacitated: vec![consequence("ilya_reed", "Ilya Reed")],
            hostiles_neutralised: 0,
            materials_awarded: 0,
        };
        campaign.apply_mission_outcome(&outcome, &data);
        assert_eq!(campaign.roster[2].availability, Availability::Recovering);
        assert!(!campaign
            .deployment_roster(&data)
            .iter()
            .any(|unit| unit.id == "ilya_reed"));
        for _ in 0..3 {
            campaign.advance_recovery();
        }
        assert_eq!(campaign.roster[2].availability, Availability::Ready);
    }
}
