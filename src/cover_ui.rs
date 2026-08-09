//! Tactical integrity rendering for destructible cover.

use crate::data::{CoverEdgeDef, EdgeDirection};
use crate::grid_ui::GridView;
use crate::state::DestructibleCover;
use macroquad::prelude::*;
use macroquad_toolkit::grid::TilePos;
use macroquad_toolkit::prelude::TextStyle;
use macroquad_toolkit::ui::draw_ui_text_ex;

pub(crate) fn draw_edges(view: GridView, edges: &[CoverEdgeDef]) {
    for edge in edges {
        let rect = view.tile_rect(TilePos::new(edge.position[0], edge.position[1]));
        let inset = 5.0;
        let strength = f32::from(edge.strength);
        let thickness = 2.0 + strength / 15.0;
        let color = Color::new(0.42, 0.82, 0.78, 0.95);
        let (start, end, label) = match edge.direction {
            EdgeDirection::North => (
                vec2(rect.x + inset, rect.y + 3.0),
                vec2(rect.right() - inset, rect.y + 3.0),
                vec2(rect.x + rect.w * 0.5 - 6.0, rect.y + 14.0),
            ),
            EdgeDirection::East => (
                vec2(rect.right() - 3.0, rect.y + inset),
                vec2(rect.right() - 3.0, rect.bottom() - inset),
                vec2(rect.right() - 19.0, rect.y + rect.h * 0.5 + 3.0),
            ),
            EdgeDirection::South => (
                vec2(rect.x + inset, rect.bottom() - 3.0),
                vec2(rect.right() - inset, rect.bottom() - 3.0),
                vec2(rect.x + rect.w * 0.5 - 6.0, rect.bottom() - 8.0),
            ),
            EdgeDirection::West => (
                vec2(rect.x + 3.0, rect.y + inset),
                vec2(rect.x + 3.0, rect.bottom() - inset),
                vec2(rect.x + 7.0, rect.y + rect.h * 0.5 + 3.0),
            ),
        };
        draw_line(start.x, start.y, end.x, end.y, thickness, color);
        draw_circle(start.x, start.y, thickness * 0.65, color);
        draw_circle(end.x, end.y, thickness * 0.65, color);
        draw_text_ex(
            edge.strength.to_string(),
            label.x,
            label.y,
            TextStyle::new(9.0, color).params(),
        );
    }
}

pub(crate) fn draw_cover(view: GridView, cover: &DestructibleCover, targetable: bool) {
    let rect = view.tile_rect(cover.position);
    let ratio = cover.health as f32 / cover.max_health.max(1) as f32;
    draw_rectangle(
        rect.x + 5.0,
        rect.bottom() - 10.0,
        (rect.w - 10.0) * ratio,
        5.0,
        Color::new(0.95, 0.54, 0.22, 1.0),
    );
    draw_ui_text_ex(
        &format!("{}/{}", cover.health, cover.max_health),
        rect.x + 7.0,
        rect.y + 16.0,
        macroquad_toolkit::prelude::TextStyle::new(12.0, WHITE).params(),
    );
    if targetable {
        draw_rectangle_lines(
            rect.x + 3.0,
            rect.y + 3.0,
            rect.w - 6.0,
            rect.h - 6.0,
            3.0,
            Color::new(0.95, 0.74, 0.24, 1.0),
        );
    }
}
