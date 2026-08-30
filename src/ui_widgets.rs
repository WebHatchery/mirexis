//! Reusable Mirexis tactical labels and buttons.

use crate::state::{BattleEvent, UnitState};
use macroquad::prelude::*;
use macroquad_toolkit::prelude::{
    dark, draw_chamfered_surface, draw_text_centered_in_box_ex, ChamferedSurfaceStyle, TextStyle,
};
use macroquad_toolkit::ui::RectExt;

pub(crate) fn event_summary(event: &BattleEvent) -> String {
    match event {
        BattleEvent::UnitMoved { unit_id, cost, .. } => format!("{} moved · {} AP", unit_id, cost),
        BattleEvent::AttackRolled {
            roll, hit_chance, ..
        } => format!("Attack roll {} · {}% target", roll, hit_chance),
        BattleEvent::DamageApplied {
            amount, remaining, ..
        } => format!("{} damage · {} vitality remains", amount, remaining),
        BattleEvent::UnitIncapacitated { unit_id } => format!("{} incapacitated", unit_id),
        BattleEvent::ObjectiveSecured { .. } => "Mission objective secured".to_owned(),
        BattleEvent::ObjectiveDamaged { amount, remaining } => {
            format!(
                "Field asset took {} damage · {} integrity",
                amount, remaining
            )
        }
        BattleEvent::ObjectiveDestroyed => "Field asset destroyed".to_owned(),
        BattleEvent::ExtractionCompleted { unit_id } => format!("{} reached extraction", unit_id),
        BattleEvent::MutationActivated { gift, .. } => gift.clone(),
        BattleEvent::ClassActionActivated { action, .. } => action.clone(),
        BattleEvent::SkillActivated { skill_id, .. } => {
            format!("{} technique activated", skill_id.replace('_', " "))
        }
        BattleEvent::EquipmentUsed {
            equipment_id,
            target_id,
            ..
        } => format!("{} used on {}", equipment_id, target_id),
        BattleEvent::CoverDamaged {
            amount, remaining, ..
        } => format!("Cover took {} damage · {} integrity", amount, remaining),
        BattleEvent::CoverDestroyed { .. } => "Cover destroyed · route opened".to_owned(),
        BattleEvent::OverwatchSet { unit_id } => format!("{} entered overwatch", unit_id),
        BattleEvent::ReactionTriggered {
            attacker_id,
            target_id,
        } => format!("{} reacted to {}", attacker_id, target_id),
        BattleEvent::HazardTriggered { unit_id, kind } => {
            format!("{} crossed {}", unit_id, kind.label())
        }
        BattleEvent::EnemyAbilityActivated { unit_id, ability } => {
            format!("{} used {}", unit_id, ability)
        }
        BattleEvent::StatusApplied { status, .. } => format!("{:?} status applied", status),
        BattleEvent::ReinforcementsArrived { count, .. } => {
            format!("{} hostile reinforcements arrived", count)
        }
        BattleEvent::UnitHealed {
            amount, remaining, ..
        } => format!("{} vitality restored · {} remains", amount, remaining),
        BattleEvent::PhaseStarted { phase, round } => {
            format!("{:?} phase · round {}", phase, round)
        }
        BattleEvent::BattleEnded { outcome } => format!("Operation {:?}", outcome),
    }
}

pub(crate) fn action_status(unit: &UnitState) -> String {
    if unit.overwatching {
        return "OVERWATCH ARMED · REACTION READY".to_owned();
    }
    let mutation = if unit.mutation_gift_used {
        "MUTATION SPENT"
    } else {
        "MUTATION READY"
    };
    let class = if unit.class_action_used {
        "CLASS SPENT"
    } else {
        "CLASS READY"
    };
    if unit.statuses.is_empty() {
        format!("{} · {}", mutation, class)
    } else {
        let statuses = unit
            .statuses
            .iter()
            .map(|status| {
                format!("{:?} {} PH", status.kind, status.remaining_phases).to_uppercase()
            })
            .collect::<Vec<_>>()
            .join(" / ");
        format!("{} · {}", statuses, class)
    }
}

pub(crate) fn button(rect: Rect, label: &str, enabled: bool, mouse: Vec2) -> bool {
    button_with_state(rect, label, enabled, false, mouse)
}

pub(crate) fn button_with_state(
    rect: Rect,
    label: &str,
    enabled: bool,
    focused: bool,
    mouse: Vec2,
) -> bool {
    let hovered = enabled && rect.contains_point(mouse);
    let fill = if !enabled {
        Color::new(0.065, 0.085, 0.09, 1.0)
    } else if focused {
        Color::new(0.10, 0.31, 0.29, 1.0)
    } else if hovered {
        Color::new(0.12, 0.40, 0.36, 1.0)
    } else {
        Color::new(0.075, 0.22, 0.22, 1.0)
    };
    draw_chamfered_surface(
        rect,
        &ChamferedSurfaceStyle::new(
            fill,
            if focused {
                Color::new(0.82, 1.0, 0.50, 1.0)
            } else if enabled {
                Color::new(0.30, 0.82, 0.70, 1.0)
            } else {
                Color::new(0.16, 0.22, 0.23, 1.0)
            },
        )
        .with_corner(6.0)
        .with_border_width(if hovered || focused { 2.0 } else { 1.0 }),
    );
    if enabled {
        draw_line(
            rect.x + 12.0,
            rect.y + 4.0,
            rect.x + rect.w - 12.0,
            rect.y + 4.0,
            1.0,
            Color::new(0.65, 1.0, 0.90, if hovered { 0.75 } else { 0.28 }),
        );
    }
    if focused {
        for x in [rect.x + 8.0, rect.right() - 8.0] {
            draw_poly(
                x,
                rect.y + rect.h * 0.5,
                4,
                3.5,
                45.0,
                Color::new(0.82, 1.0, 0.50, 1.0),
            );
        }
    }
    draw_text_centered_in_box_ex(
        label,
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        TextStyle::new(16.0, if enabled { dark::TEXT } else { dark::TEXT_DIM })
            .with_macroquad_font(),
    );
    hovered && is_mouse_button_released(MouseButton::Left)
}

#[cfg(test)]
mod tests;
