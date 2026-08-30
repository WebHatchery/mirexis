//! Tactical field-equipment controls.

use crate::state::UnitState;
use crate::ui::{TargetingView, UiAction, UiContext};
use crate::ui_widgets::button;
use macroquad::prelude::{Color, Rect, Vec2, WHITE};

fn action_button_enabled(targeting: bool, has_valid_target: bool) -> bool {
    targeting || has_valid_target
}

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
    let targeting = matches!(
        (ctx.targeting, equipment_id.as_deref()),
        (
            Some(TargetingView::Equipment { equipment_id: active, .. }),
            Some(current)
        ) if active == current
    );
    let has_valid_target = equipment_id.as_deref().is_some_and(|equipment_id| {
        selected.is_some_and(|unit| {
            crate::equipment_actions::has_valid_target(ctx.session, &unit.id, equipment_id)
        })
    });
    let clicked = button(
        rect,
        if targeting { "CANCEL" } else { equipment_label },
        action_button_enabled(targeting, has_valid_target),
        mouse,
    );
    if let Some(id) = equipment_id.as_deref() {
        if let Some(index) = crate::visual_assets::equipment_index(id) {
            ctx.visuals.draw_atlas_cell(
                ctx.assets,
                &ctx.visuals.equipment,
                index,
                Rect::new(rect.x + 5.0, rect.y + 4.0, 34.0, rect.h - 8.0),
                if targeting {
                    Color::new(1.0, 0.64, 0.30, 1.0)
                } else {
                    WHITE
                },
            );
        }
    }
    if !clicked {
        return;
    }
    if targeting {
        actions.push(UiAction::CancelTargeting);
    } else if let Some(equipment_id) = equipment_id {
        actions.push(UiAction::ArmEquipment(equipment_id));
    }
}

#[cfg(test)]
mod tests;
