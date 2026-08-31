//! Touch-visible level-two choices for the colony's upgradeable facilities.

use crate::campaign::CampaignState;
use crate::colony::BuildingKind;
use crate::ui::UiAction;
use crate::ui_widgets::button;
use macroquad::prelude::*;
use macroquad_toolkit::prelude::{dark, draw_surface_with_title, SurfaceStyle, TextStyle};

const UPGRADE_RECT: Rect = Rect::new(1064.0, 312.0, 176.0, 32.0);
const UPGRADEABLE_FACILITIES: [BuildingKind; 7] = [
    BuildingKind::CommandCentre,
    BuildingKind::Barracks,
    BuildingKind::Infirmary,
    BuildingKind::PowerPlant,
    BuildingKind::Hydroponics,
    BuildingKind::Workshop,
    BuildingKind::GeneLab,
];

pub(super) fn draw_launcher(campaign: &CampaignState, mouse: Vec2, actions: &mut Vec<UiAction>) {
    let available = UPGRADEABLE_FACILITIES.iter().any(|kind| {
        campaign.colony.buildings.iter().any(|building| {
            building.kind == *kind
                && building.level < 2
                && !building.damaged
                && campaign.colony.building_is_powered(&building.id)
                && !upgrade_queued(campaign, &building.id)
        })
    });
    let queued = UPGRADEABLE_FACILITIES.iter().any(|kind| {
        campaign
            .colony
            .buildings
            .iter()
            .any(|building| building.kind == *kind && upgrade_queued(campaign, &building.id))
    });
    let all_upgraded = UPGRADEABLE_FACILITIES.iter().all(|kind| {
        campaign
            .colony
            .buildings
            .iter()
            .any(|building| building.kind == *kind && building.level >= 2)
    });
    let (label, enabled) = if available {
        ("UPGRADES // CHOOSE LEVEL 2", true)
    } else if queued {
        ("UPGRADES // PROJECT IN PROGRESS", false)
    } else if all_upgraded {
        ("UPGRADES // LEVEL 2 ACTIVE", false)
    } else {
        ("UPGRADES // REPAIR FIRST", false)
    };
    if button(UPGRADE_RECT, label, enabled, mouse) {
        actions.push(UiAction::OpenFacilityUpgrade);
    }
}

pub(super) fn draw_modal(campaign: &CampaignState, mouse: Vec2, actions: &mut Vec<UiAction>) {
    let panel = Rect::new(862.0, 44.0, 408.0, 660.0);
    draw_surface_with_title(
        panel,
        Some("FACILITY UPGRADES // LEVEL 2"),
        &SurfaceStyle::new(Color::new(0.045, 0.075, 0.080, 0.995))
            .with_border(2.0, dark::ACCENT)
            .with_header(42.0, Color::new(0.08, 0.13, 0.13, 1.0)),
        TextStyle::new(17.0, dark::TEXT),
    );
    draw_text_ex(
        "Choose one branch per facility. Each project completes after the next operation.",
        886.0,
        126.0,
        TextParams {
            font_size: 12,
            color: dark::TEXT_DIM,
            ..Default::default()
        },
    );

    for (index, kind) in UPGRADEABLE_FACILITIES.iter().enumerate() {
        let Some(building) = campaign
            .colony
            .buildings
            .iter()
            .find(|building| building.kind == *kind)
        else {
            continue;
        };
        let y = 124.0 + index as f32 * 70.0;
        draw_text_ex(
            kind.name().to_uppercase(),
            886.0,
            y + 20.0,
            TextParams {
                font_size: 15,
                color: dark::TEXT_BRIGHT,
                ..Default::default()
            },
        );
        draw_text_ex(
            facility_status(campaign, *kind, building),
            1080.0,
            y + 20.0,
            TextParams {
                font_size: 10,
                color: dark::TEXT_DIM,
                ..Default::default()
            },
        );
        let queued = upgrade_queued(campaign, &building.id);
        let (queue_label, can_queue) = upgrade_button_state(
            building.level,
            building.damaged,
            campaign.colony.building_is_powered(&building.id),
            queued,
            campaign.colony.resources.materials,
            kind.upgrade_cost(),
        );
        for (option_index, option) in kind.upgrade_options().iter().enumerate() {
            let card_y = y + 16.0 + option_index as f32 * 26.0;
            draw_rectangle(
                886.0,
                card_y,
                360.0,
                24.0,
                Color::new(0.025, 0.055, 0.060, 1.0),
            );
            draw_text_ex(
                option.name,
                902.0,
                card_y + 10.0,
                TextParams {
                    font_size: 12,
                    color: dark::TEXT_BRIGHT,
                    ..Default::default()
                },
            );
            draw_text_ex(
                option.description,
                902.0,
                card_y + 20.0,
                TextParams {
                    font_size: 8,
                    color: dark::TEXT_DIM,
                    ..Default::default()
                },
            );
            if button(
                Rect::new(1094.0, card_y + 3.0, 136.0, 18.0),
                &queue_label,
                can_queue,
                mouse,
            ) {
                actions.push(UiAction::QueueFacilityUpgrade(
                    building.id.clone(),
                    option.id.to_owned(),
                ));
            }
        }
    }
    if let Some(message) = upgrade_message(campaign) {
        draw_text_ex(
            message,
            886.0,
            638.0,
            TextParams {
                font_size: 10,
                color: dark::WARNING,
                ..Default::default()
            },
        );
    }
    if button(
        Rect::new(1060.0, 662.0, 170.0, 30.0),
        "BACK TO OPERATIONS",
        true,
        mouse,
    ) {
        actions.push(UiAction::CloseFacilityUpgrade);
    }
}

