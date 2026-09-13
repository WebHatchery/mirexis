//! Three-quarter battlefield rendering, projected overlays, depth, and hit handling.
pub mod rendering;
pub use rendering::*;

use crate::data::{ObjectiveKind, Team};
use crate::first_hour::{FirstHourProgress, TacticalLesson};
use crate::grid_ui::{
    CameraInsets, GridView, WorldCamera, CANOPY_ART_PIVOT, CANOPY_ART_SCALE, TERRAIN_ART_PIVOT,
    TERRAIN_ART_SCALE,
};
use crate::state::{GameSession, ObjectiveState};
use crate::ui::{draw_ui_text_ex, TargetingView, UiAction, UiContext};
use macroquad::prelude::*;
use macroquad_toolkit::grid::TilePos;
use macroquad_toolkit::prelude::{draw_surface, SurfaceStyle, TextStyle};

pub mod targeting_card;

pub fn tactical_panel() -> Rect {
    crate::ui::tactical_world_rect()
}

pub fn tactical_viewport(panel: Rect) -> Rect {
    Rect::new(panel.x + 8.0, panel.y + 8.0, panel.w - 16.0, panel.h - 16.0)
}

pub fn camera_art_insets(zoom: f32) -> CameraInsets {
    CameraInsets {
        left: 4.0 * zoom,
        top: 42.0 * zoom,
        right: 4.0 * zoom,
        bottom: 18.0 * zoom,
    }
}

