//! Persistent bonds earned through shared victories and character events.

use crate::campaign::CampaignState;
use crate::data::{GameData, Team, UnitDef};
use serde::{Deserialize, Serialize};

const TRUSTED_BOND: u8 = 3;
const BONDED_BOND: u8 = 5;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationshipRecord {
    pub first_id: String,
    pub second_id: String,
    pub bond: u8,
    pub shared_victories: u32,
}

impl CampaignState {
    pub fn strengthen_shared_victory(&mut self, deployed_ids: &[String]) {
        for first in 0..deployed_ids.len() {
            for second in (first + 1)..deployed_ids.len() {
                self.strengthen_relationship(&deployed_ids[first], &deployed_ids[second], 1, true);
            }
        }
    }

    pub fn strengthen_event_participants(&mut self, participant_ids: &[String]) {
        for first in 0..participant_ids.len() {
            for second in (first + 1)..participant_ids.len() {
                self.strengthen_relationship(
                    &participant_ids[first],
                    &participant_ids[second],
                    2,
                    false,
                );
            }
        }
    }

    pub fn relationship_summaries(&self, character_id: &str) -> Vec<String> {
        let mut records = self
            .relationships
            .iter()
            .filter_map(|relationship| {
                let partner_id = if relationship.first_id == character_id {
                    &relationship.second_id
                } else if relationship.second_id == character_id {
                    &relationship.first_id
                } else {
                    return None;
                };
                let partner = self.roster.iter().find(|record| &record.id == partner_id)?;
                Some((relationship.bond, partner.name.as_str()))
            })
            .collect::<Vec<_>>();
        records.sort_by(|left, right| right.0.cmp(&left.0).then_with(|| left.1.cmp(right.1)));
        records
            .into_iter()
            .map(|(bond, name)| format!("{} // {} · BOND {}", bond_name(bond), name, bond))
            .collect()
    }

    pub fn deployed_bond_name(&self, character_id: &str) -> Option<&'static str> {
        let character_is_deployed = self.roster.iter().any(|character| {
            character.id == character_id
                && character.availability == crate::campaign::Availability::Ready
                && character.deployment_selected
        });
        if !character_is_deployed {
            return None;
        }
        let strongest = self
            .relationships
            .iter()
            .filter(|relationship| relationship.bond >= TRUSTED_BOND)
            .filter(|relationship| {
                relationship.first_id == character_id || relationship.second_id == character_id
            })
            .filter(|relationship| {
                let partner_id = if relationship.first_id == character_id {
                    &relationship.second_id
                } else {
                    &relationship.first_id
                };
                self.roster.iter().any(|character| {
                    &character.id == partner_id
                        && character.availability == crate::campaign::Availability::Ready
                        && character.deployment_selected
                })
            })
            .map(|relationship| relationship.bond)
            .max()?;
        Some(bond_name(strongest))
    }

    fn strengthen_relationship(
        &mut self,
        first_id: &str,
        second_id: &str,
        amount: u8,
        shared_victory: bool,
    ) {
        if first_id == second_id {
            return;
        }
        let (first_id, second_id) = ordered_pair(first_id, second_id);
        let relationship = if let Some(index) = self
            .relationships
            .iter()
            .position(|record| record.first_id == first_id && record.second_id == second_id)
        {
            &mut self.relationships[index]
        } else {
            self.relationships.push(RelationshipRecord {
                first_id: first_id.to_owned(),
                second_id: second_id.to_owned(),
                bond: 0,
                shared_victories: 0,
            });
            self.relationships.last_mut().unwrap()
        };
        relationship.bond = relationship.bond.saturating_add(amount).min(BONDED_BOND);
        if shared_victory {
            relationship.shared_victories = relationship.shared_victories.saturating_add(1);
        }
    }
}

pub(crate) fn apply_deployment_bonuses(
    deployment: &mut [UnitDef],
    relationships: &[RelationshipRecord],
) {
    let deployed_ids = deployment
        .iter()
        .filter(|unit| unit.team == Team::Colony)
        .map(|unit| unit.id.clone())
        .collect::<Vec<_>>();
    for unit in deployment
        .iter_mut()
        .filter(|unit| unit.team == Team::Colony)
    {
        let strongest = relationships
            .iter()
            .filter(|relationship| {
                relationship.first_id == unit.id || relationship.second_id == unit.id
            })
            .filter(|relationship| {
                let partner = if relationship.first_id == unit.id {
                    relationship.second_id.as_str()
                } else {
                    relationship.first_id.as_str()
                };
                deployed_ids.iter().any(|id| id == partner)
            })
            .map(|relationship| relationship.bond)
            .max()
            .unwrap_or(0);
        if strongest >= TRUSTED_BOND {
            unit.accuracy += 2;
        }
        if strongest >= BONDED_BOND {
            unit.armour += 1;
        }
    }
}

pub(crate) fn validate_event_definitions(data: &GameData) -> Result<(), String> {
    for event in &data.campaign.events {
        if event.participants.len() < 2 {
            return Err(format!(
                "Campaign event {} cannot build a relationship",
                event.id
            ));
        }
        if !event.attention_faction.is_empty()
            && !data
                .campaign
                .factions
                .iter()
                .any(|faction| faction.id == event.attention_faction)
        {
            return Err(format!(
                "Campaign event {} references missing attention faction {}",
                event.id, event.attention_faction
            ));
        }
        if !event.required_protocol.is_empty()
            && !data
                .campaign
                .contact_protocols
                .iter()
                .any(|protocol| protocol.id == event.required_protocol)
        {
            return Err(format!(
                "Campaign event {} references missing Contact protocol {}",
                event.id, event.required_protocol
            ));
        }
        for participant in &event.participants {
            if !data
                .characters
                .iter()
                .any(|character| &character.id == participant)
            {
                return Err(format!(
                    "Campaign event {} references missing character {}",
                    event.id, participant
                ));
            }
        }
        if !event.participants.contains(&event.legacy_character_id) {
            return Err(format!(
                "Campaign event {} gives a legacy to non-participant {}",
                event.id, event.legacy_character_id
            ));
        }
        if !matches!(
            event.legacy_stat.as_str(),
            "accuracy" | "armour" | "health" | "movement" | "damage"
        ) || !(-127..=127).contains(&event.legacy_amount)
            || event.legacy_amount == 0
        {
            return Err(format!(
                "Campaign event {} has an invalid legacy modifier",
                event.id
            ));
        }
    }
    Ok(())
}

fn ordered_pair<'a>(first: &'a str, second: &'a str) -> (&'a str, &'a str) {
    if first <= second {
        (first, second)
    } else {
        (second, first)
    }
}

fn bond_name(bond: u8) -> &'static str {
    match bond {
        BONDED_BOND.. => "BONDED",
        TRUSTED_BOND.. => "TRUSTED",
        2 => "FAMILIAR",
        _ => "TESTED",
    }
}

#[cfg(test)]
mod tests;
