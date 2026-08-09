//! Reusable Mirexis tactical labels and buttons.

use crate::state::{BattleEvent, UnitState};
use macroquad::prelude::*;
use macroquad_toolkit::prelude::{
    dark, draw_surface, draw_text_centered_in_box_ex, SurfaceStyle, TextStyle,
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
        BattleEvent::ExtractionCompleted { unit_id } => format!("{} reached extraction", unit_id),
        BattleEvent::MutationActivated { gift, .. } => gift.clone(),
        BattleEvent::ClassActionActivated { action, .. } => action.clone(),
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
            .map(|status| format!("{:?}", status.kind).to_uppercase())
            .collect::<Vec<_>>()
            .join("/");
        format!("{} · {}", statuses, class)
    }
}

pub(crate) fn button(rect: Rect, label: &str, enabled: bool, mouse: Vec2) -> bool {
    let hovered = enabled && rect.contains_point(mouse);
    let fill = if !enabled {
        Color::new(0.09, 0.11, 0.12, 1.0)
    } else if hovered {
        Color::new(0.18, 0.43, 0.37, 1.0)
    } else {
        Color::new(0.10, 0.28, 0.25, 1.0)
    };
    draw_surface(
        rect,
        &SurfaceStyle::new(fill).with_border(
            1.0,
            if enabled {
                Color::new(0.33, 0.72, 0.60, 1.0)
            } else {
                Color::new(0.20, 0.24, 0.25, 1.0)
            },
        ),
    );
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