pub fn draw(
    ctx: &UiContext<'_>,
    camera: &mut WorldCamera,
    mouse: Vec2,
    input_enabled: bool,
    actions: &mut Vec<UiAction>,
) -> bool {
    let panel = tactical_panel();
    draw_surface(
        panel,
        &SurfaceStyle::new(Color::new(0.018, 0.034, 0.040, 0.99))
            .with_border(1.0, Color::new(0.20, 0.50, 0.48, 0.9))
            .with_inner_border(6.0, 1.0, Color::new(0.16, 0.28, 0.28, 0.7))
            .with_left_accent(4.0, Color::new(0.18, 0.54, 0.48, 0.9)),
    );
    let grid_rect = tactical_viewport(panel);
    let map_input_enabled = input_enabled
        && !(ctx.tactical_panel_open && crate::ui::tactical_command_panel_rect().contains(mouse));
    if map_input_enabled {
        camera.reveal_changed_tactical_selection(ctx.session.tactical.selected_tile, grid_rect);
    }
    let camera_dragged = if map_input_enabled {
        camera.update(grid_rect, mouse)
    } else {
        camera.clear_pointer_interaction();
        false
    };
    let suppress_map_click = !input_enabled || camera_dragged;
    camera.clamp_isometric_with_insets(
        ctx.session.tactical.fog.width,
        ctx.session.tactical.fog.height,
        crate::grid_ui::TACTICAL_HALF_WIDTH,
        crate::grid_ui::TACTICAL_HALF_HEIGHT,
        grid_rect,
        camera_art_insets(camera.zoom),
    );
    let view = GridView::with_camera(
        ctx.session.tactical.fog.width,
        ctx.session.tactical.fog.height,
        grid_rect,
        camera,
    );
    draw_backdrop(grid_rect);
    if !ctx.session.tactical.hazards.is_empty() {
        crate::hazard_ui::draw_legend(panel);
    }
    let ground_hovered = grid_rect
        .contains(mouse)
        .then(|| view.tile_at(mouse))
        .flatten();
    let hovered_unit = grid_rect.contains(mouse).then(|| {
        ctx.session
            .tactical
            .units
            .iter()
            .filter(|unit| crate::tactical_unit_ui::hover_hit(view, unit, mouse))
            .max_by_key(|unit| unit.position.x + unit.position.y)
    });
    let hovered_unit = hovered_unit.flatten();
    let hovered = hovered_unit.map(|unit| unit.position).or(ground_hovered);
    crate::ui::set_ui_clip(ctx.ui, Some(grid_rect));
    let mut tiles = ctx
        .session
        .tactical
        .fog
        .iter_with_pos()
        .map(|(position, _)| position)
        .collect::<Vec<_>>();
    tiles.sort_by_key(|position| position.x + position.y);

    for position in &tiles {
        if view.terrain_is_visible(*position, grid_rect, 30.0) {
            draw_terrain_tile(ctx, view, *position, hovered);
        }
    }
    draw_obscuring_fields(ctx, view);
    targeting_card::draw_class_action_target_tiles(ctx, view);
    targeting_card::draw_skill_target_tiles(ctx, view);
    for position in &tiles {
        if view.is_visible(*position, grid_rect, 90.0) {
            draw_tile_contents(ctx, view, *position);
        }
    }

    crate::cover_ui::draw_edges(view, &ctx.session.tactical.cover_edges);
    crate::reinforcement_ui::draw(ctx.session, view, ctx.assets, ctx.visuals);
    crate::enemy_intent_ui::draw_forecast(ctx.session, ctx.data.config.max_action_points, view);
    let preview_tile = if crate::action_preview_ui::attack_card_bounds(panel).contains(mouse)
        && crate::action_preview_ui::attack_preview_is_valid(
            ctx.session,
            ctx.session.tactical.selected_tile,
        ) {
        ctx.session.tactical.selected_tile
    } else {
        hovered.unwrap_or(ctx.session.tactical.selected_tile)
    };
    crate::action_preview_ui::draw_route(ctx.session, preview_tile, view);

    let mut units = ctx.session.tactical.units.iter().collect::<Vec<_>>();
    units.sort_by_key(|unit| unit.position.x + unit.position.y);
    for unit in units {
        let targetable = match ctx.targeting {
            Some(TargetingView::Equipment {
                unit_id,
                equipment_id,
            }) => ctx
                .session
                .can_use_equipment(unit_id, equipment_id, &unit.id),
            Some(TargetingView::ClassAction {
                unit_id,
                target_kind,
            }) if target_kind != crate::data::TechniqueTarget::Tile => {
                ctx.session.can_target_class_action(unit_id, &unit.id)
            }
            Some(TargetingView::Skill { unit_id, skill_id })
                if crate::skills::target_kind(skill_id)
                    != Some(crate::data::TechniqueTarget::Tile) =>
            {
                crate::skills::can_target_unit(ctx.session, unit_id, skill_id, &unit.id)
            }
            None => false,
            Some(TargetingView::ClassAction { .. }) => false,
            Some(TargetingView::Skill { .. }) => false,
        };
        crate::tactical_unit_ui::draw_unit(
            ctx.assets,
            ctx.visuals,
            view,
            unit,
            ctx.data.config.max_action_points,
            ctx.session.tactical.selected_unit.as_deref() == Some(&unit.id),
            targetable,
        );
    }
    draw_foreground(ctx, view, hovered);
    ctx.feedback
        .draw(ctx.session, view, ctx.assets, ctx.visuals);
    if input_enabled {
        if let Some(unit) = hovered_unit {
            let tooltip_viewport = if ctx.tactical_panel_open {
                Rect::new(
                    grid_rect.x,
                    grid_rect.y,
                    crate::ui::tactical_command_panel_rect().x - grid_rect.x - 8.0,
                    grid_rect.h,
                )
            } else {
                grid_rect
            };
            crate::tactical_unit_ui::draw_hover_card(unit, view, tooltip_viewport);
        }
    }
    crate::first_hour_tactical_ui::draw_map_focus(ctx, view, grid_rect, input_enabled);
    crate::ui::set_ui_clip(ctx.ui, None);
    let preview_input_consumed = crate::action_preview_ui::draw(
        ctx.session,
        preview_tile,
        panel,
        ctx.assets,
        ctx.visuals,
        crate::action_preview_ui::PreviewInteraction {
            mouse,
            actions,
            interactive: map_input_enabled && ctx.targeting.is_none(),
        },
    );
    if ctx.targeting.is_some() {
        targeting_card::draw(ctx, preview_tile, panel, mouse, actions);
    }
    draw_ui_text_ex(
        &format!(
            "TAP TILE TO MOVE OR INSPECT // DRAG TO PAN // WHEEL TO ZOOM // {:>3}%",
            (camera.zoom * 100.0) as i32
        ),
        panel.x + 22.0,
        panel.bottom() - 7.0,
        TextStyle::new(10.0, Color::new(0.46, 0.68, 0.66, 1.0)).params(),
    );
    handle_click(
        ctx,
        hovered,
        grid_rect,
        mouse,
        suppress_map_click || preview_input_consumed || !map_input_enabled,
        actions,
    );
    camera_dragged
}
