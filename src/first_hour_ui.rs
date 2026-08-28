//! Non-blocking first-hour goal banner and revisitable help surface.

use crate::first_hour::{FirstHourProgress, FirstHourStage};
use crate::ui::UiAction;
use crate::ui_widgets::button;
use macroquad::prelude::*;
use macroquad_toolkit::prelude::{dark, draw_surface, SurfaceStyle, TextStyle};

pub(crate) fn draw(progress: &FirstHourProgress, mouse: Vec2, actions: &mut Vec<UiAction>) {
    if progress.help_open {
        actions.clear();
        draw_help(progress, mouse, actions);
        return;
    }
    draw_goal(progress, mouse, actions);
}

fn draw_goal(progress: &FirstHourProgress, mouse: Vec2, actions: &mut Vec<UiAction>) {
    let panel = Rect::new(20.0, 76.0, 520.0, 72.0);
    draw_surface(
        panel,
        &SurfaceStyle::new(Color::new(0.025, 0.065, 0.07, 0.97))
            .with_border(1.0, Color::new(0.34, 0.86, 0.68, 0.94))
            .with_left_accent(5.0, Color::new(0.34, 0.86, 0.68, 1.0)),
    );
    text("PRIMARY GOAL", 38.0, 98.0, 12.0, dark::ACCENT);
    text(
        progress.visible_goal(),
        38.0,
        124.0,
        15.0,
        dark::TEXT_BRIGHT,
    );
    if button(Rect::new(426.0, 84.0, 98.0, 24.0), "HELP", true, mouse) {
        actions.push(UiAction::ToggleFirstHourHelp);
    }
    if progress.stage == FirstHourStage::Arrival
        && button(
            Rect::new(342.0, 114.0, 182.0, 26.0),
            "BEGIN ARRIVAL",
            true,
            mouse,
        )
    {
        actions.push(UiAction::AdvanceFirstHour);
    } else if progress.stage == FirstHourStage::Promise
        && button(
            Rect::new(330.0, 114.0, 194.0, 26.0),
            "CONTINUE CAMPAIGN",
            true,
            mouse,
        )
    {
        actions.push(UiAction::AdvanceFirstHour);
    }
}

fn draw_help(progress: &FirstHourProgress, mouse: Vec2, actions: &mut Vec<UiAction>) {
    draw_rectangle(0.0, 0.0, 1280.0, 720.0, Color::new(0.01, 0.02, 0.025, 0.84));
    let panel = Rect::new(200.0, 92.0, 880.0, 536.0);
    draw_surface(
        panel,
        &SurfaceStyle::new(Color::new(0.04, 0.07, 0.075, 0.99))
            .with_border(2.0, Color::new(0.34, 0.86, 0.68, 1.0)),
    );
    text(
        "FIRST-HOUR FIELD GUIDE",
        240.0,
        138.0,
        30.0,
        dark::TEXT_BRIGHT,
    );
    text("CURRENT GOAL", 240.0, 184.0, 13.0, dark::ACCENT);
    text(progress.visible_goal(), 240.0, 212.0, 17.0, dark::TEXT);
    section(
        240.0,
        262.0,
        "COLONY",
        &[
            "Tap ground to walk; tap a speech marker to approach a colonist.",
            "Tap OPERATIONS for missions and timely preparation choices.",
            "Tap TITLE to leave safely; campaign changes autosave.",
        ],
    );
    section(
        650.0,
        262.0,
        "TACTICAL",
        &[
            "Tap a colonist, then a green tile, hostile, or visible action.",
            "Forecasts show deterministic accuracy, cover, armour, and damage.",
            "Tap HELP in battle for the full field manual; tap SAVE at any time.",
        ],
    );
    draw_metrics(progress);
    if button(Rect::new(240.0, 542.0, 210.0, 42.0), "RETURN", true, mouse) {
        actions.push(UiAction::ToggleFirstHourHelp);
    }
    if button(
        Rect::new(466.0, 542.0, 210.0, 42.0),
        "RESTART GUIDE",
        true,
        mouse,
    ) {
        actions.push(UiAction::RestartFirstHourTutorial);
    }
    if button(
        Rect::new(692.0, 542.0, 210.0, 42.0),
        if progress.guidance_enabled {
            "SKIP PROMPTS"
        } else {
            "PROMPTS SKIPPED"
        },
        progress.guidance_enabled,
        mouse,
    ) {
        actions.push(UiAction::SkipFirstHourTutorial);
    }
    text(
        "Skipping hides teaching prompts but preserves every campaign goal.",
        240.0,
        608.0,
        13.0,
        dark::TEXT_DIM,
    );
}

fn draw_metrics(progress: &FirstHourProgress) {
    let metrics = &progress.metrics;
    text("SESSION METRICS", 240.0, 390.0, 16.0, dark::ACCENT);
    text(
        &format!(
            "Elapsed {}  ·  first move {}  ·  city interaction {}  ·  first attack {}",
            duration(Some(metrics.elapsed_millis)),
            duration(metrics.first_city_move_millis),
            duration(metrics.first_city_interaction_millis),
            duration(metrics.first_tactical_attack_millis),
        ),
        240.0,
        420.0,
        14.0,
        dark::TEXT_DIM,
    );
    text(
        &format!(
            "Operation one {}  ·  Operation two {}",
            operation(
                metrics.operation_one_duration_millis,
                metrics.operation_one_rounds
            ),
            operation(
                metrics.operation_two_duration_millis,
                metrics.operation_two_rounds
            ),
        ),
        240.0,
        448.0,
        14.0,
        dark::TEXT_DIM,
    );
    text(
        &format!(
            "Invalid commands {}  ·  field-guide opens {}  ·  observer records exact input method",
            metrics.invalid_commands, metrics.guide_opens
        ),
        240.0,
        476.0,
        14.0,
        dark::TEXT_DIM,
    );
}

fn operation(millis: Option<u64>, rounds: Option<u32>) -> String {
    match (millis, rounds) {
        (Some(millis), Some(rounds)) => format!("{} / R{}", duration(Some(millis)), rounds),
        _ => "pending".to_owned(),
    }
}

fn duration(millis: Option<u64>) -> String {
    let Some(total_seconds) = millis.map(|value| value / 1_000) else {
        return "—".to_owned();
    };
    format!("{}:{:02}", total_seconds / 60, total_seconds % 60)
}

fn section(x: f32, y: f32, title: &str, lines: &[&str]) {
    text(title, x, y, 16.0, dark::ACCENT);
    for (index, line) in lines.iter().enumerate() {
        text(
            line,
            x,
            y + 30.0 + index as f32 * 28.0,
            14.0,
            dark::TEXT_DIM,
        );
    }
}

fn text(value: &str, x: f32, y: f32, size: f32, color: Color) {
    draw_text_ex(value, x, y, TextStyle::new(size, color).params());
}
