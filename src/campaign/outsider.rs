//! Route-specific recruitment rules and Waystation outsider arcs.

use super::{CampaignState, CharacterLegacy, CharacterRecord, OutsiderArcState};
use crate::colony::BuildingKind;
use crate::data::{CharacterDef, GameData};

const OUTSIDER_IDS: [&str; 2] = ["veya_orn", "sedge"];

#[derive(Debug, Clone, Copy)]
pub(crate) struct OutsiderChoice {
    pub(crate) id: &'static str,
    pub(crate) label: &'static str,
    pub(crate) description: &'static str,
    pub(crate) materials_cost: i32,
    pub(crate) food_cost: i32,
    pub(crate) power_cost: i32,
    pub(crate) biomass_cost: i32,
    pub(crate) attention_change: i32,
    pub(crate) relationship_partner: &'static str,
    pub(crate) disagreement: bool,
    pub(crate) legacy_name: &'static str,
    pub(crate) legacy_stat: &'static str,
    pub(crate) legacy_amount: i32,
}

fn recruitment_resource(definition: &CharacterDef) -> &str {
    if definition.recruitment_resource.is_empty() {
        "materials"
    } else {
        &definition.recruitment_resource
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct OutsiderBeat {
    pub(crate) outsider_id: &'static str,
    pub(crate) outsider_name: &'static str,
    pub(crate) attention_faction: &'static str,
    pub(crate) stage: u8,
    pub(crate) title: &'static str,
    pub(crate) description: &'static str,
    pub(crate) choices: [OutsiderChoice; 2],
}

pub(crate) fn outsider_beat(outsider_id: &str, stage: u8) -> Option<OutsiderBeat> {
    match (outsider_id, stage) {
        ("veya_orn", 0) => Some(OutsiderBeat {
            outsider_id: "veya_orn",
            outsider_name: "Veya Orn",
            attention_faction: "directorate",
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
                    biomass_cost: 0,
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
                    biomass_cost: 0,
                    attention_change: 3,
                    relationship_partner: "kira_voss",
                    disagreement: true,
                    legacy_name: "Open Ledger",
                    legacy_stat: "movement",
                    legacy_amount: 1,
                },
            ],
        }),
        ("veya_orn", 1) => Some(OutsiderBeat {
            outsider_id: "veya_orn",
            outsider_name: "Veya Orn",
            attention_faction: "directorate",
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
                    biomass_cost: 0,
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
                    biomass_cost: 0,
                    attention_change: 2,
                    relationship_partner: "kira_voss",
                    disagreement: true,
                    legacy_name: "Walking Range",
                    legacy_stat: "damage",
                    legacy_amount: 1,
                },
            ],
        }),
        ("veya_orn", 2) => Some(OutsiderBeat {
            outsider_id: "veya_orn",
            outsider_name: "Veya Orn",
            attention_faction: "directorate",
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
                    biomass_cost: 0,
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
                    biomass_cost: 0,
                    attention_change: 1,
                    relationship_partner: "sol_cairn",
                    disagreement: false,
                    legacy_name: "Shared Key",
                    legacy_stat: "accuracy",
                    legacy_amount: 1,
                },
            ],
        }),
        ("sedge", 0) => Some(OutsiderBeat {
            outsider_id: "sedge",
            outsider_name: "Sedge",
            attention_faction: "brood",
            stage,
            title: "THE MAP IS NOT A SPECIMEN",
            description: "Sedge brought a route-map in their body, and the Gene Lab wants a sample before the colony lets them rest.",
            choices: [
                OutsiderChoice {
                    id: "sedge_keep_map_personal",
                    label: "KEEP THE MAP PERSONAL",
                    description: "Spend food to give Sedge time before the lab asks. Nadi learns that a boundary can be a medical fact.",
                    materials_cost: 0,
                    food_cost: 2,
                    power_cost: 0,
                    biomass_cost: 0,
                    attention_change: -4,
                    relationship_partner: "nadi_vale",
                    disagreement: true,
                    legacy_name: "Named Boundary",
                    legacy_stat: "health",
                    legacy_amount: 1,
                },
                OutsiderChoice {
                    id: "sedge_trade_the_route",
                    label: "TRADE THE ROUTE",
                    description: "Spend biomass to let Sol model the map. The colony gains a route, and the Brood learns it was copied.",
                    materials_cost: 0,
                    food_cost: 0,
                    power_cost: 0,
                    biomass_cost: 2,
                    attention_change: 3,
                    relationship_partner: "sol_cairn",
                    disagreement: true,
                    legacy_name: "Open Route",
                    legacy_stat: "movement",
                    legacy_amount: 1,
                },
            ],
        }),
        ("sedge", 1) => Some(OutsiderBeat {
            outsider_id: "sedge",
            outsider_name: "Sedge",
            attention_faction: "brood",
            stage,
            title: "WHAT THE MARSH OWES",
            description: "The first operation proved Sedge can carry a route through danger. The next question is whether the colony can carry the responsibility back.",
            choices: [
                OutsiderChoice {
                    id: "sedge_draw_a_boundary",
                    label: "DRAW A BOUNDARY",
                    description: "Spend materials on a protected survey. Mara makes the route safer without claiming it belongs to the colony.",
                    materials_cost: 10,
                    food_cost: 0,
                    power_cost: 0,
                    biomass_cost: 0,
                    attention_change: -2,
                    relationship_partner: "mara_venn",
                    disagreement: true,
                    legacy_name: "Protected Route",
                    legacy_stat: "armour",
                    legacy_amount: 1,
                },
                OutsiderChoice {
                    id: "sedge_follow_the_pulse",
                    label: "FOLLOW THE PULSE",
                    description: "Spend biomass to let Ilya listen for the route's living signal instead of cutting it out of Sedge.",
                    materials_cost: 0,
                    food_cost: 0,
                    power_cost: 0,
                    biomass_cost: 2,
                    attention_change: 2,
                    relationship_partner: "ilya_reed",
                    disagreement: true,
                    legacy_name: "Living Signal",
                    legacy_stat: "damage",
                    legacy_amount: 1,
                },
            ],
        }),
        ("sedge", 2) => Some(OutsiderBeat {
            outsider_id: "sedge",
            outsider_name: "Sedge",
            attention_faction: "brood",
            stage,
            title: "A NAME THAT WALKS",
            description: "Sedge has made the Gene Lab useful without letting it own the story. The final choice is who gets to carry the route into the future.",
            choices: [
                OutsiderChoice {
                    id: "sedge_keep_family_name",
                    label: "KEEP THE FAMILY NAME",
                    description: "Keep the route with Sedge's family record and make the Gene Lab answerable to the people it studies.",
                    materials_cost: 0,
                    food_cost: 0,
                    power_cost: 0,
                    biomass_cost: 0,
                    attention_change: -2,
                    relationship_partner: "nadi_vale",
                    disagreement: false,
                    legacy_name: "Family Route",
                    legacy_stat: "health",
                    legacy_amount: 1,
                },
                OutsiderChoice {
                    id: "sedge_publish_the_route",
                    label: "PUBLISH THE ROUTE",
                    description: "Give every colony map the route and accept that shared knowledge can also become a beacon.",
                    materials_cost: 0,
                    food_cost: 0,
                    power_cost: 0,
                    biomass_cost: 0,
                    attention_change: 1,
                    relationship_partner: "kira_voss",
                    disagreement: false,
                    legacy_name: "Published Route",
                    legacy_stat: "accuracy",
                    legacy_amount: 1,
                },
            ],
        }),
        _ => None,
    }
}

