//! Touch-visible Salvage Yard choices for recovered operation objects.

use crate::campaign::{
    CampaignState, SalvageChoice, SALVAGE_MATERIALS_REWARD, SALVAGE_RESEARCH_INSIGHT,
};
use crate::colony::BuildingKind;
use crate::ui::UiAction;
use crate::ui_widgets::button;
use macroquad::prelude::*;
use macroquad_toolkit::prelude::{dark, draw_surface_with_title, SurfaceStyle, TextStyle};

const SALVAGE_RECT: Rect = Rect::new(1064.0, 312.0, 176.0, 32.0);

pub(super) fn draw_launcher(campaign: &CampaignState, mouse: Vec2, actions: &mut Vec<UiAction>) {
    if !campaign
        .colony
        .buildings
        .iter()
        .any(|building| building.kind == BuildingKind::SalvageYard)
    {
        return;
    }
    let label = if campaign.salvage_yard_operation == Some(campaign.operations_completed) {
        "SALVAGE // SORTED".to_owned()
    } else if campaign.salvage_cache_count == 0 {
        "SALVAGE // NO OBJECTS".to_owned()
    } else {
        format!(
            "SALVAGE // {} OBJECT{}",
            campaign.salvage_cache_count,
            if campaign.salvage_cache_count == 1 {
                ""
            } else {
                "S"
            }
        )
    };
    if button(SALVAGE_RECT, &label, campaign.salvage_available(), mouse) {
        actions.push(UiAction::OpenSalvage);
    }
}

pub(super) fn draw_modal(campaign: &CampaignState, mouse: Vec2, actions: &mut Vec<UiAction>) {
    let panel = Rect::new(862.0, 44.0, 408.0, 660.0);
    draw_surface_with_title(
        panel,
        Some("SALVAGE YARD // RECOVERY SORT"),
        &SurfaceStyle::new(Color::new(0.045, 0.075, 0.080, 0.995))
            .with_border(2.0, dark::ACCENT)
            .with_header(42.0, Color::new(0.08, 0.13, 0.13, 1.0)),
        TextStyle::new(17.0, dark::TEXT),
    );
    draw_text_ex(
        format!(
            "{} RECOVERED OBJECT{} // CHOOSE ONE OUTCOME",
            campaign.salvage_cache_count,
            if campaign.salvage_cache_count == 1 {
                ""
            } else {
                "S"
            }
        ),
        886.0,
        126.0,
        TextParams {
            font_size: 12,
            color: dark::TEXT_DIM,
            ..Default::default()
        },
    );
    draw_text_ex(
        "Each sort consumes one object and can be used once per operation.",
        886.0,
        148.0,
        TextParams {
            font_size: 11,
            color: dark::TEXT_DIM,
            ..Default::default()
        },
    );
    let choices = [
        (
            SalvageChoice::Materials,
            "MATERIALS",
            format!(
                "+{} materials for construction and craft",
                SALVAGE_MATERIALS_REWARD
            ),
            true,
        ),
        (
            SalvageChoice::ResearchInsight,
            "RESEARCH INSIGHT",
            format!(
                "Next doctrine costs -{} materials",
                SALVAGE_RESEARCH_INSIGHT
            ),
            campaign.research_insight == 0
                && campaign
                    .strategy
                    .research
                    .iter()
                    .any(|research| !research.completed),
        ),
        (
            SalvageChoice::Prototype,
            "EQUIPMENT PROTOTYPE",
            "One standard equipment item can be crafted free".to_owned(),
            true,
        ),
    ];
    for (index, (choice, title, description, enabled)) in choices.into_iter().enumerate() {
        let y = 184.0 + index as f32 * 94.0;
        draw_rectangle(886.0, y, 360.0, 72.0, Color::new(0.025, 0.055, 0.060, 1.0));
        draw_text_ex(
            title,
            902.0,
            y + 22.0,
            TextParams {
                font_size: 15,
                color: dark::TEXT_BRIGHT,
                ..Default::default()
            },
        );
        draw_text_ex(
            &description,
            902.0,
            y + 43.0,
            TextParams {
                font_size: 10,
                color: dark::TEXT_DIM,
                ..Default::default()
            },
        );
        if button(
            Rect::new(1080.0, y + 48.0, 150.0, 20.0),
            if enabled {
                "SORT THIS WAY"
            } else {
                "INSIGHT WAITING"
            },
            enabled,
            mouse,
        ) {
            actions.push(UiAction::ProcessSalvage(choice));
        }
    }
    if button(
        Rect::new(1060.0, 662.0, 170.0, 30.0),
        "BACK TO OPERATIONS",
        true,
        mouse,
    ) {
        actions.push(UiAction::CloseSalvage);
    }
}
