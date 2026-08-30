//! Touch-visible level-two facility choices for the colony operations panel.

use crate::campaign::CampaignState;
use crate::colony::BuildingKind;
use crate::ui::UiAction;
use crate::ui_widgets::button;
use macroquad::prelude::*;
use macroquad_toolkit::prelude::{dark, draw_surface_with_title, SurfaceStyle, TextStyle};

const UPGRADE_RECT: Rect = Rect::new(1064.0, 312.0, 176.0, 32.0);
const POWER_PLANT_UPGRADE_COST: i32 = 55;

pub(super) fn draw_launcher(campaign: &CampaignState, mouse: Vec2, actions: &mut Vec<UiAction>) {
    let Some(plant) = campaign
        .colony
        .buildings
        .iter()
        .find(|building| building.kind == BuildingKind::PowerPlant)
    else {
        return;
    };
    let queued = campaign
        .colony
        .facility_upgrade_queue
        .iter()
        .any(|project| project.building_id == plant.id);
    let (label, enabled) = if plant.level >= 2 {
        ("POWER PLANT // LEVEL 2".to_owned(), false)
    } else if plant.damaged {
        ("POWER PLANT // REPAIR FIRST".to_owned(), false)
    } else if queued {
        ("POWER PLANT // UPGRADE QUEUED".to_owned(), false)
    } else {
        ("POWER PLANT // CHOOSE LEVEL 2".to_owned(), true)
    };
    if button(UPGRADE_RECT, &label, enabled, mouse) {
        actions.push(UiAction::OpenFacilityUpgrade);
    }
}

pub(super) fn draw_modal(campaign: &CampaignState, mouse: Vec2, actions: &mut Vec<UiAction>) {
    let panel = Rect::new(862.0, 74.0, 408.0, 608.0);
    draw_surface_with_title(
        panel,
        Some("POWER PLANT // LEVEL 2"),
        &SurfaceStyle::new(Color::new(0.045, 0.075, 0.080, 0.995))
            .with_border(2.0, dark::ACCENT)
            .with_header(42.0, Color::new(0.08, 0.13, 0.13, 1.0)),
        TextStyle::new(17.0, dark::TEXT),
    );
    draw_text_ex(
        "Choose one branch. The project completes after the next operation.",
        886.0,
        164.0,
        TextParams {
            font_size: 12,
            color: dark::TEXT_DIM,
            ..Default::default()
        },
    );

    let Some(plant) = campaign
        .colony
        .buildings
        .iter()
        .find(|building| building.kind == BuildingKind::PowerPlant)
    else {
        return;
    };
    let queued = campaign
        .colony
        .facility_upgrade_queue
        .iter()
        .any(|project| project.building_id == plant.id);
    let can_queue = plant.level < 2
        && !plant.damaged
        && !queued
        && campaign.colony.resources.materials >= POWER_PLANT_UPGRADE_COST;
    for (index, option) in BuildingKind::PowerPlant
        .upgrade_options()
        .iter()
        .enumerate()
    {
        let y = 208.0 + index as f32 * 142.0;
        draw_rectangle(886.0, y, 360.0, 116.0, Color::new(0.025, 0.055, 0.060, 1.0));
        draw_text_ex(
            option.name,
            902.0,
            y + 25.0,
            TextParams {
                font_size: 16,
                color: dark::TEXT_BRIGHT,
                ..Default::default()
            },
        );
        draw_text_ex(
            option.description,
            902.0,
            y + 48.0,
            TextParams {
                font_size: 11,
                color: dark::TEXT_DIM,
                ..Default::default()
            },
        );
        let action_rect = Rect::new(902.0, y + 66.0, 328.0, 30.0);
        if button(
            action_rect,
            &format!(
                "QUEUE {} // {} MAT",
                option.name.to_uppercase(),
                POWER_PLANT_UPGRADE_COST
            ),
            can_queue,
            mouse,
        ) {
            actions.push(UiAction::QueueFacilityUpgrade(
                plant.id.clone(),
                option.id.to_owned(),
            ));
        }
    }
    if !can_queue {
        draw_text_ex(
            if plant.damaged {
                "REPAIR THE PLANT BEFORE CHOOSING A BRANCH"
            } else if queued {
                "UPGRADE PROJECT ALREADY IN PROGRESS"
            } else if plant.level >= 2 {
                "LEVEL 2 BRANCH ALREADY ACTIVE"
            } else {
                "INSUFFICIENT MATERIALS // 55 REQUIRED"
            },
            886.0,
            516.0,
            TextParams {
                font_size: 11,
                color: dark::WARNING,
                ..Default::default()
            },
        );
    }
    if button(
        Rect::new(1060.0, 638.0, 170.0, 30.0),
        "BACK TO OPERATIONS",
        true,
        mouse,
    ) {
        actions.push(UiAction::CloseFacilityUpgrade);
    }
}
