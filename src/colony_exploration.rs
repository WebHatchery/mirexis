//! Touch-first colony walking, inhabitants, and contextual conversations.

use crate::campaign::{Availability, CampaignState, CharacterRecord};
use crate::colony::{ColonyState, COLONY_HEIGHT, COLONY_WIDTH};
use crate::colony_map_ui::view::ColonyView;
use crate::tactical::{UnitAnimationState, UnitFacing};
use crate::ui::UiAction;
use crate::ui_widgets::button;
use crate::visual_assets::VisualCatalog;
use macroquad::prelude::*;
use macroquad_toolkit::assets::AssetManager;
use std::collections::{HashMap, VecDeque};

const PLAYER_START: [i32; 2] = [9, 10];
const WALK_SECONDS_PER_PLOT: f32 = 0.18;
const NPC_STATIONS: [[i32; 2]; 5] = [[8, 10], [7, 9], [10, 8], [13, 9], [11, 12]];

#[derive(Debug, Clone)]
pub(crate) struct ColonyExplorer {
    position: [i32; 2],
    visual_position: Vec2,
    route: VecDeque<[i32; 2]>,
    step_progress: f32,
    step_from: [i32; 2],
    facing: UnitFacing,
    approached_npc: Option<String>,
    talking_to: Option<String>,
    build_mode: bool,
}

