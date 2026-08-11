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
    let points = [
        vec2(center.x, rect.y),
        vec2(rect.right(), center.y),
        vec2(center.x, rect.bottom()),
        vec2(rect.x, center.y),
    ];
    draw_triangle(
        points[0],
        points[1],
        points[2],
        Color::new(color.r, color.g, color.b, 0.20),
    );
    draw_triangle(
        points[0],
        points[2],
        points[3],
        Color::new(color.r, color.g, color.b, 0.20),
    );
    match kind {
        HazardKind::FireLane => {
            for t in [0.28_f32, 0.50, 0.72] {
                let start = points[3].lerp(points[0], t);
                let end = points[2].lerp(points[1], t);
                draw_line(start.x, start.y, end.x, end.y, 2.0, color);
            }
        }
        HazardKind::SporeBloom => {
            draw_ellipse(
                center.x,
                center.y,
                rect.w * 0.18,
                rect.h * 0.24,
                0.0,
                Color::new(color.r, color.g, color.b, 0.38),
            );
            draw_circle_lines(center.x, center.y, rect.h * 0.30, 2.0, color);
            draw_circle(center.x - 9.0, center.y + 7.0, 3.0, color);
            draw_circle(center.x + 10.0, center.y - 8.0, 3.0, color);
        }
        HazardKind::StaticRift => {
            draw_poly(center.x, center.y, 4, rect.h * 0.34, 45.0, color);
            draw_poly_lines(
                center.x,
                center.y,
                6,
                rect.h * 0.47,
                0.0,
                2.0,
                Color::new(color.r, color.g, color.b, 0.72),
            );
        }
    }
}
