//! Barracks training costs, mastery gates, and class switching.

use crate::campaign::CampaignState;
use crate::colony::BuildingKind;
use crate::data::{ClassDef, GameData};

impl CampaignState {
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
        if let Some(reason) = self.class_training_lock_reason(character_id, class) {
            return Err(reason);
        }
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
        let base_cost: u32 = if class.advanced { 160 } else { 100 };
        Some(
            base_cost
                .saturating_sub(u32::from(aptitude.saturating_sub(1)) * 10)
                .max(if class.advanced { 110 } else { 60 }),
        )
    }

    pub fn class_training_lock_reason(
        &self,
        character_id: &str,
        class: &ClassDef,
    ) -> Option<String> {
        let character = self
            .roster
            .iter()
            .find(|record| record.id == character_id)?;
        if character.level < class.required_level {
            return Some(format!("REQUIRES LEVEL {}", class.required_level));
        }
        if campaign_phase_rank(&self.strategy.phase_id) < campaign_phase_rank(&class.required_phase)
        {
            return Some(format!(
                "UNLOCKS IN {}",
                class.required_phase.to_uppercase()
            ));
        }
        let missing = class
            .prerequisite_classes
            .iter()
            .filter(|id| !character.class_history.contains(id))
            .map(|id| id.replace('_', " ").to_uppercase())
            .collect::<Vec<_>>();
        (!missing.is_empty()).then(|| format!("MASTER {}", missing.join(" + ")))
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
        crate::skill_training::normalize_loadout(character, class);
        Ok(())
    }
}

pub(crate) fn validate_definitions(classes: &[ClassDef]) -> Result<(), String> {
    for class in classes {
        if class.description.trim().is_empty() {
            return Err(format!("Class {} has no tactical description", class.id));
        }
        if class.advanced && class.prerequisite_classes.is_empty() {
            return Err(format!("Advanced class {} has no prerequisites", class.id));
        }
        for prerequisite in &class.prerequisite_classes {
            if prerequisite == &class.id || !classes.iter().any(|entry| &entry.id == prerequisite) {
                return Err(format!(
                    "Class {} references invalid prerequisite {}",
                    class.id, prerequisite
                ));
            }
        }
    }
    Ok(())
}

fn campaign_phase_rank(phase: &str) -> u8 {
    match phase {
        "isolation" | "" => 0,
        "contact" => 1,
        "adaptation" => 2,
        "escalation" => 3,
        "mirexis" => 4,
        _ => u8::MAX,
    }
}
