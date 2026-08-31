//! Tactical class-action button and targeting intent.

use crate::state::UnitState;
use crate::ui::{TargetingView, UiAction, UiContext};
use crate::ui_widgets::button;
use macroquad::prelude::{Rect, Vec2};

fn class_action_button_label<'a>(
    label: &'a str,
    unit: &UnitState,
    targeting: bool,
    requires_target: bool,
    enabled: bool,
) -> &'a str {
    if targeting {
        return "CANCEL";
    }
    if enabled {
        return label;
    }
    if unit.incapacitated {
        return "INCAPACITATED";
    }
    if unit.class_action_used {
        return "SPENT";
    }
    if unit.action_points == 0 {
        return "NO AP";
    }
    if requires_target {
        return "NO TARGET";
    }
    label
}

pub(crate) fn draw_action_button(
    ctx: &UiContext<'_>,
    selected: Option<&UnitState>,
    rect: Rect,
    mouse: Vec2,
    actions: &mut Vec<UiAction>,
) {
    let label = selected
        .and_then(|unit| crate::class_actions::action_name(&unit.class_id))
        .unwrap_or("NO CLASS ACTION");
    let Some(unit) = selected else {
        button(rect, "SELECT UNIT", false, mouse);
        return;
    };
    let requires_target = crate::class_actions::requires_target(&unit.class_id);
    let targeting = matches!(
        ctx.targeting,
        Some(TargetingView::ClassAction { unit_id, .. }) if unit_id == unit.id
    );
    let enabled = targeting
        || if requires_target {
            crate::class_actions::has_valid_target(ctx.session, &unit.id)
        } else {
            ctx.session.can_activate_selected_class_action()
        };
    let button_label = class_action_button_label(label, unit, targeting, requires_target, enabled);
    if !button(rect, button_label, enabled, mouse) {
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

#[cfg(test)]
mod tests;
