//! One-round tactical telegraph for queued reinforcement entry tiles.

use crate::grid_ui::GridView;
use crate::state::GameSession;
use crate::visual_assets::VisualCatalog;
use macroquad::prelude::*;
use macroquad_toolkit::assets::AssetManager;
use macroquad_toolkit::prelude::TextStyle;

pub(crate) fn draw(
    session: &GameSession,
    view: GridView,
    assets: &AssetManager,
    visuals: &VisualCatalog,
) {
    let Some(wave) = crate::reinforcements::telegraphed_wave(session) else {
        return;
    };
    for unit in &wave.units {
        let rect = view.tile_rect(unit.position);
        let points = view.diamond(unit.position);
        let warning = Color::new(1.0, 0.34, 0.16, 1.0);
        let fill = Color::new(0.75, 0.12, 0.06, 0.22);
        draw_triangle(points[0], points[1], points[2], fill);
        draw_triangle(points[0], points[2], points[3], fill);
        for index in 0..4 {
            let next = (index + 1) % 4;
            draw_line(
                points[index].x,
                points[index].y,
                points[next].x,
                points[next].y,
                3.0,
                warning,
            );
        }
        let label = format!("IN R{}", wave.round);
        let dimensions = measure_text(&label, None, 14, 1.0);
        draw_text_ex(
            &label,
            rect.x + (rect.w - dimensions.width) * 0.5,
            rect.y + rect.h * 0.56,
            TextStyle::new(14.0, warning).params(),
        );
        visuals.draw_portrait(
            assets,
            &unit.id,
            &unit.name,
            Rect::new(rect.x + rect.w * 0.5 - 20.0, rect.y - 34.0, 40.0, 42.0),
            warning,
        );
    }
}