impl Default for ColonyExplorer {
    fn default() -> Self {
        Self {
            position: PLAYER_START,
            visual_position: vec2(PLAYER_START[0] as f32, PLAYER_START[1] as f32),
            route: VecDeque::new(),
            step_progress: 0.0,
            step_from: PLAYER_START,
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

    pub(crate) fn update(&mut self, dt: f32, colony: &ColonyState) {
        if self.route.is_empty() {
            self.step_progress = 0.0;
            self.visual_position = grid_vec(self.position);
            return;
        }
        let next = self.route[0];
        if blocked(colony, next) {
            self.route.clear();
            return;
        }
        self.step_progress = (self.step_progress + dt / WALK_SECONDS_PER_PLOT).min(1.0);
        self.visual_position = grid_vec(self.step_from).lerp(grid_vec(next), self.step_progress);
        if self.step_progress >= 1.0 {
            self.position = next;
            self.step_from = next;
            self.route.pop_front();
            self.step_progress = 0.0;
        }
    }

    pub(crate) fn request_step(&mut self, delta: [i32; 2], colony: &ColonyState) {
        self.close_dialogue();
        self.approached_npc = None;
        let destination = [self.position[0] + delta[0], self.position[1] + delta[1]];
        if in_bounds(destination) && !blocked(colony, destination) {
            self.begin_route(VecDeque::from([destination]));
            self.set_facing(delta);
        }
    }

    pub(crate) fn request_walk(&mut self, destination: [i32; 2], colony: &ColonyState) {
        self.close_dialogue();
        self.approached_npc = None;
        if let Some(route) = pathfind(self.position, destination, colony, &[]) {
            self.begin_route(route);
        }
    }

    pub(crate) fn request_approach(
        &mut self,
        npc_id: &str,
        npc_position: [i32; 2],
        colony: &ColonyState,
        occupied_npc_positions: &[[i32; 2]],
    ) {
        self.close_dialogue();
        let destination = neighbours(npc_position)
            .into_iter()
            .filter(|position| in_bounds(*position) && !blocked(colony, *position))
            .filter_map(|position| {
                pathfind(self.position, position, colony, occupied_npc_positions)
                    .map(|route| (route.len(), position, route))
            })
            .min_by_key(|(length, position, _)| (*length, position[1], position[0]));
        if let Some((_, _, route)) = destination {
            self.approached_npc = Some(npc_id.to_owned());
            self.begin_route(route);
        }
    }

    pub(crate) fn interact(&mut self, campaign: &CampaignState) {
        if let Some(character) = nearest_npc(campaign, self.position) {
            self.route.clear();
            self.approached_npc = None;
            self.talking_to = Some(character.id.clone());
        }
    }

    pub(crate) fn update_approach(&mut self, campaign: &CampaignState) {
        if !self.route.is_empty() {
            return;
        }
        let ready = self.approached_npc.as_deref().is_some_and(|id| {
            npc(campaign, id).is_some_and(|character| {
                npc_position(campaign, &character.id)
                    .is_some_and(|position| adjacent(self.position, position))
            })
        });
        if ready {
            self.interact(campaign);
        }
    }

    pub(crate) fn set_build_mode(&mut self, enabled: bool) {
        self.build_mode = enabled;
        if enabled {
            self.route.clear();
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
        for (index, character) in campaign.roster.iter().enumerate().skip(1).take(4) {
            let position = NPC_STATIONS[index];
            if position[0] + position[1] != depth {
                continue;
            }
            let center = view.plot_center(position) + vec2(0.0, 7.0 * view.zoom);
            let hit = Rect::new(center.x - 22.0, center.y - 35.0, 44.0, 48.0);
            let hovered = hit.contains(mouse);
            draw_character(
                character,
                center,
                index % 2 == 0,
                false,
                true,
                hovered,
                assets,
                visuals,
                view.zoom,
            );
            if hovered && is_mouse_button_released(MouseButton::Left) {
                clicked = Some(character.id.clone());
            }
        }
        if self.visual_position.x.floor() as i32 + self.visual_position.y.floor() as i32 == depth {
            if let Some(player) = campaign.roster.first() {
                let center = view.world_center(self.visual_position) + vec2(0.0, 7.0 * view.zoom);
                draw_character(
                    player,
                    center,
                    matches!(self.facing, UnitFacing::SouthEast | UnitFacing::NorthEast),
                    !self.route.is_empty(),
                    false,
                    false,
                    assets,
                    visuals,
                    view.zoom,
                );
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
        let panel = Rect::new(66.0, 410.0, 722.0, 154.0);
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
            &character.name.to_uppercase(),
            84.0,
            438.0,
            20.0,
            Color::new(0.76, 1.0, 0.88, 1.0),
        );
        draw_text(
            &npc_status(character),
            84.0,
            458.0,
            12.0,
            Color::new(0.45, 0.72, 0.66, 1.0),
        );
        draw_wrapped(&character.biography, 84.0, 481.0, 660.0);
        if button(
            Rect::new(84.0, 520.0, 176.0, 30.0),
            npc_action_label(character),
            true,
            mouse,
        ) {
            actions.push(npc_action(character));
            self.close_dialogue();
        }
        if button(
            Rect::new(632.0, 520.0, 136.0, 30.0),
            "CONTINUE",
            true,
            mouse,
        ) {
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

    fn begin_route(&mut self, route: VecDeque<[i32; 2]>) {
        self.route = route;
        self.step_from = self.position;
        self.step_progress = 0.0;
        if let Some(next) = self.route.front() {
            self.set_facing([next[0] - self.position[0], next[1] - self.position[1]]);
        }
    }

    fn set_facing(&mut self, delta: [i32; 2]) {
        self.facing = match delta {
            [1, 0] | [0, 1] => UnitFacing::SouthEast,
            [-1, 0] | [0, -1] => UnitFacing::NorthWest,
            _ => self.facing,
        };
    }
}

fn draw_character(
    character: &CharacterRecord,
    center: Vec2,
    facing_right: bool,
    moving: bool,
    interactable: bool,
    hovered: bool,
    assets: &AssetManager,
    visuals: &VisualCatalog,
    zoom: f32,
) {
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
    if hovered {
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

pub(crate) fn npc_position(campaign: &CampaignState, id: &str) -> Option<[i32; 2]> {
    campaign
        .roster
        .iter()
        .position(|character| character.id == id)
        .and_then(|index| (index > 0 && index < NPC_STATIONS.len()).then_some(NPC_STATIONS[index]))
}

pub(crate) fn npc_positions(campaign: &CampaignState) -> Vec<[i32; 2]> {
    campaign
        .roster
        .iter()
        .skip(1)
        .filter_map(|character| npc_position(campaign, &character.id))
        .collect()
}

fn npc<'a>(campaign: &'a CampaignState, id: &str) -> Option<&'a CharacterRecord> {
    campaign
        .roster
        .iter()
        .skip(1)
        .find(|character| character.id == id)
}

fn nearest_npc(campaign: &CampaignState, position: [i32; 2]) -> Option<&CharacterRecord> {
    campaign.roster.iter().skip(1).find(|character| {
        npc_position(campaign, &character.id)
            .is_some_and(|npc_position| adjacent(position, npc_position))
    })
}

fn pathfind(
    start: [i32; 2],
    goal: [i32; 2],
    colony: &ColonyState,
    occupied_npcs: &[[i32; 2]],
) -> Option<VecDeque<[i32; 2]>> {
    if !in_bounds(goal) || blocked(colony, goal) || occupied_npcs.contains(&goal) {
        return None;
    }
    let mut frontier = VecDeque::from([start]);
    let mut came_from = HashMap::from([(start, start)]);
    while let Some(current) = frontier.pop_front() {
        if current == goal {
            break;
        }
        for next in neighbours(current) {
            if in_bounds(next)
                && !blocked(colony, next)
                && !occupied_npcs.contains(&next)
                && !came_from.contains_key(&next)
            {
                came_from.insert(next, current);
                frontier.push_back(next);
            }
        }
    }
    if !came_from.contains_key(&goal) {
        return None;
    }
    let mut reversed = vec![goal];
    let mut current = goal;
    while current != start {
        current = came_from[&current];
        if current != start {
            reversed.push(current);
        }
    }
    reversed.reverse();
    Some(reversed.into())
}

fn neighbours(position: [i32; 2]) -> [[i32; 2]; 4] {
    [
        [position[0] + 1, position[1]],
        [position[0] - 1, position[1]],
        [position[0], position[1] + 1],
        [position[0], position[1] - 1],
    ]
}

fn adjacent(a: [i32; 2], b: [i32; 2]) -> bool {
    (a[0] - b[0]).abs() + (a[1] - b[1]).abs() == 1
}

fn in_bounds(position: [i32; 2]) -> bool {
    (0..COLONY_WIDTH).contains(&position[0]) && (0..COLONY_HEIGHT).contains(&position[1])
}

fn blocked(colony: &ColonyState, position: [i32; 2]) -> bool {
    colony.building_at(position).is_some() || colony.project_at(position).is_some()
}

fn grid_vec(position: [i32; 2]) -> Vec2 {
    vec2(position[0] as f32, position[1] as f32)
}

#[cfg(test)]
mod tests;
