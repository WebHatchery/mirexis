//! Operations-panel affordance for the Commons social preparation action.

use crate::campaign::CampaignState;
use crate::colony::BuildingKind;
use crate::ui::UiAction;
use crate::ui_widgets::button;
use macroquad::prelude::{Rect, Vec2};

fn commons_button_label(
    already_hosted: bool,
    ready_squad_count: usize,
    food: i32,
    food_cost: i32,
) -> String {
    if already_hosted {
        "MEAL // HOSTED".to_owned()
    } else if ready_squad_count < 2 {
        "MEAL // NEED 2 READY".to_owned()
    } else if food < food_cost {
        format!("MEAL // NEED {} FOOD", food_cost)
    } else {
        format!("HOST MEAL // {} FOOD", food_cost)
    }
}

pub(super) fn draw(
    campaign: &CampaignState,
    routine_operations_visible: bool,
    mouse: Vec2,
    actions: &mut Vec<UiAction>,
) {
    if !routine_operations_visible || !campaign.colony.has_facility(BuildingKind::Commons) {
        return;
    }
    let already_hosted = campaign.commons_meal_operation == Some(campaign.operations_completed);
    let label = commons_button_label(
        already_hosted,
        campaign.selected_squad_count(),
        campaign.colony.resources.food,
        campaign.commons_meal_food_cost(),
    );
    if button(
        Rect::new(878.0, 344.0, 176.0, 30.0),
        &label,
        campaign.commons_meal_available(),
        mouse,
    ) {
        actions.push(UiAction::HostCommonsMeal);
    }
}

#[cfg(test)]
mod tests;
