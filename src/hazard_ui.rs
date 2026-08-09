//! Compact battlefield hazard presentation.

use crate::data::HazardKind;
use macroquad::prelude::*;
use macroquad_toolkit::prelude::{dark, TextStyle};

pub(crate) fn draw_legend(panel: Rect) {
    draw_text_ex(
        "HAZARDS // + FIRE LANE   O SPORE BLOOM   <> STATIC RIFT",
        panel.x + 310.0,
        panel.y + 54.0,
        TextStyle::new(12.0, dark::TEXT_DIM).params(),
    );
}

pub(crate) fn draw_tile(rect: Rect, kind: HazardKind) {
    let center = vec2(rect.x + rect.w * 0.5, rect.y + rect.h * 0.5);
    let color = match kind {
        HazardKind::FireLane => Color::new(0.96, 0.34, 0.18, 0.9),
        HazardKind::SporeBloom => Color::new(0.58, 0.84, 0.22, 0.9),
        HazardKind::StaticRift => Color::new(0.68, 0.42, 0.96, 0.9),
    };
    draw_rectangle(
        rect.x + 3.0,
        rect.y + 3.0,
        rect.w - 6.0,
        rect.h - 6.0,
        Color::new(color.r, color.g, color.b, 0.12),
    );
    match kind {
        HazardKind::FireLane => {
            draw_line(
                rect.x + 8.0,
                center.y,
                rect.right() - 8.0,
                center.y,
                3.0,
                color,
            );
            draw_line(
                center.x,
                rect.y + 8.0,
                center.x,
                rect.bottom() - 8.0,
                3.0,
                color,
            );
        }
        HazardKind::SporeBloom => {
            draw_circle_lines(center.x, center.y, rect.w * 0.22, 3.0, color);
            draw_circle(center.x - 9.0, center.y + 7.0, 3.0, color);
            draw_circle(center.x + 10.0, center.y - 8.0, 3.0, color);
        }
        HazardKind::StaticRift => draw_poly(center.x, center.y, 4, rect.w * 0.25, 45.0, color),
    }
}
