//! Static ambient signals that make each identity building feel inhabited.

use crate::colony::BuildingKind;
use macroquad::prelude::*;

pub(super) fn draw_ambient(
    kind: BuildingKind,
    center: Vec2,
    zoom: f32,
    powered: bool,
    damaged: bool,
) {
    let accent = if damaged {
        Color::new(1.0, 0.26, 0.20, 0.72)
    } else {
        match kind {
            BuildingKind::RedoubtArsenal => Color::new(0.38, 0.78, 1.0, 0.72),
            BuildingKind::ChoirGarden => Color::new(0.62, 0.98, 0.36, 0.72),
            BuildingKind::ThresholdSpire => Color::new(0.76, 0.58, 1.0, 0.76),
            _ => return,
        }
    };
    let opacity = if powered { 1.0 } else { 0.35 };
    match kind {
        BuildingKind::RedoubtArsenal => {
            for offset in [-18.0, 0.0, 18.0] {
                draw_line(
                    center.x + offset * zoom,
                    center.y - 10.0 * zoom,
                    center.x + offset * zoom + 6.0 * zoom,
                    center.y - 16.0 * zoom,
                    1.5 * zoom,
                    Color::new(accent.r, accent.g, accent.b, accent.a * opacity),
                );
            }
        }
        BuildingKind::ChoirGarden => {
            for (x, y, radius) in [(-18.0, -9.0, 2.0), (15.0, -24.0, 2.5), (24.0, -4.0, 1.5)] {
                draw_circle(
                    center.x + x * zoom,
                    center.y + y * zoom,
                    radius * zoom,
                    Color::new(accent.r, accent.g, accent.b, accent.a * opacity),
                );
            }
        }
        BuildingKind::ThresholdSpire => {
            draw_circle_lines(
                center.x,
                center.y - 24.0 * zoom,
                31.0 * zoom,
                1.2 * zoom,
                Color::new(accent.r, accent.g, accent.b, accent.a * opacity),
            );
            draw_circle_lines(
                center.x,
                center.y - 24.0 * zoom,
                37.0 * zoom,
                1.0 * zoom,
                Color::new(accent.r, accent.g, accent.b, accent.a * opacity * 0.45),
            );
        }
        _ => {}
    }
}
