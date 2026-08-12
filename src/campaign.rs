//! Persistent character identity, progression, and deployment derivation.

mod deployment;

use crate::colony::{BuildingKind, ColonyState};
use crate::data::{CharacterDef, EquipmentDef, GameData, MutationDef, Team, UnitDef};
use crate::relationships::RelationshipRecord;
use crate::state::{MissionOutcome, ObjectiveState};
use crate::strategy::{MissionInstance, StrategyState};
use crate::trauma::TraumaRecord;
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
    #[serde(default)]
    pub traumas: Vec<TraumaRecord>,
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
            traumas: Vec::new(),
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
    #[serde(default)]
    pub relationships: Vec<RelationshipRecord>,
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
            relationships: Vec::new(),
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
        crate::relationships::apply_deployment_bonuses(&mut deployment, &self.relationships);
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
        deployment::spread_hostile_deployment(
            &mut deployment,
            mission,
            data.config.world_width as i32,
            data.config.world_height as i32,
        );
        deployment
    }

    pub fn derived_character_unit(&self, character_id: &str, data: &GameData) -> Option<UnitDef> {
        let character = self
            .roster
            .iter()
            .find(|record| record.id == character_id)?;
        let base = data
            .roster
            .iter()
            .find(|unit| unit.team == Team::Colony && unit.id == character_id)?;
        Some(derive_unit(base, character, data))
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
        let deployed_ids = self
            .roster
            .iter()
            .filter(|character| {
                character.availability == Availability::Ready && character.deployment_selected
            })
            .take(SQUAD_LIMIT)
            .map(|character| character.id.clone())
            .collect::<Vec<_>>();
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
        if outcome.result == ObjectiveState::Victory {
            self.strengthen_shared_victory(&deployed_ids);
        }
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
                crate::trauma::record_incapacitation(character, self.operations_completed);
                character.availability = Availability::Recovering;
            }
        }
        self.refresh_contact_completion(data);
        self.refresh_adaptation_completion(data);
        self.refresh_escalation_completion(data);
    }

    pub fn resolve_first_character_event(&mut self, data: &GameData) -> Result<String, String> {
        let recipient_id = self
            .strategy
            .available_event()
            .and_then(|event| {
                (!event.legacy_character_id.is_empty())
                    .then_some(event.legacy_character_id.as_str())
            })
            .or_else(|| {
                self.strategy.available_event().and_then(|event| {
                    data.campaign
                        .events
                        .iter()
                        .find(|definition| definition.id == event.id)
                        .map(|definition| definition.legacy_character_id.as_str())
                })
            })
            .ok_or_else(|| "No event legacy recipient".to_owned())?
            .to_owned();
        self.resolve_first_character_event_for(&recipient_id, data)
    }

    pub fn resolve_first_character_event_for(
        &mut self,
        recipient_id: &str,
        data: &GameData,
    ) -> Result<String, String> {
        let event = self
            .strategy
            .available_event()
            .cloned()
            .ok_or_else(|| "No unresolved character events".to_owned())?;
        if !event
            .participants
            .iter()
            .any(|participant| participant == recipient_id)
        {
            return Err("Choose one of the event participants".to_owned());
        }
        let data_legacy = data
            .campaign
            .events
            .iter()
            .find(|definition| definition.id == event.id)
            .map(|definition| {
                (
                    definition.legacy_name.clone(),
                    definition.legacy_stat.clone(),
                    definition.legacy_amount,
                )
            });
        let (legacy_name, legacy_stat, legacy_amount) = if event.legacy_amount != 0 {
            (
                event.legacy_name.clone(),
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
                .any(|character| character.id == recipient_id)
        {
            return Err(format!("Unknown legacy recipient: {}", recipient_id));
        }
        let title = self.strategy.resolve_first_event(&mut self.colony)?;
        self.strengthen_event_participants(&event.participants);
        if legacy_amount != 0 {
            let character = self
                .roster
                .iter_mut()
                .find(|character| character.id == recipient_id)
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
            unit.move_range = add_signed(unit.move_range, item.move_bonus);
            if item.weapon_range_override > 0 {
                unit.weapon_range = item.weapon_range_override;
            }
            if item.weapon_ap_cost_override > 0 {
                unit.weapon_ap_cost = item.weapon_ap_cost_override;
            }
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
    crate::trauma::apply_deployment_traits(&mut unit, &character.traumas);
    unit
}

fn add_signed(value: u8, change: i8) -> u8 {
    (i16::from(value) + i16::from(change)).clamp(1, i16::from(u8::MAX)) as u8
}

#[cfg(test)]
mod tests;
