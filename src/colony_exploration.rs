//! Touch-first colony walking, inhabitants, and contextual conversations.
pub mod rendering;
pub use rendering::*;

use crate::campaign::{Availability, CampaignState, CharacterRecord};
use crate::colony::{BuildingKind, ColonyState, COLONY_HEIGHT, COLONY_WIDTH};
use crate::colony_map_ui::view::ColonyView;
use crate::colony_story;
use crate::data::{GameConfig, GameData};
use crate::tactical::{UnitAnimationState, UnitFacing};
use crate::ui::UiAction;
use crate::ui_widgets::button;
use crate::visual_assets::VisualCatalog;
use macroquad::prelude::*;
use macroquad_toolkit::assets::AssetManager;
pub const PLAYER_START: Vec2 = Vec2::new(9.0, 10.0);
pub const WALK_SPEED: f32 = 3.6;
pub const PLAYER_RADIUS: f32 = 0.22;
pub const INTERACTION_DISTANCE: f32 = 1.15;
pub const NPC_STATIONS: [[i32; 2]; 7] = [
    [8, 10],
    [7, 9],
    [10, 8],
    [13, 9],
    [11, 12],
    [15, 6],
    [17, 6],
];

pub fn dialogue_continue_button_bounds() -> Rect {
    Rect::new(632.0, 532.0, 136.0, 30.0)
}

fn dialogue_copy<'a>(
    campaign: &CampaignState,
    character: &'a CharacterRecord,
) -> (&'static str, &'a str, bool) {
    let beat = colony_story::identity_arc_beat(
        &campaign.strategy.mirexis_path_id,
        &character.id,
        campaign.strategy.campaign_complete,
        campaign.strategy.post_campaign_operations_completed,
        colony_story::IdentityArcProgress {
            preparations_completed: campaign.identity_preparations_completed,
            stewardship_completed: campaign.identity_stewardship_completed,
        },
        &campaign.colony_story,
        campaign.identity_building_story_state(),
    )
    .or_else(|| {
        colony_story::finale_beat(
            &campaign.strategy.mirexis_path_id,
            &character.id,
            campaign.strategy.campaign_complete,
            &campaign.colony_story,
        )
    })
    .or_else(|| {
        colony_story::contact_route_beat(
            &campaign.strategy.contact_protocol_id,
            &character.id,
            campaign.strategy.contact_trace_completed,
            campaign.strategy.contact_complete,
        )
    })
    .or_else(|| {
        campaign
            .colony
            .facility_upgrades
            .iter()
            .rev()
            .find_map(|upgrade| {
                colony_story::facility_upgrade_beat(
                    &upgrade.upgrade_id,
                    &character.id,
                    &campaign.colony_story,
                )
            })
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
    beat.map_or(
        ("COLONY BIOGRAPHY", character.biography.as_str(), true),
        |beat| {
            (
                beat.title,
                beat.text,
                campaign.colony_story.has_heard(beat.id),
            )
        },
    )
}

pub mod dialogue;
pub use dialogue::{npc_action_button_label, npc_action_enabled};

#[derive(Debug, Clone)]
pub struct ColonyExplorer {
    pub position: Vec2,
    pub destination: Option<Vec2>,
    pub keyboard_direction: Vec2,
    pub touch_direction: Vec2,
    pub facing: UnitFacing,
    pub approached_npc: Option<String>,
    pub talking_to: Option<String>,
    pub build_mode: bool,
    pub player_start: Vec2,
    pub walk_speed: f32,
    pub player_radius: f32,
    pub interaction_distance: f32,
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
            player_start: PLAYER_START,
            walk_speed: WALK_SPEED,
            player_radius: PLAYER_RADIUS,
            interaction_distance: INTERACTION_DISTANCE,
        }
    }
}

impl ColonyExplorer {
    pub fn from_config(config: &GameConfig) -> Self {
        let mut explorer = Self::default();
        explorer.player_start = vec2(
            config.exploration.player_start[0],
            config.exploration.player_start[1],
        );
        explorer.position = explorer.player_start;
        explorer.walk_speed = config.exploration.walk_speed;
        explorer.player_radius = config.exploration.player_radius;
        explorer.interaction_distance = config.exploration.interaction_distance;
        explorer
    }

    pub fn reset(&mut self) {
        let player_start = self.player_start;
        let walk_speed = self.walk_speed;
        let player_radius = self.player_radius;
        let interaction_distance = self.interaction_distance;
        *self = Self::default();
        self.player_start = player_start;
        self.position = player_start;
        self.walk_speed = walk_speed;
        self.player_radius = player_radius;
        self.interaction_distance = interaction_distance;
    }

