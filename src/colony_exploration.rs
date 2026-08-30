//! Touch-first colony walking, inhabitants, and contextual conversations.

use crate::campaign::{Availability, CampaignState, CharacterRecord};
use crate::colony::{BuildingKind, ColonyState, COLONY_HEIGHT, COLONY_WIDTH};
use crate::colony_map_ui::view::ColonyView;
use crate::colony_story;
use crate::tactical::{UnitAnimationState, UnitFacing};
use crate::ui::UiAction;
use crate::ui_widgets::button;
use crate::visual_assets::VisualCatalog;
use macroquad::prelude::*;
use macroquad_toolkit::assets::AssetManager;
const PLAYER_START: Vec2 = Vec2::new(9.0, 10.0);
const WALK_SPEED: f32 = 3.6;
const PLAYER_RADIUS: f32 = 0.22;
const INTERACTION_DISTANCE: f32 = 1.15;
const NPC_STATIONS: [[i32; 2]; 7] = [
    [8, 10],
    [7, 9],
    [10, 8],
    [13, 9],
    [11, 12],
    [15, 6],
    [17, 6],
];

#[derive(Debug, Clone)]
pub(crate) struct ColonyExplorer {
    position: Vec2,
    destination: Option<Vec2>,
    keyboard_direction: Vec2,
    touch_direction: Vec2,
    facing: UnitFacing,
    approached_npc: Option<String>,
    talking_to: Option<String>,
    build_mode: bool,
}

impl Default for ColonyExplorer {
    fn default() -> Self {
        Self {
            position: PLAYER_START,
            destination: None,
            keyboard_direction: Vec2::ZERO,
            touch_direction: Vec2::ZERO,
            facing: UnitFacing::SouthEast,
            approached_npc: None,
            talking_to: None,
            build_mode: false,
        }
    }
}

impl ColonyExplorer {
    pub(crate) fn reset(&mut self) {
        *self = Self::default();
    }

    pub(crate) fn has_left_start(&self) -> bool {
        self.position.distance_squared(PLAYER_START) > 0.01
    }

    pub(crate) fn update(&mut self, dt: f32, colony: &ColonyState) {
        let manual = self.keyboard_direction + self.touch_direction;
        let (direction, remaining) = if manual.length_squared() > 0.01 {
            self.destination = None;
            self.approached_npc = None;
            self.close_dialogue();
            (manual.normalize(), f32::INFINITY)
        } else if let Some(destination) = self.destination {
            let offset = destination - self.position;
            if offset.length() <= 0.025 {
                self.destination = None;
                return;
            }
            (offset.normalize(), offset.length())
        } else {
            return;
        };
        self.set_facing(direction);
        let distance = (WALK_SPEED * dt).min(remaining);
        let movement = direction * distance;
        let before = self.position;
        self.move_with_collision(movement, colony);
        if self.position.distance_squared(before) < 0.000_001 {
            self.destination = None;
            self.approached_npc = None;
        } else if remaining <= distance + 0.001 {
            self.destination = None;
        }
    }

    pub(crate) fn set_keyboard_direction(&mut self, direction: Vec2) {
        self.keyboard_direction = direction;
    }

    pub(crate) fn set_touch_direction(&mut self, direction: Vec2) {
        self.touch_direction = direction;
    }

    pub(crate) fn request_walk(&mut self, destination: Vec2, colony: &ColonyState) {
        self.close_dialogue();
        self.approached_npc = None;
        if can_occupy(colony, destination) {
            self.destination = Some(destination);
        }
    }

    pub(crate) fn request_approach(
        &mut self,
        npc_id: &str,
        npc_position: Vec2,
        colony: &ColonyState,
    ) {
        self.close_dialogue();
        let away = (self.position - npc_position).normalize_or_zero();
        let direction = if away.length_squared() > 0.01 {
            away
        } else {
            vec2(-1.0, 0.0)
        };
        let destination = npc_position + direction * (INTERACTION_DISTANCE * 0.72);
        if can_occupy(colony, destination) {
            self.approached_npc = Some(npc_id.to_owned());
            self.destination = Some(destination);
        }
    }

