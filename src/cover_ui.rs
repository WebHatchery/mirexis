//! Projected cover edges and authored destructible barricades.

use crate::data::{CoverEdgeDef, EdgeDirection};
use crate::grid_ui::GridView;
use crate::state::DestructibleCover;
use crate::visual_assets::VisualCatalog;
use macroquad::prelude::*;
use macroquad_toolkit::assets::AssetManager;
use macroquad_toolkit::grid::TilePos;
use macroquad_toolkit::prelude::TextStyle;

pub(crate) fn draw_edges(view: GridView, edges: &[CoverEdgeDef]) {
    for edge in edges {
        let tile = TilePos::new(edge.position[0], edge.position[1]);
        let diamond = view.diamond(tile);
        let thickness = 2.0 + f32::from(edge.strength) / 18.0;
        let color = Color::new(0.42, 0.88, 0.80, 0.96);
        let (start, end) = match edge.direction {
            EdgeDirection::North => (diamond[3], diamond[0]),
            EdgeDirection::East => (diamond[0], diamond[1]),
            EdgeDirection::South => (diamond[1], diamond[2]),
            EdgeDirection::West => (diamond[2], diamond[3]),
        };
        draw_line(
            start.x,
            start.y,
            end.x,
            end.y,
            thickness,
            Color::new(0.02, 0.06, 0.06, 1.0),
        );
        draw_line(
            start.x,
            start.y - 2.0,
            end.x,
            end.y - 2.0,
            thickness * 0.55,
            color,
        );
        draw_circle(start.x, start.y - 2.0, thickness * 0.62, color);
        draw_circle(end.x, end.y - 2.0, thickness * 0.62, color);
        let label = (start + end) * 0.5;
        draw_text_ex(
            edge.strength.to_string(),
            label.x - 6.0,
            label.y - 6.0,
            TextStyle::new(10.0, color).params(),
        );
    }
}

pub(crate) fn draw_cover(
    assets: &AssetManager,
    visuals: &VisualCatalog,
    view: GridView,
    cover: &DestructibleCover,
    targetable: bool,
) {
    let rect = view.tile_rect(cover.position);
    let ratio = cover.health as f32 / cover.max_health.max(1) as f32;
    let tint = if ratio > 0.5 {
        WHITE
    } else {
        Color::new(1.0, 0.58, 0.42, 1.0)
    };
    visuals.draw_atlas_cell(
        assets,
        &visuals.terrain,
        4,
        Rect::new(
            rect.x - 5.0,
            rect.y - rect.w * 0.82,
            rect.w + 10.0,
            rect.w * 1.22,
        ),
        tint,
    );
    let bar = Rect::new(rect.x + 8.0, rect.y - 11.0, rect.w - 16.0, 4.0);
    draw_rectangle(
        bar.x - 1.0,
        bar.y - 1.0,
        bar.w + 2.0,
        bar.h + 2.0,
        Color::new(0.01, 0.02, 0.02, 0.9),
    );
    draw_rectangle(
        bar.x,
        bar.y,
        bar.w * ratio,
        bar.h,
        Color::new(1.0, 0.54, 0.20, 1.0),
    );
    draw_text_ex(
        format!("{}/{}", cover.health, cover.max_health),
        rect.x + 8.0,
        rect.y - 14.0,
        TextStyle::new(10.0, Color::new(1.0, 0.86, 0.60, 1.0)).params(),
    );
    if targetable {
        let diamond = view.diamond(cover.position);
        for index in 0..4 {
            let next = (index + 1) % 4;
            draw_line(
                diamond[index].x,
                diamond[index].y,
                diamond[next].x,
                diamond[next].y,
                3.0,
                Color::new(1.0, 0.76, 0.20, 1.0),
            );
        }
    }
}
