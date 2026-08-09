//! Tactical unit tokens and target outlines.

use crate::data::Team;
use crate::grid_ui::GridView;
use crate::state::UnitState;
use macroquad::prelude::*;
use macroquad_toolkit::prelude::draw_text_centered_in_box;

pub(crate) fn draw_unit(view: GridView, unit: &UnitState, selected: bool, targetable: bool) {
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
}
