//! Briefing squad rows, formation choice, and deployment controls.

use crate::campaign::{Availability, CampaignState, SQUAD_LIMIT};
use crate::data::GameData;
use crate::formation::FormationKind;
use crate::ui::UiAction;
use crate::ui_widgets::button;
use macroquad::prelude::*;
use macroquad_toolkit::prelude::{dark, TextStyle};

pub(crate) fn draw(
    campaign: &CampaignState,
    data: &GameData,
    formation: FormationKind,
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
        370.0,
        TextStyle::new(15.0, dark::ACCENT).params(),
    );
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
            "[{}] {} · {} · LV{} · {} XP{}",
            state, character.name, class_name, character.level, character.experience, bond
        );
        if button(
            Rect::new(200.0, 384.0 + index as f32 * 32.0, 650.0, 28.0),
            &label,
            character.availability == Availability::Ready,
            mouse,
        ) {
            actions.push(UiAction::ToggleDeployment(character.id.clone()));
        }
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
