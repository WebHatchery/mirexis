//! Persistent character identity, progression, and deployment derivation.

use crate::colony::{BuildingKind, ColonyState};
use crate::data::{CharacterDef, ClassDef, EquipmentDef, GameData, MutationDef, Team, UnitDef};
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
    #[serde(default)]
    pub mutation_evolution_id: String,
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
            mutation_evolution_id: String::new(),
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

    pub fn ensure_roster_characters(&mut self, data: &GameData) -> usize {
        let mut added = 0;
        for definition in &data.characters {
            if self
                .roster
                .iter()
                .any(|character| character.id == definition.id)
            {
                continue;
            }
            let mut character = CharacterRecord::from_def(definition);
            character.deployment_selected = false;
            self.roster.push(character);
            added += 1;
        }
        added
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
                    if base.team != Team::Hostile {
                        return false;
                    }
                    if mission.hostile_unit_ids.is_empty() {
                        base.faction.as_deref() == Some(&mission.hostile_faction)
                    } else {
                        mission.hostile_unit_ids.contains(&base.id)
                    }
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

    pub fn deployment_food_cost(&self, data: &GameData) -> i32 {
        let base = self
            .roster
            .iter()
            .filter(|character| {
                character.availability == Availability::Ready && character.deployment_selected
            })
            .take(SQUAD_LIMIT)
            .map(|character| {
                1 + derived_mutation_traits(character, data)
                    .get("food_upkeep")
                    .copied()
                    .unwrap_or(0)
                    .max(0)
            })
            .sum::<i32>();
        if base == 0 {
            return 0;
        }
        let discount = data
            .campaign
            .mirexis_paths
            .iter()
            .find(|path| path.id == self.strategy.mirexis_path_id)
            .map_or(0, |path| path.deployment_food_discount);
        (base - discount).max(1)
    }

    pub fn prepare_deployment(&mut self, data: &GameData) -> Result<i32, String> {
        let cost = self.deployment_food_cost(data);
        if cost == 0 {
            return Err("At least one ready colonist must deploy".to_owned());
        }
        if self.colony.resources.food < cost {
            return Err(format!("Deployment requires {} food", cost));
        }
        self.colony.resources.food -= cost;
        Ok(cost)
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
        self.colony.resources.biomass += outcome.biomass_awarded;
        self.colony.resources.power += outcome.power_awarded;
        if mission.map_recipe == "colony_defense" && outcome.result == ObjectiveState::Failed {
            self.colony.damage_for_failed_defense(mission.seed);
        }
        self.strategy.resolve_mission(outcome, mission, data);
        self.strategy.refresh_isolation_completion(&mut self.colony);
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
        self.refresh_contact_completion(data);
        self.refresh_adaptation_completion(data);
        self.refresh_escalation_completion(data);
    }

    pub fn resolve_first_character_event(&mut self, data: &GameData) -> Result<String, String> {
        let event = self
            .strategy
            .available_event()
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
        self.refresh_contact_completion(data);
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
        if !self.equipment_is_unlocked(equipment) {
            return Err(format!(
                "{} requires a completed Contact trace",
                equipment.name
            ));
        }
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
        self.refresh_contact_completion(data);
        Ok(cost)
    }

    pub fn refresh_contact_completion(&mut self, data: &GameData) -> bool {
        let (_, aftermath_resolved, prototype_equipped) = self.contact_completion_progress(data);
        self.strategy
            .refresh_contact_completion(aftermath_resolved, prototype_equipped)
    }

    pub fn contact_completion_progress(&self, data: &GameData) -> (bool, bool, bool) {
        let protocol_id = self.strategy.contact_protocol_id.as_str();
        let aftermath_resolved = data
            .campaign
            .events
            .iter()
            .find(|event| event.required_protocol == protocol_id)
            .is_some_and(|definition| {
                self.strategy
                    .character_events
                    .iter()
                    .any(|event| event.id == definition.id && event.resolved)
            });
        let prototype_equipped = data
            .equipment
            .iter()
            .filter(|equipment| equipment.required_protocol == protocol_id)
            .any(|equipment| {
                self.roster
                    .iter()
                    .any(|character| character.equipment_ids.contains(&equipment.id))
            });
        (
            self.strategy.contact_trace_completed,
            aftermath_resolved,
            prototype_equipped,
        )
    }

    pub fn equipment_is_unlocked(&self, equipment: &EquipmentDef) -> bool {
        equipment.required_protocol.is_empty()
            || (equipment.required_protocol == self.strategy.contact_protocol_id
                && (!equipment.requires_contact_trace || self.strategy.contact_trace_completed))
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
        if self.colony.resources.biomass < evolution.biomass_cost {
            return Err(format!(
                "{} requires {} biomass",
                evolution.name, evolution.biomass_cost
            ));
        }
        self.colony.resources.biomass -= evolution.biomass_cost;
        self.roster
            .iter_mut()
            .find(|character| character.id == character_id)
            .expect("evolution character was validated")
            .mutation_evolution_id = evolution.id.clone();
        if !self.refresh_adaptation_completion(data) {
            self.strategy.regenerate_missions(data);
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
            self.strategy.regenerate_missions(data);
        }
        changed
    }

    pub fn refresh_escalation_completion(&mut self, data: &GameData) -> bool {
        let changed = self.strategy.refresh_escalation_completion();
        if changed {
            self.strategy.regenerate_missions(data);
        }
        changed
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
        if let Some(evolution) = mutation
            .evolutions
            .iter()
            .find(|evolution| evolution.id == character.mutation_evolution_id)
        {
            for modifier in evolution.gift.iter().chain(&evolution.complication) {
                *traits.entry(modifier.stat.clone()).or_default() += modifier.amount;
            }
        }
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
    unit.accuracy += traits.get("accuracy").copied().unwrap_or(0);
    unit.weapon_damage += traits.get("weapon_damage").copied().unwrap_or(0);
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
            unit.armour +=
                (item.armour + traits.get("heavy_armour_efficiency").copied().unwrap_or(0)).max(0);
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
    fn three_knives_deploys_one_hostile_from_each_power() {
        let data = GameData::load().unwrap();
        let mut campaign = CampaignState::new(&data);
        campaign.strategy.phase_id = "escalation".to_owned();
        campaign.strategy.regenerate_missions(&data);
        let mission_id = campaign
            .strategy
            .mission_offers
            .iter()
            .find(|mission| mission.template_id == "escalation_three_knives")
            .unwrap()
            .id
            .clone();
        campaign.strategy.select_mission(&mission_id).unwrap();
        let mission = campaign
            .strategy
            .materialize_selected(&data, &campaign.colony);
        let hostile_factions = campaign
            .deployment_roster(&data, &mission)
            .into_iter()
            .filter(|unit| unit.team == Team::Hostile)
            .filter_map(|unit| unit.faction)
            .collect::<std::collections::HashSet<_>>();

        assert_eq!(hostile_factions.len(), 3);
        assert!(hostile_factions.contains("directorate"));
        assert!(hostile_factions.contains("brood"));
        assert!(hostile_factions.contains("ascendants"));
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
            biomass_awarded: 0,
            power_awarded: 0,
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
            biomass_awarded: 0,
            power_awarded: 0,
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
    fn successful_operations_apply_all_recovered_resources() {
        let data = GameData::load().unwrap();
        let mut campaign = CampaignState::new(&data);
        let before = campaign.colony.resources.clone();
        let outcome = MissionOutcome {
            result: ObjectiveState::Victory,
            colonists_deployed: 3,
            colonists_incapacitated: Vec::new(),
            hostiles_neutralised: 2,
            materials_awarded: 7,
            biomass_awarded: 5,
            power_awarded: 3,
        };
        let mission = campaign.strategy.selected_mission().unwrap().clone();
        campaign.apply_mission_outcome(&outcome, &mission, &data);
        assert_eq!(campaign.colony.resources.materials, before.materials + 7);
        assert_eq!(campaign.colony.resources.biomass, before.biomass + 5);
        assert_eq!(campaign.colony.resources.power, before.power + 3);
    }

    #[test]
    fn failed_colony_defense_damages_a_saved_facility() {
        let data = GameData::load().unwrap();
        let mut campaign = CampaignState::new(&data);
        let mut mission = campaign.strategy.selected_mission().unwrap().clone();
        mission.map_recipe = "colony_defense".to_owned();
        let outcome = MissionOutcome {
            result: ObjectiveState::Failed,
            colonists_deployed: 3,
            colonists_incapacitated: Vec::new(),
            hostiles_neutralised: 0,
            materials_awarded: 0,
            biomass_awarded: 0,
            power_awarded: 0,
        };
        campaign.apply_mission_outcome(&outcome, &mission, &data);
        assert!(campaign
            .colony
            .buildings
            .iter()
            .any(|building| building.damaged));
    }

    #[test]
    fn deployment_rations_include_mutation_upkeep_and_hydroponics_recovery() {
        let data = GameData::load().unwrap();
        let mut campaign = CampaignState::new(&data);
        campaign.roster[0].mutation_id = "symbiotic_organism".to_owned();
        let starting_food = campaign.colony.resources.food;
        assert_eq!(campaign.deployment_food_cost(&data), 4);
        assert_eq!(campaign.prepare_deployment(&data).unwrap(), 4);
        let outcome = MissionOutcome {
            result: ObjectiveState::Victory,
            colonists_deployed: 3,
            colonists_incapacitated: Vec::new(),
            hostiles_neutralised: 3,
            materials_awarded: 0,
            biomass_awarded: 0,
            power_awarded: 0,
        };
        let mission = campaign.strategy.selected_mission().unwrap().clone();
        campaign.apply_mission_outcome(&outcome, &mission, &data);
        assert_eq!(campaign.colony.resources.food, starting_food - 1);
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

    #[test]
    fn contact_prototypes_require_the_matching_completed_trace() {
        let data = GameData::load().unwrap();
        let mut campaign = CampaignState::new(&data);
        assert!(campaign
            .craft_equipment("kira_voss", "directorate_smartlink", &data)
            .is_err());
        campaign.strategy.isolation_victories = 3;
        campaign.strategy.first_assault_repulsed = true;
        campaign.strategy.research[0].completed = true;
        campaign
            .strategy
            .refresh_isolation_completion(&mut campaign.colony);
        campaign
            .strategy
            .choose_contact_protocol("directorate_requisition", &mut campaign.colony, &data)
            .unwrap();
        assert!(campaign
            .craft_equipment("kira_voss", "directorate_smartlink", &data)
            .is_err());
        campaign.strategy.contact_trace_completed = true;
        campaign
            .craft_equipment("kira_voss", "directorate_smartlink", &data)
            .unwrap();
        assert!(campaign.roster[0]
            .equipment_ids
            .iter()
            .any(|id| id == "directorate_smartlink"));
        assert!(campaign
            .craft_equipment("kira_voss", "brood_living_plate", &data)
            .is_err());
    }

    #[test]
    fn contact_aftermath_event_changes_its_faction_and_character() {
        let data = GameData::load().unwrap();
        let mut campaign = CampaignState::new(&data);
        campaign.strategy.isolation_victories = 3;
        campaign.strategy.first_assault_repulsed = true;
        campaign.strategy.research[0].completed = true;
        campaign
            .strategy
            .refresh_isolation_completion(&mut campaign.colony);
        campaign
            .strategy
            .choose_contact_protocol("ascendant_capacitor", &mut campaign.colony, &data)
            .unwrap();
        campaign.strategy.contact_trace_completed = true;
        campaign.resolve_first_character_event(&data).unwrap();
        campaign.resolve_first_character_event(&data).unwrap();
        let attention_before = campaign
            .strategy
            .factions
            .iter()
            .find(|faction| faction.id == "ascendants")
            .unwrap()
            .attention;
        assert_eq!(
            campaign.resolve_first_character_event(&data).unwrap(),
            "The Light Between Seconds"
        );
        assert_eq!(
            campaign
                .strategy
                .factions
                .iter()
                .find(|faction| faction.id == "ascendants")
                .unwrap()
                .attention,
            (attention_before - 5).max(0)
        );
        assert!(campaign
            .roster
            .iter()
            .find(|character| character.id == "sol_cairn")
            .unwrap()
            .event_legacies
            .iter()
            .any(|legacy| legacy.id == "ascendant_contact_aftermath"));
        campaign
            .craft_equipment("sol_cairn", "ascendant_phase_lens", &data)
            .unwrap();
        assert!(campaign.strategy.contact_complete);
        assert_eq!(campaign.strategy.phase_id, "adaptation");
    }

    #[test]
    fn adaptation_evolution_applies_its_gift_and_complication() {
        let data = GameData::load().unwrap();
        let mut campaign = CampaignState::new(&data);
        campaign.strategy.contact_complete = true;
        assert!(campaign
            .choose_mutation_evolution("kira_voss", "expanded_cortex", &data)
            .is_err());
        campaign.colony.ensure_gene_lab();
        campaign.colony.resources.power += 2;
        let accuracy_before = campaign
            .deployment_roster(&data, &data.mission)
            .into_iter()
            .find(|unit| unit.id == "kira_voss")
            .unwrap()
            .accuracy;
        let food_before = campaign.deployment_food_cost(&data);
        assert_eq!(
            campaign
                .choose_mutation_evolution("kira_voss", "expanded_cortex", &data)
                .unwrap(),
            "Expanded Cortex"
        );
        let accuracy_after = campaign
            .deployment_roster(&data, &data.mission)
            .into_iter()
            .find(|unit| unit.id == "kira_voss")
            .unwrap()
            .accuracy;
        assert_eq!(accuracy_after, accuracy_before + 10);
        assert_eq!(campaign.deployment_food_cost(&data), food_before + 1);
        assert!(campaign
            .choose_mutation_evolution("kira_voss", "echo_mind", &data)
            .is_err());
    }

    #[test]
    fn mara_can_trade_mobility_for_a_fortress_carapace() {
        let data = GameData::load().unwrap();
        let mut campaign = CampaignState::new(&data);
        campaign.strategy.contact_complete = true;
        campaign.colony.ensure_gene_lab();
        campaign.colony.resources.power += 2;
        let before = campaign
            .deployment_roster(&data, &data.mission)
            .into_iter()
            .find(|unit| unit.id == "mara_venn")
            .unwrap();
        assert_eq!(
            campaign
                .choose_mutation_evolution("mara_venn", "fortress_carapace", &data)
                .unwrap(),
            "Fortress Carapace"
        );
        let after = campaign
            .deployment_roster(&data, &data.mission)
            .into_iter()
            .find(|unit| unit.id == "mara_venn")
            .unwrap();
        assert_eq!(after.armour, before.armour + 2);
        assert_eq!(after.move_range, before.move_range - 1);
        assert!(campaign.roster[0].mutation_evolution_id.is_empty());
    }

    #[test]
    fn clean_marrow_stabilizes_ilyas_injury_recovery_at_a_damage_cost() {
        let data = GameData::load().unwrap();
        let mut campaign = CampaignState::new(&data);
        campaign.strategy.contact_complete = true;
        campaign.colony.ensure_gene_lab();
        campaign.colony.resources.power += 2;
        let damage_before = campaign
            .deployment_roster(&data, &data.mission)
            .into_iter()
            .find(|unit| unit.id == "ilya_reed")
            .unwrap()
            .weapon_damage;
        campaign
            .choose_mutation_evolution("ilya_reed", "clean_marrow", &data)
            .unwrap();
        let damage_after = campaign
            .deployment_roster(&data, &data.mission)
            .into_iter()
            .find(|unit| unit.id == "ilya_reed")
            .unwrap()
            .weapon_damage;
        assert_eq!(damage_after, damage_before - 1);
        let outcome = MissionOutcome {
            result: ObjectiveState::Failed,
            colonists_deployed: 3,
            colonists_incapacitated: vec![consequence("ilya_reed", "Ilya Reed")],
            hostiles_neutralised: 0,
            materials_awarded: 0,
            biomass_awarded: 0,
            power_awarded: 0,
        };
        let mission = campaign.strategy.selected_mission().unwrap().clone();
        campaign.apply_mission_outcome(&outcome, &mission, &data);
        assert_eq!(campaign.roster[2].injuries[0].recovery_operations, 2);
    }

    #[test]
    fn load_bearing_fascia_restores_sols_armour_efficiency() {
        let data = GameData::load().unwrap();
        let mut campaign = CampaignState::new(&data);
        campaign.strategy.contact_complete = true;
        campaign.colony.ensure_gene_lab();
        campaign.colony.resources.power += 2;
        campaign.roster[3]
            .equipment_ids
            .push("chitin_plate".to_owned());
        let base = data
            .roster
            .iter()
            .find(|unit| unit.id == "sol_cairn")
            .unwrap();
        let before = derive_unit(base, &campaign.roster[3], &data);
        campaign
            .choose_mutation_evolution("sol_cairn", "load_bearing_fascia", &data)
            .unwrap();
        let after = derive_unit(base, &campaign.roster[3], &data);
        assert_eq!(after.armour, before.armour + 1);
        assert_eq!(after.weapon_damage, before.weapon_damage - 1);
    }

    #[test]
    fn nadis_symbiote_evolves_toward_cooperation_or_predation() {
        let data = GameData::load().unwrap();
        let base = data
            .roster
            .iter()
            .find(|unit| unit.id == "nadi_vale")
            .unwrap();

        let mut cooperative = CampaignState::new(&data);
        cooperative.strategy.contact_complete = true;
        cooperative.colony.ensure_gene_lab();
        cooperative.colony.resources.power += 2;
        cooperative.colony.resources.biomass = 20;
        let nadi = cooperative
            .roster
            .iter()
            .find(|character| character.id == "nadi_vale")
            .unwrap();
        let before = derive_unit(base, nadi, &data);
        cooperative
            .choose_mutation_evolution("nadi_vale", "cooperative_symbiote", &data)
            .unwrap();
        let nadi = cooperative
            .roster
            .iter()
            .find(|character| character.id == "nadi_vale")
            .unwrap();
        let after = derive_unit(base, nadi, &data);
        assert_eq!(after.armour, before.armour + 2);
        assert_eq!(after.round_regeneration, before.round_regeneration + 1);
        assert_eq!(after.move_range, before.move_range - 1);

        let mut predatory = CampaignState::new(&data);
        predatory.strategy.contact_complete = true;
        predatory.colony.ensure_gene_lab();
        predatory.colony.resources.power += 2;
        predatory.colony.resources.biomass = 20;
        predatory
            .choose_mutation_evolution("nadi_vale", "predatory_symbiote", &data)
            .unwrap();
        let nadi = predatory
            .roster
            .iter()
            .find(|character| character.id == "nadi_vale")
            .unwrap();
        let after = derive_unit(base, nadi, &data);
        assert_eq!(after.weapon_damage, before.weapon_damage + 3);
        assert_eq!(after.accuracy, before.accuracy - 10);
        assert_eq!(derived_mutation_traits(nadi, &data)["food_upkeep"], 2);
    }

    #[test]
    fn adaptation_completion_requires_glass_nerve_and_two_evolutions() {
        let data = GameData::load().unwrap();
        let mut campaign = CampaignState::new(&data);
        campaign.strategy.contact_complete = true;
        campaign.strategy.phase_id = "adaptation".to_owned();
        campaign.colony.ensure_gene_lab();
        campaign.colony.resources.power += 2;
        campaign.colony.resources.biomass = 50;
        campaign
            .choose_mutation_evolution("kira_voss", "expanded_cortex", &data)
            .unwrap();
        campaign
            .choose_mutation_evolution("mara_venn", "fortress_carapace", &data)
            .unwrap();
        assert!(!campaign.strategy.adaptation_complete);
        let mission = campaign.strategy.selected_mission().unwrap().clone();
        assert_eq!(mission.template_id, "adaptation_glass_nerve");
        let outcome = MissionOutcome {
            result: ObjectiveState::Victory,
            colonists_deployed: 3,
            colonists_incapacitated: Vec::new(),
            hostiles_neutralised: 3,
            materials_awarded: mission.materials_reward,
            biomass_awarded: mission.biomass_reward,
            power_awarded: mission.power_reward,
        };
        campaign.apply_mission_outcome(&outcome, &mission, &data);
        assert!(campaign.strategy.adaptation_operation_completed);
        assert!(campaign.strategy.adaptation_complete);
        assert_eq!(campaign.strategy.phase_id, "escalation");
    }

    #[test]
    fn escalation_completion_requires_the_matching_response_operation() {
        let data = GameData::load().unwrap();
        let mut campaign = CampaignState::new(&data);
        campaign.strategy.adaptation_complete = true;
        campaign.strategy.phase_id = "escalation".to_owned();
        campaign.strategy.escalation_operation_completed = true;
        campaign.colony.resources.biomass = 20;
        campaign
            .strategy
            .choose_escalation_response("living_decoy", &mut campaign.colony, &data)
            .unwrap();
        assert!(!campaign.strategy.escalation_complete);
        let mission = campaign.strategy.selected_mission().unwrap().clone();
        assert_eq!(mission.template_id, "escalation_living_false_heart");
        let outcome = MissionOutcome {
            result: ObjectiveState::Victory,
            colonists_deployed: 3,
            colonists_incapacitated: Vec::new(),
            hostiles_neutralised: 3,
            materials_awarded: mission.materials_reward,
            biomass_awarded: mission.biomass_reward,
            power_awarded: mission.power_reward,
        };
        campaign.apply_mission_outcome(&outcome, &mission, &data);

        assert!(campaign.strategy.escalation_branch_completed);
        assert!(campaign.strategy.escalation_complete);
        assert_eq!(campaign.strategy.phase_id, "mirexis");
    }

    #[test]
    fn mirexis_paths_change_defense_supply_and_recovery() {
        let data = GameData::load().unwrap();

        let mut redoubt = CampaignState::new(&data);
        assert!(redoubt
            .strategy
            .choose_mirexis_path("human_redoubt", &mut redoubt.colony, &data)
            .is_err());
        redoubt.strategy.escalation_complete = true;
        redoubt.strategy.phase_id = "mirexis".to_owned();
        redoubt.colony.resources.materials = 100;
        redoubt
            .strategy
            .choose_mirexis_path("human_redoubt", &mut redoubt.colony, &data)
            .unwrap();
        assert_eq!(
            redoubt.strategy.selected_mission().unwrap().template_id,
            "mirexis_redoubt_last_wall"
        );
        redoubt.strategy.threats[0].operations_until = 0;
        redoubt.strategy.regenerate_missions(&data);
        assert_eq!(
            redoubt
                .strategy
                .materialize_selected(&data, &redoubt.colony)
                .cover_integrity,
            data.mission.cover_integrity + 4
        );

        let mut commonwealth = CampaignState::new(&data);
        commonwealth.strategy.escalation_complete = true;
        commonwealth.strategy.phase_id = "mirexis".to_owned();
        commonwealth.colony.resources.biomass = 20;
        let food_before = commonwealth.deployment_food_cost(&data);
        commonwealth
            .strategy
            .choose_mirexis_path("living_commonwealth", &mut commonwealth.colony, &data)
            .unwrap();
        assert_eq!(commonwealth.deployment_food_cost(&data), food_before - 1);
        assert_eq!(
            commonwealth
                .strategy
                .selected_mission()
                .unwrap()
                .template_id,
            "mirexis_commonwealth_root_choir"
        );

        let mut threshold = CampaignState::new(&data);
        threshold.strategy.escalation_complete = true;
        threshold.strategy.phase_id = "mirexis".to_owned();
        threshold.colony.resources.power = 10;
        threshold
            .strategy
            .choose_mirexis_path("open_threshold", &mut threshold.colony, &data)
            .unwrap();
        assert_eq!(
            threshold.strategy.selected_mission().unwrap().template_id,
            "mirexis_threshold_door_of_light"
        );
        let power_before = threshold.strategy.selected_mission().unwrap().power_reward;
        assert_eq!(
            threshold
                .strategy
                .materialize_selected(&data, &threshold.colony)
                .power_reward,
            power_before + 3
        );
        assert!(threshold
            .strategy
            .choose_mirexis_path("human_redoubt", &mut threshold.colony, &data)
            .is_err());
    }

    #[test]
    fn matching_mirexis_operation_reveals_the_chosen_campaign_end() {
        let data = GameData::load().unwrap();
        for (path_id, template_id, ending_title) in [
            (
                "human_redoubt",
                "mirexis_redoubt_last_wall",
                "THE LAST WALL HOLDS",
            ),
            (
                "living_commonwealth",
                "mirexis_commonwealth_root_choir",
                "THE ROOT CHOIR ANSWERS",
            ),
            (
                "open_threshold",
                "mirexis_threshold_door_of_light",
                "THE DOOR OF LIGHT OPENS",
            ),
        ] {
            let mut campaign = CampaignState::new(&data);
            campaign.strategy.escalation_complete = true;
            campaign.strategy.phase_id = "mirexis".to_owned();
            campaign.colony.resources.materials = 100;
            campaign.colony.resources.biomass = 30;
            campaign.colony.resources.power = 20;
            campaign
                .strategy
                .choose_mirexis_path(path_id, &mut campaign.colony, &data)
                .unwrap();
            let mission = campaign.strategy.selected_mission().unwrap().clone();
            assert_eq!(mission.template_id, template_id);
            let outcome = MissionOutcome {
                result: ObjectiveState::Victory,
                colonists_deployed: 3,
                colonists_incapacitated: Vec::new(),
                hostiles_neutralised: 3,
                materials_awarded: mission.materials_reward,
                biomass_awarded: mission.biomass_reward,
                power_awarded: mission.power_reward,
            };
            campaign.apply_mission_outcome(&outcome, &mission, &data);

            assert!(campaign.strategy.mirexis_operation_completed);
            assert!(campaign.strategy.campaign_complete);
            assert_eq!(campaign.strategy.phase_name, ending_title);
            assert!(!campaign.strategy.phase_summary.is_empty());
            assert!(campaign
                .strategy
                .mission_offers
                .iter()
                .all(|offer| offer.template_id != template_id));
        }
    }
}
