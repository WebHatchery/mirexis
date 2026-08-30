//! Three-quarter battlefield rendering, projected overlays, depth, and hit handling.

use crate::data::{ObjectiveKind, Team};
use crate::grid_ui::{
    CameraInsets, GridView, WorldCamera, CANOPY_ART_PIVOT, CANOPY_ART_SCALE, STRUCTURE_ART_PIVOT,
    STRUCTURE_ART_SCALE, TERRAIN_ART_PIVOT, TERRAIN_ART_SCALE,
};
use crate::state::ObjectiveState;
use crate::ui::{draw_ui_text_ex, TargetingView, UiAction, UiContext};
use macroquad::prelude::*;
use macroquad_toolkit::grid::TilePos;
use macroquad_toolkit::prelude::{dark, draw_surface_with_title, SurfaceStyle, TextStyle};

mod targeting_card;
#[cfg(test)]
mod tests;

fn tactical_panel() -> Rect {
    Rect::new(10.0, 74.0, 900.0, 608.0)
}

fn tactical_viewport(panel: Rect) -> Rect {
    Rect::new(
        panel.x + 8.0,
        panel.y + 32.0,
        panel.w - 16.0,
        panel.h - 40.0,
    )
}

fn camera_controls_origin(panel: Rect) -> Vec2 {
    vec2(
        panel.right() - crate::camera_controls::STRIP_WIDTH - 6.0,
        panel.y,
    )
}

fn camera_art_insets(zoom: f32) -> CameraInsets {
    CameraInsets {
        left: 4.0 * zoom,
        top: 42.0 * zoom,
        right: 4.0 * zoom,
        bottom: 18.0 * zoom,
    }
}

pub(crate) fn draw(
    ctx: &UiContext<'_>,
    camera: &mut WorldCamera,
    mouse: Vec2,
    input_enabled: bool,
    actions: &mut Vec<UiAction>,
) -> bool {
    let panel = tactical_panel();
    draw_surface_with_title(
        panel,
        Some("OUTER SETTLEMENT // TACTICAL CAMERA"),
        &SurfaceStyle::new(Color::new(0.018, 0.034, 0.040, 0.99))
            .with_border(1.0, Color::new(0.20, 0.50, 0.48, 0.9))
            .with_inner_border(6.0, 1.0, Color::new(0.16, 0.28, 0.28, 0.7))
            .with_header(28.0, Color::new(0.06, 0.10, 0.11, 1.0)),
        TextStyle::new(13.0, dark::TEXT),
    );
    let grid_rect = tactical_viewport(panel);
    let camera_control_clicked = crate::camera_controls::draw(
        camera,
        grid_rect,
        mouse,
        camera_controls_origin(panel),
        input_enabled && !camera.primary_gesture_active(),
    );
    camera.reveal_changed_tactical_selection(ctx.session.tactical.selected_tile, grid_rect);
    let camera_dragged = input_enabled && camera.update(grid_rect, mouse);
    if camera_control_clicked {
        camera.guard_next_primary_release();
    }
    let suppress_map_click = !input_enabled || camera_control_clicked || camera_dragged;
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
    let hovered = grid_rect
        .contains(mouse)
        .then(|| view.tile_at(mouse))
        .flatten();
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
    targeting_card::draw_skill_target_tiles(ctx, view);
    for position in &tiles {
        if view.is_visible(*position, grid_rect, 90.0) {
            draw_tile_contents(ctx, view, *position);
        }
    }

    crate::cover_ui::draw_edges(view, &ctx.session.tactical.cover_edges);
    crate::reinforcement_ui::draw(ctx.session, view, ctx.assets, ctx.visuals);
    crate::enemy_intent_ui::draw_forecast(ctx.session, ctx.data.config.max_action_points, view);
    let preview_tile = hovered.unwrap_or(ctx.session.tactical.selected_tile);
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
            Some(TargetingView::ClassAction { unit_id }) => {
                ctx.session.can_target_class_action(unit_id, &unit.id)
            }
            Some(TargetingView::Skill { unit_id, skill_id })
                if crate::skills::target_kind(skill_id)
                    != Some(crate::data::TechniqueTarget::Tile) =>
            {
                crate::skills::can_target_unit(ctx.session, unit_id, skill_id, &unit.id)
            }
            None => false,
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
    crate::ui::set_ui_clip(ctx.ui, None);
    crate::action_preview_ui::draw(ctx.session, preview_tile, panel, ctx.assets, ctx.visuals);
    if ctx.targeting.is_some() {
        targeting_card::draw(ctx, preview_tile, panel, mouse, actions);
    }
    draw_ui_text_ex(
        &format!(
            "DRAG MAP // PAN < ^ v > // WHEEL OR -/+ ZOOM // {:>3}%",
            (camera.zoom * 100.0) as i32
        ),
        panel.x + 22.0,
        panel.bottom() - 7.0,
        TextStyle::new(10.0, Color::new(0.46, 0.68, 0.66, 1.0)).params(),
    );
    handle_click(ctx, view, grid_rect, mouse, suppress_map_click, actions);
    camera_dragged
}

