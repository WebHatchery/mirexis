//! Non-blocking first-hour goal banner and revisitable help surface.

use crate::first_hour::{FirstHourProgress, FirstHourStage};
use crate::ui::UiAction;
use crate::ui_widgets::button;
use macroquad::prelude::*;
use macroquad_toolkit::prelude::{dark, draw_surface, draw_text_block, SurfaceStyle, TextStyle};

#[derive(Debug, Clone, Copy, PartialEq)]
struct GoalBannerLayout {
    panel: Rect,
    text_origin: Vec2,
    text_width: f32,
    text_height: f32,
    help_button: Rect,
}

pub(crate) fn draw(progress: &FirstHourProgress, mouse: Vec2, actions: &mut Vec<UiAction>) {
    if progress.help_open {
        actions.clear();
        draw_help(progress, mouse, actions);
        return;
    }
    draw_goal(progress, mouse, actions);
}

pub(crate) fn debrief_return_button_bounds() -> Rect {
    Rect::new(860.0, 574.0, 220.0, 48.0)
}

fn begin_arrival_button_bounds() -> Rect {
    Rect::new(342.0, 114.0, 182.0, 26.0)
}

fn continue_campaign_button_bounds() -> Rect {
    Rect::new(330.0, 114.0, 194.0, 26.0)
}

fn draw_goal(progress: &FirstHourProgress, mouse: Vec2, actions: &mut Vec<UiAction>) {
    let layout = goal_banner_layout(progress.stage);
    draw_surface(
        layout.panel,
        &SurfaceStyle::new(Color::new(0.025, 0.065, 0.07, 0.97))
            .with_border(1.0, Color::new(0.34, 0.86, 0.68, 0.94))
            .with_left_accent(5.0, Color::new(0.34, 0.86, 0.68, 1.0)),
    );
    text(
        "PRIMARY GOAL",
        layout.text_origin.x,
        layout.text_origin.y - 10.0,
        12.0,
        dark::ACCENT,
    );
    draw_text_block(
        progress.visible_goal(),
        layout.text_origin.x,
        layout.text_origin.y,
        layout.text_width,
        layout.text_height,
        15.0,
        2.0,
        dark::TEXT_BRIGHT,
    );
    if button(layout.help_button, "HELP", true, mouse) {
        actions.push(UiAction::ToggleFirstHourHelp);
    }
    match progress.stage {
        FirstHourStage::Arrival
            if button(begin_arrival_button_bounds(), "BEGIN ARRIVAL", true, mouse) =>
        {
            actions.push(UiAction::AdvanceFirstHour);
        }
        FirstHourStage::Promise
            if button(
                continue_campaign_button_bounds(),
                "CONTINUE CAMPAIGN",
                true,
                mouse,
            ) =>
        {
            actions.push(UiAction::AdvanceFirstHour);
        }
        _ => {}
    }
    if let Some(rect) = focus_target(progress) {
        draw_focus(rect);
    }
}

fn focus_target(progress: &FirstHourProgress) -> Option<Rect> {
    if !progress.guidance_enabled {
        return None;
    }
    advance_focus_target(progress.stage).or_else(|| debrief_focus_target(progress.stage))
}

fn advance_focus_target(stage: FirstHourStage) -> Option<Rect> {
    match stage {
        FirstHourStage::Arrival => Some(begin_arrival_button_bounds()),
        FirstHourStage::Promise => Some(continue_campaign_button_bounds()),
        _ => None,
    }
}

fn debrief_focus_target(stage: FirstHourStage) -> Option<Rect> {
    matches!(
        stage,
        FirstHourStage::FirstReturn | FirstHourStage::SecondReturn
    )
    .then_some(debrief_return_button_bounds())
}

fn draw_focus(rect: Rect) {
    let color = Color::new(1.0, 0.74, 0.18, 0.96);
    draw_rectangle_lines(
        rect.x - 4.0,
        rect.y - 4.0,
        rect.w + 8.0,
        rect.h + 8.0,
        2.0,
        color,
    );
    draw_rectangle_lines(
        rect.x - 1.0,
        rect.y - 1.0,
        rect.w + 2.0,
        rect.h + 2.0,
        1.0,
        Color::new(color.r, color.g, color.b, 0.42),
    );
    text("NEXT", rect.x + 2.0, rect.y - 7.0, 10.0, dark::ACCENT);
}

fn goal_banner_layout(stage: FirstHourStage) -> GoalBannerLayout {
    if matches!(
        stage,
        FirstHourStage::FirstReturn | FirstHourStage::SecondReturn
    ) {
        return GoalBannerLayout {
            panel: Rect::new(580.0, 8.0, 680.0, 60.0),
            text_origin: vec2(598.0, 37.0),
            text_width: 520.0,
            text_height: 24.0,
            help_button: Rect::new(1144.0, 16.0, 98.0, 24.0),
        };
    }

    GoalBannerLayout {
        panel: Rect::new(20.0, 76.0, 520.0, 96.0),
        text_origin: vec2(38.0, 108.0),
        text_width: 276.0,
        text_height: 58.0,
        help_button: Rect::new(426.0, 84.0, 98.0, 24.0),
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

#[cfg(test)]
mod tests;
