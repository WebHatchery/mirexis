//! Touch-visible archive for acknowledged colony field notes.

use crate::colony_story::ColonyStoryState;
use crate::ui::UiAction;
use crate::ui_widgets::{button, button_with_state};
use macroquad::prelude::*;
use macroquad_toolkit::prelude::{dark, draw_surface_with_title, SurfaceStyle, TextStyle};

const LAUNCHER_BOUNDS: Rect = Rect::new(878.0, 350.0, 362.0, 24.0);
const NOTE_ROWS: usize = 8;
const PANEL: Rect = Rect::new(160.0, 60.0, 960.0, 600.0);

#[cfg(test)]
mod tests;

#[cfg(test)]
fn launcher_bounds() -> Rect {
    LAUNCHER_BOUNDS
}

pub(crate) fn draw_launcher(story: &ColonyStoryState, mouse: Vec2, actions: &mut Vec<UiAction>) {
    let label = format!("FIELD NOTES // {} ARCHIVED", story.archived_notes().len());
    if button(LAUNCHER_BOUNDS, &label, true, mouse) {
        actions.push(UiAction::ToggleFieldNotes);
    }
}

pub(crate) fn draw_modal(
    story: &ColonyStoryState,
    selected_index: usize,
    mouse: Vec2,
    actions: &mut Vec<UiAction>,
) {
    actions.clear();
    draw_rectangle(0.0, 0.0, 1280.0, 720.0, Color::new(0.01, 0.02, 0.025, 0.88));
    draw_surface_with_title(
        PANEL,
        Some("FIELD NOTES // STORY ARCHIVE"),
        &SurfaceStyle::new(Color::new(0.045, 0.075, 0.080, 0.995))
            .with_border(2.0, dark::ACCENT)
            .with_header(48.0, Color::new(0.08, 0.13, 0.13, 1.0)),
        TextStyle::new(18.0, dark::TEXT),
    );
    let notes = story.archived_notes();
    draw_ui_text_ex(
        &format!(
            "{} TRANSCRIPT{} // CHRONOLOGICAL ACKNOWLEDGEMENTS",
            notes.len(),
            if notes.len() == 1 { "" } else { "S" }
        ),
        190.0,
        142.0,
        TextStyle::new(12.0, dark::TEXT_DIM).params(),
    );

    if notes.is_empty() {
        let message = if story.acknowledged_count() > 0 {
            "Earlier acknowledgements have no transcript in this save. New field notes will be archived here."
        } else {
            "Approach a colonist and tap CONTINUE to archive their next field note."
        };
        draw_wrapped(message, 190.0, 220.0, 840.0);
    } else {
        let selected = selected_index.min(notes.len() - 1);
        let start = note_window_start(selected, notes.len());
        draw_ui_text_ex(
            "ARCHIVE // TAP A NOTE TO READ",
            190.0,
            164.0,
            TextStyle::new(10.0, dark::ACCENT).params(),
        );
        for row in 0..NOTE_ROWS {
            let index = start + row;
            let Some(note) = notes.get(index) else {
                break;
            };
            let bounds = note_row_bounds(row);
            if button_with_state(bounds, "", true, index == selected, mouse) {
                actions.push(UiAction::SelectFieldNote(index));
            }
            draw_ui_text_ex(
                &format!("{} // {}", index + 1, note.title.to_uppercase()),
                bounds.x + 10.0,
                bounds.y + 16.0,
                TextStyle::new(10.0, dark::TEXT_BRIGHT).params(),
            );
            draw_ui_text_ex(
                &note.speaker.to_uppercase(),
                bounds.x + 10.0,
                bounds.bottom() - 5.0,
                TextStyle::new(8.0, dark::TEXT_DIM).params(),
            );
        }
        let note = &notes[selected];
        draw_ui_text_ex(
            &note.speaker.to_uppercase(),
            520.0,
            190.0,
            TextStyle::new(13.0, dark::ACCENT).params(),
        );
        draw_ui_text_ex(
            &note.title,
            520.0,
            224.0,
            TextStyle::new(26.0, dark::TEXT_BRIGHT).params(),
        );
        draw_wrapped(&note.text, 520.0, 270.0, 520.0);
        draw_ui_text_ex(
            &format!("ARCHIVED NOTE {} // {}", selected + 1, notes.len()),
            520.0,
            522.0,
            TextStyle::new(11.0, dark::TEXT_DIM).params(),
        );
        if button(
            Rect::new(190.0, 522.0, 136.0, 30.0),
            "OLDER NOTES",
            selected > 0,
            mouse,
        ) {
            actions.push(UiAction::SelectFieldNote(selected - 1));
        }
        if button(
            Rect::new(340.0, 522.0, 136.0, 30.0),
            "NEWER NOTES",
            selected + 1 < notes.len(),
            mouse,
        ) {
            actions.push(UiAction::SelectFieldNote(selected + 1));
        }
    }
    if button(
        Rect::new(190.0, 596.0, 286.0, 34.0),
        "BACK TO OPERATIONS",
        true,
        mouse,
    ) {
        actions.push(UiAction::ToggleFieldNotes);
    }
}

fn note_window_start(selected: usize, note_count: usize) -> usize {
    if note_count <= NOTE_ROWS {
        0
    } else {
        selected
            .saturating_sub(NOTE_ROWS - 1)
            .min(note_count - NOTE_ROWS)
    }
}

fn note_row_bounds(row: usize) -> Rect {
    Rect::new(190.0, 176.0 + row as f32 * 42.0, 286.0, 36.0)
}

fn draw_wrapped(value: &str, x: f32, y: f32, width: f32) {
    let mut line = String::new();
    let mut baseline = y;
    for word in value.split_whitespace() {
        let candidate = if line.is_empty() {
            word.to_owned()
        } else {
            format!("{line} {word}")
        };
        if measure_text(&candidate, None, 16, 1.0).width > width && !line.is_empty() {
            draw_ui_text_ex(
                &line,
                x,
                baseline,
                TextStyle::new(16.0, dark::TEXT).params(),
            );
            baseline += 24.0;
            line = word.to_owned();
        } else {
            line = candidate;
        }
    }
    if !line.is_empty() {
        draw_ui_text_ex(
            &line,
            x,
            baseline,
            TextStyle::new(16.0, dark::TEXT).params(),
        );
    }
}

fn draw_ui_text_ex<'a>(value: &str, x: f32, y: f32, params: TextParams<'a>) -> TextDimensions {
    crate::ui::draw_ui_text_ex(value, x, y, params)
}
