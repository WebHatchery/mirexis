//! Contextual battlefield and command-rail emphasis for the first operation lesson.

use crate::first_hour::{FirstHourProgress, TacticalLesson};
use crate::grid_ui::GridView;
use crate::state::GameSession;
use crate::ui::{draw_ui_text_ex, UiContext};
use macroquad::prelude::*;
use macroquad_toolkit::grid::TilePos;
use macroquad_toolkit::prelude::TextStyle;

const TACTICAL_PANEL: Rect = Rect::new(920.0, 74.0, 350.0, 608.0);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AbilityFocusSlot {
    Mutation,
    ClassAction,
    Equipment,
}

pub(crate) fn map_focus_tile(
    progress: &FirstHourProgress,
    session: &GameSession,
) -> Option<TilePos> {
    if !progress.guidance_enabled || !progress.is_tactical_stage() {
        return None;
    }
    match progress.lesson {
        TacticalLesson::Select => session
            .tactical
            .units
            .iter()
            .find(|unit| unit.team == crate::data::Team::Colony && !unit.incapacitated)
            .map(|unit| unit.position),
        TacticalLesson::MoveToCover => move_focus_tile(session),
        TacticalLesson::Attack => session
            .tactical
            .units
            .iter()
            .filter(|unit| unit.team == crate::data::Team::Hostile && !unit.incapacitated)
            .min_by_key(|unit| focus_distance(session, unit.position))
            .map(|unit| unit.position),
        TacticalLesson::Objective => Some(session.tactical.objective_tile),
        TacticalLesson::EnemyPhase | TacticalLesson::Ability => None,
        TacticalLesson::ApplyLearning => apply_learning_focus_tile(session),
    }
}

pub(crate) fn draw_map_focus(
    ctx: &UiContext<'_>,
    view: GridView,
    viewport: Rect,
    input_enabled: bool,
) {
    if !input_enabled || !guidance_is_active(ctx) {
        return;
    }
    let Some(tile) = map_focus_tile(ctx.first_hour, ctx.session) else {
        return;
    };
    if !view.is_visible(tile, viewport, 24.0) {
        return;
    }

    let color = focus_color();
    let points = view.diamond(tile);
    for index in 0..points.len() {
        let next = (index + 1) % points.len();
        draw_line(
            points[index].x,
            points[index].y,
            points[next].x,
            points[next].y,
            3.0,
            color,
        );
    }
    let rect = view.tile_rect(tile);
    let center = rect.center();
    let arrow_tip = vec2(center.x, rect.y - 10.0);
    draw_line(center.x, rect.y - 2.0, arrow_tip.x, arrow_tip.y, 2.0, color);
    draw_triangle(
        arrow_tip,
        vec2(arrow_tip.x - 5.0, arrow_tip.y + 8.0),
        vec2(arrow_tip.x + 5.0, arrow_tip.y + 8.0),
        color,
    );
    draw_ui_text_ex(
        map_label(ctx.first_hour.lesson),
        rect.x + 4.0,
        rect.y - 16.0,
        TextStyle::new(10.0, color).params(),
    );
}

pub(crate) fn draw_command_focus(ctx: &UiContext<'_>) {
    if !guidance_is_active(ctx) {
        return;
    }
    let x = TACTICAL_PANEL.x + 18.0;
    let rect = match ctx.first_hour.lesson {
        TacticalLesson::EnemyPhase => {
            let phase_width = (TACTICAL_PANEL.w - 44.0) * 0.46;
            Rect::new(
                x + phase_width + 8.0,
                TACTICAL_PANEL.bottom() - 104.0,
                TACTICAL_PANEL.w - 44.0 - phase_width,
                44.0,
            )
        }
        TacticalLesson::Ability => {
            let Some(slot) = ability_focus_slot(ctx.session) else {
                return;
            };
            ability_focus_rect(x, TACTICAL_PANEL.bottom() - 148.0, slot)
        }
        TacticalLesson::Attack | TacticalLesson::ApplyLearning => {
            let Some(rect) = attack_focus_rect(ctx.session) else {
                return;
            };
            rect
        }
        TacticalLesson::Objective => {
            let Some(rect) = objective_focus_rect(ctx.session) else {
                return;
            };
            rect
        }
        TacticalLesson::Select | TacticalLesson::MoveToCover => return,
    };
    let color = focus_color();
    draw_rectangle_lines(
        rect.x - 4.0,
        rect.y - 4.0,
        rect.w + 8.0,
        rect.h + 8.0,
        2.0,
        color,
    );
    draw_rectangle_lines(
        rect.x - 1.0,
        rect.y - 1.0,
        rect.w + 2.0,
        rect.h + 2.0,
        1.0,
        Color::new(color.r, color.g, color.b, 0.42),
    );
    draw_ui_text_ex(
        "NEXT",
        rect.x + 2.0,
        rect.y - 7.0,
        TextStyle::new(10.0, color).params(),
    );
}

