//! In-battle quick reference that blocks tactical commands while open.

use crate::data::TutorialCopy;
use crate::ui::UiAction;
use crate::ui_widgets::button;
use macroquad::prelude::*;
use macroquad_toolkit::prelude::{dark, draw_surface, SurfaceStyle, TextStyle};

pub fn draw(copy: &TutorialCopy, mouse: Vec2, actions: &mut Vec<UiAction>) {
    draw_rectangle(0.0, 0.0, 1280.0, 720.0, Color::new(0.01, 0.02, 0.025, 0.82));
    let panel = Rect::new(190.0, 90.0, 900.0, 540.0);
    draw_surface(
        panel,
        &SurfaceStyle::new(Color::new(0.045, 0.07, 0.075, 0.99))
            .with_border(2.0, Color::new(0.32, 0.78, 0.63, 1.0)),
    );
    text(
        &copy.tactical_manual_title,
        230.0,
        138.0,
        30.0,
        dark::TEXT_BRIGHT,
    );
    text(
        &copy.tactical_manual_paused,
        230.0,
        166.0,
        15.0,
        dark::TEXT_DIM,
    );
    section(
        230.0,
        210.0,
        "1 // READ THE FIELD",
        &copy.tactical_read_field_lines,
    );
    section(
        230.0,
        350.0,
        "2 // SPEND THE PHASE",
        &copy.tactical_spend_phase_lines,
    );
    section(
        660.0,
        210.0,
        "3 // SURVIVE THE GROUND",
        &copy.tactical_survive_ground_lines,
    );
    section(
        660.0,
        350.0,
        "4 // WIN THE CONTRACT",
        &copy.tactical_win_contract_lines,
    );
    text(
        &copy.tactical_touch_controls,
        230.0,
        516.0,
        15.0,
        dark::ACCENT,
    );
    text(
        &copy.tactical_optional_controls,
        230.0,
        540.0,
        15.0,
        Color::new(0.60, 0.82, 0.96, 1.0),
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

pub fn section<S: AsRef<str>>(x: f32, y: f32, title: &str, lines: &[S]) {
    text(title, x, y, 17.0, Color::new(0.44, 0.88, 0.70, 1.0));
    for (index, line) in lines.iter().enumerate() {
        text(
            line.as_ref(),
            x,
            y + 30.0 + index as f32 * 24.0,
            15.0,
            dark::TEXT_DIM,
        );
    }
}

pub fn text(value: impl AsRef<str>, x: f32, y: f32, size: f32, color: Color) {
    draw_text_ex(value.as_ref(), x, y, TextStyle::new(size, color).params());
}