fn upgrade_button_state(
    level: u8,
    damaged: bool,
    powered: bool,
    queued: bool,
    materials: i32,
    cost: i32,
) -> (String, bool) {
    if level >= 2 {
        ("LEVEL 2 ACTIVE".to_owned(), false)
    } else if damaged {
        ("REPAIR FIRST".to_owned(), false)
    } else if queued {
        ("PROJECT QUEUED".to_owned(), false)
    } else if !powered {
        ("NEED POWER".to_owned(), false)
    } else if materials < cost {
        (format!("NEED {cost} MAT"), false)
    } else {
        (format!("QUEUE // {cost} MAT"), true)
    }
}

fn upgrade_queued(campaign: &CampaignState, building_id: &str) -> bool {
    campaign
        .colony
        .facility_upgrade_queue
        .iter()
        .any(|project| project.building_id == building_id)
}

#[cfg(test)]
mod tests;

fn facility_status(
    campaign: &CampaignState,
    kind: BuildingKind,
    building: &crate::colony::BuildingState,
) -> &'static str {
    if building.level >= 2 {
        "LEVEL 2 ACTIVE"
    } else if building.damaged {
        "REPAIR FIRST"
    } else if upgrade_queued(campaign, &building.id) {
        "PROJECT QUEUED"
    } else if !campaign.colony.building_is_powered(&building.id) {
        "NEEDS POWER"
    } else if campaign.colony.resources.materials < kind.upgrade_cost() {
        "NEEDS MATERIALS"
    } else {
        "READY"
    }
}

fn upgrade_message(campaign: &CampaignState) -> Option<&'static str> {
    if UPGRADEABLE_FACILITIES.iter().any(|kind| {
        campaign.colony.buildings.iter().any(|building| {
            building.kind == *kind
                && building.level < 2
                && !building.damaged
                && campaign.colony.building_is_powered(&building.id)
                && !upgrade_queued(campaign, &building.id)
                && campaign.colony.resources.materials < kind.upgrade_cost()
        })
    }) {
        Some("INSUFFICIENT MATERIALS // 55 REQUIRED")
    } else if UPGRADEABLE_FACILITIES.iter().any(|kind| {
        campaign
            .colony
            .buildings
            .iter()
            .any(|building| building.kind == *kind && building.level < 2 && building.damaged)
    }) {
        Some("REPAIR A FACILITY BEFORE CHOOSING ITS BRANCH")
    } else if UPGRADEABLE_FACILITIES.iter().any(|kind| {
        campaign.colony.buildings.iter().any(|building| {
            building.kind == *kind
                && building.level < 2
                && !building.damaged
                && !campaign.colony.building_is_powered(&building.id)
                && !upgrade_queued(campaign, &building.id)
        })
    }) {
        Some("RESTORE POWER BEFORE CHOOSING A FACILITY BRANCH")
    } else {
        None
    }
}
