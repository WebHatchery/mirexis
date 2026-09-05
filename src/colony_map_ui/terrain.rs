//! Layered wetland ground and exposed colony-map elevation faces.

use super::view::ColonyView;
use crate::visual_assets::VisualCatalog;
use macroquad::prelude::*;
use macroquad_toolkit::assets::AssetManager;

pub(super) fn draw_ground(
    assets: &AssetManager,
    visuals: &VisualCatalog,
    view: ColonyView,
    position: [i32; 2],
    top_color: Color,
    hovered: bool,
    occupied: bool,
) {
    let center = view.plot_center(position);
    let top = diamond(view, center);
    draw_cliff(
        view,
        position,
        [position[0], position[1] + 1],
        top[3],
        top[2],
    );
    draw_cliff(
        view,
        position,
        [position[0] + 1, position[1]],
        top[2],
        top[1],
    );
    draw_fill(
        top,
        surface_color(top_color, position, view.elevation(position)),
    );

    let pattern = (position[0] * 41 + position[1] * 67).unsigned_abs();
    let elevation = view.elevation(position);
    let cell = if occupied || elevation > 0 {
        0
    } else if elevation < 0 || pattern.is_multiple_of(11) {
        8
    } else if pattern.is_multiple_of(5) {
        1
    } else {
        0
    };
    let art_size = view.half_width * 2.12;
    visuals.draw_atlas_cell(
        assets,
        &visuals.terrain,
        cell,
        Rect::new(
            center.x - art_size * 0.5,
            center.y - art_size * 0.58,
            art_size,
            art_size,
        ),
        terrain_art_tint(cell, hovered),
    );
    draw_surface_detail(view, center, position, elevation, occupied, hovered);
}

fn surface_color(base: Color, position: [i32; 2], elevation: i8) -> Color {
    let variation = ((position[0] * 13 + position[1] * 29).unsigned_abs() % 5) as f32 * 0.008;
    let lift = f32::from(elevation.max(0)) * 0.012;
    Color::new(
        (base.r + variation + lift).min(1.0),
        (base.g + variation * 1.5 + lift).min(1.0),
        (base.b + variation + lift * 0.7).min(1.0),
        base.a,
    )
}

fn terrain_art_tint(cell: usize, hovered: bool) -> Color {
    let alpha = if hovered { 0.84 } else { 0.64 };
    match cell {
        8 => Color::new(0.64, 0.92, 0.70, alpha),
        1 => Color::new(0.84, 0.88, 0.76, alpha),
        _ => Color::new(0.76, 0.92, 0.82, alpha),
    }
}

fn draw_surface_detail(
    view: ColonyView,
    center: Vec2,
    position: [i32; 2],
    elevation: i8,
    occupied: bool,
    hovered: bool,
) {
    let pattern = (position[0] * 23 + position[1] * 47).unsigned_abs();
    if elevation < 0 && !occupied {
        draw_ellipse(
            center.x,
            center.y + view.half_height * 0.18,
            view.half_width * 0.38,
            view.half_height * 0.22,
            0.0,
            Color::new(0.08, 0.34, 0.32, if hovered { 0.30 } else { 0.20 }),
        );
        draw_ellipse_lines(
            center.x,
            center.y + view.half_height * 0.18,
            view.half_width * 0.38,
            view.half_height * 0.22,
            0.0,
            1.0,
            Color::new(0.24, 0.66, 0.55, if hovered { 0.60 } else { 0.34 }),
        );
    } else if !occupied && pattern.is_multiple_of(7) {
        let patch = [
            vec2(center.x, center.y - view.half_height * 0.62),
            vec2(center.x + view.half_width * 0.62, center.y),
            vec2(center.x, center.y + view.half_height * 0.62),
            vec2(center.x - view.half_width * 0.62, center.y),
        ];
        draw_fill(
            patch,
            Color::new(0.10, 0.25, 0.22, if hovered { 0.42 } else { 0.24 }),
        );
        draw_line(
            center.x - view.half_width * 0.34,
            center.y - view.half_height * 0.02,
            center.x - view.half_width * 0.08,
            center.y + view.half_height * 0.18,
            1.0,
            Color::new(0.30, 0.64, 0.52, if hovered { 0.52 } else { 0.28 }),
        );
    }
    if elevation > 0 {
        draw_line(
            center.x,
            center.y - view.half_height,
            center.x + view.half_width * 0.72,
            center.y - view.half_height * 0.30,
            1.0,
            Color::new(0.42, 0.76, 0.60, if hovered { 0.62 } else { 0.34 }),
        );
    }
}

fn diamond(view: ColonyView, center: Vec2) -> [Vec2; 4] {
    [
        vec2(center.x, center.y - view.half_height),
        vec2(center.x + view.half_width, center.y),
        vec2(center.x, center.y + view.half_height),
        vec2(center.x - view.half_width, center.y),
    ]
}

fn draw_fill(points: [Vec2; 4], color: Color) {
    draw_triangle(points[0], points[1], points[2], color);
    draw_triangle(points[0], points[2], points[3], color);
}

fn draw_cliff(view: ColonyView, position: [i32; 2], neighbor: [i32; 2], a: Vec2, b: Vec2) {
    let drop = view.cliff_drop(position, neighbor);
    if drop == 0 {
        return;
    }
    let offset = vec2(0.0, f32::from(drop) * view.elevation_step());
    let color = if neighbor[0] > position[0] {
        Color::new(0.025, 0.07, 0.07, 1.0)
    } else {
        Color::new(0.06, 0.12, 0.10, 1.0)
    };
    draw_triangle(a, b, b + offset, color);
    draw_triangle(a, b + offset, a + offset, color);
    draw_line(
        a.x + offset.x,
        a.y + offset.y,
        b.x + offset.x,
        b.y + offset.y,
        1.0,
        Color::new(0.22, 0.44, 0.34, 0.55),
    );
    draw_line(a.x, a.y, b.x, b.y, 1.0, Color::new(0.30, 0.58, 0.46, 0.62));
}
