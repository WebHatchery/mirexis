//! Colony hub presentation and strategic intent production.

use crate::campaign::{Availability, CampaignState};
use crate::colony::{COLONY_HEIGHT, COLONY_WIDTH};
use crate::data::GameData;
use crate::ui::{UiAction, LOGICAL_HEIGHT, LOGICAL_WIDTH};
use macroquad::prelude::*;
use macroquad_toolkit::prelude::*;
use macroquad_toolkit::ui::{draw_ui_text_ex, VirtualUi};

pub fn draw_colony(data: &GameData, campaign: &CampaignState, ui: &VirtualUi) -> Vec<UiAction> {
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
    draw_operations(data, campaign, mouse, &mut actions);
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
        Some("SETTLEMENT LAYOUT // CLICK AN EMPTY PLOT TO PLAN A BARRICADE"),
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
            let fill = if building.is_some() {
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
                &SurfaceStyle::new(fill).with_border(1.0, Color::new(0.18, 0.38, 0.36, 0.8)),
            );
            if let Some(building) = building {
                draw_building_label(rect, building.kind.name());
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
    data: &GameData,
    campaign: &CampaignState,
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
        230.0,
        TextStyle::new(13.0, dark::TEXT_DIM).params(),
    );
    if colony_button(
        Rect::new(878.0, 252.0, 362.0, 42.0),
        "BARRACKS: TRAIN KIRA AS SOLDIER",
        campaign.roster[0].active_class != "soldier",
        mouse,
    ) {
        actions.push(UiAction::TrainKira);
    }
    if colony_button(
        Rect::new(878.0, 304.0, 362.0, 42.0),
        "INFIRMARY: PRIORITY TREATMENT",
        recovering > 0,
        mouse,
    ) {
        actions.push(UiAction::TreatInjury);
    }
    if colony_button(
        Rect::new(878.0, 356.0, 362.0, 42.0),
        "WORKSHOP: CRAFT KIRA ARMOUR",
        !campaign.roster[0]
            .equipment_ids
            .iter()
            .any(|id| id == "chitin_plate"),
        mouse,
    ) {
        actions.push(UiAction::CraftKiraArmour);
    }
    draw_ui_text_ex(
        "MISSION OFFER",
        878.0,
        450.0,
        TextStyle::new(15.0, dark::ACCENT).params(),
    );
    draw_ui_text_ex(
        &data.mission.name,
        878.0,
        482.0,
        TextStyle::new(23.0, dark::TEXT_BRIGHT).params(),
    );
    if colony_button(
        Rect::new(878.0, 520.0, 362.0, 50.0),
        "SELECT OPERATION GLASSROOT",
        campaign
            .colony
            .mission_offers
            .iter()
            .any(|id| id == &data.mission.id),
        mouse,
    ) {
        actions.push(UiAction::OpenMissionBriefing);
    }
    draw_ui_text_ex(
        &format!(
            "Barricade: 20 materials · one operation · {} structures mapped",
            defense.blocked_tiles.len()
        ),
        878.0,
        626.0,
        TextStyle::new(13.0, dark::TEXT_DIM).params(),
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