fn draw_backdrop(rect: Rect) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.012, 0.026, 0.030, 1.0),
    );
    draw_circle(
        rect.x + rect.w * 0.72,
        rect.y + 60.0,
        160.0,
        Color::new(0.08, 0.24, 0.22, 0.10),
    );
    for index in 0..9 {
        let y = rect.y + 26.0 + index as f32 * 48.0;
        draw_line(
            rect.x + 18.0,
            y,
            rect.right() - 18.0,
            y + 22.0,
            1.0,
            Color::new(0.10, 0.24, 0.23, 0.20),
        );
    }
}

fn draw_terrain_tile(
    ctx: &UiContext<'_>,
    view: GridView,
    position: TilePos,
    hovered: Option<TilePos>,
) {
    let blocked = ctx.session.tactical.blocked.contains(&position);
    let movable = !blocked && ctx.session.can_move_selected_to(position);
    let terrain_cost = ctx
        .session
        .tactical
        .terrain_costs
        .iter()
        .find(|(tile, _)| *tile == position)
        .map_or(1, |(_, cost)| *cost);
    let top = view.diamond(position);
    let elevation = view.elevation(position);
    draw_exposed_cliffs(view, position, top);
    let parity = ((position.x * 17 + position.y * 31).unsigned_abs() % 3) as f32;
    let top_color = if blocked {
        Color::new(0.20 + parity * 0.012, 0.16, 0.12, 1.0)
    } else if elevation >= 2 {
        Color::new(0.15 + parity * 0.008, 0.23, 0.21, 1.0)
    } else if elevation == 1 {
        Color::new(0.11 + parity * 0.008, 0.18, 0.17, 1.0)
    } else if elevation < 0 {
        Color::new(0.045 + parity * 0.005, 0.095, 0.11, 1.0)
    } else {
        Color::new(0.080 + parity * 0.008, 0.125 + parity * 0.006, 0.125, 1.0)
    };
    draw_diamond_fill(top, top_color);

    if let Some(atlas_index) = terrain_art_index(ctx, position, blocked, terrain_cost) {
        let (scale, pivot) = if blocked {
            (STRUCTURE_ART_SCALE, STRUCTURE_ART_PIVOT)
        } else {
            (TERRAIN_ART_SCALE, TERRAIN_ART_PIVOT)
        };
        let art_rect = view.art_bounds(position, scale, pivot);
        ctx.visuals.draw_atlas_cell(
            ctx.assets,
            &ctx.visuals.terrain,
            atlas_index,
            art_rect,
            Color::new(0.92, 0.96, 0.94, if blocked { 0.90 } else { 0.78 }),
        );
    }

    if terrain_cost > 1 && !blocked {
        draw_projected_hatch(top, Color::new(0.90, 0.58, 0.20, 0.50));
    }
    if movable {
        draw_diamond_fill(top, Color::new(0.06, 0.72, 0.56, 0.30));
        draw_poly(
            (top[0].x + top[2].x) * 0.5,
            (top[0].y + top[2].y) * 0.5,
            4,
            3.0,
            45.0,
            Color::new(0.70, 1.0, 0.90, 0.86),
        );
    }
    if position == ctx.session.tactical.selected_tile {
        draw_diamond_outline(top, Color::new(0.94, 1.0, 0.98, 1.0), 3.0);
        draw_diamond_outline(
            inset_diamond(top, 0.82),
            Color::new(0.20, 0.92, 0.78, 1.0),
            2.0,
        );
    } else if hovered == Some(position) {
        draw_diamond_outline(top, Color::new(0.58, 0.98, 0.90, 0.92), 2.0);
    } else {
        draw_diamond_outline(top, Color::new(0.20, 0.38, 0.36, 0.72), 1.0);
    }
}

