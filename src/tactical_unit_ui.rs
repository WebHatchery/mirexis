//! Authored tactical sprites, allegiance bases, and compact state markers.

use crate::data::Team;
use crate::grid_ui::GridView;
use crate::state::UnitState;
use crate::visual_assets::VisualCatalog;
use macroquad::prelude::*;
use macroquad_toolkit::assets::AssetManager;
use macroquad_toolkit::prelude::TextStyle;

pub(crate) fn draw_unit(
    assets: &AssetManager,
    visuals: &VisualCatalog,
    view: GridView,
    unit: &UnitState,
    max_action_points: u8,
    selected: bool,
    targetable: bool,
) {
    let rect = view.tile_rect(unit.position);
    let channel = visuals.unit_faction_channel(unit);
    let accent = visuals.faction_accent(unit);
    let tint = if unit.incapacitated {
        Color::new(0.46, 0.48, 0.48, 0.88)
    } else {
        WHITE
    };
    draw_allegiance_base(rect, accent, selected, channel.hostile_notches);
    visuals.draw_unit_sprite(assets, unit, rect, tint);
    draw_state_markers(rect, unit, targetable);
    draw_vitality(rect, unit);
    if unit.team == Team::Colony && !unit.incapacitated {
        draw_readiness(rect, unit.action_points, max_action_points);
    }
}

fn draw_allegiance_base(rect: Rect, accent: Color, selected: bool, hostile_notches: bool) {
    let center = vec2(rect.x + rect.w * 0.5, rect.y + rect.h * 0.70);
    draw_ellipse(
        center.x,
        center.y,
        rect.w * 0.29,
        rect.h * 0.20,
        0.0,
        Color::new(
            accent.r,
            accent.g,
            accent.b,
            if selected { 0.58 } else { 0.32 },
        ),
    );
    draw_ellipse_ring(
        center,
        rect.w * 0.30,
        rect.h * 0.21,
        accent,
        if selected { 3.0 } else { 2.0 },
    );
    if selected {
        draw_selection_brackets(rect);
    }
    if hostile_notches {
        for side in [-1.0_f32, 1.0] {
            draw_triangle(
                vec2(center.x + side * rect.w * 0.28, center.y),
                vec2(center.x + side * rect.w * 0.38, center.y - 3.0),
                vec2(center.x + side * rect.w * 0.38, center.y + 3.0),
                accent,
            );
        }
    }
}

fn draw_ellipse_ring(center: Vec2, rx: f32, ry: f32, color: Color, width: f32) {
    let segments = 28;
    for index in 0..segments {
        let a = index as f32 / segments as f32 * std::f32::consts::TAU;
        let b = (index + 1) as f32 / segments as f32 * std::f32::consts::TAU;
        draw_line(
            center.x + a.cos() * rx,
            center.y + a.sin() * ry,
            center.x + b.cos() * rx,
            center.y + b.sin() * ry,
            width,
            color,
        );
    }
}

fn draw_state_markers(rect: Rect, unit: &UnitState, targetable: bool) {
    if unit.overwatching && !unit.incapacitated {
        draw_ellipse_ring(
            vec2(rect.x + rect.w * 0.5, rect.y + rect.h * 0.66),
            rect.w * 0.38,
            rect.h * 0.27,
            Color::new(1.0, 0.74, 0.18, 1.0),
            2.5,
        );
    }
    if !unit.statuses.is_empty() && !unit.incapacitated {
        draw_poly(
            rect.right() - 6.0,
            rect.y + 4.0,
            4,
            7.0,
            45.0,
            Color::new(1.0, 0.74, 0.18, 1.0),
        );
    }
    if targetable {
        draw_diamond_outline(rect, Color::new(1.0, 0.76, 0.20, 1.0), 3.0);
    }
}

fn draw_selection_brackets(rect: Rect) {
    let color = Color::new(0.94, 1.0, 0.98, 1.0);
    let center = vec2(rect.x + rect.w * 0.5, rect.y + rect.h * 0.5);
    for corner in [
        vec2(center.x, rect.y - 2.0),
        vec2(rect.right() + 2.0, center.y),
        vec2(center.x, rect.bottom() + 2.0),
        vec2(rect.x - 2.0, center.y),
    ] {
        draw_poly(corner.x, corner.y, 4, 3.5, 45.0, color);
    }
}

fn draw_diamond_outline(rect: Rect, color: Color, width: f32) {
    let center = vec2(rect.x + rect.w * 0.5, rect.y + rect.h * 0.5);
    let points = [
        vec2(center.x, rect.y),
        vec2(rect.right(), center.y),
        vec2(center.x, rect.bottom()),
        vec2(rect.x, center.y),
    ];
    for index in 0..4 {
        let next = (index + 1) % 4;
        draw_line(
            points[index].x,
            points[index].y,
            points[next].x,
            points[next].y,
            width,
            color,
        );
    }
}

fn draw_vitality(rect: Rect, unit: &UnitState) {
    let bar = Rect::new(rect.x + 7.0, rect.y - 14.0, rect.w - 14.0, 7.0);
    let ratio = if unit.max_health > 0 {
        (unit.health.max(0) as f32 / unit.max_health as f32).clamp(0.0, 1.0)
    } else {
        0.0
    };
    draw_rectangle(
        bar.x - 1.0,
        bar.y - 1.0,
        bar.w + 2.0,
        bar.h + 2.0,
        Color::new(0.01, 0.02, 0.02, 0.92),
    );
    let health = if ratio > 0.6 {
        Color::new(0.32, 0.88, 0.45, 1.0)
    } else if ratio > 0.3 {
        Color::new(1.0, 0.66, 0.18, 1.0)
    } else {
        Color::new(1.0, 0.24, 0.20, 1.0)
    };
    draw_rectangle(bar.x, bar.y, bar.w * ratio, bar.h, health);
    if unit.effective_armour() > 0 && !unit.incapacitated {
        draw_text_ex(
            format!("A{}", unit.effective_armour()),
            rect.right() - 12.0,
            rect.y - 5.0,
            TextStyle::new(10.0, Color::new(0.54, 0.82, 1.0, 1.0)).params(),
        );
    }
}

fn draw_readiness(rect: Rect, action_points: u8, max_action_points: u8) {
    let count = max_action_points.max(1);
    let spacing = 8.0;
    let start = rect.x + rect.w * 0.5 - count.saturating_sub(1) as f32 * spacing * 0.5;
    for index in 0..count {
        draw_poly(
            start + index as f32 * spacing,
            rect.bottom() + 7.0,
            4,
            4.0,
            45.0,
            if index < action_points {
                Color::new(0.48, 0.94, 1.0, 1.0)
            } else {
                Color::new(0.12, 0.22, 0.23, 1.0)
            },
        );
    }
}
