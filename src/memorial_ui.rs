//! Touch-visible register for the permanent costs and legacies carried by the colony.

use crate::campaign::{CampaignState, CharacterRecord};
use crate::ui::UiAction;
use crate::ui_widgets::button;
use macroquad::prelude::*;
use macroquad_toolkit::prelude::{dark, draw_surface_with_title, SurfaceStyle, TextStyle};

const LAUNCHER_BOUNDS: Rect = Rect::new(1064.0, 130.0, 176.0, 24.0);
const PANEL: Rect = Rect::new(160.0, 60.0, 960.0, 600.0);

#[cfg(test)]
mod tests;

pub(crate) fn record_count(campaign: &CampaignState) -> usize {
    campaign
        .roster
        .iter()
        .map(|character| {
            character.injuries.len() + character.traumas.len() + character.event_legacies.len()
        })
        .sum()
}

pub(crate) fn draw_launcher(campaign: &CampaignState, mouse: Vec2, actions: &mut Vec<UiAction>) {
    let count = record_count(campaign);
    let label = if count == 0 {
        "MEMORIAL // READY".to_owned()
    } else {
        format!("MEMORIAL // {} RECORDS", count)
    };
    if button(LAUNCHER_BOUNDS, &label, true, mouse) {
        actions.push(UiAction::ToggleMemorial);
    }
}

pub(crate) fn draw_modal(campaign: &CampaignState, mouse: Vec2, actions: &mut Vec<UiAction>) {
    actions.clear();
    draw_rectangle(0.0, 0.0, 1280.0, 720.0, Color::new(0.01, 0.02, 0.025, 0.88));
    draw_surface_with_title(
        PANEL,
        Some("MEMORIAL REGISTER // COLONY RECORDS"),
        &SurfaceStyle::new(Color::new(0.045, 0.075, 0.080, 0.995))
            .with_border(2.0, Color::new(0.82, 0.66, 0.32, 1.0))
            .with_header(48.0, Color::new(0.10, 0.12, 0.11, 1.0)),
        TextStyle::new(18.0, dark::TEXT),
    );
    draw_text_ex(
        format!(
            "{} RECORD{} // SCARS, RECOVERY, AND THE WORK PEOPLE LEFT BEHIND",
            record_count(campaign),
            if record_count(campaign) == 1 { "" } else { "S" }
        ),
        190.0,
        142.0,
        TextStyle::new(12.0, dark::TEXT_DIM).params(),
    );

    let records = campaign
        .roster
        .iter()
        .filter(|character| character_has_records(character))
        .collect::<Vec<_>>();
    if records.is_empty() {
        draw_text_ex(
            "NO PERMANENT RECORDS YET // THE COLONY HAS NOT FORGOTTEN, BUT IT HAS NOT HAD TO MARK THE COST.",
            190.0,
            222.0,
            TextStyle::new(13.0, dark::TEXT_DIM).params(),
        );
    } else {
        for (index, character) in records.into_iter().take(6).enumerate() {
            draw_record_row(character, index);
        }
    }
    if button(
        Rect::new(190.0, 596.0, 286.0, 34.0),
        "BACK TO OPERATIONS",
        true,
        mouse,
    ) {
        actions.push(UiAction::ToggleMemorial);
    }
}

fn character_has_records(character: &CharacterRecord) -> bool {
    !character.injuries.is_empty()
        || !character.traumas.is_empty()
        || !character.event_legacies.is_empty()
}

fn draw_record_row(character: &CharacterRecord, index: usize) {
    let row = Rect::new(190.0, 160.0 + index as f32 * 70.0, 860.0, 60.0);
    draw_rectangle(
        row.x,
        row.y,
        row.w,
        row.h,
        Color::new(0.065, 0.095, 0.09, 0.98),
    );
    draw_rectangle_lines(
        row.x,
        row.y,
        row.w,
        row.h,
        1.0,
        Color::new(0.46, 0.38, 0.22, 0.85),
    );
    draw_text_ex(
        &character.name,
        row.x + 14.0,
        row.y + 23.0,
        TextStyle::new(17.0, dark::TEXT_BRIGHT).params(),
    );
    draw_text_ex(
        format!(
            "SCARS {}  //  RECOVERY {}  //  LEGACIES {}",
            character.traumas.len(),
            character.injuries.len(),
            character.event_legacies.len()
        ),
        row.x + 14.0,
        row.y + 44.0,
        TextStyle::new(10.0, Color::new(0.82, 0.66, 0.32, 1.0)).params(),
    );
    let detail = record_detail(character);
    draw_text_ex(
        detail,
        row.x + 264.0,
        row.y + 34.0,
        TextStyle::new(12.0, dark::TEXT).params(),
    );
}

fn record_detail(character: &CharacterRecord) -> String {
    let records = character
        .traumas
        .iter()
        .map(|trauma| format!("SCAR // {} ({})", trauma.name, trauma.effect))
        .chain(character.injuries.iter().map(|injury| {
            format!(
                "RECOVERY // {} ({} OPS)",
                injury.name, injury.recovery_operations
            )
        }))
        .chain(character.event_legacies.iter().map(|legacy| {
            format!(
                "LEGACY // {} (+{} {})",
                legacy.name, legacy.amount, legacy.stat
            )
        }))
        .collect::<Vec<_>>();
    match records.len() {
        0 => "NO RECORD DETAIL".to_owned(),
        1 | 2 => records.join(" · "),
        count => format!("{} · +{} MORE", records[..2].join(" · "), count - 2),
    }
}