fn draw_exposed_cliffs(view: GridView, position: TilePos, top: [Vec2; 4]) {
    draw_cliff_face(
        view,
        position,
        TilePos::new(position.x, position.y + 1),
        top[3],
        top[2],
        Color::new(0.07, 0.13, 0.13, 1.0),
    );
    draw_cliff_face(
        view,
        position,
        TilePos::new(position.x + 1, position.y),
        top[2],
        top[1],
        Color::new(0.035, 0.075, 0.085, 1.0),
    );
}

fn draw_cliff_face(
    view: GridView,
    position: TilePos,
    neighbor: TilePos,
    a: Vec2,
    b: Vec2,
    color: Color,
) {
    let drop = view.cliff_drop(position, neighbor);
    if drop == 0 {
        return;
    }
    let offset = vec2(0.0, f32::from(drop) * view.elevation_step());
    draw_side_quad(a, b, b + offset, a + offset, color);
    for band in 1..=drop {
        let t = f32::from(band) / f32::from(drop);
        let line_offset = offset * t;
        draw_line(
            a.x + line_offset.x,
            a.y + line_offset.y,
            b.x + line_offset.x,
            b.y + line_offset.y,
            1.0,
            Color::new(0.24, 0.47, 0.43, 0.52),
        );
    }
}

fn terrain_art_index(
    ctx: &UiContext<'_>,
    position: TilePos,
    blocked: bool,
    cost: u8,
) -> Option<usize> {
    if blocked {
        return Some(3);
    }
    if cost > 1 {
        return Some(1);
    }
    let pattern =
        (position.x as u32).wrapping_mul(73_856_093) ^ (position.y as u32).wrapping_mul(19_349_663);
    if pattern.is_multiple_of(31) {
        Some(
            ctx.visuals
                .faction_terrain_cell(&ctx.mission.hostile_faction),
        )
    } else if pattern.is_multiple_of(13) {
        Some(0)
    } else {
        Some(0)
    }
}

fn draw_tile_contents(ctx: &UiContext<'_>, view: GridView, position: TilePos) {
    let rect = view.tile_rect(position);
    let blocked = ctx.session.tactical.blocked.contains(&position);
    let occupied = ctx
        .session
        .tactical
        .units
        .iter()
        .any(|unit| unit.position == position);
    let cluttered = ctx
        .session
        .tactical
        .hazards
        .iter()
        .any(|hazard| hazard.position == position)
        || ctx
            .session
            .tactical
            .destructible_cover
            .iter()
            .any(|cover| cover.position == position)
        || is_objective(ctx, position);
    crate::world_art::draw_tactical_dressing(
        ctx.assets,
        ctx.visuals,
        view,
        position,
        blocked,
        occupied,
        cluttered,
        &ctx.mission.hostile_faction,
    );
    if let Some(hazard) = ctx
        .session
        .tactical
        .hazards
        .iter()
        .find(|hazard| hazard.position == position)
    {
        crate::hazard_ui::draw_tile(rect, hazard.kind);
    }
    if is_objective(ctx, position) {
        draw_objective(
            ctx,
            rect,
            ctx.mission.objective_kind == ObjectiveKind::DefendAsset,
        );
    }
    if let Some(cover) = ctx
        .session
        .tactical
        .destructible_cover
        .iter()
        .find(|cover| cover.position == position)
    {
        crate::cover_ui::draw_cover(
            ctx.assets,
            ctx.visuals,
            view,
            cover,
            ctx.session.can_attack_selected_cover(position),
        );
    }
}

