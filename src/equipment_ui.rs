//! Tactical field-equipment controls.

use crate::state::UnitState;
use crate::ui::{UiAction, UiContext};
use crate::ui_widgets::button;
use macroquad::prelude::{Rect, Vec2};

pub(crate) fn draw_action_button(
    ctx: &UiContext<'_>,
    selected: Option<&UnitState>,
    rect: Rect,
    mouse: Vec2,
    actions: &mut Vec<UiAction>,
) {
    let equipment_id =
        selected.and_then(|unit| crate::equipment_actions::available_action(ctx.session, &unit.id));
    let equipment_label = equipment_id
        .as_deref()
        .and_then(crate::equipment_actions::action_name)
        .unwrap_or("FIELD ITEM");
    let targeting = ctx.targeted_equipment == equipment_id.as_deref();
    if !button(
        rect,
        if targeting {
            "CANCEL TARGET"
        } else {
            equipment_label
        },
        equipment_id.as_deref().is_some_and(|equipment_id| {
            selected.is_some_and(|unit| {
                crate::equipment_actions::has_valid_target(ctx.session, &unit.id, equipment_id)
            })
        }),
        mouse,
    ) {
        return;
    }
    if targeting {
        actions.push(UiAction::CancelEquipmentTargeting);
    } else if let Some(equipment_id) = equipment_id {
        actions.push(UiAction::ArmEquipment(equipment_id));
    }
}
