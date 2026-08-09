//! In-battle quick reference that blocks tactical commands while open.

use crate::ui::UiAction;
use crate::ui_widgets::button;
use macroquad::prelude::*;
use macroquad_toolkit::prelude::{dark, draw_surface, SurfaceStyle, TextStyle};

pub(crate) fn draw(mouse: Vec2, actions: &mut Vec<UiAction>) {
    draw_rectangle(0.0, 0.0, 1280.0, 720.0, Color::new(0.01, 0.02, 0.025, 0.82));
    let panel = Rect::new(190.0, 90.0, 900.0, 540.0);
    draw_surface(
        panel,
        &SurfaceStyle::new(Color::new(0.045, 0.07, 0.075, 0.99))
            .with_border(2.0, Color::new(0.32, 0.78, 0.63, 1.0)),
    );
    text(
        "TACTICAL FIELD MANUAL",
        230.0,
        138.0,
        30.0,
        dark::TEXT_BRIGHT,
    );
    text(
        "The battle is paused while this reference is open.",
        230.0,
        166.0,
        15.0,
        dark::TEXT_DIM,
    );
    section(
        230.0,
        210.0,
        "1 // READ THE FIELD",
        &[
            "Click a colonist to make them active.",
            "Green tiles are reachable this activation.",
            "Hover/focus a tile to preview AP and danger.",
            "Click a hostile to inspect intent or attack.",
        ],
    );
    section(
        230.0,
        350.0,
        "2 // SPEND THE PHASE",
        &[
            "Movement and weapons spend action points.",
            "Class, mutation, gear, and overwatch add options.",
            "Enter or END COLONY PHASE releases hostile AI.",
            "Orange intent lines forecast the first response.",
        ],
    );
    section(
        660.0,
        210.0,
        "3 // SURVIVE THE GROUND",
        &[
            "+ Fire lane: 2 direct damage.",
            "O Spore bloom: 1 damage and Hindered.",
            "<> Static rift: Disrupted accuracy.",
            "Each hostile faction ignores its own hazard.",
        ],
    );
    section(
        660.0,
        350.0,
        "4 // WIN THE CONTRACT",
        &[
            "Read the objective and live progress at right.",
            "Gold rings mark objectives and extraction.",
            "Some missions require interaction; others survival.",
            "Save and load remain available during battle.",
        ],
    );
    text(
        "KEYS // ARROWS inspect tiles · ENTER end phase · S save · L load · H help",
        230.0,
        526.0,
        15.0,
        dark::ACCENT,
    );
    if button(
        Rect::new(
            panel.x + panel.w - 220.0,
            panel.bottom() - 68.0,
            180.0,
            42.0,
        ),
        "RETURN TO BATTLE",
        true,
        mouse,
    ) {
        actions.push(UiAction::ToggleTacticalHelp);
    }
}

fn section(x: f32, y: f32, title: &str, lines: &[&str]) {
    text(title, x, y, 17.0, Color::new(0.44, 0.88, 0.70, 1.0));
    for (index, line) in lines.iter().enumerate() {
        text(
            *line,
            x,
            y + 30.0 + index as f32 * 24.0,
            15.0,
            dark::TEXT_DIM,
        );
    }
}

fn text(value: impl AsRef<str>, x: f32, y: f32, size: f32, color: Color) {
    draw_text_ex(value.as_ref(), x, y, TextStyle::new(size, color).params());
}
