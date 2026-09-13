//! Colony roster, training, and equipment presentation.
pub mod detail;
pub use detail::*;

use crate::campaign::{equipment_cost, CampaignState};
use crate::colony::{BuildingKind, SIMULATION_HALL_UPGRADE, TRAUMA_WARD_UPGRADE};
use crate::data::GameData;
use crate::ui::{UiAction, LOGICAL_HEIGHT, LOGICAL_WIDTH};
use crate::ui_widgets::{button, button_with_state};
use crate::visual_assets::VisualCatalog;
use macroquad::prelude::*;
use macroquad_toolkit::assets::AssetManager;
use macroquad_toolkit::prelude::*;
use macroquad_toolkit::ui::{draw_ui_text_ex, VirtualUi};

pub fn character_list_row_layout(roster_len: usize) -> (f32, f32) {
    if roster_len <= 5 {
        return (60.0, 54.0);
    }
    let row_step = (374.0 / roster_len as f32).min(60.0);
    (row_step, (row_step - 4.0).max(44.0))
}

pub fn class_training_label(
    active: bool,
    lock_reason: Option<&str>,
    has_barracks: bool,
    affordable: bool,
    class_name: &str,
    cost: u32,
) -> String {
    if active {
        format!("ACTIVE: {}", class_name)
    } else if let Some(reason) = lock_reason {
        format!("{} · {}", class_name.to_uppercase(), reason)
    } else if !has_barracks {
        format!("{} · REQUIRES BARRACKS", class_name.to_uppercase())
    } else if !affordable {
        format!("{} · NEEDS {} MAT", class_name.to_uppercase(), cost)
    } else {
        format!("TRAIN: {} · {} MAT", class_name, cost)
    }
}

pub fn technique_label(
    active: bool,
    learned: bool,
    has_barracks: bool,
    has_experience: bool,
    has_slot: bool,
    technique_name: &str,
    required_experience: u32,
) -> String {
    if active {
        format!("EQUIPPED: {}", technique_name)
    } else if learned && !has_slot {
        format!("SLOTS FULL: {}", technique_name)
    } else if learned {
        format!("EQUIP: {}", technique_name)
    } else if !has_barracks {
        format!("REQUIRES BARRACKS: {}", technique_name)
    } else if !has_experience {
        format!("NEEDS {} XP: {}", required_experience, technique_name)
    } else {
        format!("LEARN: {} · {} XP", technique_name, required_experience)
    }
}

pub fn equipment_label(
    unlocked: bool,
    equipped: bool,
    prototype_available: bool,
    has_workshop: bool,
    affordable: bool,
    item_name: &str,
    cost: u32,
) -> String {
    if !unlocked {
        format!("LOCKED: {}", item_name)
    } else if equipped {
        format!("EQUIPPED: {}", item_name)
    } else if prototype_available {
        format!("PROTOTYPE: {}", item_name)
    } else if !has_workshop {
        format!("REQUIRES WORKSHOP: {}", item_name)
    } else if !affordable {
        format!("NEEDS {} MAT: {}", cost, item_name)
    } else {
        format!("CRAFT: {} · {} MAT", item_name, cost)
    }
}

pub const EQUIPMENT_ROW_START_Y: f32 = 548.0;
pub const EQUIPMENT_ROW_STEP_Y: f32 = 25.0;
pub const EQUIPMENT_ROW_HEIGHT: f32 = 24.0;

pub fn equipment_row_rect(index: usize) -> Rect {
    let column = index % 3;
    let row = index / 3;
    Rect::new(
        344.0 + column as f32 * 296.0,
        EQUIPMENT_ROW_START_Y + row as f32 * EQUIPMENT_ROW_STEP_Y,
        282.0,
        EQUIPMENT_ROW_HEIGHT,
    )
}

pub fn draw_roster(
    campaign: &CampaignState,
    data: &GameData,
    assets: &AssetManager,
    visuals: &VisualCatalog,
    ui: &VirtualUi,
    roster_inspection_id: Option<&str>,
) -> Vec<UiAction> {
    let mouse = crate::ui::pointer_position(ui);
    let mut actions = Vec::new();
    draw_rectangle(
        0.0,
        0.0,
        LOGICAL_WIDTH,
        LOGICAL_HEIGHT,
        Color::new(0.025, 0.04, 0.055, 1.0),
    );
    draw_header(campaign);
    draw_character_list(campaign, assets, visuals, mouse, &mut actions);
    draw_selected_character(
        campaign,
        data,
        assets,
        visuals,
        mouse,
        roster_inspection_id,
        &mut actions,
    );
    draw_ui_text_ex(
        "TOUCH / MOUSE // TAP COLONIST · TRAIN · CRAFT · EQUIP · COLONY  //  PAD // D-PAD · A · B",
        28.0,
        707.0,
        TextStyle::new(10.0, dark::TEXT_DIM).params(),
    );
    actions
}

