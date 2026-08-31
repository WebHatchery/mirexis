//! Contextual colony emphasis for first-hour mission handoffs.

use crate::first_hour::{FirstHourProgress, FirstHourStage};
use macroquad::prelude::*;
use macroquad_toolkit::prelude::{dark, TextStyle};

const OPERATIONS_BUTTON: Rect = Rect::new(1028.0, 22.0, 124.0, 28.0);
const BRIEFING_BUTTON: Rect = Rect::new(878.0, 478.0, 362.0, 30.0);
const INVESTMENT_CHOICES: Rect = Rect::new(878.0, 348.0, 362.0, 212.0);

pub(crate) fn draw_focus(
    progress: &FirstHourProgress,
    operations_open: bool,
    dialogue_target: Option<&str>,
) {
    let Some((rect, _)) = focus_target(progress, operations_open, dialogue_target) else {
        return;
    };
    let color = focus_color();
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
    draw_text_ex(
        "NEXT",
        rect.x + 2.0,
        rect.y - 7.0,
        TextStyle::new(10.0, dark::ACCENT).params(),
    );
}

fn focus_target(
    progress: &FirstHourProgress,
    operations_open: bool,
    dialogue_target: Option<&str>,
) -> Option<(Rect, &'static str)> {
    if !progress.guidance_enabled || progress.help_open {
        return None;
    }
    if dialogue_target.is_some() && dialogue_target == progress.colony_guidance_target() {
        return Some((
            crate::colony_exploration::dialogue_continue_button_bounds(),
            "CONTINUE",
        ));
    }
    match progress.stage {
        FirstHourStage::PrepareFirstOperation | FirstHourStage::SecondOperation => {
            Some(if operations_open {
                (BRIEFING_BUTTON, "BRIEF SELECTED MISSION")
            } else {
                (OPERATIONS_BUTTON, "OPERATIONS")
            })
        }
        FirstHourStage::MakeInvestment => Some(if operations_open {
            (INVESTMENT_CHOICES, "CHOOSE PREPARATION")
        } else {
            (OPERATIONS_BUTTON, "OPERATIONS")
        }),
        _ => None,
    }
}

fn focus_color() -> Color {
    Color::new(1.0, 0.74, 0.18, 0.96)
}

#[cfg(test)]
mod tests;
