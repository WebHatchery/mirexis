//! Expandable recent battle-event history over the live tactical field.

use crate::state::GameSession;
use crate::ui::UiAction;
use crate::ui_widgets::{button, event_summary};
use macroquad::prelude::*;
use macroquad_toolkit::prelude::{dark, draw_surface, SurfaceStyle, TextStyle};

const VISIBLE_EVENTS: usize = 15;

pub(crate) fn draw(session: &GameSession, mouse: Vec2, actions: &mut Vec<UiAction>) {
    draw_rectangle(0.0, 0.0, 1280.0, 720.0, Color::new(0.01, 0.02, 0.025, 0.62));
    let panel = Rect::new(690.0, 88.0, 550.0, 544.0);
    draw_surface(
        panel,
        &SurfaceStyle::new(Color::new(0.045, 0.065, 0.07, 0.99))
            .with_border(2.0, Color::new(0.95, 0.55, 0.25, 0.9)),
    );
    draw_text_ex(
        "BATTLE HISTORY // RECENT EVENTS",
        panel.x + 24.0,
        panel.y + 42.0,
        TextStyle::new(24.0, dark::TEXT_BRIGHT).params(),
    );
    draw_text_ex(
        "Oldest at top · newest at bottom · review pauses commands",
        panel.x + 24.0,
        panel.y + 67.0,
        TextStyle::new(13.0, dark::TEXT_DIM).params(),
    );
    let start = session
        .tactical
        .event_log
        .len()
        .saturating_sub(VISIBLE_EVENTS);
    for (row, event) in session.tactical.event_log[start..].iter().enumerate() {
        let y = panel.y + 100.0 + row as f32 * 25.0;
        draw_text_ex(
            format!("{:02}", start + row + 1),
            panel.x + 24.0,
            y,
            TextStyle::new(12.0, Color::new(0.46, 0.66, 0.62, 1.0)).params(),
        );
        draw_text_ex(
            event_summary(event).replace('_', " "),
            panel.x + 62.0,
            y,
            TextStyle::new(14.0, event_color(event)).params(),
        );
    }
    if session.tactical.event_log.is_empty() {
        draw_text_ex(
            "No tactical events recorded yet.",
            panel.x + 24.0,
            panel.y + 118.0,
            TextStyle::new(15.0, dark::TEXT_DIM).params(),
        );
    }
    if button(
        Rect::new(
            panel.x + panel.w - 210.0,
            panel.bottom() - 60.0,
            180.0,
            40.0,
        ),
        "RETURN TO BATTLE",
        true,
        mouse,
    ) {
        actions.push(UiAction::ToggleBattleLog);
    }
}

fn event_color(event: &crate::state::BattleEvent) -> Color {
    match event {
        crate::state::BattleEvent::DamageApplied { .. }
        | crate::state::BattleEvent::UnitIncapacitated { .. }
        | crate::state::BattleEvent::HazardTriggered { .. } => Color::new(0.98, 0.55, 0.38, 1.0),
        crate::state::BattleEvent::PhaseStarted { .. }
        | crate::state::BattleEvent::BattleEnded { .. } => Color::new(0.46, 0.86, 0.72, 1.0),
        _ => dark::TEXT_DIM,
    }
}