impl CampaignState {
    pub(crate) fn waystation_unlocked(&self) -> bool {
        self.strategy.contact_protocol_id == "directorate_requisition"
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
        if definition.id == "sedge" {
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

    pub(crate) fn recruitment_resource_amount(&self, resource: &str) -> i32 {
        match resource {
            "materials" => self.colony.resources.materials,
            "power" => self.colony.resources.power,
            "food" => self.colony.resources.food,
            "biomass" => self.colony.resources.biomass,
            _ => 0,
        }
    }

    fn spend_recruitment_resource(&mut self, resource: &str, amount: i32) {
        match resource {
            "materials" => self.colony.resources.materials -= amount,
            "power" => self.colony.resources.power -= amount,
            "food" => self.colony.resources.food -= amount,
            "biomass" => self.colony.resources.biomass -= amount,
            _ => {}
        }
    }

    fn recruitment_route_matches(&self, definition: &CharacterDef) -> bool {
        if definition.recruitment_phase.is_empty() {
            self.strategy.contact_protocol_id == definition.recruitment_protocol
        } else {
            self.strategy.phase_id == definition.recruitment_phase && self.strategy.contact_complete
        }
    }

    fn outsider_arc_state(&self, outsider_id: &str) -> OutsiderArcState {
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

    fn set_outsider_arc_state(&mut self, outsider_id: &str, state: OutsiderArcState) {
        if outsider_id == "veya_orn" {
            self.outsider_arc_stage = state.stage;
            self.outsider_disagreements = state.disagreements;
            self.outsider_final_choice = state.final_choice.clone();
        }
        self.outsider_arc_states
            .insert(outsider_id.to_owned(), state);
    }

    pub(crate) fn outsider_arc_beat(&self) -> Option<OutsiderBeat> {
        for outsider_id in OUTSIDER_IDS {
            if !self
                .roster
                .iter()
                .any(|character| character.id == outsider_id)
            {
                continue;
            }
            let state = self.outsider_arc_state(outsider_id);
            let Some(beat) = outsider_beat(outsider_id, state.stage) else {
                continue;
            };
            if self.operations_completed >= u32::from(state.stage) {
                return Some(beat);
            }
        }
        None
    }

    pub(crate) fn outsider_arc_disagreements(&self) -> u8 {
        self.outsider_arc_beat()
            .map(|beat| self.outsider_arc_state(beat.outsider_id).disagreements)
            .unwrap_or_default()
    }

    pub fn outsider_arc_available(&self) -> bool {
        self.outsider_arc_beat().is_some()
    }

    pub fn resolve_outsider_beat(&mut self, stage: u8, choice_id: &str) -> Result<String, String> {
        let beat = self
            .outsider_arc_beat()
            .ok_or_else(|| "No recruited outsider has a current conversation".to_owned())?;
        if beat.stage != stage {
            return Err("That outsider conversation is no longer current".to_owned());
        }
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
        let mut arc_state = self.outsider_arc_state(beat.outsider_id);
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
        self.set_outsider_arc_state(beat.outsider_id, arc_state);
        Ok(format!("{} // {}", beat.title, choice.label))
    }
}
