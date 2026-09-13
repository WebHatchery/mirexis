//! Colony exploration characters, depth ordering, and touch dialogue rendering.

use super::*;

pub struct ColonyDepthContext<'a> {
    pub campaign: &'a CampaignState,
    pub data: &'a GameData,
    pub assets: &'a AssetManager,
    pub visuals: &'a VisualCatalog,
    pub view: ColonyView,
    pub mouse: Vec2,
}

pub struct CharacterDrawContext<'a> {
    pub character: &'a CharacterRecord,
    pub center: Vec2,
    pub facing_right: bool,
    pub moving: bool,
    pub interactable: bool,
    pub hovered: bool,
    pub highlighted: bool,
    pub assets: &'a AssetManager,
    pub visuals: &'a VisualCatalog,
    pub zoom: f32,
}

pub fn draw_character(context: CharacterDrawContext<'_>) {
    let CharacterDrawContext {
        character,
        center,
        facing_right,
        moving,
        interactable,
        hovered,
        highlighted,
        assets,
        visuals,
        zoom,
    } = context;
    draw_colony_contact(center, zoom, highlighted);
    if hovered {
        draw_circle(
            center.x,
            center.y - 6.0,
            19.0 * zoom,
            Color::new(0.24, 0.78, 0.62, 0.22),
        );
    }
    if highlighted {
        draw_circle_lines(
            center.x,
            center.y - 6.0,
            23.0 * zoom,
            3.0,
            Color::new(0.95, 0.78, 0.30, 0.96),
        );
    }
    visuals.draw_unit_pose(
        assets,
        &character.id,
        &character.name,
        if facing_right {
            UnitFacing::SouthEast
        } else {
            UnitFacing::SouthWest
        },
        if moving {
            UnitAnimationState::Move
        } else {
            UnitAnimationState::Idle
        },
        Rect::new(
            center.x - 20.0 * zoom,
            center.y - 22.0 * zoom,
            40.0 * zoom,
            30.0 * zoom,
        ),
        WHITE,
    );
    if interactable {
        let marker = center + vec2(0.0, -29.0 * zoom);
        draw_circle(
            marker.x,
            marker.y,
            8.0,
            if hovered {
                Color::new(0.54, 1.0, 0.78, 1.0)
            } else {
                Color::new(0.18, 0.58, 0.50, 0.96)
            },
        );
        for offset in [-3.5, 0.0, 3.5] {
            draw_circle(
                marker.x + offset,
                marker.y,
                1.1,
                Color::new(0.02, 0.08, 0.08, 1.0),
            );
        }
    }
    if hovered || highlighted {
        let width = measure_text(&character.name, None, 12, 1.0).width + 14.0;
        draw_rectangle(
            center.x - width / 2.0,
            center.y - 38.0,
            width,
            18.0,
            Color::new(0.02, 0.08, 0.08, 0.94),
        );
        draw_text(
            &character.name,
            center.x - width / 2.0 + 7.0,
            center.y - 25.0,
            12.0,
            Color::new(0.76, 1.0, 0.88, 1.0),
        );
    }
}

pub fn draw_colony_contact(center: Vec2, zoom: f32, highlighted: bool) {
    let accent = if highlighted {
        Color::new(0.96, 0.78, 0.30, 0.76)
    } else {
        Color::new(0.24, 0.82, 0.68, 0.62)
    };
    draw_ellipse(
        center.x,
        center.y + 4.0 * zoom,
        8.0 * zoom,
        2.5 * zoom,
        0.0,
        Color::new(accent.r, accent.g, accent.b, 0.20),
    );
    draw_ellipse_lines(
        center.x,
        center.y + 4.0 * zoom,
        8.0 * zoom,
        2.5 * zoom,
        0.0,
        1.0 * zoom,
        accent,
    );
}

pub fn draw_wrapped(text: &str, x: f32, y: f32, width: f32) {
    macroquad_toolkit::ui::draw_text_block_ex(
        text,
        x,
        y - 14.0,
        width,
        50.0,
        macroquad_toolkit::ui::TextStyle::new(14.0, Color::new(0.72, 0.84, 0.80, 1.0))
            .with_macroquad_font()
            .with_line_gap(4.0),
        14.0,
    );
}
pub fn npc_status(character: &CharacterRecord, guest: bool) -> String {
    let duty = match character.id.as_str() {
        "mara_venn" => "SECURITY LEAD",
        "ilya_reed" => "COLONY CLINICIAN",
        "sol_cairn" => "CHIEF ENGINEER",
        "nadi_vale" => "XENOBIOLOGY LEAD",
        "veya_orn" => "DIRECTORATE EXILE",
        "sedge" => "MIREBORN COURIER",
        _ => "COLONIST",
    };
    if guest {
        format!("{duty} // WAYSTATION GUEST // NOT ON ROSTER")
    } else {
        format!(
            "{} // {}",
            duty,
            match character.availability {
                Availability::Ready => "ON DUTY",
                Availability::Recovering => "RECOVERING",
            }
        )
    }
}

