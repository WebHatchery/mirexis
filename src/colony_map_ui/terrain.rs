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
    draw_fill(top, top_color);

    let pattern = (position[0] * 41 + position[1] * 67).unsigned_abs();
    let cell = if occupied || view.elevation(position) > 0 {
        0
    } else if pattern.is_multiple_of(5) {
        8
    } else {
        1
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
        Color::new(0.72, 0.88, 0.78, if hovered { 0.80 } else { 0.58 }),
    );
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
}
