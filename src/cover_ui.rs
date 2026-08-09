//! Tactical integrity rendering for destructible cover.

use crate::grid_ui::GridView;
use crate::state::DestructibleCover;
use macroquad::prelude::*;
use macroquad_toolkit::ui::draw_ui_text_ex;

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
