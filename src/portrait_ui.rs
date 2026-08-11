//! Authored portrait atlas presentation shared by tactical and strategy screens.

use crate::visual_assets::VisualCatalog;
use macroquad::prelude::*;
use macroquad_toolkit::assets::AssetManager;

pub(crate) fn draw_character_portrait(
    assets: &AssetManager,
    visuals: &VisualCatalog,
    rect: Rect,
    unit_id: &str,
    name: &str,
    accent: Color,
) {
    visuals.draw_portrait(assets, unit_id, name, rect, accent);
    draw_line(
        rect.x + 7.0,
        rect.y + 7.0,
        rect.x + rect.w * 0.35,
        rect.y + 7.0,
        1.0,
        Color::new(0.88, 1.0, 0.96, 0.72),
    );
    draw_line(
        rect.right() - 7.0,
        rect.bottom() - 10.0,
        rect.right() - rect.w * 0.30,
        rect.bottom() - 10.0,
        1.0,
        Color::new(accent.r, accent.g, accent.b, 0.75),
    );
}
