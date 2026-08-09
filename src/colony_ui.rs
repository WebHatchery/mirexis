//! Colony hub presentation and strategic intent production.

use crate::campaign::{Availability, CampaignState};
use crate::colony::{COLONY_HEIGHT, COLONY_WIDTH};
use crate::data::GameData;
use crate::ui::{UiAction, LOGICAL_HEIGHT, LOGICAL_WIDTH};
use macroquad::prelude::*;
use macroquad_toolkit::prelude::*;
use macroquad_toolkit::ui::{draw_ui_text_ex, VirtualUi};

pub fn draw_colony(campaign: &CampaignState, data: &GameData, ui: &VirtualUi) -> Vec<UiAction> {
    let mut actions = Vec::new();
    let mouse = ui.mouse_position();
    draw_rectangle(
        0.0,
        0.0,
        LOGICAL_WIDTH,
        LOGICAL_HEIGHT,
        Color::new(0.025, 0.04, 0.055, 1.0),
    );
    draw_header(campaign);
    draw_layout(campaign, mouse, &mut actions);
    draw_operations(campaign, data, mouse, &mut actions);
    actions
}

fn draw_header(campaign: &CampaignState) {
    draw_surface(
        Rect::new(18.0, 16.0, LOGICAL_WIDTH - 36.0, 66.0),
        &SurfaceStyle::new(Color::new(0.045, 0.075, 0.085, 0.98))
            .with_border(1.0, Color::new(0.22, 0.62, 0.56, 0.8))
            .with_left_accent(5.0, Color::new(0.34, 0.86, 0.68, 1.0)),
    );
    draw_ui_text_ex(
        "MIREXIS COLONY",
        42.0,
        57.0,
        TextStyle::new(30.0, dark::TEXT_BRIGHT).params(),
    );
    draw_ui_text_ex(
        &format!("OPERATIONS COMPLETED  {}", campaign.operations_completed),
        940.0,
        54.0,
        TextStyle::new(16.0, dark::TEXT_DIM).params(),
    );
}

fn draw_layout(campaign: &CampaignState, mouse: Vec2, actions: &mut Vec<UiAction>) {
    let panel = Rect::new(18.0, 96.0, 820.0, 580.0);
    draw_surface_with_title(
        panel,
        Some("SETTLEMENT LAYOUT // PLAN BARRICADES · CLICK DAMAGED BUILDINGS TO REPAIR"),
        &SurfaceStyle::new(Color::new(0.035, 0.052, 0.062, 0.98))
            .with_border(1.0, Color::new(0.19, 0.40, 0.40, 0.9))
            .with_header(42.0, Color::new(0.06, 0.10, 0.11, 1.0)),
        TextStyle::new(15.0, dark::TEXT),
    );
    let origin = vec2(105.0, 172.0);
    let tile_size = 78.0;
    for y in 0..COLONY_HEIGHT {
        for x in 0..COLONY_WIDTH {
            let rect = Rect::new(
                origin.x + x as f32 * tile_size,
                origin.y + y as f32 * tile_size,
                tile_size - 4.0,
                tile_size - 4.0,
            );
            let building = campaign
                .colony
                .buildings
                .iter()
                .find(|building| building.position == [x, y]);
            let project = campaign
                .colony
                .construction_queue
                .iter()
                .find(|project| project.position == [x, y]);
            let damaged = building.is_some_and(|building| building.damaged);
            let fill = if damaged {
                Color::new(0.34, 0.10, 0.10, 1.0)
            } else if building.is_some() {
                Color::new(0.10, 0.30, 0.26, 1.0)
            } else if project.is_some() {
                Color::new(0.36, 0.27, 0.10, 1.0)
            } else if rect.contains_point(mouse) {
                Color::new(0.12, 0.22, 0.21, 1.0)
            } else {
                Color::new(0.07, 0.12, 0.125, 1.0)
            };
            draw_surface(
                rect,
                &SurfaceStyle::new(fill).with_border(
                    1.0,
                    if damaged {
                        dark::NEGATIVE
                    } else {
                        Color::new(0.18, 0.38, 0.36, 0.8)
                    },
                ),
            );
            if let Some(building) = building {
                draw_building_label(rect, building.kind.name());
                if building.damaged {
                    draw_text(
                        format!("REPAIR {} MAT", building.kind.repair_cost()),
                        rect.x + 7.0,
                        rect.y + 66.0,
                        10.0,
                        dark::NEGATIVE,
                    );
                    if rect.contains_point(mouse) && is_mouse_button_released(MouseButton::Left) {
                        actions.push(UiAction::RepairBuilding(building.id.clone()));
                    }
                }
            } else if let Some(project) = project {
                draw_building_label(rect, &format!("{}\nPLANNED", project.kind.name()));
            } else if rect.contains_point(mouse) && is_mouse_button_released(MouseButton::Left) {
                actions.push(UiAction::ConstructBarricade([x, y]));
            }
        }
    }
}

