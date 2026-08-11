//! Persistent, bounded tradeoff scars earned through tactical incapacitation.

use crate::campaign::CharacterRecord;
use crate::data::UnitDef;
use serde::{Deserialize, Serialize};

const MAX_TRAUMAS: usize = 2;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraumaRecord {
    pub id: String,
    pub name: String,
    pub effect: String,
}

struct TraumaDefinition {
    id: &'static str,
    name: &'static str,
    effect: &'static str,
}

const TRAUMAS: [TraumaDefinition; 3] = [
    TraumaDefinition {
        id: "clouded_eye",
        name: "Clouded Eye",
        effect: "-8 ACC · +1 DMG",
    },
    TraumaDefinition {
        id: "reinforced_ribs",
        name: "Reinforced Ribs",
        effect: "+1 ARMOUR · -1 MOVE",
    },
    TraumaDefinition {
        id: "mire_reflex",
        name: "Mire Reflex",
        effect: "+1 MOVE · -2 VITALS",
    },
];

pub(crate) fn record_incapacitation(character: &mut CharacterRecord, operation: u32) -> bool {
    if character.traumas.len() >= MAX_TRAUMAS {
        return false;
    }
    let seed = character
        .id
        .bytes()
        .fold(operation as usize, |sum, byte| sum + usize::from(byte));
    let definition = (0..TRAUMAS.len())
        .map(|offset| &TRAUMAS[(seed + offset) % TRAUMAS.len()])
        .find(|definition| {
            !character
                .traumas
                .iter()
                .any(|trauma| trauma.id == definition.id)
        });
    let Some(definition) = definition else {
        return false;
    };
    character.traumas.push(TraumaRecord {
        id: definition.id.to_owned(),
        name: definition.name.to_owned(),
        effect: definition.effect.to_owned(),
    });
    true
}

pub(crate) fn apply_deployment_traits(unit: &mut UnitDef, traumas: &[TraumaRecord]) {
    for trauma in traumas {
        match trauma.id.as_str() {
            "clouded_eye" => {
                unit.accuracy -= 8;
                unit.weapon_damage += 1;
            }
            "reinforced_ribs" => {
                unit.armour += 1;
                unit.move_range = unit.move_range.saturating_sub(1).max(1);
            }
            "mire_reflex" => {
                unit.move_range = unit.move_range.saturating_add(1);
                unit.max_health = (unit.max_health - 2).max(1);
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests;
