//! Tactical class-action button and targeting intent.

use crate::state::UnitState;
use crate::ui::{TargetingView, UiAction, UiContext};
use crate::ui_widgets::button;
use macroquad::prelude::{Rect, Vec2};

pub(crate) fn draw_action_button(
    ctx: &UiContext<'_>,
    selected: Option<&UnitState>,
    rect: Rect,
    mouse: Vec2,
    actions: &mut Vec<UiAction>,
) {
    let label = selected
        .and_then(|unit| crate::class_actions::action_name(&unit.class_id))
        .unwrap_or("CLASS ACTION");
    let Some(unit) = selected else {
        button(rect, label, false, mouse);
        return;
    };
    let requires_target = crate::class_actions::requires_target(&unit.class_id);
    let targeting = matches!(
        ctx.targeting,
        Some(TargetingView::ClassAction { unit_id }) if unit_id == unit.id
    );
    let enabled = if requires_target {
        crate::class_actions::has_valid_target(ctx.session, &unit.id)
    } else {
        ctx.session.can_activate_selected_class_action()
    };
    if !button(
        rect,
        if targeting { "CANCEL" } else { label },
        enabled,
        mouse,
    ) {
        return;
    }
    actions.push(if targeting {
        UiAction::CancelTargeting
    } else if requires_target {
        UiAction::ArmClassAction
    } else {
        UiAction::ActivateClassAction
    });
}
