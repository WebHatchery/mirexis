//! Touch-visible non-linear doctrine choices in the Operations panel.

use crate::campaign::CampaignState;
use crate::ui::UiAction;
use crate::visual_assets::VisualCatalog;
use macroquad::prelude::*;
use macroquad_toolkit::assets::AssetManager;
use macroquad_toolkit::prelude::{dark, TextStyle};

const RESEARCH_HEADING_Y: f32 = 518.0;
const RESEARCH_ROW_START_Y: f32 = 524.0;
const RESEARCH_ROW_STEP_Y: f32 = 40.0;
const RESEARCH_ROW_HEIGHT: f32 = 34.0;

#[cfg(test)]
mod tests;

pub(super) fn has_pending(campaign: &CampaignState) -> bool {
    campaign
        .strategy
        .research
        .iter()
        .any(|research| !research.completed)
}

pub(super) fn draw_available(
    campaign: &CampaignState,
    assets: &AssetManager,
    visuals: &VisualCatalog,
    mouse: Vec2,
    actions: &mut Vec<UiAction>,
) {
    let pending = campaign
        .strategy
        .research
        .iter()
        .filter(|research| !research.completed)
        .collect::<Vec<_>>();
    if pending.is_empty() {
        return;
    }

    super::draw_ui_text_ex(
        "FIELD DOCTRINES // CHOOSE ONE",
        878.0,
        RESEARCH_HEADING_Y,
        TextStyle::new(12.0, dark::ACCENT).params(),
    );
    for (index, research) in pending.into_iter().enumerate() {
        let row = research_row_bounds(index);
        draw_rectangle(
            row.x,
            row.y,
            row.w,
            row.h,
            Color::new(0.035, 0.075, 0.08, 1.0),
        );
        draw_rectangle_lines(row.x, row.y, row.w, row.h, 1.0, dark::ACCENT);
        visuals.draw_atlas_cell(
            assets,
            &visuals.terrain,
            research_icon_cell(&research.id),
            Rect::new(row.x + 4.0, row.y + 5.0, 30.0, 28.0),
            WHITE,
        );
        super::draw_ui_text_ex(
            &research.name.to_uppercase(),
            row.x + 42.0,
            row.y + 15.0,
            TextStyle::new(10.5, dark::TEXT_BRIGHT).params(),
        );
        super::draw_ui_text_ex(
            &research.description,
            row.x + 42.0,
            row.y + 31.0,
            TextStyle::new(9.5, dark::TEXT_DIM).params(),
        );
        let cost = campaign.research_material_cost(research.materials_cost);
        if super::colony_button(
            Rect::new(row.right() - 144.0, row.y + 4.0, 138.0, 20.0),
            &super::research_affordance::choice_button_label(
                campaign.colony.resources.materials,
                cost,
            ),
            campaign.can_complete_research(&research.id),
            mouse,
        ) {
            actions.push(UiAction::CompleteResearch(research.id.clone()));
        }
    }
}

pub(super) fn draw_completed_summary(
    campaign: &CampaignState,
    assets: &AssetManager,
    visuals: &VisualCatalog,
    research_surface_visible: bool,
) {
    let completed = campaign
        .strategy
        .research
        .iter()
        .filter(|research| research.completed)
        .collect::<Vec<_>>();
    if research_surface_visible {
        let names = if completed.is_empty() {
            "NONE".to_owned()
        } else {
            completed
                .iter()
                .map(|research| research.name.to_uppercase())
                .collect::<Vec<_>>()
                .join(" / ")
        };
        super::draw_ui_text_ex(
            &format!("ACTIVE DOCTRINES // {names}"),
            878.0,
            656.0,
            TextStyle::new(9.5, dark::POSITIVE).params(),
        );
        return;
    }

    super::draw_ui_text_ex(
        "ACTIVE DOCTRINES",
        878.0,
        600.0,
        TextStyle::new(12.0, dark::ACCENT).params(),
    );
    if completed.is_empty() {
        super::draw_ui_text_ex(
            "No completed field doctrine",
            878.0,
            616.0,
            TextStyle::new(10.0, dark::TEXT_DIM).params(),
        );
    } else {
        for (index, research) in completed.into_iter().take(3).enumerate() {
            let rect = Rect::new(878.0 + index as f32 * 120.0, 606.0, 114.0, 32.0);
            draw_rectangle(
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                Color::new(0.045, 0.105, 0.11, 1.0),
            );
            visuals.draw_atlas_cell(
                assets,
                &visuals.terrain,
                [5, 8, 10][index],
                Rect::new(rect.x + 2.0, rect.y + 2.0, 30.0, 28.0),
                WHITE,
            );
            super::draw_ui_text_ex(
                &research.name.to_uppercase(),
                rect.x + 36.0,
                rect.y + 20.0,
                TextStyle::new(10.0, dark::TEXT).params(),
            );
            draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1.0, dark::POSITIVE);
        }
    }
}

fn research_row_bounds(index: usize) -> Rect {
    Rect::new(
        878.0,
        RESEARCH_ROW_START_Y + index as f32 * RESEARCH_ROW_STEP_Y,
        362.0,
        RESEARCH_ROW_HEIGHT,
    )
}

fn research_icon_cell(id: &str) -> usize {
    match id {
        "xeno_triage" => 8,
        "salvage_doctrine" => 10,
        _ => 5,
    }
}
