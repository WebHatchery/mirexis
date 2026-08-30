//! Briefing squad rows, formation choice, and deployment controls.

use crate::campaign::{Availability, CampaignState, SQUAD_LIMIT};
use crate::data::GameData;
use crate::formation::FormationKind;
use crate::ui::UiAction;
use crate::ui_widgets::{button, button_with_state};
use crate::visual_assets::VisualCatalog;
use macroquad::prelude::*;
use macroquad_toolkit::assets::AssetManager;
use macroquad_toolkit::prelude::{dark, TextStyle};

fn deployment_row_layout(roster_len: usize) -> (f32, f32) {
    if roster_len <= 5 {
        return (32.0, 29.0);
    }
    let row_step = 191.0 / roster_len as f32;
    (row_step, (row_step - 2.0).max(20.0))
}

pub(crate) fn draw(
    campaign: &CampaignState,
    data: &GameData,
    formation: FormationKind,
    assets: &AssetManager,
    visuals: &VisualCatalog,
    mouse: Vec2,
    actions: &mut Vec<UiAction>,
) {
    draw_text_ex(
        format!(
            "DEPLOYMENT // {}/{} SELECTED // SUPPLY {} FOOD",
            campaign.selected_squad_count(),
            SQUAD_LIMIT,
            campaign.deployment_food_cost(data)
        ),
        200.0,
        356.0,
        TextStyle::new(15.0, dark::ACCENT).params(),
    );
    let (row_step, row_height) = deployment_row_layout(campaign.roster.len());
    for (index, character) in campaign.roster.iter().enumerate() {
        let class_name = data
            .classes
            .iter()
            .find(|class| class.id == character.active_class)
            .map_or(character.active_class.as_str(), |class| class.name.as_str());
        let state = if character.availability != Availability::Ready {
            "RECOVERING"
        } else if character.deployment_selected {
            "DEPLOY"
        } else {
            "RESERVE"
        };
        let bond = campaign
            .deployed_bond_name(&character.id)
            .map_or(String::new(), |name| format!(" · {}", name));
        let label = format!(
            "{} // {} · LV{} · XP{}{}",
            state, class_name, character.level, character.experience, bond
        );
        let row = Rect::new(200.0, 366.0 + index as f32 * row_step, 650.0, row_height);
        if button_with_state(
            row,
            &label,
            character.availability == Availability::Ready,
            character.deployment_selected,
            mouse,
        ) {
            actions.push(UiAction::ToggleDeployment(character.id.clone()));
        }
        crate::portrait_ui::draw_character_portrait(
            assets,
            visuals,
            Rect::new(row.x + 4.0, row.y + 2.0, 25.0, 25.0),
            &character.id,
            &character.name,
            if character.deployment_selected {
                dark::POSITIVE
            } else {
                dark::TEXT_DIM
            },
        );
        draw_text_ex(
            character.name.to_uppercase(),
            row.x + 40.0,
            row.y + 22.0,
            TextStyle::new(13.0, dark::TEXT_BRIGHT).params(),
        );
    }
    if button(
        Rect::new(400.0, 562.0, 200.0, 48.0),
        &format!("FORMATION // {}", formation.label()),
        true,
        mouse,
    ) {
        actions.push(UiAction::CycleFormation);
    }
    if button(
        Rect::new(820.0, 562.0, 250.0, 48.0),
        &format!(
            "DEPLOY SQUAD · {} FOOD",
            campaign.deployment_food_cost(data)
        ),
        campaign.selected_squad_count() > 0
            && campaign.colony.resources.food >= campaign.deployment_food_cost(data),
        mouse,
    ) {
        actions.push(UiAction::DeployMission);
    }
    if button(
        Rect::new(200.0, 562.0, 180.0, 48.0),
        "STAND DOWN",
        true,
        mouse,
    ) {
        actions.push(UiAction::ReturnToColony);
    }
}

#[cfg(test)]
mod tests;
