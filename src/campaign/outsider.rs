//! Contact-era recruitment rules and the Waystation outsider arc.

use super::{CampaignState, CharacterLegacy, CharacterRecord};
use crate::colony::BuildingKind;
use crate::data::GameData;

#[derive(Debug, Clone, Copy)]
pub(crate) struct OutsiderChoice {
    pub(crate) id: &'static str,
    pub(crate) label: &'static str,
    pub(crate) description: &'static str,
    pub(crate) materials_cost: i32,
    pub(crate) food_cost: i32,
    pub(crate) power_cost: i32,
    pub(crate) attention_change: i32,
    pub(crate) relationship_partner: &'static str,
    pub(crate) disagreement: bool,
    pub(crate) legacy_name: &'static str,
    pub(crate) legacy_stat: &'static str,
    pub(crate) legacy_amount: i32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct OutsiderBeat {
    pub(crate) stage: u8,
    pub(crate) title: &'static str,
    pub(crate) description: &'static str,
    pub(crate) choices: [OutsiderChoice; 2],
}

pub(crate) fn outsider_beat(stage: u8) -> Option<OutsiderBeat> {
    match stage {
        0 => Some(OutsiderBeat {
            stage,
            title: "TERMS FOR A BORROWED SHELTER",
            description: "Veya will not sleep under a Directorate roof, but the cipher still opens doors that the refuge cannot afford to ignore.",
            choices: [
                OutsiderChoice {
                    id: "shelter_cipher",
                    label: "SHELTER THE CIPHER",
                    description: "Spend food to let Veya hide the useful parts. Mara learns the code can protect people.",
                    materials_cost: 0,
                    food_cost: 2,
                    power_cost: 0,
                    attention_change: -4,
                    relationship_partner: "mara_venn",
                    disagreement: true,
                    legacy_name: "Cipher Shelter",
                    legacy_stat: "accuracy",
                    legacy_amount: 2,
                },
                OutsiderChoice {
                    id: "open_ledger",
                    label: "OPEN THE LEDGER",
                    description: "Spend food to expose the whole route. Kira gets the truth, and the Directorate gets a signal.",
                    materials_cost: 0,
                    food_cost: 2,
                    power_cost: 0,
                    attention_change: 3,
                    relationship_partner: "kira_voss",
                    disagreement: true,
                    legacy_name: "Open Ledger",
                    legacy_stat: "movement",
                    legacy_amount: 1,
                },
            ],
        }),
        1 => Some(OutsiderBeat {
            stage,
            title: "THE RANGE BETWEEN THEM",
            description: "The first operation proved Veya can read a Directorate line. The second question is who gets to decide where that line ends.",
            choices: [
                OutsiderChoice {
                    id: "mara_tests_range",
                    label: "MARA TESTS THE RANGE",
                    description: "Spend materials on a hard field test. Veya gains armour through a partnership built on challenge.",
                    materials_cost: 12,
                    food_cost: 0,
                    power_cost: 0,
                    attention_change: -2,
                    relationship_partner: "mara_venn",
                    disagreement: true,
                    legacy_name: "Measured Range",
                    legacy_stat: "armour",
                    legacy_amount: 1,
                },
                OutsiderChoice {
                    id: "kira_walks_range",
                    label: "KIRA WALKS THE RANGE",
                    description: "Spend materials on a quiet survey. Veya gains damage through a partnership built on shared risk.",
                    materials_cost: 12,
                    food_cost: 0,
                    power_cost: 0,
                    attention_change: 2,
                    relationship_partner: "kira_voss",
                    disagreement: true,
                    legacy_name: "Walking Range",
                    legacy_stat: "damage",
                    legacy_amount: 1,
                },
            ],
        }),
        2 => Some(OutsiderBeat {
            stage,
            title: "WHO KEEPS THE KEY",
            description: "Veya has made the Waystation useful. The final decision is whether the cipher belongs to one person or to everyone who stayed to hear the argument.",
            choices: [
                OutsiderChoice {
                    id: "stay_on_line",
                    label: "STAY ON THE LINE",
                    description: "Keep the cipher close and make the Waystation answerable to its witnesses.",
                    materials_cost: 0,
                    food_cost: 0,
                    power_cost: 0,
                    attention_change: -2,
                    relationship_partner: "nadi_vale",
                    disagreement: false,
                    legacy_name: "Witnessed Line",
                    legacy_stat: "health",
                    legacy_amount: 1,
                },
                OutsiderChoice {
                    id: "share_the_key",
                    label: "SHARE THE KEY",
                    description: "Give the colony the whole cipher and accept that every open door can be opened from outside.",
                    materials_cost: 0,
                    food_cost: 0,
                    power_cost: 0,
                    attention_change: 1,
                    relationship_partner: "sol_cairn",
                    disagreement: false,
                    legacy_name: "Shared Key",
                    legacy_stat: "accuracy",
                    legacy_amount: 1,
                },
            ],
        }),
        _ => None,
    }
}

impl CampaignState {
    pub fn outsider_recruit_available(&self, data: &GameData) -> bool {
        let Some(definition) = data
            .characters
            .iter()
            .find(|definition| !definition.recruitment_protocol.is_empty())
        else {
            return false;
        };
        self.colony.has_facility(BuildingKind::Waystation)
            && self.strategy.contact_protocol_id == definition.recruitment_protocol
            && !self
                .roster
                .iter()
                .any(|character| character.id == definition.id)
    }