fn attack_focus_rect(session: &GameSession) -> Option<Rect> {
    crate::action_preview_ui::attack_preview_is_valid(session, session.tactical.selected_tile).then(
        || {
            let panel = Rect::new(10.0, 74.0, 900.0, 608.0);
            let card = crate::action_preview_ui::attack_card_bounds(panel);
            crate::action_preview_ui::attack_button_bounds(card)
        },
    )
}

fn objective_focus_rect(session: &GameSession) -> Option<Rect> {
    session
        .can_interact_selected()
        .then_some(crate::objective_ui::action_button_bounds(TACTICAL_PANEL))
}

fn ability_focus_slot(session: &GameSession) -> Option<AbilityFocusSlot> {
    if session.can_activate_selected_mutation() {
        return Some(AbilityFocusSlot::Mutation);
    }
    let unit = session.selected_unit()?;
    let class_action_available = if crate::class_actions::requires_target(&unit.class_id) {
        crate::class_actions::has_valid_target(session, &unit.id)
    } else {
        session.can_activate_selected_class_action()
    };
    if class_action_available {
        return Some(AbilityFocusSlot::ClassAction);
    }
    crate::equipment_actions::available_action(session, &unit.id).and_then(|equipment_id| {
        crate::equipment_actions::has_valid_target(session, &unit.id, &equipment_id)
            .then_some(AbilityFocusSlot::Equipment)
    })
}

fn ability_focus_rect(x: f32, y: f32, slot: AbilityFocusSlot) -> Rect {
    let action_width = (TACTICAL_PANEL.w - 52.0) / 3.0;
    let offset = match slot {
        AbilityFocusSlot::Mutation => 0.0,
        AbilityFocusSlot::ClassAction => action_width + 8.0,
        AbilityFocusSlot::Equipment => (action_width + 8.0) * 2.0,
    };
    Rect::new(x + offset, y, action_width, 34.0)
}

fn guidance_is_active(ctx: &UiContext<'_>) -> bool {
    ctx.first_hour.guidance_enabled
        && ctx.first_hour.is_tactical_stage()
        && !ctx.first_hour.help_open
        && !ctx.show_help
        && !ctx.show_battle_log
        && !ctx.phase_replay.is_active()
}

fn move_focus_tile(session: &GameSession) -> Option<TilePos> {
    let selected = session.selected_unit()?;
    let cover_tiles = session
        .tactical
        .cover_edges
        .iter()
        .map(|edge| TilePos::new(edge.position[0], edge.position[1]))
        .collect::<Vec<_>>();
    let radius = i32::from(selected.effective_move_range());
    let mut candidates = Vec::new();
    for y in selected.position.y - radius..=selected.position.y + radius {
        for x in selected.position.x - radius..=selected.position.x + radius {
            let tile = TilePos::new(x, y);
            if tile != selected.position && session.can_move_selected_to(tile) {
                candidates.push(tile);
            }
        }
    }
    candidates.into_iter().min_by_key(|tile| {
        (
            nearest_cover_distance(*tile, &cover_tiles),
            focus_distance(session, *tile),
            tile.y,
            tile.x,
        )
    })
}

fn nearest_cover_distance(tile: TilePos, cover_tiles: &[TilePos]) -> i32 {
    cover_tiles
        .iter()
        .map(|cover| manhattan(tile, *cover))
        .min()
        .unwrap_or(i32::MAX)
}

fn focus_distance(session: &GameSession, tile: TilePos) -> i32 {
    session
        .selected_unit()
        .map_or(0, |selected| manhattan(selected.position, tile))
}

fn apply_learning_focus_tile(session: &GameSession) -> Option<TilePos> {
    let hostiles = session
        .tactical
        .units
        .iter()
        .filter(|unit| unit.team == crate::data::Team::Hostile && !unit.incapacitated);
    hostiles
        .clone()
        .filter(|unit| session.can_attack_selected(&unit.id))
        .min_by_key(|unit| focus_key(session, unit.position))
        .or_else(|| hostiles.min_by_key(|unit| focus_key(session, unit.position)))
        .map(|unit| unit.position)
}

fn focus_key(session: &GameSession, tile: TilePos) -> (i32, i32, i32) {
    (focus_distance(session, tile), tile.y, tile.x)
}

fn manhattan(from: TilePos, to: TilePos) -> i32 {
    (from.x - to.x).abs() + (from.y - to.y).abs()
}

fn map_label(lesson: TacticalLesson) -> &'static str {
    match lesson {
        TacticalLesson::Select => "NEXT // TAP COLONIST",
        TacticalLesson::MoveToCover => "NEXT // MOVE TO COVER",
        TacticalLesson::Attack => "NEXT // REVIEW FORECAST",
        TacticalLesson::Objective => "NEXT // SECURE OBJECTIVE",
        TacticalLesson::EnemyPhase | TacticalLesson::Ability => "",
        TacticalLesson::ApplyLearning => "NEXT // CLEAR HOSTILES",
    }
}

fn focus_color() -> Color {
    Color::new(1.0, 0.74, 0.18, 0.96)
}

#[cfg(test)]
mod tests;
