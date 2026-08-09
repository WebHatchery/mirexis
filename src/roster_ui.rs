//! Colony roster, training, and equipment presentation.

use crate::campaign::{equipment_cost, CampaignState};
use crate::colony::BuildingKind;
use crate::data::GameData;
use crate::ui::{UiAction, LOGICAL_HEIGHT, LOGICAL_WIDTH};
use crate::ui_widgets::button;
use macroquad::prelude::*;
use macroquad_toolkit::prelude::*;
use macroquad_toolkit::ui::{draw_ui_text_ex, VirtualUi};

pub(crate) fn draw_roster(
    campaign: &CampaignState,
    data: &GameData,
    ui: &VirtualUi,
) -> Vec<UiAction> {
    let mouse = ui.mouse_position();
    let mut actions = Vec::new();
    draw_rectangle(
        0.0,
        0.0,
        LOGICAL_WIDTH,
        LOGICAL_HEIGHT,
        Color::new(0.025, 0.04, 0.055, 1.0),
    );
    draw_header(campaign);
    draw_character_list(campaign, mouse, &mut actions);
    draw_selected_character(campaign, data, mouse, &mut actions);
    actions
}

fn draw_header(campaign: &CampaignState) {
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

fn draw_character_list(campaign: &CampaignState, mouse: Vec2, actions: &mut Vec<UiAction>) {
    let panel = Rect::new(18.0, 96.0, 280.0, 580.0);
    draw_surface_with_title(
        panel,
        Some("COLONISTS"),
        &SurfaceStyle::new(Color::new(0.04, 0.06, 0.07, 0.98))
            .with_border(1.0, Color::new(0.19, 0.40, 0.40, 0.9))
            .with_header(42.0, Color::new(0.06, 0.10, 0.11, 1.0)),
        TextStyle::new(17.0, dark::TEXT),
    );
    for (index, character) in campaign.roster.iter().enumerate() {
        let selected = character.id == campaign.selected_character_id;
        let deployment = if character.deployment_selected {
            "SQUAD"
        } else {
            "RESERVE"
        };
        if button(
            Rect::new(36.0, 154.0 + index as f32 * 72.0, 244.0, 58.0),
            &format!(
                "{}{} · LV{} · {}",
                if selected { "> " } else { "" },
                character.name,
                character.level,
                deployment
            ),
            true,
            mouse,
        ) {
            actions.push(UiAction::SelectColonist(character.id.clone()));
        }
    }
    draw_text_block(
        "Select a colonist to inspect training, mutation, and loadout.",
        40.0,
        578.0,
        230.0,
        54.0,
        14.0,
        3.0,
        dark::TEXT_DIM,
    );
}

fn draw_selected_character(
    campaign: &CampaignState,
    data: &GameData,
    mouse: Vec2,
    actions: &mut Vec<UiAction>,
) {
    let panel = Rect::new(316.0, 96.0, 946.0, 580.0);
    draw_surface_with_title(
        panel,
        Some("TRAINING // WORKSHOP"),
        &SurfaceStyle::new(Color::new(0.055, 0.07, 0.08, 0.98))
            .with_border(1.0, Color::new(0.25, 0.38, 0.40, 0.8))
            .with_header(42.0, Color::new(0.08, 0.105, 0.115, 1.0)),
        TextStyle::new(17.0, dark::TEXT),
    );
    let Some(character) = campaign.selected_character() else {
        return;
    };
    let mutation = data
        .mutations
        .iter()
        .find(|mutation| mutation.id == character.mutation_id)
        .map_or(character.mutation_id.as_str(), |mutation| {
            mutation.name.as_str()
        });
    draw_ui_text_ex(
        &character.name,
        344.0,
        172.0,
        TextStyle::new(30.0, dark::TEXT_BRIGHT).params(),
    );
    draw_text_block(
        &character.biography,
        344.0,
        190.0,
        870.0,
        52.0,
        17.0,
        4.0,
        dark::TEXT_DIM,
    );
    draw_ui_text_ex(
        &format!(
            "LEVEL {} · {} XP · {} · {:?}",
            character.level, character.experience, mutation, character.availability
        ),
        344.0,
        260.0,
        TextStyle::new(15.0, dark::ACCENT).params(),
    );
    let equipment = character
        .equipment_ids
        .iter()
        .filter_map(|id| data.equipment.iter().find(|item| &item.id == id))
        .map(|item| item.name.as_str())
        .collect::<Vec<_>>()
        .join(" · ");
    draw_ui_text_ex(
        &format!("LOADOUT  {}", equipment),
        344.0,
        288.0,
        TextStyle::new(14.0, dark::TEXT_DIM).params(),
    );
    draw_ui_text_ex(
        "BARRACKS // CLASS TRAINING",
        344.0,
        322.0,
        TextStyle::new(15.0, dark::ACCENT).params(),
    );
    for (index, class) in data.classes.iter().enumerate() {
        let cost = campaign.training_cost(&character.id, class).unwrap_or(100);
        let enabled = campaign.colony.has_facility(BuildingKind::Barracks)
            && character.active_class != class.id
            && campaign.colony.resources.materials >= cost as i32;
        let column = index % 2;
        let row = index / 2;
        if button(
            Rect::new(
                344.0 + column as f32 * 444.0,
                334.0 + row as f32 * 36.0,
                430.0,
                30.0,
            ),
            &if character.active_class == class.id {
                format!("ACTIVE: {}", class.name)
            } else {
                format!("TRAIN: {} · {} MAT", class.name, cost)
            },
            enabled,
            mouse,
        ) {
            actions.push(UiAction::TrainSelected(class.id.clone()));
        }
    }
    draw_ui_text_ex(
        "WORKSHOP // EQUIPMENT",
        344.0,
        488.0,
        TextStyle::new(15.0, dark::ACCENT).params(),
    );
    for (index, item) in data.equipment.iter().enumerate() {
        let cost = equipment_cost(&item.slot);
        let equipped = character.equipment_ids.contains(&item.id);
        let enabled = campaign.colony.has_facility(BuildingKind::Workshop)
            && !equipped
            && campaign.colony.resources.materials >= cost as i32;
        let column = index % 3;
        let row = index / 3;
        if button(
            Rect::new(
                344.0 + column as f32 * 296.0,
                500.0 + row as f32 * 36.0,
                282.0,
                30.0,
            ),
            &if equipped {
                format!("EQUIPPED: {}", item.name)
            } else {
                format!("CRAFT: {} · {} MAT", item.name, cost)
            },
            enabled,
            mouse,
        ) {
            actions.push(UiAction::CraftSelected(item.id.clone()));
        }
    }
    if button(
        Rect::new(1020.0, 624.0, 214.0, 38.0),
        "RETURN TO COLONY",
        true,
        mouse,
    ) {
        actions.push(UiAction::ReturnToColony);
    }
}
