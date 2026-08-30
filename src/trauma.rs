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
    apply_deployment_traits_with_options(unit, traumas, false);
}

pub(crate) fn apply_deployment_traits_with_options(
    unit: &mut UnitDef,
    traumas: &[TraumaRecord],
    soften_tradeoffs: bool,
) {
    for trauma in traumas {
        match trauma.id.as_str() {
            "clouded_eye" => {
                unit.accuracy -= if soften_tradeoffs { 4 } else { 8 };
                unit.weapon_damage += 1;
            }
            "reinforced_ribs" => {
                unit.armour += 1;
                if !soften_tradeoffs {
                    unit.move_range = unit.move_range.saturating_sub(1).max(1);
                }
            }
            "mire_reflex" => {
                unit.move_range = unit.move_range.saturating_add(1);
                unit.max_health = (unit.max_health - if soften_tradeoffs { 1 } else { 2 }).max(1);
            }
            _ => {}
        }
    }
}

pub(crate) fn effective_effect(trauma: &TraumaRecord, soften_tradeoffs: bool) -> String {
    if !soften_tradeoffs {
        return trauma.effect.clone();
    }
    match trauma.id.as_str() {
        "clouded_eye" => "-4 ACC · +1 DMG".to_owned(),
        "reinforced_ribs" => "+1 ARMOUR · NO MOVE LOSS".to_owned(),
        "mire_reflex" => "+1 MOVE · -1 VITALS".to_owned(),
        _ => trauma.effect.clone(),
    }
}

#[cfg(test)]
mod tests;
