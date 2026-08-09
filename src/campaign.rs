//! Persistent character identity, progression, and deployment derivation.

use crate::colony::{BuildingKind, ColonyState};
use crate::data::{CharacterDef, ClassDef, GameData, MutationDef, Team, UnitDef};
use crate::state::{MissionOutcome, ObjectiveState};
use crate::strategy::{MissionInstance, StrategyState};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const SQUAD_LIMIT: usize = 3;

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CharacterLegacy {
    pub id: String,
    pub name: String,
    pub stat: String,
    pub amount: i32,
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
    #[serde(default = "deployment_selected_default")]
    pub deployment_selected: bool,
    pub equipment_ids: Vec<String>,
    #[serde(default)]
    pub event_legacies: Vec<CharacterLegacy>,
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
            deployment_selected: true,
            equipment_ids: def.equipment.clone(),
            event_legacies: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignState {
    pub roster: Vec<CharacterRecord>,
    #[serde(default)]
    pub selected_character_id: String,
    pub colony: ColonyState,
    pub strategy: StrategyState,
    pub operations_completed: u32,
}

impl CampaignState {
    pub fn new(data: &GameData) -> Self {
        let mut roster = data
            .characters
            .iter()
            .map(CharacterRecord::from_def)
            .collect::<Vec<_>>();
        for (index, character) in roster.iter_mut().enumerate() {
            character.deployment_selected = index < SQUAD_LIMIT;
        }
        Self {
            selected_character_id: roster
                .first()
                .map(|character| character.id.clone())
                .unwrap_or_default(),
            roster,
            colony: ColonyState::new(),
            strategy: StrategyState::new(data),
            operations_completed: 0,
        }
    }

    pub fn deployment_roster(
        &self,
        data: &GameData,
        mission: &crate::data::MissionDef,
    ) -> Vec<UnitDef> {
        let mut deployment = data
            .roster
            .iter()
            .filter_map(|base| {
                if base.team != Team::Colony {
                    return None;
                }
                let character = self.roster.iter().find(|record| record.id == base.id)?;
                (character.availability == Availability::Ready && character.deployment_selected)
                    .then(|| derive_unit(base, character, data))
            })
            .take(SQUAD_LIMIT)
            .collect::<Vec<_>>();
        deployment.extend(
            data.roster
                .iter()
                .filter(|base| {
                    base.team == Team::Hostile
                        && base.faction.as_deref() == Some(&mission.hostile_faction)
                })
                .cloned(),
        );
        deployment
    }

    pub fn selected_squad_count(&self) -> usize {
        self.roster
            .iter()
            .filter(|character| {
                character.availability == Availability::Ready && character.deployment_selected
            })
            .count()
            .min(SQUAD_LIMIT)
    }

    pub fn selected_character(&self) -> Option<&CharacterRecord> {
        self.roster
            .iter()
            .find(|character| character.id == self.selected_character_id)
            .or_else(|| self.roster.first())
    }

    pub fn select_character(&mut self, character_id: &str) -> Result<(), String> {
        if !self
            .roster
            .iter()
            .any(|character| character.id == character_id)
        {
            return Err(format!("Unknown colonist: {}", character_id));
        }
        self.selected_character_id = character_id.to_owned();
        Ok(())
    }

    pub fn toggle_deployment(&mut self, character_id: &str) -> Result<bool, String> {
        let selected_count = self.selected_squad_count();
        let character = self
            .roster
            .iter_mut()
            .find(|character| character.id == character_id)
            .ok_or_else(|| format!("Unknown colonist: {}", character_id))?;
        if character.availability != Availability::Ready {
            return Err(format!("{} is still recovering", character.name));
        }
        if character.deployment_selected {
            if selected_count <= 1 {
                return Err("At least one colonist must deploy".to_owned());
            }
            character.deployment_selected = false;
        } else {
            if selected_count >= SQUAD_LIMIT {
                return Err(format!("Squad limit is {} colonists", SQUAD_LIMIT));
            }
            character.deployment_selected = true;
        }
        Ok(character.deployment_selected)
    }

    pub fn apply_mission_outcome(
        &mut self,
        outcome: &MissionOutcome,
        mission: &MissionInstance,
        data: &GameData,
    ) {
        self.operations_completed += 1;
        self.colony.advance_operation();
        self.colony.resources.materials += outcome.materials_awarded;
        self.strategy.resolve_mission(outcome, mission, data);
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
                let triage_bonus = u8::from(self.strategy.research_completed("xeno_triage"));
                let recovery_operations =
                    (if delayed_healing { 3 } else { 2 } - triage_bonus).max(1);
                character.injuries.push(InjuryRecord {
                    id: format!("operation_{}_trauma", self.operations_completed),
                    name: "Mire exposure trauma".to_owned(),
                    recovery_operations,
                });
                character.availability = Availability::Recovering;
            }
        }
    }

    pub fn resolve_first_character_event(&mut self, data: &GameData) -> Result<String, String> {
        let event = self
            .strategy
            .character_events
            .iter()
            .find(|event| !event.resolved)
            .cloned()
            .ok_or_else(|| "No unresolved character events".to_owned())?;
        let data_legacy = data
            .campaign
            .events
            .iter()
            .find(|definition| definition.id == event.id)
            .map(|definition| {
                (
                    definition.legacy_name.clone(),
                    definition.legacy_character_id.clone(),
                    definition.legacy_stat.clone(),
                    definition.legacy_amount,
                )
            });
        let (legacy_name, legacy_character_id, legacy_stat, legacy_amount) =
            if event.legacy_amount != 0 {
                (
                    event.legacy_name.clone(),
                    event.legacy_character_id.clone(),
                    event.legacy_stat.clone(),
                    event.legacy_amount,
                )
            } else {
                data_legacy.unwrap_or_default()
            };
        if legacy_amount != 0
            && !self
                .roster
                .iter()
                .any(|character| character.id == legacy_character_id)
        {
            return Err(format!("Unknown legacy recipient: {}", legacy_character_id));
        }
        let title = self.strategy.resolve_first_event(&mut self.colony)?;
        if legacy_amount != 0 {
            let character = self
                .roster
                .iter_mut()
                .find(|character| character.id == legacy_character_id)
                .expect("legacy recipient was validated before event mutation");
            if !character
                .event_legacies
                .iter()
                .any(|legacy| legacy.id == event.id)
            {
                character.event_legacies.push(CharacterLegacy {
                    id: event.id,
                    name: legacy_name,
                    stat: legacy_stat,
                    amount: legacy_amount,
                });
            }
        }
        Ok(title)
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

    pub fn train_character(
        &mut self,
        character_id: &str,
        class_id: &str,
        data: &GameData,
    ) -> Result<u32, String> {
        if !self.colony.has_facility(BuildingKind::Barracks) {
            return Err("An operational barracks is required".to_owned());
        }
        let class = data
            .classes
            .iter()
            .find(|class| class.id == class_id)
            .ok_or_else(|| format!("Unknown class: {}", class_id))?;
        let cost = self
            .training_cost(character_id, class)
            .ok_or_else(|| format!("Unknown character: {}", character_id))?;
        if self.colony.resources.materials < cost as i32 {
            return Err(format!("Training requires {} materials", cost));
        }
        self.colony.resources.materials -= cost as i32;
        self.switch_class(character_id, class_id, data)?;
        Ok(cost)
    }

    pub fn treat_first_injury(&mut self) -> Result<String, String> {
        if !self.colony.has_facility(BuildingKind::Infirmary) {
            return Err("An operational infirmary is required".to_owned());
        }
        if self.colony.resources.biomass < 5 {
            return Err("Treatment requires 5 biomass".to_owned());
        }
        let character = self
            .roster
            .iter_mut()
            .find(|character| !character.injuries.is_empty())
            .ok_or_else(|| "No colonist currently needs treatment".to_owned())?;
        self.colony.resources.biomass -= 5;
        for injury in &mut character.injuries {
            injury.recovery_operations = injury.recovery_operations.saturating_sub(1);
        }
        character
            .injuries
            .retain(|injury| injury.recovery_operations > 0);
        if character.injuries.is_empty() {
            character.availability = Availability::Ready;
        }
        Ok(character.name.clone())
    }

    pub fn craft_equipment(
        &mut self,
        character_id: &str,
        equipment_id: &str,
        data: &GameData,
    ) -> Result<u32, String> {
        if !self.colony.has_facility(BuildingKind::Workshop) {
            return Err("An operational workshop is required".to_owned());
        }
        let equipment = data
            .equipment
            .iter()
            .find(|equipment| equipment.id == equipment_id)
            .ok_or_else(|| format!("Unknown equipment: {}", equipment_id))?;
        let cost = equipment_cost(&equipment.slot);
        let character = self
            .roster
            .iter_mut()
            .find(|character| character.id == character_id)
            .ok_or_else(|| format!("Unknown character: {}", character_id))?;
        if character.equipment_ids.contains(&equipment.id) {
            return Err(format!(
                "{} already carries {}",
                character.name, equipment.name
            ));
        }
        if self.colony.resources.materials < cost as i32 {
            return Err(format!("Crafting requires {} materials", cost));
        }
        character.equipment_ids.retain(|id| {
            data.equipment
                .iter()
                .find(|item| &item.id == id)
                .is_none_or(|item| item.slot != equipment.slot)
        });
        self.colony.resources.materials -= cost as i32;
        character.equipment_ids.push(equipment.id.clone());
        Ok(cost)
    }

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

fn deployment_selected_default() -> bool {
    true
}

pub fn equipment_cost(slot: &str) -> u32 {
    match slot {
        "primary" => 30,
        "armour" => 25,
        "tool" | "module" => 20,
        _ => 25,
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
    unit.equipment_ids = character.equipment_ids.clone();
    let class = data
        .classes
        .iter()
        .find(|entry| entry.id == character.active_class);
    if let Some(class) = class {
        unit.role = class.name.clone();
        unit.class_id = class.id.clone();
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
    for legacy in &character.event_legacies {
        match legacy.stat.as_str() {
            "accuracy" => unit.accuracy += legacy.amount,
            "armour" => unit.armour += legacy.amount,
            "health" => unit.max_health += legacy.amount,
            "movement" => unit.move_range = add_signed(unit.move_range, legacy.amount as i8),
            "damage" => unit.weapon_damage += legacy.amount,
            _ => {}
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
        let roster = campaign.deployment_roster(&data, &data.mission);
        let mara = roster.iter().find(|unit| unit.id == "mara_venn").unwrap();
        assert!(mara.armour >= 5);
        assert_eq!(mara.role, "Defender");
    }

    #[test]
    fn deployment_uses_only_the_mission_factions_hostiles() {
        let data = GameData::load().unwrap();
        let campaign = CampaignState::new(&data);
        for faction in ["brood", "directorate", "ascendants"] {
            let mut mission = data.mission.clone();
            mission.hostile_faction = faction.to_owned();
            let roster = campaign.deployment_roster(&data, &mission);
            let hostiles = roster
                .iter()
                .filter(|unit| unit.team == Team::Hostile)
                .collect::<Vec<_>>();
            assert!(!hostiles.is_empty());
            assert!(hostiles
                .iter()
                .all(|unit| unit.faction.as_deref() == Some(faction)));
        }
    }

    #[test]
    fn squad_selection_enforces_reserves_and_a_three_colonist_limit() {
        let data = GameData::load().unwrap();
        let mut campaign = CampaignState::new(&data);
        assert_eq!(campaign.selected_squad_count(), SQUAD_LIMIT);
        assert_eq!(
            campaign
                .deployment_roster(&data, &data.mission)
                .iter()
                .filter(|unit| unit.team == Team::Colony)
                .count(),
            SQUAD_LIMIT
        );
        assert!(campaign.toggle_deployment("sol_cairn").is_err());
        assert!(!campaign.toggle_deployment("kira_voss").unwrap());
        assert!(campaign.toggle_deployment("sol_cairn").unwrap());
        assert_eq!(campaign.selected_squad_count(), SQUAD_LIMIT);
        assert!(campaign
            .deployment_roster(&data, &data.mission)
            .iter()
            .any(|unit| unit.id == "sol_cairn"));
        assert!(!campaign
            .deployment_roster(&data, &data.mission)
            .iter()
            .any(|unit| unit.id == "kira_voss"));
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
        let mission = campaign.strategy.selected_mission().unwrap().clone();
        campaign.apply_mission_outcome(&outcome, &mission, &data);
        assert_eq!(campaign.roster[2].availability, Availability::Recovering);
        assert!(!campaign
            .deployment_roster(&data, &data.mission)
            .iter()
            .any(|unit| unit.id == "ilya_reed"));
        for _ in 0..3 {
            campaign.advance_recovery();
        }
        assert_eq!(campaign.roster[2].availability, Availability::Ready);
    }

    #[test]
    fn xeno_triage_shortens_new_injury_recovery() {
        let data = GameData::load().unwrap();
        let mut campaign = CampaignState::new(&data);
        campaign
            .strategy
            .research
            .iter_mut()
            .find(|research| research.id == "xeno_triage")
            .unwrap()
            .completed = true;
        let outcome = MissionOutcome {
            result: ObjectiveState::Failed,
            colonists_deployed: 4,
            colonists_incapacitated: vec![consequence("ilya_reed", "Ilya Reed")],
            hostiles_neutralised: 0,
            materials_awarded: 0,
        };
        let mission = campaign.strategy.selected_mission().unwrap().clone();
        campaign.apply_mission_outcome(&outcome, &mission, &data);
        assert_eq!(campaign.roster[2].injuries[0].recovery_operations, 2);
    }

    #[test]
    fn character_events_leave_participant_legacies_in_later_deployments() {
        let data = GameData::load().unwrap();
        let mut campaign = CampaignState::new(&data);
        let sol_record = campaign
            .roster
            .iter()
            .find(|character| character.id == "sol_cairn")
            .unwrap();
        let sol_base = data
            .roster
            .iter()
            .find(|unit| unit.id == "sol_cairn")
            .unwrap();
        let movement_before = derive_unit(sol_base, sol_record, &data).move_range;
        campaign.strategy.character_events[0].legacy_name.clear();
        campaign.strategy.character_events[0]
            .legacy_character_id
            .clear();
        campaign.strategy.character_events[0].legacy_stat.clear();
        campaign.strategy.character_events[0].legacy_amount = 0;
        campaign.resolve_first_character_event(&data).unwrap();
        let sol_after = campaign
            .roster
            .iter()
            .find(|character| character.id == "sol_cairn")
            .unwrap();
        assert_eq!(sol_after.event_legacies[0].name, "Survey Family Routes");
        assert_eq!(
            derive_unit(sol_base, sol_after, &data).move_range,
            movement_before + 1
        );

        let mara_base = data
            .roster
            .iter()
            .find(|unit| unit.id == "mara_venn")
            .unwrap();
        let mara_before = derive_unit(mara_base, &campaign.roster[1], &data).armour;
        campaign.resolve_first_character_event(&data).unwrap();
        assert_eq!(
            campaign.roster[1].event_legacies[0].name,
            "Documented Carapace"
        );
        assert_eq!(
            derive_unit(mara_base, &campaign.roster[1], &data).armour,
            mara_before + 1
        );
    }

    #[test]
    fn facilities_gate_training_treatment_and_crafting() {
        let data = GameData::load().unwrap();
        let mut campaign = CampaignState::new(&data);
        let cost = campaign
            .train_character("kira_voss", "soldier", &data)
            .unwrap();
        assert!(cost >= 60);
        assert_eq!(campaign.roster[0].active_class, "soldier");
        campaign
            .craft_equipment("kira_voss", "chitin_plate", &data)
            .unwrap();
        assert!(campaign.roster[0]
            .equipment_ids
            .iter()
            .any(|id| id == "chitin_plate"));
        campaign.colony.resources.materials += 30;
        campaign
            .craft_equipment("kira_voss", "service_pistol", &data)
            .unwrap();
        assert!(campaign.roster[0]
            .equipment_ids
            .iter()
            .any(|id| id == "service_pistol"));
        assert!(!campaign.roster[0]
            .equipment_ids
            .iter()
            .any(|id| id == "frontier_rifle"));
    }
}
