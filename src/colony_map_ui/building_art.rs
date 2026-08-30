//! Procedural accents that distinguish special colony facilities on the map.

use macroquad::prelude::*;

pub(super) fn draw_waystation(center: Vec2, zoom: f32, powered: bool, damaged: bool) {
    let accent = if damaged {
        Color::new(1.0, 0.28, 0.20, 0.92)
    } else if powered {
        Color::new(0.40, 0.82, 1.0, 0.92)
    } else {
        Color::new(0.92, 0.60, 0.20, 0.80)
    };
    draw_line(
        center.x,
        center.y - 52.0 * zoom,
        center.x,
        center.y - 18.0 * zoom,
        2.0 * zoom,
        accent,
    );
    draw_circle(
        center.x,
        center.y - 55.0 * zoom,
        5.0 * zoom,
        Color::new(accent.r, accent.g, accent.b, 0.22),
    );
    draw_circle_lines(
        center.x,
        center.y - 55.0 * zoom,
        11.0 * zoom,
        1.5 * zoom,
        Color::new(accent.r, accent.g, accent.b, 0.64),
    );
    draw_circle_lines(
        center.x,
        center.y - 55.0 * zoom,
        18.0 * zoom,
        1.0 * zoom,
        Color::new(accent.r, accent.g, accent.b, 0.34),
    );
}

pub(super) fn draw_commons(center: Vec2, zoom: f32, powered: bool, damaged: bool) {
    let accent = if damaged {
        Color::new(1.0, 0.28, 0.20, 0.92)
    } else if powered {
        Color::new(1.0, 0.72, 0.30, 0.94)
    } else {
        Color::new(0.66, 0.48, 0.26, 0.82)
    };
    draw_rectangle(
        center.x - 24.0 * zoom,
        center.y - 47.0 * zoom,
        48.0 * zoom,
        6.0 * zoom,
        accent,
    );
    draw_line(
        center.x - 21.0 * zoom,
        center.y - 41.0 * zoom,
        center.x - 16.0 * zoom,
        center.y - 24.0 * zoom,
        2.0 * zoom,
        accent,
    );
    draw_line(
        center.x + 21.0 * zoom,
        center.y - 41.0 * zoom,
        center.x + 16.0 * zoom,
        center.y - 24.0 * zoom,
        2.0 * zoom,
        accent,
    );
    for offset in [-14.0, 0.0, 14.0] {
        draw_circle(
            center.x + offset * zoom,
            center.y - 51.0 * zoom,
            2.5 * zoom,
            Color::new(1.0, 0.86, 0.46, if powered { 0.95 } else { 0.32 }),
        );
    }
}

pub(super) fn draw_relay_mast(center: Vec2, zoom: f32, powered: bool, damaged: bool) {
    let accent = if damaged {
        Color::new(1.0, 0.28, 0.20, 0.92)
    } else if powered {
        Color::new(0.48, 0.72, 1.0, 0.94)
    } else {
        Color::new(0.38, 0.46, 0.62, 0.82)
    };
    draw_line(
        center.x,
        center.y - 56.0 * zoom,
        center.x,
        center.y - 17.0 * zoom,
        2.0 * zoom,
        accent,
    );
    draw_line(
        center.x - 15.0 * zoom,
        center.y - 43.0 * zoom,
        center.x + 15.0 * zoom,
        center.y - 43.0 * zoom,
        1.5 * zoom,
        accent,
    );
    draw_circle(
        center.x,
        center.y - 58.0 * zoom,
        4.0 * zoom,
        Color::new(
            accent.r,
            accent.g,
            accent.b,
            if powered { 0.9 } else { 0.3 },
        ),
    );
    draw_circle_lines(
        center.x,
        center.y - 58.0 * zoom,
        12.0 * zoom,
        1.2 * zoom,
        Color::new(accent.r, accent.g, accent.b, 0.56),
    );
}

pub(super) fn draw_watchtower(center: Vec2, zoom: f32, powered: bool, damaged: bool) {
    let accent = if damaged {
        Color::new(1.0, 0.28, 0.20, 0.92)
    } else if powered {
        Color::new(1.0, 0.86, 0.36, 0.94)
    } else {
        Color::new(0.58, 0.52, 0.30, 0.78)
    };
    draw_line(
        center.x,
        center.y - 54.0 * zoom,
        center.x,
        center.y - 19.0 * zoom,
        3.0 * zoom,
        accent,
    );
    draw_line(
        center.x - 14.0 * zoom,
        center.y - 44.0 * zoom,
        center.x + 14.0 * zoom,
        center.y - 44.0 * zoom,
        2.0 * zoom,
        accent,
    );
    draw_rectangle(
        center.x - 12.0 * zoom,
        center.y - 59.0 * zoom,
        24.0 * zoom,
        6.0 * zoom,
        accent,
    );
    draw_circle(
        center.x,
        center.y - 63.0 * zoom,
        4.0 * zoom,
        Color::new(
            accent.r,
            accent.g,
            accent.b,
            if powered { 0.95 } else { 0.30 },
        ),
    );
    if powered && !damaged {
        draw_line(
            center.x + 10.0 * zoom,
            center.y - 58.0 * zoom,
            center.x + 27.0 * zoom,
            center.y - 72.0 * zoom,
            1.5 * zoom,
            Color::new(1.0, 0.90, 0.48, 0.62),
        );
        draw_line(
            center.x + 27.0 * zoom,
            center.y - 72.0 * zoom,
            center.x + 24.0 * zoom,
            center.y - 63.0 * zoom,
            1.0 * zoom,
            Color::new(1.0, 0.90, 0.48, 0.44),
        );
    }
}