pub fn npc_action_label(character: &CharacterRecord, guest: bool) -> &'static str {
    if guest {
        return "RECRUIT CONTACT";
    }
    match character.id.as_str() {
        "ilya_reed" => "REQUEST TREATMENT",
        "nadi_vale" | "sedge" => "ENTER GENE LAB",
        _ => "OPEN ROSTER",
    }
}

pub fn npc_action(character: &CharacterRecord, guest: bool) -> Option<UiAction> {
    if guest {
        return Some(UiAction::RecruitOutsider);
    }
    match character.id.as_str() {
        "ilya_reed" if !character.injuries.is_empty() => Some(UiAction::TreatInjury),
        "ilya_reed" => None,
        "nadi_vale" | "sedge" => Some(UiAction::OpenGeneLab),
        _ => Some(UiAction::OpenRoster),
    }
}

pub fn npc_position(campaign: &CampaignState, id: &str) -> Option<Vec2> {
    npc_grid_position(campaign, id).map(grid_vec)
}

pub fn npc_grid_position(campaign: &CampaignState, id: &str) -> Option<[i32; 2]> {
    if colony_story::identity_npc(&campaign.strategy.mirexis_path_id) == Some(id) {
        if let Some(kind) = BuildingKind::identity_for_path(&campaign.strategy.mirexis_path_id) {
            if let Some(identity_building) = campaign
                .colony
                .buildings
                .iter()
                .find(|building| building.kind == kind)
            {
                return Some(identity_building.position);
            }
        }
    }
    let rostered = campaign.roster.iter().any(|character| character.id == id);
    if id == "veya_orn" || (id == "sedge" && !rostered) {
        if let Some(waystation) = campaign
            .colony
            .buildings
            .iter()
            .find(|building| building.kind == BuildingKind::Waystation)
        {
            return Some(waystation.position);
        }
    }
    if id == "sedge" && rostered {
        if let Some(gene_lab) = campaign
            .colony
            .buildings
            .iter()
            .find(|building| building.kind == BuildingKind::GeneLab)
        {
            return Some(gene_lab.position);
        }
    }
    campaign
        .roster
        .iter()
        .position(|character| character.id == id)
        .and_then(|index| (index > 0 && index < NPC_STATIONS.len()).then_some(NPC_STATIONS[index]))
}

pub struct NpcView {
    pub character: CharacterRecord,
    pub guest: bool,
}

pub fn npc(campaign: &CampaignState, data: &GameData, id: &str) -> Option<NpcView> {
    if let Some(character) = campaign
        .roster
        .iter()
        .skip(1)
        .find(|character| character.id == id)
    {
        return Some(NpcView {
            character: character.clone(),
            guest: false,
        });
    }
    let definition = campaign.available_outsider(data)?;
    (definition.id == id).then(|| NpcView {
        character: CharacterRecord::from_def(definition),
        guest: true,
    })
}

pub fn nearest_npc(campaign: &CampaignState, data: &GameData, position: Vec2) -> Option<String> {
    nearest_npc_with_distance(campaign, data, position, INTERACTION_DISTANCE)
}

pub fn nearest_npc_with_distance(
    campaign: &CampaignState,
    data: &GameData,
    position: Vec2,
    interaction_distance: f32,
) -> Option<String> {
    if let Some(character) = campaign.roster.iter().skip(1).find(|character| {
        npc_position(campaign, &character.id)
            .is_some_and(|npc_position| position.distance(npc_position) <= interaction_distance)
    }) {
        return Some(character.id.clone());
    }
    let definition = campaign.available_outsider(data)?;
    npc_position(campaign, &definition.id)
        .is_some_and(|npc_position| position.distance(npc_position) <= interaction_distance)
        .then(|| definition.id.clone())
}

pub fn can_occupy(colony: &ColonyState, position: Vec2) -> bool {
    can_occupy_with_radius(colony, position, PLAYER_RADIUS)
}

pub fn can_occupy_with_radius(colony: &ColonyState, position: Vec2, player_radius: f32) -> bool {
    if position.x < player_radius
        || position.y < player_radius
        || position.x > (COLONY_WIDTH - 1) as f32 - player_radius
        || position.y > (COLONY_HEIGHT - 1) as f32 - player_radius
    {
        return false;
    }
    let collides = |anchor: [i32; 2]| {
        let center = grid_vec(anchor);
        (position.x - center.x).abs() < 0.50 + player_radius
            && (position.y - center.y).abs() < 0.50 + player_radius
    };
    let clear_of_npcs = NPC_STATIONS
        .iter()
        .skip(1)
        .all(|station| position.distance(grid_vec(*station)) >= 0.52);
    clear_of_npcs
        && !colony
            .buildings
            .iter()
            .any(|building| collides(building.position))
        && !colony
            .construction_queue
            .iter()
            .any(|project| collides(project.position))
}

pub fn grid_vec(position: [i32; 2]) -> Vec2 {
    vec2(position[0] as f32, position[1] as f32)
}

pub fn player_draw_depth(position: Vec2) -> i32 {
    (position.x + position.y).floor() as i32
}