    pub(crate) fn interact(&mut self, campaign: &CampaignState) {
        if let Some(character) = nearest_npc(campaign, self.position) {
            self.destination = None;
            self.approached_npc = None;
            self.talking_to = Some(character.id.clone());
        }
    }

    pub(crate) fn update_approach(&mut self, campaign: &CampaignState) {
        if self.destination.is_some() {
            return;
        }
        let ready = self.approached_npc.as_deref().is_some_and(|id| {
            npc(campaign, id).is_some_and(|character| {
                npc_position(campaign, &character.id).is_some_and(|position| {
                    self.position.distance(position) <= INTERACTION_DISTANCE
                })
            })
        });
        if ready {
            self.interact(campaign);
        }
    }

    pub(crate) fn set_build_mode(&mut self, enabled: bool) {
        self.build_mode = enabled;
        if enabled {
            self.destination = None;
            self.close_dialogue();
        }
    }

    pub(crate) fn build_mode(&self) -> bool {
        self.build_mode
    }

    pub(crate) fn close_dialogue(&mut self) {
        self.talking_to = None;
    }

    pub(crate) fn draw_depth(
        &self,
        depth: i32,
        campaign: &CampaignState,
        assets: &AssetManager,
        visuals: &VisualCatalog,
        view: ColonyView,
        mouse: Vec2,
    ) -> Option<String> {
        let mut clicked = None;
        for (index, character) in campaign
            .roster
            .iter()
            .enumerate()
            .skip(1)
            .take(NPC_STATIONS.len() - 1)
        {
            let position =
                npc_grid_position(campaign, &character.id).unwrap_or(NPC_STATIONS[index]);
            if position[0] + position[1] != depth {
                continue;
            }
            let center = view.plot_center(position) + vec2(0.0, 7.0 * view.zoom);
            let hit = Rect::new(center.x - 22.0, center.y - 35.0, 44.0, 48.0);
            let hovered = hit.contains(mouse);
            let highlighted = campaign.first_hour.stage
                == crate::first_hour::FirstHourStage::MeetCoordinator
                && character.id == "mara_venn";
            draw_character(CharacterDrawContext {
                character,
                center,
                facing_right: index % 2 == 0,
                moving: false,
                interactable: true,
                hovered,
                highlighted,
                assets,
                visuals,
                zoom: view.zoom,
            });
            if hovered && is_mouse_button_released(MouseButton::Left) {
                clicked = Some(character.id.clone());
            }
        }
        if (self.position.x + self.position.y).round() as i32 == depth {
            if let Some(player) = campaign.roster.first() {
                let center = view.world_center(self.position) + vec2(0.0, 7.0 * view.zoom);
                draw_character(CharacterDrawContext {
                    character: player,
                    center,
                    facing_right: matches!(
                        self.facing,
                        UnitFacing::SouthEast | UnitFacing::NorthEast
                    ),
                    moving: self.is_moving(),
                    interactable: false,
                    hovered: false,
                    highlighted: false,
                    assets,
                    visuals,
                    zoom: view.zoom,
                });
                draw_circle_lines(
                    center.x,
                    center.y + 5.0,
                    10.0 * view.zoom,
                    2.0,
                    Color::new(0.66, 1.0, 0.76, 0.84),
                );
            }
        }
        clicked
    }

