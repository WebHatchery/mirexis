//! One-round tactical telegraph for queued reinforcement entry tiles.

use crate::grid_ui::GridView;
use crate::state::GameSession;
use macroquad::prelude::*;
use macroquad_toolkit::prelude::TextStyle;

pub(crate) fn draw(session: &GameSession, view: GridView) {
    let Some(wave) = crate::reinforcements::telegraphed_wave(session) else {
        return;
    };
    for unit in &wave.units {
        let rect = view.tile_rect(unit.position);
        let warning = Color::new(1.0, 0.34, 0.16, 1.0);
        draw_rectangle(
            rect.x + 3.0,
            rect.y + 3.0,
            rect.w - 6.0,
            rect.h - 6.0,
            Color::new(0.75, 0.12, 0.06, 0.22),
        );
        draw_rectangle_lines(
            rect.x + 4.0,
            rect.y + 4.0,
            rect.w - 8.0,
            rect.h - 8.0,
            3.0,
            warning,
        );
        let label = format!("IN R{}", wave.round);
        let dimensions = measure_text(&label, None, 14, 1.0);
        draw_text_ex(
            &label,
            rect.x + (rect.w - dimensions.width) * 0.5,
            rect.y + rect.h * 0.56,
            TextStyle::new(14.0, warning).params(),
        );
    }
}