fn draw_foreground(ctx: &UiContext<'_>, view: GridView, hovered: Option<TilePos>) {
    let canopy_tiles = [TilePos::new(12, 19), TilePos::new(13, 19)];
    for tile in canopy_tiles {
        if tile.x >= ctx.session.tactical.fog.width as i32
            || tile.y >= ctx.session.tactical.fog.height as i32
        {
            continue;
        }
        let focused = hovered == Some(tile)
            || ctx.session.tactical.selected_tile == tile
            || ctx
                .session
                .tactical
                .units
                .iter()
                .any(|unit| unit.position == tile);
        ctx.visuals.draw_atlas_cell(
            ctx.assets,
            &ctx.visuals.terrain,
            7,
            view.art_bounds(tile, CANOPY_ART_SCALE, CANOPY_ART_PIVOT),
            Color::new(1.0, 1.0, 1.0, if focused { 0.18 } else { 0.70 }),
        );
    }
}

fn draw_side_quad(a: Vec2, b: Vec2, c: Vec2, d: Vec2, color: Color) {
    draw_triangle(a, b, c, color);
    draw_triangle(a, c, d, color);
}

fn draw_diamond_fill(points: [Vec2; 4], color: Color) {
    draw_triangle(points[0], points[1], points[2], color);
    draw_triangle(points[0], points[2], points[3], color);
}

fn draw_diamond_outline(points: [Vec2; 4], color: Color, width: f32) {
    for index in 0..4 {
        let next = (index + 1) % 4;
        draw_line(
            points[index].x,
            points[index].y,
            points[next].x,
            points[next].y,
            width,
            color,
        );
    }
}

fn inset_diamond(points: [Vec2; 4], factor: f32) -> [Vec2; 4] {
    let center = (points[0] + points[2]) * 0.5;
    points.map(|point| center + (point - center) * factor)
}

fn draw_projected_hatch(points: [Vec2; 4], color: Color) {
    for step in 1..5 {
        let t = step as f32 / 6.0;
        let left = points[3].lerp(points[0], t);
        let right = points[2].lerp(points[1], t);
        draw_line(left.x, left.y, right.x, right.y, 1.5, color);
    }
}

fn is_objective(ctx: &UiContext<'_>, position: TilePos) -> bool {
    matches!(
        ctx.mission.objective_kind,
        ObjectiveKind::SecureAndClear
            | ObjectiveKind::Extraction
            | ObjectiveKind::SignalTrace
            | ObjectiveKind::DefendAsset
    ) && position == ctx.session.tactical.objective_tile
        && ctx.session.tactical.objective_state == ObjectiveState::Active
}

fn draw_objective(ctx: &UiContext<'_>, rect: Rect, defended_asset: bool) {
    let center = vec2(rect.x + rect.w * 0.5, rect.y + rect.h * 0.48);
    ctx.visuals.draw_atlas_cell(
        ctx.assets,
        ctx.visuals.concept_atlas("objectives"),
        objective_art_cell(ctx.mission.objective_kind, defended_asset),
        Rect::new(center.x - 30.0, center.y - 42.0, 60.0, 60.0),
        Color::new(1.0, 1.0, 1.0, 0.88),
    );
    for radius in [rect.w * 0.18, rect.w * 0.26] {
        draw_ellipse_ring(
            center,
            radius,
            radius * 0.46,
            Color::new(1.0, 0.76, 0.20, 1.0),
            2.0,
        );
    }
    draw_poly(
        center.x,
        center.y - 10.0,
        4,
        if defended_asset { 8.0 } else { 5.0 },
        45.0,
        Color::new(1.0, 0.76, 0.20, 0.94),
    );
    draw_line(
        center.x,
        center.y - 10.0,
        center.x,
        center.y - 30.0,
        2.0,
        Color::new(1.0, 0.82, 0.34, 0.78),
    );
}

