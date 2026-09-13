//! Mutation traits and equipment costs shared by campaign and roster views.

use super::CharacterRecord;
use crate::data::{GameData, MutationDef};
use std::collections::BTreeMap;

pub fn equipment_cost(slot: &str) -> u32 {
    match slot {
        "primary" => 30,
        "armour" => 25,
        "tool" | "module" => 20,
        _ => 25,
    }
}

// This helper is part of the mutation validation seam and is intentionally
// available to authored-content tests before a live campaign consumes it.
#[allow(dead_code)]
pub fn derived_mutation_traits(
    character: &CharacterRecord,
    data: &GameData,
) -> BTreeMap<String, i32> {
    derived_mutation_traits_with_options(character, data, false)
}

pub fn derived_mutation_traits_with_options(
    character: &CharacterRecord,
    data: &GameData,
    suppress_evolution_complications: bool,
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
            for modifier in evolution.gift.iter().chain(
                evolution
                    .complication
                    .iter()
                    .filter(|_| !suppress_evolution_complications),
            ) {
                *traits.entry(modifier.stat.clone()).or_default() += modifier.amount;
            }
        }
    }
    traits
}

pub fn apply_mutation(mutation: &MutationDef, traits: &mut BTreeMap<String, i32>) {
    for modifier in mutation.gift.iter().chain(&mutation.complication) {
        *traits.entry(modifier.stat.clone()).or_default() += modifier.amount;
    }
}
