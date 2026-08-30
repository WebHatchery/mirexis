//! Campaign and tactical integration for technique actions.

use super::{Game, TacticalTargeting};
use crate::ui::UiAction;

impl Game {
    pub(super) fn apply_skill_action(&mut self, action: &UiAction, before_events: usize) -> bool {
        let handled = match action {
            UiAction::LearnSelectedSkill(skill_id) => {
                let character_id = self.campaign.selected_character_id.clone();
                match self
                    .campaign
                    .learn_skill(&character_id, skill_id, &self.data)
                {
                    Ok(name) => {
                        self.notifications
                            .success(format!("Technique learned · {}", name));
                        self.autosave_campaign_only("Technique learning autosaved");
                    }
                    Err(error) => self.notifications.warning(error),
                }
                true
            }
            UiAction::ToggleSelectedSkill(skill_id) => {
                let character_id = self.campaign.selected_character_id.clone();
                match self
                    .campaign
                    .toggle_skill(&character_id, skill_id, &self.data)
                {
                    Ok(active) => {
                        self.notifications.info(if active {
                            "Technique equipped"
                        } else {
                            "Technique removed from loadout"
                        });
                        self.autosave_campaign_only("Technique loadout autosaved");
                    }
                    Err(error) => self.notifications.warning(error),
                }
                true
            }
            UiAction::ActivateSkill(skill_id) => {
                match self.session.activate_selected_skill(skill_id) {
                    Ok(events) => self.first_hour_ability_success(events, "Technique activated"),
                    Err(_) => self.notifications.warning("Technique is unavailable"),
                }
                true
            }
            UiAction::ArmSkill(skill_id) => {
                if let Some(unit_id) = self.session.tactical.selected_unit.clone() {
                    self.targeting = Some(TacticalTargeting::Skill {
                        unit_id,
                        skill_id: skill_id.clone(),
                    });
                    self.notifications
                        .info("Choose a highlighted technique target");
                }
                true
            }
            UiAction::UseSkillOn(target_id) => {
                let result = match self.targeting.take() {
                    Some(TacticalTargeting::Skill { unit_id, skill_id }) => self
                        .session
                        .activate_skill_on(&unit_id, &skill_id, target_id)
                        .map_err(|_| ()),
                    _ => Err(()),
                };
                match result {
                    Ok(events) => self.first_hour_ability_success(events, "Technique activated"),
                    Err(()) => self
                        .notifications
                        .warning("Technique target is no longer valid"),
                }
                true
            }
            UiAction::UseSkillOnTile(tile) => {
                let result = match self.targeting.take() {
                    Some(TacticalTargeting::Skill { unit_id, skill_id }) => self
                        .session
                        .activate_skill_on_tile(&unit_id, &skill_id, *tile)
                        .map_err(|_| ()),
                    _ => Err(()),
                };
                match result {
                    Ok(events) => self.first_hour_ability_success(events, "Technique activated"),
                    Err(()) => self
                        .notifications
                        .warning("Technique tile is no longer valid"),
                }
                true
            }
            _ => false,
        };
        if handled {
            self.finish_action_audio(action, before_events);
            self.enter_debrief_if_finished();
        }
        handled
    }
}