    pub fn recruit_outsider(&mut self, data: &GameData) -> Result<String, String> {
        let definition = data
            .characters
            .iter()
            .find(|definition| !definition.recruitment_protocol.is_empty())
            .cloned()
            .ok_or_else(|| "No route-exclusive outsider is authored".to_owned())?;
        if self
            .roster
            .iter()
            .any(|character| character.id == definition.id)
        {
            return Err(format!(
                "{} is already in the colony roster",
                definition.name
            ));
        }
        if self.strategy.contact_protocol_id != definition.recruitment_protocol {
            return Err("This Waystation is tuned to another Contact route".to_owned());
        }
        if !self.colony.has_facility(BuildingKind::Waystation) {
            return Err("An operational Waystation is required".to_owned());
        }
        if self.colony.resources.materials < definition.recruitment_cost {
            return Err(format!(
                "Recruitment requires {} materials",
                definition.recruitment_cost
            ));
        }
        self.colony.resources.materials -= definition.recruitment_cost;
        let mut character = CharacterRecord::from_def(&definition);
        character.deployment_selected = false;
        let name = character.name.clone();
        self.roster.push(character);
        Ok(name)
    }

    pub fn outsider_arc_available(&self) -> bool {
        self.roster
            .iter()
            .any(|character| character.id == "veya_orn")
            && outsider_beat(self.outsider_arc_stage).is_some()
            && self.operations_completed >= u32::from(self.outsider_arc_stage)
    }

    pub fn resolve_outsider_beat(&mut self, stage: u8, choice_id: &str) -> Result<String, String> {
        if !self
            .roster
            .iter()
            .any(|character| character.id == "veya_orn")
        {
            return Err("Veya Orn is not in the colony roster".to_owned());
        }
        if self.outsider_arc_stage != stage {
            return Err("That outsider conversation is no longer current".to_owned());
        }
        if self.operations_completed < u32::from(stage) {
            return Err("The next outsider conversation needs another operation".to_owned());
        }
        let beat = outsider_beat(stage).ok_or_else(|| "Outsider arc is complete".to_owned())?;
        let choice = beat
            .choices
            .iter()
            .find(|choice| choice.id == choice_id)
            .copied()
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
        self.colony.resources.materials -= choice.materials_cost;
        self.colony.resources.food -= choice.food_cost;
        self.colony.resources.power -= choice.power_cost;
        if let Some(faction) = self
            .strategy
            .factions
            .iter_mut()
            .find(|faction| faction.id == "directorate")
        {
            faction.attention = (faction.attention + choice.attention_change).clamp(0, 100);
        }
        if choice.disagreement {
            self.outsider_disagreements = self.outsider_disagreements.saturating_add(1);
        }
        let participants = [
            "veya_orn".to_owned(),
            choice.relationship_partner.to_owned(),
        ];
        self.strengthen_event_participants(&participants);
        if choice.legacy_amount != 0 {
            let character = self
                .roster
                .iter_mut()
                .find(|character| character.id == "veya_orn")
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
            self.outsider_final_choice = choice.id.to_owned();
        }
        self.outsider_arc_stage = self.outsider_arc_stage.saturating_add(1);
        Ok(format!("{} // {}", beat.title, choice.label))
    }
}