    pub(crate) fn draw_dialogue(
        &mut self,
        campaign: &CampaignState,
        mouse: Vec2,
        actions: &mut Vec<UiAction>,
    ) -> bool {
        let Some(character) = self.talking_to.as_deref().and_then(|id| npc(campaign, id)) else {
            return false;
        };
        let panel = Rect::new(66.0, 400.0, 722.0, 170.0);
        draw_rectangle(
            panel.x,
            panel.y,
            panel.w,
            panel.h,
            Color::new(0.025, 0.07, 0.075, 0.98),
        );
        draw_rectangle_lines(
            panel.x,
            panel.y,
            panel.w,
            panel.h,
            2.0,
            Color::new(0.32, 0.86, 0.72, 0.92),
        );
        draw_text(
            character.name.to_uppercase(),
            84.0,
            428.0,
            20.0,
            Color::new(0.76, 1.0, 0.88, 1.0),
        );
        draw_text(
            npc_status(character),
            84.0,
            448.0,
            12.0,
            Color::new(0.45, 0.72, 0.66, 1.0),
        );
        let beat = colony_story::identity_arc_beat(
            &campaign.strategy.mirexis_path_id,
            &character.id,
            campaign.strategy.campaign_complete,
            &campaign.colony_story,
        )
        .or_else(|| {
            colony_story::contact_route_beat(
                &campaign.strategy.contact_protocol_id,
                &character.id,
                campaign.strategy.contact_trace_completed,
                campaign.strategy.contact_complete,
            )
        })
        .or_else(|| colony_story::phase_beat(&campaign.strategy.phase_id, &character.id))
        .or_else(|| {
            campaign
                .last_operation_had_commons_meal()
                .then(|| colony_story::commons_meal_beat(&character.id))
                .flatten()
        })
        .or_else(|| {
            colony_story::current_beat(
                &character.id,
                campaign.operations_completed,
                campaign.first_hour.first_outcome_won,
                campaign.first_hour.second_outcome_won,
            )
        });
        let (title, text, heard) = beat.map_or(
            ("COLONY BIOGRAPHY", character.biography.as_str(), true),
            |beat| {
                (
                    beat.title,
                    beat.text,
                    campaign.colony_story.has_heard(beat.id),
                )
            },
        );
        draw_text(
            format!("FIELD NOTE // {title}"),
            84.0,
            470.0,
            12.0,
            Color::new(0.95, 0.78, 0.30, 1.0),
        );
        draw_text(
            if heard { "ARCHIVED" } else { "NEW FIELD NOTE" },
            668.0,
            470.0,
            11.0,
            if heard {
                Color::new(0.45, 0.72, 0.66, 1.0)
            } else {
                Color::new(0.95, 0.78, 0.30, 1.0)
            },
        );
        draw_wrapped(text, 84.0, 491.0, 660.0);
        if button(
            Rect::new(84.0, 532.0, 176.0, 30.0),
            npc_action_label(character),
            true,
            mouse,
        ) {
            actions.push(npc_action(character));
            actions.push(UiAction::AcknowledgeColonist(character.id.clone()));
            self.close_dialogue();
        }
        if button(
            Rect::new(632.0, 532.0, 136.0, 30.0),
            "CONTINUE",
            true,
            mouse,
        ) {
            actions.push(UiAction::AcknowledgeColonist(character.id.clone()));
            self.close_dialogue();
        }
        true
    }

    pub(crate) fn is_talking(&self) -> bool {
        self.talking_to.is_some()
    }

    pub(crate) fn can_talk(&self, campaign: &CampaignState) -> bool {
        nearest_npc(campaign, self.position).is_some()
    }

    fn is_moving(&self) -> bool {
        self.destination.is_some()
            || (self.keyboard_direction + self.touch_direction).length_squared() > 0.01
    }

    fn set_facing(&mut self, direction: Vec2) {
        self.facing = if direction.x + direction.y >= 0.0 {
            UnitFacing::SouthEast
        } else {
            UnitFacing::NorthWest
        };
    }

    fn move_with_collision(&mut self, movement: Vec2, colony: &ColonyState) {
        let horizontal = self.position + vec2(movement.x, 0.0);
        if can_occupy(colony, horizontal) {
            self.position = horizontal;
        }
        let vertical = self.position + vec2(0.0, movement.y);
        if can_occupy(colony, vertical) {
            self.position = vertical;
        }
    }
}

struct CharacterDrawContext<'a> {
    character: &'a CharacterRecord,
    center: Vec2,
    facing_right: bool,
    moving: bool,
    interactable: bool,
    hovered: bool,
    highlighted: bool,
    assets: &'a AssetManager,
    visuals: &'a VisualCatalog,
    zoom: f32,
}

