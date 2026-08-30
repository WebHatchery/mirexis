//! Operations-panel affordance for the Commons social preparation action.

use crate::campaign::{CampaignState, COMMONS_MEAL_FOOD_COST};
use crate::colony::BuildingKind;
use crate::ui::UiAction;
use crate::ui_widgets::button;
use macroquad::prelude::{Rect, Vec2};

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
    let label = if already_hosted {
        "COMMONS MEAL // HOSTED THIS OPERATION".to_owned()
    } else {
        format!("HOST COMMONS MEAL // {} FOOD", COMMONS_MEAL_FOOD_COST)
    };
    if button(
        Rect::new(878.0, 344.0, 176.0, 30.0),
        &label,
        campaign.commons_meal_available(),
        mouse,
    ) {
        actions.push(UiAction::HostCommonsMeal);
    }
}