    pub fn has_left_start(&self) -> bool {
        self.position.distance_squared(self.player_start) > 0.01
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn update(&mut self, dt: f32, colony: &ColonyState) {
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
        let distance = (self.walk_speed * dt).min(remaining);
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

    pub fn set_keyboard_direction(&mut self, direction: Vec2) {
        self.keyboard_direction = direction;
    }

    pub fn set_touch_direction(&mut self, direction: Vec2) {
        self.touch_direction = direction;
    }

    pub fn request_walk(&mut self, destination: Vec2, colony: &ColonyState) {
        self.close_dialogue();
        self.approached_npc = None;
        if can_occupy_with_radius(colony, destination, self.player_radius) {
            self.destination = Some(destination);
        }
    }

    pub fn request_approach(&mut self, npc_id: &str, npc_position: Vec2, colony: &ColonyState) {
        self.close_dialogue();
        let away = (self.position - npc_position).normalize_or_zero();
        let direction = if away.length_squared() > 0.01 {
            away
        } else {
            vec2(-1.0, 0.0)
        };
        let destination = npc_position + direction * (self.interaction_distance * 0.72);
        if can_occupy_with_radius(colony, destination, self.player_radius) {
            self.approached_npc = Some(npc_id.to_owned());
            self.destination = Some(destination);
        }
    }

    pub fn interact(&mut self, campaign: &CampaignState, data: &GameData) {
        if let Some(character_id) =
            nearest_npc_with_distance(campaign, data, self.position, self.interaction_distance)
        {
            self.destination = None;
            self.approached_npc = None;
            self.talking_to = Some(character_id);
        }
    }

    pub fn update_approach(&mut self, campaign: &CampaignState, data: &GameData) {
        if self.destination.is_some() {
            return;
        }
        let ready = self.approached_npc.as_deref().is_some_and(|id| {
            npc(campaign, data, id).is_some()
                && npc_position(campaign, id).is_some_and(|position| {
                    self.position.distance(position) <= self.interaction_distance
                })
        });
        if ready {
            self.interact(campaign, data);
        }
    }

    pub fn set_build_mode(&mut self, enabled: bool) {
        self.build_mode = enabled;
        if enabled {
            self.destination = None;
            self.close_dialogue();
        }
    }

    pub fn build_mode(&self) -> bool {
        self.build_mode
    }

    pub fn close_dialogue(&mut self) {
        self.talking_to = None;
    }

    pub fn draw_depth(&self, depth: i32, context: ColonyDepthContext<'_>) -> Option<String> {
        let ColonyDepthContext {
            campaign,
            data,
            assets,
            visuals,
            view,
            mouse,
        } = context;
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
            let highlighted =
                campaign.first_hour.colony_guidance_target() == Some(character.id.as_str());
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
            if clicked.is_none() && hovered && is_mouse_button_released(MouseButton::Left) {
                clicked = Some(character.id.clone());
            }
        }
        if let Some(definition) = campaign.available_outsider(data) {
            let guest_index = NPC_STATIONS.len() - 1;
            let position =
                npc_grid_position(campaign, &definition.id).unwrap_or(NPC_STATIONS[guest_index]);
            if position[0] + position[1] == depth {
                let guest = CharacterRecord::from_def(definition);
                let center = view.plot_center(position) + vec2(0.0, 7.0 * view.zoom);
                let hit = Rect::new(center.x - 22.0, center.y - 35.0, 44.0, 48.0);
                let hovered = hit.contains(mouse);
                draw_character(CharacterDrawContext {
                    character: &guest,
                    center,
                    facing_right: true,
                    moving: false,
                    interactable: true,
                    hovered,
                    highlighted: false,
                    assets,
                    visuals,
                    zoom: view.zoom,
                });
                if clicked.is_none() && hovered && is_mouse_button_released(MouseButton::Left) {
                    clicked = Some(guest.id);
                }
            }
        }
        if player_draw_depth(self.position) == depth {
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

    pub fn draw_dialogue(
        &self,
        campaign: &CampaignState,
        data: &GameData,
        mouse: Vec2,
        actions: &mut Vec<UiAction>,
    ) -> bool {
        let Some(npc) = self
            .talking_to
            .as_deref()
            .and_then(|id| npc(campaign, data, id))
        else {
            return false;
        };
        let character = &npc.character;
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
            npc_status(character, npc.guest),
            84.0,
            448.0,
            12.0,
            Color::new(0.45, 0.72, 0.66, 1.0),
        );
        let (title, text, heard) = dialogue_copy(campaign, character);
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
        if let Some(action) = npc_action(character, npc.guest) {
            let action_label = npc_action_button_label(campaign, data, character, npc.guest);
            if button(
                Rect::new(84.0, 532.0, 176.0, 30.0),
                &action_label,
                npc_action_enabled(campaign, data, &action, npc.guest),
                mouse,
            ) {
                actions.push(action);
                actions.push(UiAction::AcknowledgeColonist(character.id.clone()));
            }
        }
        if button(dialogue_continue_button_bounds(), "CONTINUE", true, mouse) {
            actions.push(UiAction::AcknowledgeColonist(character.id.clone()));
        }
        true
    }

    pub fn is_talking(&self) -> bool {
        self.talking_to.is_some()
    }

    pub fn talking_to(&self) -> Option<&str> {
        self.talking_to.as_deref()
    }

    pub fn can_talk(&self, campaign: &CampaignState, data: &GameData) -> bool {
        nearest_npc_with_distance(campaign, data, self.position, self.interaction_distance)
            .is_some()
    }

    pub fn is_moving(&self) -> bool {
        self.destination.is_some()
            || (self.keyboard_direction + self.touch_direction).length_squared() > 0.01
    }

    pub fn set_facing(&mut self, direction: Vec2) {
        self.facing = if direction.x + direction.y >= 0.0 {
            UnitFacing::SouthEast
        } else {
            UnitFacing::NorthWest
        };
    }

    pub fn move_with_collision(&mut self, movement: Vec2, colony: &ColonyState) {
        let horizontal = self.position + vec2(movement.x, 0.0);
        if can_occupy_with_radius(colony, horizontal, self.player_radius) {
            self.position = horizontal;
        }
        let vertical = self.position + vec2(0.0, movement.y);
        if can_occupy_with_radius(colony, vertical, self.player_radius) {
            self.position = vertical;
        }
    }
}