fn draw_character(context: CharacterDrawContext<'_>) {
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
    draw_ellipse(
        center.x,
        center.y + 5.0,
        7.0 * zoom,
        3.0 * zoom,
        0.0,
        Color::new(0.0, 0.0, 0.0, 0.42),
    );
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

fn draw_wrapped(text: &str, x: f32, y: f32, width: f32) {
    let mut line = String::new();
    let mut baseline = y;
    for word in text.split_whitespace() {
        let candidate = if line.is_empty() {
            word.to_owned()
        } else {
            format!("{line} {word}")
        };
        if measure_text(&candidate, None, 14, 1.0).width > width && !line.is_empty() {
            draw_text(&line, x, baseline, 14.0, Color::new(0.72, 0.84, 0.80, 1.0));
            baseline += 18.0;
            line = word.to_owned();
        } else {
            line = candidate;
        }
    }
    draw_text(&line, x, baseline, 14.0, Color::new(0.72, 0.84, 0.80, 1.0));
}

fn npc_status(character: &CharacterRecord) -> String {
    let duty = match character.id.as_str() {
        "mara_venn" => "SECURITY LEAD",
        "ilya_reed" => "COLONY CLINICIAN",
        "sol_cairn" => "CHIEF ENGINEER",
        "nadi_vale" => "XENOBIOLOGY LEAD",
        "veya_orn" => "DIRECTORATE EXILE",
        _ => "COLONIST",
    };
    format!(
        "{} // {}",
        duty,
        match character.availability {
            Availability::Ready => "ON DUTY",
            Availability::Recovering => "RECOVERING",
        }
    )
}

fn npc_action_label(character: &CharacterRecord) -> &'static str {
    match character.id.as_str() {
        "ilya_reed" => "REQUEST TREATMENT",
        "nadi_vale" => "ENTER GENE LAB",
        _ => "OPEN ROSTER",
    }
}

fn npc_action(character: &CharacterRecord) -> UiAction {
    match character.id.as_str() {
        "ilya_reed" => UiAction::TreatInjury,
        "nadi_vale" => UiAction::OpenGeneLab,
        _ => UiAction::OpenRoster,
    }
}

pub(crate) fn npc_position(campaign: &CampaignState, id: &str) -> Option<Vec2> {
    npc_grid_position(campaign, id).map(grid_vec)
}

fn npc_grid_position(campaign: &CampaignState, id: &str) -> Option<[i32; 2]> {
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
    if id == "veya_orn" {
        if let Some(waystation) = campaign
            .colony
            .buildings
            .iter()
            .find(|building| building.kind == crate::colony::BuildingKind::Waystation)
        {
            return Some(waystation.position);
        }
    }
    if id == "sedge" {
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

fn npc<'a>(campaign: &'a CampaignState, id: &str) -> Option<&'a CharacterRecord> {
    campaign
        .roster
        .iter()
        .skip(1)
        .find(|character| character.id == id)
}

fn nearest_npc(campaign: &CampaignState, position: Vec2) -> Option<&CharacterRecord> {
    campaign.roster.iter().skip(1).find(|character| {
        npc_position(campaign, &character.id)
            .is_some_and(|npc_position| position.distance(npc_position) <= INTERACTION_DISTANCE)
    })
}

fn can_occupy(colony: &ColonyState, position: Vec2) -> bool {
    if position.x < PLAYER_RADIUS
        || position.y < PLAYER_RADIUS
        || position.x > (COLONY_WIDTH - 1) as f32 - PLAYER_RADIUS
        || position.y > (COLONY_HEIGHT - 1) as f32 - PLAYER_RADIUS
    {
        return false;
    }
    let collides = |anchor: [i32; 2]| {
        let center = grid_vec(anchor);
        (position.x - center.x).abs() < 0.50 + PLAYER_RADIUS
            && (position.y - center.y).abs() < 0.50 + PLAYER_RADIUS
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

fn grid_vec(position: [i32; 2]) -> Vec2 {
    vec2(position[0] as f32, position[1] as f32)
}

#[cfg(test)]
mod tests;