fn draw_building_label(rect: Rect, label: &str) {
    let short = match label {
        "Command Centre" => "COMMAND\nCENTRE",
        "Power Plant" => "POWER\nPLANT",
        other => other,
    };
    for (index, line) in short.lines().enumerate() {
        draw_text(
            line,
            rect.x + 7.0,
            rect.y + 30.0 + index as f32 * 18.0,
            14.0,
            dark::TEXT,
        );
    }
}

fn draw_operations(
    campaign: &CampaignState,
    data: &GameData,
    mouse: Vec2,
    actions: &mut Vec<UiAction>,
) {
    let panel = Rect::new(856.0, 96.0, 406.0, 580.0);
    draw_surface_with_title(
        panel,
        Some("COLONY OPERATIONS"),
        &SurfaceStyle::new(Color::new(0.055, 0.07, 0.08, 0.98))
            .with_border(1.0, Color::new(0.25, 0.38, 0.40, 0.8))
            .with_header(42.0, Color::new(0.08, 0.105, 0.115, 1.0)),
        TextStyle::new(17.0, dark::TEXT),
    );
    let resources = &campaign.colony.resources;
    draw_ui_text_ex(
        &format!(
            "MATERIALS {:>3}  //  POWER {:>2}  //  FOOD {:>3}",
            resources.materials, resources.power, resources.food
        ),
        878.0,
        166.0,
        TextStyle::new(15.0, dark::TEXT).params(),
    );
    draw_ui_text_ex(
        &format!(
            "BIOMASS {:>2}  //  ALIEN COMPONENTS {:>2}",
            resources.biomass, resources.alien_components
        ),
        878.0,
        192.0,
        TextStyle::new(15.0, dark::TEXT).params(),
    );
    let attention = campaign
        .strategy
        .factions
        .iter()
        .map(|faction| format!("{} {}", faction.name, faction.attention))
        .collect::<Vec<_>>()
        .join("  /  ");
    draw_ui_text_ex(
        &attention,
        878.0,
        218.0,
        TextStyle::new(12.0, dark::TEXT_DIM).params(),
    );
    if let Some(threat) = campaign.strategy.active_threat() {
        draw_ui_text_ex(
            &format!(
                "{}  //  {} OPERATIONS  //  STRENGTH {}",
                threat.name, threat.operations_until, threat.strength
            ),
            878.0,
            242.0,
            TextStyle::new(12.0, dark::WARNING).params(),
        );
    }
    let ready = campaign
        .roster
        .iter()
        .filter(|character| character.availability == Availability::Ready)
        .count();
    let recovering = campaign.roster.len() - ready;
    let defense = campaign.colony.defense_map();
    draw_ui_text_ex(
        &format!(
            "ROSTER {} READY / {} RECOVERING  //  DEFENCE {} COVER / {} CRITICAL",
            ready,
            recovering,
            defense.cover_tiles.len(),
            defense.critical_objectives.len()
        ),
        878.0,
        262.0,
        TextStyle::new(12.0, dark::TEXT_DIM).params(),
    );
    if colony_button(
        Rect::new(878.0, 274.0, 362.0, 32.0),
        "MANAGE ROSTER // TRAINING & GEAR",
        true,
        mouse,
    ) {
        actions.push(UiAction::OpenRoster);
    }
    if colony_button(
        Rect::new(878.0, 312.0, 362.0, 32.0),
        "INFIRMARY: PRIORITY TREATMENT",
        recovering > 0,
        mouse,
    ) {
        actions.push(UiAction::TreatInjury);
    }
    draw_ui_text_ex(
        "MISSION OFFERS",
        878.0,
        372.0,
        TextStyle::new(15.0, dark::ACCENT).params(),
    );
    for (index, mission) in campaign.strategy.mission_offers.iter().take(2).enumerate() {
        let selected = mission.id == campaign.strategy.selected_mission_id;
        let pressure = if mission.operation_modifier == crate::data::OperationModifier::None {
            ""
        } else {
            " [PRESSURE]"
        };
        if colony_button(
            Rect::new(878.0, 384.0 + index as f32 * 38.0, 362.0, 32.0),
            &format!(
                "{}{}{}",
                if selected { "> " } else { "" },
                mission.name,
                pressure
            ),
            true,
            mouse,
        ) {
            actions.push(UiAction::SelectMission(mission.id.clone()));
        }
    }
    if colony_button(
        Rect::new(878.0, 462.0, 362.0, 36.0),
        "BRIEF SELECTED MISSION",
        campaign.strategy.selected_mission().is_some(),
        mouse,
    ) {
        actions.push(UiAction::OpenMissionBriefing);
    }
    if let Some(research) = campaign
        .strategy
        .research
        .iter()
        .find(|entry| !entry.completed)
    {
        if colony_button(
            Rect::new(878.0, 510.0, 362.0, 32.0),
            &format!(
                "RESEARCH {} · {} MAT",
                research.name, research.materials_cost
            ),
            true,
            mouse,
        ) {
            actions.push(UiAction::CompleteResearch(research.id.clone()));
        }
    }
    if let Some(event) = campaign
        .strategy
        .character_events
        .iter()
        .find(|entry| !entry.resolved)
    {
        let definition = data
            .campaign
            .events
            .iter()
            .find(|definition| definition.id == event.id);
        let legacy_character_id = if event.legacy_character_id.is_empty() {
            definition.map_or("", |definition| definition.legacy_character_id.as_str())
        } else {
            event.legacy_character_id.as_str()
        };
        let legacy_stat = if event.legacy_stat.is_empty() {
            definition.map_or("", |definition| definition.legacy_stat.as_str())
        } else {
            event.legacy_stat.as_str()
        };
        let legacy_amount = if event.legacy_amount == 0 {
            definition.map_or(0, |definition| definition.legacy_amount)
        } else {
            event.legacy_amount
        };
        if colony_button(
            Rect::new(878.0, 548.0, 362.0, 32.0),
            &format!("EVENT: {}", event.title),
            true,
            mouse,
        ) {
            actions.push(UiAction::ResolveCharacterEvent);
        }
        let recipient = campaign
            .roster
            .iter()
            .find(|character| character.id == legacy_character_id)
            .map_or("UNKNOWN", |character| {
                character
                    .name
                    .split_whitespace()
                    .next()
                    .unwrap_or("UNKNOWN")
            });
        draw_ui_text_ex(
            &format!(
                "CHOICE EFFECT // {} {:+} {} · {} FOOD · DIRECTORATE {:+}",
                recipient.to_uppercase(),
                legacy_amount,
                legacy_stat.to_uppercase(),
                event.food_cost,
                event.attention_change
            ),
            878.0,
            588.0,
            TextStyle::new(10.0, dark::TEXT_DIM).params(),
        );
    }
    draw_ui_text_ex(
        "ACTIVE DOCTRINES",
        878.0,
        600.0,
        TextStyle::new(12.0, dark::ACCENT).params(),
    );
    let mut doctrine_y = 614.0;
    for research in campaign
        .strategy
        .research
        .iter()
        .filter(|entry| entry.completed)
    {
        draw_ui_text_ex(
            &format!("{} // {}", research.name, research.description),
            878.0,
            doctrine_y,
            TextStyle::new(10.0, dark::TEXT_DIM).params(),
        );
        doctrine_y += 11.0;
    }
    if doctrine_y == 614.0 {
        draw_ui_text_ex(
            "No completed field doctrine",
            878.0,
            doctrine_y,
            TextStyle::new(10.0, dark::TEXT_DIM).params(),
        );
    }
    draw_ui_text_ex(
        &format!(
            "Barricade: 20 materials · one operation · {} structures mapped",
            defense.blocked_tiles.len()
        ),
        878.0,
        650.0,
        TextStyle::new(12.0, dark::TEXT_DIM).params(),
    );
}

fn colony_button(rect: Rect, label: &str, enabled: bool, mouse: Vec2) -> bool {
    let hovered = enabled && rect.contains_point(mouse);
    let fill = if !enabled {
        Color::new(0.08, 0.10, 0.11, 1.0)
    } else if hovered {
        Color::new(0.18, 0.43, 0.37, 1.0)
    } else {
        Color::new(0.10, 0.28, 0.25, 1.0)
    };
    draw_surface(
        rect,
        &SurfaceStyle::new(fill).with_border(1.0, Color::new(0.33, 0.72, 0.60, 1.0)),
    );
    draw_text(
        label,
        rect.x + 14.0,
        rect.y + 27.0,
        15.0,
        if enabled { dark::TEXT } else { dark::TEXT_DIM },
    );
    hovered && is_mouse_button_released(MouseButton::Left)
}
