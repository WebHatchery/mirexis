//! Tactical unit tokens and target outlines.

use crate::data::Team;
use crate::grid_ui::GridView;
use crate::state::UnitState;
use macroquad::prelude::*;
use macroquad_toolkit::prelude::{draw_text_centered_in_box, TextStyle};

pub(crate) fn draw_unit(
    view: GridView,
    unit: &UnitState,
    max_action_points: u8,
    selected: bool,
    targetable: bool,
) {
    let rect = view.tile_rect(unit.position);
    let color = match unit.team {
        Team::Colony => Color::new(0.22, 0.75, 0.63, 1.0),
        Team::Hostile => Color::new(0.86, 0.27, 0.25, 1.0),
    };
    let center = vec2(rect.x + rect.w * 0.5, rect.y + rect.h * 0.48);
    let color = if unit.incapacitated {
        Color::new(color.r * 0.35, color.g * 0.35, color.b * 0.35, 1.0)
    } else {
        color
    };
    draw_circle(center.x, center.y, rect.w * 0.28, color);
    draw_circle_lines(
        center.x,
        center.y,
        rect.w * 0.28,
        if selected { 4.0 } else { 2.0 },
        if selected {
            WHITE
        } else {
            Color::new(0.04, 0.08, 0.08, 1.0)
        },
    );
    if unit.overwatching && !unit.incapacitated {
        draw_circle_lines(
            center.x,
            center.y,
            rect.w * 0.36,
            3.0,
            Color::new(0.95, 0.74, 0.24, 1.0),
        );
    }
    if !unit.statuses.is_empty() && !unit.incapacitated {
        draw_circle(
            rect.x + rect.w - 9.0,
            rect.y + 9.0,
            5.0,
            Color::new(0.95, 0.74, 0.24, 1.0),
        );
    }
    if targetable {
        draw_rectangle_lines(
            rect.x + 5.0,
            rect.y + 5.0,
            rect.w - 10.0,
            rect.h - 10.0,
            3.0,
            Color::new(0.95, 0.74, 0.24, 1.0),
        );
    }
    let initials = unit
        .name
        .split_whitespace()
        .filter_map(|word| word.chars().next())
        .take(2)
        .collect::<String>();
    draw_text_centered_in_box(
        &initials,
        rect.x,
        rect.y,
        rect.w,
        rect.h - 2.0,
        15.0,
        Color::new(0.03, 0.07, 0.07, 1.0),
    );
    draw_vitality(rect, unit);
    if unit.team == Team::Colony && !unit.incapacitated {
        draw_readiness(rect, unit.action_points, max_action_points);
    }
}

fn draw_vitality(rect: Rect, unit: &UnitState) {
    let bar = Rect::new(rect.x + 8.0, rect.y + 3.0, rect.w - 16.0, 4.0);
    let ratio = if unit.max_health > 0 {
        (unit.health.max(0) as f32 / unit.max_health as f32).clamp(0.0, 1.0)
    } else {
        0.0
    };
    draw_rectangle(
        bar.x,
        bar.y,
        bar.w,
        bar.h,
        Color::new(0.08, 0.12, 0.12, 1.0),
    );
    let health_color = if ratio > 0.6 {
        Color::new(0.35, 0.82, 0.43, 1.0)
    } else if ratio > 0.3 {
        Color::new(0.96, 0.68, 0.24, 1.0)
    } else {
        Color::new(0.93, 0.28, 0.25, 1.0)
    };
    draw_rectangle(bar.x, bar.y, bar.w * ratio, bar.h, health_color);

    let armour = unit.effective_armour().max(0);
    if armour > 0 && !unit.incapacitated {
        draw_text_ex(
            format!("A{armour}"),
            rect.x + 4.0,
            rect.y + 17.0,
            TextStyle::new(9.0, Color::new(0.48, 0.78, 1.0, 1.0)).params(),
        );
    }
}

fn draw_readiness(rect: Rect, action_points: u8, max_action_points: u8) {
    if action_points == 0 {
        let label = "SPENT";
        let dimensions = measure_text(label, None, 9, 1.0);
        draw_text_ex(
            label,
            rect.x + (rect.w - dimensions.width) * 0.5,
            rect.y + rect.h - 3.0,
            TextStyle::new(9.0, Color::new(0.96, 0.55, 0.35, 1.0)).params(),
        );
        return;
    }
    let count = max_action_points.max(1);
    let spacing = 7.0;
    let start_x = rect.x + rect.w * 0.5 - (count.saturating_sub(1) as f32 * spacing) * 0.5;
    for index in 0..count {
        let center = vec2(start_x + index as f32 * spacing, rect.y + rect.h - 6.0);
        draw_circle(
            center.x,
            center.y,
            2.4,
            if index < action_points {
                Color::new(0.48, 0.90, 1.0, 1.0)
            } else {
                Color::new(0.14, 0.24, 0.26, 1.0)
            },
        );
    }
}