fn objective_art_cell(kind: ObjectiveKind, defended_asset: bool) -> usize {
    if defended_asset {
        return 3;
    }
    match kind {
        ObjectiveKind::SecureAndClear => 2,
        ObjectiveKind::Extraction => 0,
        ObjectiveKind::SignalTrace => 6,
        ObjectiveKind::DefendAsset => 3,
        ObjectiveKind::Holdout => 1,
        ObjectiveKind::EliminateAll => 10,
    }
}

fn draw_ellipse_ring(center: Vec2, rx: f32, ry: f32, color: Color, width: f32) {
    let segments = 24;
    for index in 0..segments {
        let a = index as f32 / segments as f32 * std::f32::consts::TAU;
        let b = (index + 1) as f32 / segments as f32 * std::f32::consts::TAU;
        draw_line(
            center.x + a.cos() * rx,
            center.y + a.sin() * ry,
            center.x + b.cos() * rx,
            center.y + b.sin() * ry,
            width,
            color,
        );
    }
}

fn handle_click(
    ctx: &UiContext<'_>,
    view: GridView,
    viewport: Rect,
    mouse: Vec2,
    suppress_click: bool,
    actions: &mut Vec<UiAction>,
) {
    if suppress_click || !is_mouse_button_released(MouseButton::Left) {
        return;
    }
    if !viewport.contains(mouse) {
        return;
    }
    let Some(tile) = view.tile_at(mouse) else {
        return;
    };
    if let Some(targeting) = ctx.targeting {
        let target = ctx
            .session
            .tactical
            .units
            .iter()
            .find(|unit| unit.position == tile);
        let action = match targeting {
            TargetingView::Equipment {
                unit_id,
                equipment_id,
            } => target.and_then(|target| {
                ctx.session
                    .can_use_equipment(unit_id, equipment_id, &target.id)
                    .then(|| UiAction::UseEquipmentOn(target.id.clone()))
            }),
            TargetingView::ClassAction { unit_id } => target.and_then(|target| {
                ctx.session
                    .can_target_class_action(unit_id, &target.id)
                    .then(|| UiAction::UseClassActionOn(target.id.clone()))
            }),
            TargetingView::Skill { unit_id, skill_id }
                if crate::skills::target_kind(skill_id)
                    == Some(crate::data::TechniqueTarget::Tile) =>
            {
                crate::skills::can_target_tile(ctx.session, unit_id, skill_id, tile)
                    .then_some(UiAction::UseSkillOnTile(tile))
            }
            TargetingView::Skill { unit_id, skill_id } => target.and_then(|target| {
                crate::skills::can_target_unit(ctx.session, unit_id, skill_id, &target.id)
                    .then(|| UiAction::UseSkillOn(target.id.clone()))
            }),
        };
        actions.push(action.unwrap_or(UiAction::CancelTargeting));
        return;
    }
    let hostile = ctx
        .session
        .tactical
        .units
        .iter()
        .find(|unit| unit.position == tile && unit.team == Team::Hostile);
    if let Some(hostile) = hostile.filter(|unit| ctx.session.can_attack_selected(&unit.id)) {
        actions.push(UiAction::AttackSelected(hostile.id.clone()));
    } else if ctx.session.can_attack_selected_cover(tile) {
        actions.push(UiAction::AttackCover(tile));
    } else if ctx.session.can_move_selected_to(tile) {
        actions.push(UiAction::MoveSelected(tile));
    } else {
        actions.push(UiAction::SelectTile(tile));
    }
}
