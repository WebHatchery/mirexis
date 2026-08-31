//! Audio settings overlay shared by title, city, and tactical screens.

use crate::audio::AudioSettings;
use crate::ui::UiAction;
use crate::ui_widgets::button;
use macroquad::prelude::*;
use macroquad_toolkit::prelude::{dark, draw_surface, SurfaceStyle, TextStyle};

#[cfg(test)]
mod tests;

pub(crate) fn draw_modal(
    settings: AudioSettings,
    open: bool,
    mouse: Vec2,
    actions: &mut Vec<UiAction>,
) {
    if !open {
        return;
    }
    actions.clear();
    draw_rectangle(0.0, 0.0, 1280.0, 720.0, Color::new(0.01, 0.02, 0.025, 0.86));
    let panel = Rect::new(390.0, 156.0, 500.0, 400.0);
    draw_surface(
        panel,
        &SurfaceStyle::new(Color::new(0.04, 0.07, 0.075, 0.99))
            .with_border(2.0, Color::new(0.34, 0.86, 0.68, 1.0)),
    );
    text("AUDIO & READABILITY", 432.0, 224.0, 28.0, dark::TEXT_BRIGHT);
    text(
        "All critical events remain visible while muted.",
        432.0,
        258.0,
        15.0,
        dark::TEXT_DIM,
    );
    text("MASTER VOLUME", 432.0, 314.0, 14.0, dark::ACCENT);
    let meter = volume_meter_bounds();
    draw_rectangle(
        meter.x,
        meter.y,
        meter.w,
        meter.h,
        Color::new(0.08, 0.13, 0.14, 1.0),
    );
    let fill_width = volume_meter_fill_width(settings.volume_percent);
    if fill_width > 0.0 {
        draw_rectangle(
            meter.x,
            meter.y,
            fill_width,
            meter.h,
            if settings.muted {
                dark::TEXT_DIM
            } else {
                dark::ACCENT
            },
        );
    }
    draw_rectangle_lines(
        meter.x,
        meter.y,
        meter.w,
        meter.h,
        1.0,
        Color::new(0.34, 0.86, 0.68, 0.7),
    );
    if button(Rect::new(432.0, 336.0, 72.0, 42.0), "-", true, mouse) {
        actions.push(UiAction::AudioVolumeDown);
    }
    text(
        &format!("{}%", settings.volume_percent),
        598.0,
        367.0,
        28.0,
        dark::TEXT_BRIGHT,
    );
    if button(Rect::new(746.0, 336.0, 72.0, 42.0), "+", true, mouse) {
        actions.push(UiAction::AudioVolumeUp);
    }
    if button(
        Rect::new(432.0, 398.0, 184.0, 42.0),
        if settings.muted { "UNMUTE" } else { "MUTE" },
        true,
        mouse,
    ) {
        actions.push(UiAction::ToggleMute);
    }
    if button(
        Rect::new(634.0, 398.0, 184.0, 42.0),
        if settings.reduced_motion {
            "REDUCED MOTION: ON"
        } else {
            "REDUCED MOTION: OFF"
        },
        true,
        mouse,
    ) {
        actions.push(UiAction::ToggleReducedMotion);
    }
    if button(Rect::new(432.0, 458.0, 386.0, 42.0), "RETURN", true, mouse) {
        actions.push(UiAction::ToggleSettings);
    }
    text(
        "Procedural palette // no external audio attribution required",
        432.0,
        534.0,
        12.0,
        dark::TEXT_DIM,
    );
}

fn volume_meter_bounds() -> Rect {
    Rect::new(432.0, 322.0, 386.0, 6.0)
}

fn volume_meter_fill_width(volume_percent: u8) -> f32 {
    volume_meter_bounds().w * volume_percent.min(100) as f32 / 100.0
}

fn text(value: &str, x: f32, y: f32, size: f32, color: Color) {
    draw_text_ex(value, x, y, TextStyle::new(size, color).params());
}
