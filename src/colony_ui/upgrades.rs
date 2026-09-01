//! Touch-visible level-two choices for the colony's upgradeable facilities.

use crate::campaign::CampaignState;
use crate::colony::BuildingKind;
use crate::ui::UiAction;
use crate::ui_widgets::button;
use macroquad::prelude::*;
use macroquad_toolkit::prelude::{
    dark, draw_chamfered_surface, draw_surface_with_title, ChamferedSurfaceStyle, SurfaceStyle,
    TextStyle,
};
use macroquad_toolkit::ui::RectExt;

const UPGRADE_RECT: Rect = Rect::new(1064.0, 312.0, 176.0, 32.0);
const UPGRADE_PANEL: Rect = Rect::new(862.0, 44.0, 408.0, 660.0);
const CONTENT_RECT: Rect = Rect::new(884.0, 0.0, 364.0, 0.0);
const FACILITY_LIST_Y: f32 = 164.0;
const FACILITY_ROW_STEP: f32 = 66.0;
const FACILITY_ROW_HEIGHT: f32 = 62.0;
const OPTION_CARD_WIDTH: f32 = 170.0;
const OPTION_CARD_HEIGHT: f32 = 42.0;
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
    draw_rectangle(
        0.0,
        0.0,
        1280.0,
        720.0,
        Color::new(0.005, 0.012, 0.016, 0.62),
    );
    draw_surface_with_title(
        UPGRADE_PANEL,
        Some("FACILITY UPGRADES // LEVEL 2"),
        &SurfaceStyle::new(Color::new(0.045, 0.075, 0.080, 0.995))
            .with_border(2.0, dark::ACCENT)
            .with_header(42.0, Color::new(0.08, 0.13, 0.13, 1.0)),
        TextStyle::new(17.0, dark::TEXT),
    );
    super::draw_ui_text_ex(
        "CHOOSE ONE UPGRADE FOR EACH FACILITY",
        CONTENT_RECT.x,
        110.0,
        TextStyle::new(12.0, dark::TEXT_BRIGHT).params(),
    );
    super::draw_ui_text_ex(
        "PROJECT RESOLVES AFTER THE NEXT OPERATION",
        CONTENT_RECT.x,
        127.0,
        TextStyle::new(10.0, dark::TEXT_DIM).params(),
    );
    draw_rectangle(
        CONTENT_RECT.x,
        136.0,
        CONTENT_RECT.w,
        22.0,
        Color::new(0.025, 0.095, 0.095, 1.0),
    );
    draw_rectangle_lines(
        CONTENT_RECT.x,
        136.0,
        CONTENT_RECT.w,
        22.0,
        1.0,
        Color::new(0.18, 0.54, 0.49, 0.75),
    );
    super::draw_ui_text_ex(
        &format!("{} MATERIALS", campaign.colony.resources.materials),
        CONTENT_RECT.x + 10.0,
        152.0,
        TextStyle::new(13.0, dark::TEXT_BRIGHT).params(),
    );
    let cost_note = format!(
        "EACH LEVEL 2 UPGRADE COSTS {}",
        UPGRADEABLE_FACILITIES[0].upgrade_cost()
    );
    let cost_width = measure_text(&cost_note, None, 9, 1.0).width;
    super::draw_ui_text_ex(
        &cost_note,
        CONTENT_RECT.right() - cost_width - 10.0,
        151.0,
        TextStyle::new(9.0, dark::ACCENT).params(),
    );

    let mut row_index = 0;
    for kind in UPGRADEABLE_FACILITIES {
        let Some(building) = campaign
            .colony
            .buildings
            .iter()
            .find(|building| building.kind == kind)
        else {
            continue;
        };
        let y = FACILITY_LIST_Y + row_index as f32 * FACILITY_ROW_STEP;
        draw_facility_row(campaign, kind, building, y, mouse, actions);
        row_index += 1;
    }
    if let Some(message) = upgrade_message(campaign) {
        super::draw_ui_text_ex(
            &fit_text(message, CONTENT_RECT.w, 10),
            CONTENT_RECT.x,
            632.0,
            TextStyle::new(10.0, dark::WARNING).params(),
        );
    }
    draw_line(
        CONTENT_RECT.x,
        646.0,
        CONTENT_RECT.right(),
        646.0,
        1.0,
        Color::new(0.16, 0.30, 0.30, 0.8),
    );
    if button(
        Rect::new(1060.0, 660.0, 170.0, 32.0),
        "BACK TO OPERATIONS",
        true,
        mouse,
    ) {
        actions.push(UiAction::CloseFacilityUpgrade);
    }
}

