//! Campaign-side technique learning and active loadout rules.

use crate::campaign::{CampaignState, CharacterRecord};
use crate::colony::{BuildingKind, SIMULATION_HALL_UPGRADE};
use crate::data::{ClassDef, GameData};

impl CampaignState {
    pub fn learn_skill(
        &mut self,
        character_id: &str,
        skill_id: &str,
        data: &GameData,
    ) -> Result<String, String> {
        let (class, technique) = technique_for_character(self, character_id, skill_id, data)?;
        let required_experience = self.skill_experience_required(technique.experience_required);
        let character = self
            .roster
            .iter_mut()
            .find(|character| character.id == character_id)
            .ok_or_else(|| format!("Unknown character: {}", character_id))?;
        if character.experience < required_experience {
            return Err(format!("REQUIRES {} XP", required_experience));
        }
        if character.learned_skills.contains(&technique.id) {
            return Err("Technique already learned".to_owned());
        }
        character.learned_skills.push(technique.id.clone());
        normalize_loadout(character, &class);
        Ok(technique.name)
    }

    pub fn toggle_skill(
        &mut self,
        character_id: &str,
        skill_id: &str,
        data: &GameData,
    ) -> Result<bool, String> {
        let (class, technique) = technique_for_character(self, character_id, skill_id, data)?;
        let character = self
            .roster
            .iter_mut()
            .find(|character| character.id == character_id)
            .ok_or_else(|| format!("Unknown character: {}", character_id))?;
        if !character.learned_skills.contains(&technique.id) {
            return Err("Learn the technique before equipping it".to_owned());
        }
        if let Some(index) = character
            .active_skills
            .iter()
            .position(|skill| skill == &technique.id)
        {
            character.active_skills.remove(index);
            return Ok(false);
        }
        let equipped = character
            .active_skills
            .iter()
            .filter(|skill| class.techniques.iter().any(|entry| &entry.id == *skill))
            .count();
        if equipped >= class.technique_slots as usize {
            return Err(format!(
                "Loadout holds {} technique{}",
                class.technique_slots,
                if class.technique_slots == 1 { "" } else { "s" }
            ));
        }
        character.active_skills.push(technique.id);
        Ok(true)
    }

    pub fn skill_experience_required(&self, base_requirement: u32) -> u32 {
        let trial_discount = if self
            .colony
            .has_active_upgrade(BuildingKind::Barracks, SIMULATION_HALL_UPGRADE)
        {
            10
        } else {
            0
        };
        base_requirement.saturating_sub(trial_discount).max(1)
    }
}

pub(crate) fn learn_after_operation(
    campaign: &mut CampaignState,
    deployed_ids: &[String],
    data: &GameData,
) -> Vec<String> {
    let mut learned = Vec::new();
    for character_id in deployed_ids {
        let Some(character) = campaign
            .roster
            .iter()
            .find(|entry| &entry.id == character_id)
        else {
            continue;
        };
        let Some(class) = data
            .classes
            .iter()
            .find(|class| class.id == character.active_class)
            .cloned()
        else {
            continue;
        };
        let eligible = class
            .techniques
            .iter()
            .filter(|technique| {
                character.experience
                    >= campaign.skill_experience_required(technique.experience_required)
                    && !character.learned_skills.contains(&technique.id)
            })
            .map(|technique| technique.id.clone())
            .collect::<Vec<_>>();
        for skill_id in eligible {
            if let Ok(name) = campaign.learn_skill(character_id, &skill_id, data) {
                learned.push(format!("{}: {}", character_id, name));
            }
        }
    }
    learned
}

pub(crate) fn normalize_loadout(character: &mut CharacterRecord, class: &ClassDef) {
    let fundamental = format!("{}_fundamentals", class.id);
    if !character.learned_skills.contains(&fundamental) {
        character.learned_skills.push(fundamental.clone());
    }
    character.active_skills.retain(|skill| {
        skill == &fundamental
            || class
                .techniques
                .iter()
                .any(|technique| &technique.id == skill && character.learned_skills.contains(skill))
    });
    if !character.active_skills.contains(&fundamental) {
        character.active_skills.insert(0, fundamental);
    }
    let mut technique_count = character
        .active_skills
        .iter()
        .filter(|skill| {
            class
                .techniques
                .iter()
                .any(|technique| &technique.id == *skill)
        })
        .count();
    for technique in &class.techniques {
        if technique_count >= class.technique_slots as usize {
            break;
        }
        if character.learned_skills.contains(&technique.id)
            && !character.active_skills.contains(&technique.id)
        {
            character.active_skills.push(technique.id.clone());
            technique_count += 1;
        }
    }
    let mut technique_count = 0;
    character.active_skills.retain(|skill| {
        if skill.ends_with("_fundamentals") {
            true
        } else if technique_count < class.technique_slots as usize {
            technique_count += 1;
            true
        } else {
            false
        }
    });
}

fn technique_for_character(
    campaign: &CampaignState,
    character_id: &str,
    skill_id: &str,
    data: &GameData,
) -> Result<(ClassDef, crate::data::TechniqueDef), String> {
    let character = campaign
        .roster
        .iter()
        .find(|character| character.id == character_id)
        .ok_or_else(|| format!("Unknown character: {}", character_id))?;
    let class = data
        .classes
        .iter()
        .find(|class| class.id == character.active_class)
        .ok_or_else(|| format!("Unknown class: {}", character.active_class))?;
    let technique = class
        .techniques
        .iter()
        .find(|technique| technique.id == skill_id)
        .cloned()
        .ok_or_else(|| format!("Technique {} is not part of {}", skill_id, class.name))?;
    Ok((class.clone(), technique))
}

pub(crate) fn validate_definitions(classes: &[ClassDef]) -> Result<(), String> {
    let mut technique_ids = std::collections::HashSet::new();
    for class in classes {
        if class.technique_slots > class.skill_slots {
            return Err(format!(
                "Class {} has more technique slots than skill slots",
                class.id
            ));
        }
        for technique in &class.techniques {
            if technique.id.trim().is_empty()
                || technique.name.trim().is_empty()
                || technique.description.trim().is_empty()
            {
                return Err(format!("Class {} has an incomplete technique", class.id));
            }
            if technique.action_points == 0 || technique.experience_required == 0 {
                return Err(format!(
                    "Technique {} has an invalid cost or learning threshold",
                    technique.id
                ));
            }
            if !technique_ids.insert(technique.id.clone()) {
                return Err(format!(
                    "Technique {} is defined more than once",
                    technique.id
                ));
            }
            if crate::skills::target_kind(&technique.id) != Some(technique.target) {
                return Err(format!(
                    "Technique {} has a target that does not match its implementation",
                    technique.id
                ));
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;
