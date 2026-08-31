//! Colony roster, training, and equipment presentation.

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

fn character_list_row_layout(roster_len: usize) -> (f32, f32) {
    if roster_len <= 5 {
        return (60.0, 54.0);
    }
    let row_step = (374.0 / roster_len as f32).min(60.0);
    (row_step, (row_step - 4.0).max(44.0))
}

fn class_training_label(
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

fn technique_label(
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

fn equipment_label(
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

const EQUIPMENT_ROW_START_Y: f32 = 548.0;
const EQUIPMENT_ROW_STEP_Y: f32 = 25.0;
const EQUIPMENT_ROW_HEIGHT: f32 = 24.0;

fn equipment_row_rect(index: usize) -> Rect {
    let column = index % 3;
    let row = index / 3;
    Rect::new(
        344.0 + column as f32 * 296.0,
        EQUIPMENT_ROW_START_Y + row as f32 * EQUIPMENT_ROW_STEP_Y,
        282.0,
        EQUIPMENT_ROW_HEIGHT,
    )
}

pub(crate) fn draw_roster(
    campaign: &CampaignState,
    data: &GameData,
    assets: &AssetManager,
    visuals: &VisualCatalog,
    ui: &VirtualUi,
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
    draw_selected_character(campaign, data, assets, visuals, mouse, &mut actions);
    draw_ui_text_ex(
        "TOUCH / MOUSE // TAP COLONIST · TRAIN · CRAFT · EQUIP · COLONY  //  PAD // D-PAD · A · B",
        28.0,
        707.0,
        TextStyle::new(10.0, dark::TEXT_DIM).params(),
    );
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

fn draw_character_list(
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

fn draw_selected_character(
    campaign: &CampaignState,
    data: &GameData,
    assets: &AssetManager,
    visuals: &VisualCatalog,
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
    let evolution = data
        .mutations
        .iter()
        .find(|entry| entry.id == character.mutation_id)
        .and_then(|mutation| {
            mutation
                .evolutions
                .iter()
                .find(|evolution| evolution.id == character.mutation_evolution_id)
        });
    let mutation = evolution.map_or_else(
        || mutation.to_owned(),
        |evolution| format!("{} / {}", mutation, evolution.name),
    );
    crate::portrait_ui::draw_character_portrait(
        assets,
        visuals,
        Rect::new(344.0, 150.0, 120.0, 110.0),
        &character.id,
        &character.name,
        Color::new(0.28, 0.88, 0.72, 1.0),
    );
    draw_ui_text_ex(
        &character.name,
        484.0,
        172.0,
        TextStyle::new(30.0, dark::TEXT_BRIGHT).params(),
    );
    draw_text_block(
        &character.biography,
        484.0,
        190.0,
        730.0,
        52.0,
        17.0,
        4.0,
        dark::TEXT_DIM,
    );
    draw_ui_text_ex(
        &format!(
            "LEVEL {} · {} XP · {} · {:?} · ORIGIN {}",
            character.level,
            character.experience,
            mutation,
            character.availability,
            if character.origin.is_empty() {
                "COLONY FOUNDER"
            } else {
                character.origin.as_str()
            }
        ),
        344.0,
        276.0,
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
        298.0,
        TextStyle::new(14.0, dark::TEXT_DIM).params(),
    );
    let mut history = Vec::new();
    if !character.event_legacies.is_empty() {
        history.push(
            character
                .event_legacies
                .iter()
                .map(|legacy| format!("{} ({:+} {})", legacy.name, legacy.amount, legacy.stat))
                .collect::<Vec<_>>()
                .join(" · "),
        );
    }
    if !character.traumas.is_empty() {
        let trauma_ward_active = campaign
            .colony
            .has_active_upgrade(BuildingKind::Infirmary, TRAUMA_WARD_UPGRADE);
        history.push(
            character
                .traumas
                .iter()
                .map(|trauma| {
                    format!(
                        "{} ({})",
                        trauma.name,
                        crate::trauma::effective_effect(trauma, trauma_ward_active)
                    )
                })
                .collect::<Vec<_>>()
                .join(" · "),
        );
    }
    if !history.is_empty() {
        draw_ui_text_ex(
            &format!("HISTORY  {}", history.join("  //  ")),
            344.0,
            306.0,
            TextStyle::new(11.0, dark::ACCENT).params(),
        );
    }
    draw_ui_text_ex(
        "BARRACKS // CLASS TRAINING · ADVANCED DISCIPLINES REQUIRE LV3 + ADAPTATION",
        344.0,
        322.0,
        TextStyle::new(13.0, dark::ACCENT).params(),
    );
    let mut inspected_class = data
        .classes
        .iter()
        .find(|class| class.id == character.active_class);
    for (index, class) in data.classes.iter().enumerate() {
        let cost = campaign.training_cost(&character.id, class).unwrap_or(100);
        let lock = campaign.class_training_lock_reason(&character.id, class);
        let has_barracks = campaign.colony.has_facility(BuildingKind::Barracks);
        let affordable = campaign.colony.resources.materials >= cost as i32;
        let enabled =
            has_barracks && character.active_class != class.id && lock.is_none() && affordable;
        let label = class_training_label(
            character.active_class == class.id,
            lock.as_deref(),
            has_barracks,
            affordable,
            &class.name,
            cost,
        );
        let column = index % 3;
        let row = index / 3;
        let rect = Rect::new(
            344.0 + column as f32 * 296.0,
            334.0 + row as f32 * 36.0,
            282.0,
            30.0,
        );
        if rect.contains(mouse) {
            inspected_class = Some(class);
        }
        if button(rect, &label, enabled, mouse) {
            actions.push(UiAction::TrainSelected(class.id.clone()));
        }
    }
    let active_class = data
        .classes
        .iter()
        .find(|class| class.id == character.active_class);
    let mut inspected_technique = None;
    draw_ui_text_ex(
        &format!(
            "TECHNIQUES // {} SLOT{} · OPERATION XP UNLOCKS NEW TECHNIQUES{}",
            active_class.map_or(0, |class| class.technique_slots),
            if active_class.is_some_and(|class| class.technique_slots == 1) {
                ""
            } else {
                "S"
            },
            if campaign
                .colony
                .has_active_upgrade(BuildingKind::Barracks, SIMULATION_HALL_UPGRADE)
            {
                " · SIMULATION HALL -10 XP TRIALS"
            } else {
                ""
            }
        ),
        344.0,
        478.0,
        TextStyle::new(12.0, dark::ACCENT).params(),
    );
    if let Some(class) = active_class {
        for (index, technique) in class.techniques.iter().enumerate() {
            let required_experience =
                campaign.skill_experience_required(technique.experience_required);
            let rect = Rect::new(344.0 + index as f32 * 434.0, 486.0, 426.0, 28.0);
            if rect.contains(mouse) {
                inspected_technique = Some(technique);
            }
            let learned = character.learned_skills.contains(&technique.id);
            let active = character.active_skills.contains(&technique.id);
            let equipped_count = character
                .active_skills
                .iter()
                .filter(|skill| class.techniques.iter().any(|entry| &entry.id == *skill))
                .count();
            let has_barracks = campaign.colony.has_facility(BuildingKind::Barracks);
            let has_experience = character.experience >= required_experience;
            let has_slot = equipped_count < class.technique_slots as usize;
            let enabled = if learned {
                active || has_slot
            } else {
                has_barracks && has_experience
            };
            let label = technique_label(
                active,
                learned,
                has_barracks,
                has_experience,
                has_slot,
                &technique.name,
                required_experience,
            );
            if button(rect, "", enabled, mouse) {
                actions.push(if learned {
                    UiAction::ToggleSelectedSkill(technique.id.clone())
                } else {
                    UiAction::LearnSelectedSkill(technique.id.clone())
                });
            }
            draw_ui_text_ex(
                &label.to_uppercase(),
                rect.x + 10.0,
                rect.y + 19.0,
                TextStyle::new(
                    11.0,
                    if enabled || active {
                        dark::TEXT
                    } else {
                        dark::TEXT_DIM
                    },
                )
                .params(),
            );
        }
    }
    let inspected_equipment = data
        .equipment
        .iter()
        .enumerate()
        .find_map(|(index, item)| equipment_row_rect(index).contains(mouse).then_some(item));
    if let Some(item) = inspected_equipment {
        draw_text_block(
            &format!("{} // {}", item.name.to_uppercase(), item.description),
            344.0,
            516.0,
            870.0,
            28.0,
            11.0,
            2.0,
            dark::TEXT_DIM,
        );
    } else if let Some(technique) = inspected_technique {
        draw_text_block(
            &format!(
                "{} // {} // REQUIRES {} XP",
                technique.name.to_uppercase(),
                technique.description,
                campaign.skill_experience_required(technique.experience_required)
            ),
            344.0,
            516.0,
            870.0,
            20.0,
            11.0,
            2.0,
            dark::TEXT_DIM,
        );
    } else if let Some(class) = inspected_class {
        draw_text_block(
            &format!("{} // {}", class.name.to_uppercase(), class.description),
            344.0,
            516.0,
            870.0,
            28.0,
            11.0,
            2.0,
            dark::TEXT_DIM,
        );
    }
    draw_ui_text_ex(
        "WORKSHOP // EQUIPMENT",
        344.0,
        540.0,
        TextStyle::new(15.0, dark::ACCENT).params(),
    );
    for (index, item) in data.equipment.iter().enumerate() {
        let cost = equipment_cost(&item.slot);
        let equipped = character.equipment_ids.contains(&item.id);
        let unlocked = campaign.equipment_is_unlocked(item);
        let prototype_available =
            campaign.salvage_prototypes > 0 && item.required_protocol.is_empty();
        let has_workshop = campaign.colony.has_facility(BuildingKind::Workshop);
        let affordable = prototype_available || campaign.colony.resources.materials >= cost as i32;
        let enabled = has_workshop && unlocked && !equipped && affordable;
        let rect = equipment_row_rect(index);
        let label = equipment_label(
            unlocked,
            equipped,
            prototype_available,
            has_workshop,
            affordable,
            &item.name,
            cost,
        );
        if button(rect, "", enabled, mouse) {
            actions.push(UiAction::CraftSelected(item.id.clone()));
        }
        if let Some(index) = crate::visual_assets::equipment_index(&item.id) {
            visuals.draw_atlas_cell(
                assets,
                &visuals.equipment,
                index,
                Rect::new(rect.x + 5.0, rect.y + 1.0, 30.0, 22.0),
                if unlocked {
                    WHITE
                } else {
                    Color::new(0.35, 0.38, 0.40, 1.0)
                },
            );
        }
        draw_ui_text_ex(
            &label.to_uppercase(),
            rect.x + 42.0,
            rect.y + 17.0,
            TextStyle::new(
                9.5,
                if enabled || equipped {
                    dark::TEXT
                } else {
                    dark::TEXT_DIM
                },
            )
            .params(),
        );
    }
    if button(
        Rect::new(1020.0, 103.0, 214.0, 28.0),
        "RETURN TO COLONY",
        true,
        mouse,
    ) {
        actions.push(UiAction::ReturnToColony);
    }
}

#[cfg(test)]
mod tests;
