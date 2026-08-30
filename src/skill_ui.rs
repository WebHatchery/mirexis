//! Touch-first tactical technique controls.

use crate::data::TechniqueTarget;
use crate::state::UnitState;
use crate::ui::{TargetingView, UiAction, UiContext};
use crate::ui_widgets::button;
use macroquad::prelude::{Rect, Vec2};

pub(crate) fn draw_action_buttons(
    ctx: &UiContext<'_>,
    selected: Option<&UnitState>,
    rect: Rect,
    mouse: Vec2,
    actions: &mut Vec<UiAction>,
) {
    let Some(unit) = selected else {
        button(rect, "TECHNIQUES // SELECT A COLONIST", false, mouse);
        return;
    };
    let skills = unit
        .active_skills
        .iter()
        .filter(|skill| !skill.ends_with("_fundamentals"))
        .take(2)
        .filter_map(|skill_id| {
            ctx.data
                .classes
                .iter()
                .find(|class| class.id == unit.class_id)
                .and_then(|class| {
                    class
                        .techniques
                        .iter()
                        .find(|technique| &technique.id == skill_id)
                })
        })
        .collect::<Vec<_>>();
    if skills.is_empty() {
        button(rect, "TECHNIQUES // LEARN IN BARRACKS", false, mouse);
        return;
    }
    let width = (rect.w - 8.0) / 2.0;
    for slot in 0..2 {
        let Some(technique) = skills.get(slot) else {
            button(
                Rect::new(rect.x + slot as f32 * (width + 8.0), rect.y, width, rect.h),
                "EMPTY SLOT",
                false,
                mouse,
            );
            continue;
        };
        let targeting = matches!(
            ctx.targeting,
            Some(TargetingView::Skill { unit_id, skill_id })
                if unit_id == unit.id && skill_id == technique.id
        );
        let enabled = targeting
            || if technique.target == TechniqueTarget::SelfTarget {
                ctx.session.can_activate_selected_skill(&technique.id)
            } else {
                crate::skills::has_valid_target(ctx.session, &unit.id, &technique.id)
            };
        let label = if targeting {
            "CANCEL"
        } else {
            technique.name.as_str()
        };
        if button(
            Rect::new(rect.x + slot as f32 * (width + 8.0), rect.y, width, rect.h),
            label,
            enabled,
            mouse,
        ) {
            actions.push(if targeting {
                UiAction::CancelTargeting
            } else if !crate::skills::requires_target(&technique.id) {
                UiAction::ActivateSkill(technique.id.clone())
            } else {
                UiAction::ArmSkill(technique.id.clone())
            });
        }
    }
}
