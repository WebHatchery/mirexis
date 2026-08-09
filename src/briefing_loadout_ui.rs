//! Derived colonist combat profile beside briefing squad selection.

use crate::campaign::CampaignState;
use crate::data::GameData;
use macroquad::prelude::*;
use macroquad_toolkit::prelude::{dark, TextStyle};

pub(crate) fn draw(campaign: &CampaignState, data: &GameData, origin: Vec2) {
    let Some(character) = campaign.selected_character() else {
        return;
    };
    let Some(unit) = campaign.derived_character_unit(&character.id, data) else {
        return;
    };
    let class_name = data
        .classes
        .iter()
        .find(|class| class.id == character.active_class)
        .map_or(character.active_class.as_str(), |class| class.name.as_str());
    let weapon = character
        .equipment_ids
        .iter()
        .filter_map(|id| data.equipment.iter().find(|item| &item.id == id))
        .find(|item| item.slot == "primary")
        .map_or("Unarmed", |item| item.name.as_str());
    let scars = if character.traumas.is_empty() {
        "NONE".to_owned()
    } else {
        character
            .traumas
            .iter()
            .map(|trauma| trauma.name.to_uppercase())
            .collect::<Vec<_>>()
            .join(" · ")
    };
    draw_text_ex(
        "SELECTED LOADOUT",
        origin.x,
        origin.y,
        TextStyle::new(15.0, dark::ACCENT).params(),
    );
    for (index, line) in [
        format!(
            "{} // {}",
            character.name.to_uppercase(),
            class_name.to_uppercase()
        ),
        format!(
            "{} // {}",
            unit.mutation.to_uppercase(),
            weapon.to_uppercase()
        ),
        format!(
            "HP {} · ACC {} · ARM {} · MOV {}",
            unit.max_health, unit.accuracy, unit.armour, unit.move_range
        ),
        format!(
            "WEAPON {} DMG · R{} · {} AP",
            unit.weapon_damage, unit.weapon_range, unit.weapon_ap_cost
        ),
        format!("SCARS // {}", scars),
    ]
    .iter()
    .enumerate()
    {
        draw_text_ex(
            line,
            origin.x,
            origin.y + 20.0 + index as f32 * 17.0,
            TextStyle::new(11.5, dark::TEXT_DIM).params(),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selected_loadout_uses_the_same_derived_profile_as_deployment() {
        let data = GameData::load().unwrap();
        let campaign = CampaignState::new(&data);
        let selected = campaign.selected_character().unwrap();
        let unit = campaign
            .derived_character_unit(&selected.id, &data)
            .unwrap();
        let deployed = campaign
            .deployment_roster(&data, &data.mission)
            .into_iter()
            .find(|candidate| candidate.id == selected.id)
            .unwrap();

        assert_eq!(unit.max_health, deployed.max_health);
        assert_eq!(unit.weapon_damage, deployed.weapon_damage);
        assert_eq!(unit.equipment_ids, deployed.equipment_ids);
    }
}
