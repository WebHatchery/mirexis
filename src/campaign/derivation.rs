//! Build a tactical unit from persistent character state and authored data.

use super::{derived_mutation_traits_with_options, CharacterRecord};
use crate::data::{GameData, UnitDef};

#[allow(dead_code)]
pub(crate) fn derive_unit(base: &UnitDef, character: &CharacterRecord, data: &GameData) -> UnitDef {
    derive_unit_with_evolution_options(base, character, data, false)
}

pub(crate) fn derive_unit_with_evolution_options(
    base: &UnitDef,
    character: &CharacterRecord,
    data: &GameData,
    suppress_evolution_complications: bool,
) -> UnitDef {
    let mut unit = base.clone();
    unit.equipment_ids = character.equipment_ids.clone();
    unit.learned_skills = character.learned_skills.clone();
    unit.active_skills = character.active_skills.clone();
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
    if let Some(definition) = data
        .characters
        .iter()
        .find(|definition| definition.id == character.id)
    {
        unit.accuracy += definition.origin_accuracy_bonus;
        unit.move_range = add_signed(unit.move_range, definition.origin_move_bonus);
    }
    let traits =
        derived_mutation_traits_with_options(character, data, suppress_evolution_complications);
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