fn draw_facility_row(
    campaign: &CampaignState,
    kind: BuildingKind,
    building: &crate::colony::BuildingState,
    y: f32,
    mouse: Vec2,
    actions: &mut Vec<UiAction>,
) {
    let row = Rect::new(CONTENT_RECT.x, y, CONTENT_RECT.w, FACILITY_ROW_HEIGHT);
    let status = facility_status(campaign, kind, building);
    draw_rectangle(
        row.x,
        row.y,
        row.w,
        row.h,
        Color::new(0.025, 0.055, 0.060, 1.0),
    );
    draw_rectangle_lines(
        row.x,
        row.y,
        row.w,
        row.h,
        1.0,
        Color::new(0.12, 0.25, 0.25, 0.9),
    );
    draw_rectangle(row.x, row.y, 3.0, row.h, facility_status_color(status));
    super::draw_ui_text_ex(
        &kind.name().to_uppercase(),
        row.x + 12.0,
        row.y + 14.0,
        TextStyle::new(13.0, dark::TEXT_BRIGHT).params(),
    );
    let status_width = measure_text(status, None, 9, 1.0).width;
    super::draw_ui_text_ex(
        status,
        row.right() - status_width - 10.0,
        row.y + 14.0,
        TextStyle::new(9.0, facility_status_color(status)).params(),
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
        let option_card = Rect::new(
            row.x + 8.0 + option_index as f32 * (OPTION_CARD_WIDTH + 8.0),
            row.y + 19.0,
            OPTION_CARD_WIDTH,
            OPTION_CARD_HEIGHT,
        );
        draw_option_card(
            option_card,
            option_index,
            option,
            &queue_label,
            can_queue,
            kind.upgrade_cost(),
            mouse,
            building,
            actions,
        );
    }
}

fn draw_option_card(
    card: Rect,
    option_index: usize,
    option: &crate::colony::FacilityUpgradeOption,
    queue_label: &str,
    can_queue: bool,
    cost: i32,
    mouse: Vec2,
    building: &crate::colony::BuildingState,
    actions: &mut Vec<UiAction>,
) {
    let action_rect = Rect::new(card.x + 98.0, card.y + 25.0, 64.0, 16.0);
    let hovered = can_queue && action_rect.contains_point(mouse);
    draw_rectangle(
        card.x,
        card.y,
        card.w,
        card.h,
        if hovered {
            Color::new(0.055, 0.13, 0.13, 1.0)
        } else {
            Color::new(0.035, 0.085, 0.085, 1.0)
        },
    );
    draw_rectangle_lines(
        card.x,
        card.y,
        card.w,
        card.h,
        1.0,
        if hovered {
            dark::ACCENT
        } else {
            Color::new(0.15, 0.34, 0.33, 0.95)
        },
    );
    draw_rectangle(
        card.x,
        card.y,
        2.0,
        card.h,
        if option_index == 0 {
            dark::ACCENT
        } else {
            Color::new(0.24, 0.48, 0.45, 1.0)
        },
    );
    super::draw_ui_text_ex(
        option.name,
        card.x + 8.0,
        card.y + 11.0,
        TextStyle::new(11.0, dark::TEXT_BRIGHT).params(),
    );
    super::draw_ui_text_ex(
        &fit_text(option.description, 154.0, 8),
        card.x + 8.0,
        card.y + 22.0,
        TextStyle::new(8.0, dark::TEXT_DIM).params(),
    );
    super::draw_ui_text_ex(
        &format!("{cost} MAT"),
        card.x + 8.0,
        card.y + 36.0,
        TextStyle::new(8.0, dark::ACCENT).params(),
    );
    if choice_action_button(
        action_rect,
        compact_action_label(queue_label),
        can_queue,
        mouse,
    ) {
        actions.push(UiAction::QueueFacilityUpgrade(
            building.id.clone(),
            option.id.to_owned(),
        ));
    }
}

fn choice_action_button(rect: Rect, label: &str, enabled: bool, mouse: Vec2) -> bool {
    let hovered = enabled && rect.contains_point(mouse);
    let fill = if !enabled {
        Color::new(0.045, 0.075, 0.075, 1.0)
    } else if hovered {
        Color::new(0.10, 0.31, 0.29, 1.0)
    } else {
        Color::new(0.075, 0.20, 0.20, 1.0)
    };
    draw_chamfered_surface(
        rect,
        &ChamferedSurfaceStyle::new(
            fill,
            if hovered {
                dark::ACCENT
            } else if enabled {
                Color::new(0.22, 0.58, 0.53, 1.0)
            } else {
                Color::new(0.12, 0.20, 0.20, 1.0)
            },
        )
        .with_corner(4.0)
        .with_border_width(if hovered { 2.0 } else { 1.0 }),
    );
    let text_width = measure_text(label, None, 9, 1.0).width;
    super::draw_ui_text_ex(
        label,
        rect.x + (rect.w - text_width) * 0.5,
        rect.y + 11.5,
        TextStyle::new(9.0, if enabled { dark::TEXT } else { dark::TEXT_DIM }).params(),
    );
    hovered && is_mouse_button_released(MouseButton::Left)
}

fn compact_action_label(label: &str) -> &str {
    match label {
        "LEVEL 2 ACTIVE" => "ACTIVE",
        "REPAIR FIRST" => "REPAIR",
        "PROJECT QUEUED" => "QUEUED",
        "NEED POWER" => "POWER",
        "NEED 55 MAT" => "NEED MAT",
        _ => "QUEUE",
    }
}

fn facility_status_color(status: &str) -> Color {
    match status {
        "READY" | "LEVEL 2 ACTIVE" => dark::POSITIVE,
        "PROJECT QUEUED" => dark::ACCENT,
        "REPAIR FIRST" => dark::NEGATIVE,
        _ => dark::WARNING,
    }
}

fn fit_text(text: &str, max_width: f32, font_size: u16) -> String {
    if measure_text(text, None, font_size, 1.0).width <= max_width {
        return text.to_owned();
    }
    let suffix = "...";
    let mut fitted = String::new();
    for character in text.chars() {
        let candidate = format!("{fitted}{character}{suffix}");
        if measure_text(&candidate, None, font_size, 1.0).width > max_width {
            break;
        }
        fitted.push(character);
    }
    format!("{}{}", fitted.trim_end(), suffix)
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
