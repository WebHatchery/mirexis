//! Touch-visible register for every permanent cost and legacy carried by the colony.

use crate::campaign::{
    CampaignState, CharacterLegacy, CharacterRecord, InjuryRecord, LostObjectiveRecord,
};
use crate::trauma::TraumaRecord;
use crate::ui::UiAction;
use crate::ui_widgets::button;
use macroquad::prelude::*;
use macroquad_toolkit::prelude::{dark, draw_surface_with_title, SurfaceStyle, TextStyle};

const LAUNCHER_BOUNDS: Rect = Rect::new(1064.0, 130.0, 176.0, 24.0);
const PANEL: Rect = Rect::new(160.0, 60.0, 960.0, 600.0);
const PAGE_SIZE: usize = 6;

#[cfg(test)]
mod tests;

pub(crate) fn record_count(campaign: &CampaignState) -> usize {
    memorial_entries(campaign).len()
}

pub(crate) fn page_count(campaign: &CampaignState) -> usize {
    let entry_count = memorial_entries(campaign).len();
    if entry_count == 0 {
        1
    } else {
        entry_count.div_ceil(PAGE_SIZE)
    }
}

pub(crate) fn previous_page(page: usize) -> usize {
    page.saturating_sub(1)
}

pub(crate) fn next_page(page: usize, pages: usize) -> usize {
    page.saturating_add(1).min(pages.saturating_sub(1))
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

pub(crate) fn draw_modal(
    campaign: &CampaignState,
    requested_page: usize,
    mouse: Vec2,
    actions: &mut Vec<UiAction>,
) {
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
            "{} RECORD{} // SCARS, RECOVERY, LOST OBJECTIVES, AND THE WORK PEOPLE LEFT BEHIND",
            record_count(campaign),
            if record_count(campaign) == 1 { "" } else { "S" }
        ),
        190.0,
        142.0,
        TextStyle::new(12.0, dark::TEXT_DIM).params(),
    );

    let entries = memorial_entries(campaign);
    let pages = page_count(campaign);
    let page = requested_page.min(pages.saturating_sub(1));
    let start = page * PAGE_SIZE;
    let end = (start + PAGE_SIZE).min(entries.len());
    if entries.is_empty() {
        draw_text_ex(
            "NO PERMANENT RECORDS YET // THE COLONY HAS NOT FORGOTTEN, BUT IT HAS NOT HAD TO MARK THE COST.",
            190.0,
            222.0,
            TextStyle::new(13.0, dark::TEXT_DIM).params(),
        );
    } else {
        for (index, entry) in entries[start..end].iter().enumerate() {
            match entry {
                MemorialEntry::Character { character, detail } => {
                    draw_record_row(character, *detail, index)
                }
                MemorialEntry::LostObjective(objective) => {
                    draw_lost_objective_row(objective, index)
                }
            }
        }
    }
    draw_page_controls(page, pages, start, end, entries.len(), mouse, actions);
}

fn draw_page_controls(
    page: usize,
    pages: usize,
    start: usize,
    end: usize,
    entry_count: usize,
    mouse: Vec2,
    actions: &mut Vec<UiAction>,
) {
    let page_status = if entry_count == 0 {
        "PAGE 1 / 1 // EMPTY REGISTER".to_owned()
    } else {
        format!(
            "PAGE {} / {} // ENTRIES {}-{} OF {}",
            page + 1,
            pages,
            start + 1,
            end,
            entry_count
        )
    };
    draw_text_ex(
        page_status,
        190.0,
        582.0,
        TextStyle::new(11.0, dark::TEXT_DIM).params(),
    );
    if button(
        Rect::new(492.0, 596.0, 138.0, 34.0),
        "PREVIOUS",
        page > 0,
        mouse,
    ) {
        actions.push(UiAction::PreviousMemorialPage);
    }
    if button(
        Rect::new(644.0, 596.0, 138.0, 34.0),
        "NEXT",
        page + 1 < pages,
        mouse,
    ) {
        actions.push(UiAction::NextMemorialPage);
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

#[derive(Clone, Copy)]
enum CharacterDetail<'a> {
    Trauma(&'a TraumaRecord),
    Injury(&'a InjuryRecord),
    Legacy(&'a CharacterLegacy),
}

enum MemorialEntry<'a> {
    Character {
        character: &'a CharacterRecord,
        detail: CharacterDetail<'a>,
    },
    LostObjective(&'a LostObjectiveRecord),
}

fn memorial_entries(campaign: &CampaignState) -> Vec<MemorialEntry<'_>> {
    let mut entries = Vec::new();
    for character in &campaign.roster {
        entries.extend(
            character
                .traumas
                .iter()
                .map(|record| MemorialEntry::Character {
                    character,
                    detail: CharacterDetail::Trauma(record),
                }),
        );
        entries.extend(
            character
                .injuries
                .iter()
                .map(|record| MemorialEntry::Character {
                    character,
                    detail: CharacterDetail::Injury(record),
                }),
        );
        entries.extend(
            character
                .event_legacies
                .iter()
                .map(|record| MemorialEntry::Character {
                    character,
                    detail: CharacterDetail::Legacy(record),
                }),
        );
    }
    entries.extend(
        campaign
            .lost_objectives
            .iter()
            .map(MemorialEntry::LostObjective),
    );
    entries
}

fn draw_record_row(character: &CharacterRecord, detail: CharacterDetail<'_>, index: usize) {
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
    let (label, description) = match detail {
        CharacterDetail::Trauma(trauma) => {
            (format!("SCAR // {}", trauma.name), trauma.effect.clone())
        }
        CharacterDetail::Injury(injury) => (
            format!("RECOVERY // {}", injury.name),
            format!("{} OPS REMAIN", injury.recovery_operations),
        ),
        CharacterDetail::Legacy(legacy) => (
            format!("LEGACY // {}", legacy.name),
            format!("+{} {}", legacy.amount, legacy.stat),
        ),
    };
    draw_text_ex(
        label,
        row.x + 14.0,
        row.y + 44.0,
        TextStyle::new(10.0, Color::new(0.82, 0.66, 0.32, 1.0)).params(),
    );
    draw_text_ex(
        description,
        row.x + 264.0,
        row.y + 34.0,
        TextStyle::new(12.0, dark::TEXT).params(),
    );
}

fn draw_lost_objective_row(objective: &LostObjectiveRecord, index: usize) {
    let row = Rect::new(190.0, 160.0 + index as f32 * 70.0, 860.0, 60.0);
    draw_rectangle(
        row.x,
        row.y,
        row.w,
        row.h,
        Color::new(0.10, 0.075, 0.065, 0.98),
    );
    draw_rectangle_lines(
        row.x,
        row.y,
        row.w,
        row.h,
        1.0,
        Color::new(0.60, 0.32, 0.22, 0.90),
    );
    draw_text_ex(
        format!("OBJECTIVE LOST // {}", objective.mission_name),
        row.x + 14.0,
        row.y + 23.0,
        TextStyle::new(15.0, dark::TEXT_BRIGHT).params(),
    );
    draw_text_ex(
        format!("OPERATION {}  //  ROUTE NOT RECOVERED", objective.operation),
        row.x + 14.0,
        row.y + 44.0,
        TextStyle::new(10.0, Color::new(0.92, 0.48, 0.32, 1.0)).params(),
    );
    draw_text_ex(
        &objective.objective,
        row.x + 264.0,
        row.y + 34.0,
        TextStyle::new(12.0, dark::TEXT).params(),
    );
}