pub fn draw_header(campaign: &CampaignState) {
    let rect = Rect::new(18.0, 16.0, LOGICAL_WIDTH - 36.0, 66.0);
    draw_surface(
        rect,
        &SurfaceStyle::new(Color::new(0.045, 0.075, 0.085, 0.98))
            .with_border(1.0, Color::new(0.22, 0.62, 0.56, 0.8))
            .with_left_accent(5.0, Color::new(0.34, 0.86, 0.68, 1.0)),
    );
    draw_ui_text_ex(
        "COLONY ROSTER",
        42.0,
        57.0,
        TextStyle::new(30.0, dark::TEXT_BRIGHT).params(),
    );
    draw_ui_text_ex(
        &format!("MATERIALS {}", campaign.colony.resources.materials),
        1040.0,
        54.0,
        TextStyle::new(16.0, dark::TEXT_DIM).params(),
    );
}

pub fn draw_character_list(
    campaign: &CampaignState,
    assets: &AssetManager,
    visuals: &VisualCatalog,
    mouse: Vec2,
    actions: &mut Vec<UiAction>,
) {
    let panel = Rect::new(18.0, 96.0, 280.0, 580.0);
    draw_surface_with_title(
        panel,
        Some("COLONISTS"),
        &SurfaceStyle::new(Color::new(0.04, 0.06, 0.07, 0.98))
            .with_border(1.0, Color::new(0.19, 0.40, 0.40, 0.9))
            .with_header(42.0, Color::new(0.06, 0.10, 0.11, 1.0)),
        TextStyle::new(17.0, dark::TEXT),
    );
    let (row_step, row_height) = character_list_row_layout(campaign.roster.len());
    for (index, character) in campaign.roster.iter().enumerate() {
        let selected = character.id == campaign.selected_character_id;
        let deployment = if character.deployment_selected {
            "SQUAD"
        } else {
            "RESERVE"
        };
        let legacy = if character.event_legacies.is_empty() {
            ""
        } else {
            " · LEGACY"
        };
        let row = Rect::new(36.0, 150.0 + index as f32 * row_step, 244.0, row_height);
        if button_with_state(row, "", true, selected, mouse) {
            actions.push(UiAction::SelectColonist(character.id.clone()));
        }
        crate::portrait_ui::draw_character_portrait(
            assets,
            visuals,
            Rect::new(row.x + 5.0, row.y + 3.0, 44.0, 44.0),
            &character.id,
            &character.name,
            if selected {
                dark::POSITIVE
            } else {
                Color::new(0.23, 0.48, 0.44, 1.0)
            },
        );
        draw_ui_text_ex(
            &format!(
                "{}{}",
                if selected { "> " } else { "" },
                character.name.to_uppercase()
            ),
            row.x + 62.0,
            row.y + 22.0,
            TextStyle::new(13.0, dark::TEXT_BRIGHT).params(),
        );
        draw_ui_text_ex(
            &format!("LV{} // {}{}", character.level, deployment, legacy),
            row.x + 62.0,
            row.y + 40.0,
            TextStyle::new(10.0, dark::TEXT_DIM).params(),
        );
    }
    let relationships = campaign.relationship_summaries(&campaign.selected_character_id);
    draw_ui_text_ex(
        "RELATIONSHIPS",
        40.0,
        540.0,
        TextStyle::new(13.0, dark::ACCENT).params(),
    );
    if relationships.is_empty() {
        draw_text_block(
            "No shared victories yet. Deploy colonists together or resolve their events.",
            40.0,
            550.0,
            230.0,
            74.0,
            12.0,
            3.0,
            dark::TEXT_DIM,
        );
    } else {
        for (index, relationship) in relationships.iter().take(2).enumerate() {
            draw_ui_text_ex(
                relationship,
                92.0,
                570.0 + index as f32 * 42.0,
                TextStyle::new(11.0, dark::TEXT_DIM).params(),
            );
        }
        let selected_id = &campaign.selected_character_id;
        for (index, relation) in campaign
            .relationships
            .iter()
            .filter(|relation| {
                relation.first_id == *selected_id || relation.second_id == *selected_id
            })
            .take(2)
            .enumerate()
        {
            let partner_id = if relation.first_id == *selected_id {
                &relation.second_id
            } else {
                &relation.first_id
            };
            if let Some(partner) = campaign
                .roster
                .iter()
                .find(|character| character.id == *partner_id)
            {
                crate::portrait_ui::draw_character_portrait(
                    assets,
                    visuals,
                    Rect::new(40.0, 548.0 + index as f32 * 42.0, 40.0, 40.0),
                    &partner.id,
                    &partner.name,
                    Color::new(0.45, 0.78, 0.96, 1.0),
                );
            }
        }
    }
}

pub fn equipment_info_rect(rect: Rect) -> Rect {
    Rect::new(rect.x + rect.w - 28.0, rect.y, 28.0, rect.h)
}
