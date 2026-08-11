//! Three-quarter battlefield rendering, projected overlays, depth, and hit handling.

use crate::data::{ObjectiveKind, Team};
use crate::grid_ui::GridView;
use crate::state::ObjectiveState;
use crate::ui::{draw_ui_text_ex, TargetingView, UiAction, UiContext};
use macroquad::prelude::*;
use macroquad_toolkit::grid::TilePos;
use macroquad_toolkit::prelude::{
    dark, draw_surface, draw_surface_with_title, SurfaceStyle, TextStyle,
};

pub(crate) fn draw(ctx: &UiContext<'_>, mouse: Vec2, actions: &mut Vec<UiAction>) {
    let panel = Rect::new(18.0, 96.0, 820.0, 530.0);
    draw_surface_with_title(
        panel,
        Some("OUTER SETTLEMENT // THREE-QUARTER TACTICAL DIORAMA"),
        &SurfaceStyle::new(Color::new(0.018, 0.034, 0.040, 0.99))
            .with_border(1.0, Color::new(0.20, 0.50, 0.48, 0.9))
            .with_inner_border(6.0, 1.0, Color::new(0.16, 0.28, 0.28, 0.7))
            .with_header(42.0, Color::new(0.06, 0.10, 0.11, 1.0)),
        TextStyle::new(17.0, dark::TEXT),
    );
    let grid_rect = Rect::new(
        panel.x + 20.0,
        panel.y + 52.0,
        panel.w - 40.0,
        panel.h - 72.0,
    );
    let view = GridView::new(
        ctx.session.tactical.fog.width,
        ctx.session.tactical.fog.height,
        grid_rect,
    );
    draw_backdrop(grid_rect);
    if !ctx.session.tactical.hazards.is_empty() {
        crate::hazard_ui::draw_legend(panel);
    }
    let hovered = view.tile_at(mouse);
    let mut tiles = ctx
        .session
        .tactical
        .fog
        .iter_with_pos()
        .map(|(position, _)| position)
        .collect::<Vec<_>>();
    tiles.sort_by_key(|position| position.x + position.y);

    for position in &tiles {
        draw_terrain_tile(ctx, view, *position, hovered);
    }
    for position in &tiles {
        draw_tile_contents(ctx, view, *position);
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
            None => false,
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
    crate::action_preview_ui::draw(ctx.session, preview_tile, panel, ctx.assets, ctx.visuals);
    if ctx.targeting.is_some() {
        draw_targeting_card(ctx, preview_tile, panel);
    }
    draw_ui_text_ex(
        "CYAN SOLID  MOVE / ALLY    RED HATCH  THREAT    AMBER DOUBLE  OBJECTIVE    H0-H2  HEIGHT",
        panel.x + 22.0,
        panel.bottom() - 7.0,
        TextStyle::new(10.0, Color::new(0.46, 0.68, 0.66, 1.0)).params(),
    );
    handle_click(ctx, view, mouse, actions);
}

fn draw_targeting_card(ctx: &UiContext<'_>, tile: TilePos, panel: Rect) {
    let card = Rect::new(panel.x + 16.0, panel.bottom() - 94.0, panel.w - 32.0, 80.0);
    draw_surface(
        card,
        &SurfaceStyle::new(Color::new(0.026, 0.052, 0.058, 0.98))
            .with_border(2.0, Color::new(1.0, 0.64, 0.22, 0.96)),
    );
    let target = ctx
        .session
        .tactical
        .units
        .iter()
        .find(|unit| unit.position == tile);
    match ctx.targeting {
        Some(TargetingView::Equipment {
            unit_id,
            equipment_id,
        }) => {
            let item = ctx
                .data
                .equipment
                .iter()
                .find(|item| item.id == equipment_id);
            if let Some(index) = crate::visual_assets::equipment_index(equipment_id) {
                ctx.visuals.draw_atlas_cell(
                    ctx.assets,
                    &ctx.visuals.equipment,
                    index,
                    Rect::new(card.x + 8.0, card.y + 8.0, 64.0, 64.0),
                    WHITE,
                );
            }
            let user = ctx
                .session
                .unit(unit_id)
                .map_or("UNKNOWN", |unit| unit.name.as_str());
            let valid = target.is_some_and(|target| {
                ctx.session
                    .can_use_equipment(unit_id, equipment_id, &target.id)
            });
            draw_ui_text_ex(
                &format!(
                    "FIELD ITEM TARGETING // {}",
                    crate::equipment_actions::action_name(equipment_id)
                        .unwrap_or(item.map_or("FIELD ITEM", |item| item.name.as_str()))
                ),
                card.x + 82.0,
                card.y + 25.0,
                TextStyle::new(16.0, dark::WARNING).params(),
            );
            draw_ui_text_ex(
                &format!(
                    "{} > {} // {} // 1 AP",
                    user.to_uppercase(),
                    target.map_or("SELECT A HIGHLIGHTED TARGET", |unit| unit.name.as_str()),
                    if valid {
                        "VALID TARGET"
                    } else {
                        "INVALID TARGET"
                    }
                ),
                card.x + 82.0,
                card.y + 49.0,
                TextStyle::new(
                    13.0,
                    if valid {
                        dark::POSITIVE
                    } else {
                        dark::NEGATIVE
                    },
                )
                .params(),
            );
        }
        Some(TargetingView::ClassAction { unit_id }) => {
            let user = ctx.session.unit(unit_id);
            let valid = target
                .is_some_and(|target| ctx.session.can_target_class_action(unit_id, &target.id));
            if let Some(user) = user {
                ctx.visuals.draw_portrait(
                    ctx.assets,
                    &user.id,
                    &user.name,
                    Rect::new(card.x + 8.0, card.y + 8.0, 64.0, 64.0),
                    dark::ACCENT,
                );
            }
            draw_ui_text_ex(
                "CLASS ACTION TARGETING // CHOOSE HIGHLIGHTED UNIT",
                card.x + 82.0,
                card.y + 25.0,
                TextStyle::new(16.0, dark::WARNING).params(),
            );
            draw_ui_text_ex(
                &format!(
                    "{} // {} // B OR ESC CANCEL",
                    target.map_or("NO UNIT SELECTED", |unit| unit.name.as_str()),
                    if valid {
                        "VALID TARGET"
                    } else {
                        "INVALID TARGET"
                    }
                ),
                card.x + 82.0,
                card.y + 49.0,
                TextStyle::new(
                    13.0,
                    if valid {
                        dark::POSITIVE
                    } else {
                        dark::NEGATIVE
                    },
                )
                .params(),
            );
        }
        None => {}
    }
    if let Some(target) = target {
        let accent = if target.team == Team::Colony {
            dark::ACCENT
        } else {
            dark::NEGATIVE
        };
        ctx.visuals.draw_portrait(
            ctx.assets,
            &target.id,
            &target.name,
            Rect::new(card.right() - 72.0, card.y + 8.0, 64.0, 64.0),
            accent,
        );
    }
    draw_ui_text_ex(
        "CLICK / A CONFIRM    B / ESC CANCEL",
        card.right() - 318.0,
        card.bottom() - 9.0,
        TextStyle::new(10.0, dark::TEXT_DIM).params(),
    );
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
    let base = view.base_diamond(position);
    let elevation = view.elevation(position);
    if elevation > 0 {
        draw_side_quad(
            top[3],
            top[2],
            base[2],
            base[3],
            Color::new(0.055, 0.095, 0.10, 1.0),
        );
        draw_side_quad(
            top[2],
            top[1],
            base[1],
            base[2],
            Color::new(0.035, 0.065, 0.075, 1.0),
        );
        for band in 1..=elevation {
            let y = top[2].y + band as f32 * view.elevation_step();
            draw_line(
                top[3].x,
                y,
                top[2].x,
                y + view.tile_rect(position).h * 0.5,
                1.0,
                Color::new(0.22, 0.44, 0.42, 0.55),
            );
        }
    }
    let parity = ((position.x * 17 + position.y * 31).unsigned_abs() % 3) as f32;
    let top_color = if blocked {
        Color::new(0.20 + parity * 0.012, 0.16, 0.12, 1.0)
    } else {
        Color::new(0.080 + parity * 0.008, 0.125 + parity * 0.006, 0.125, 1.0)
    };
    draw_diamond_fill(top, top_color);

    let atlas_index = terrain_art_index(ctx, position, blocked, terrain_cost);
    let bounds = view.tile_rect(position);
    let art_rect = Rect::new(
        bounds.x,
        bounds.y - bounds.w * 0.62,
        bounds.w,
        bounds.w * 1.22,
    );
    ctx.visuals.draw_atlas_cell(
        ctx.assets,
        &ctx.visuals.terrain,
        atlas_index,
        art_rect,
        Color::new(0.92, 0.96, 0.94, if blocked { 0.90 } else { 0.78 }),
    );

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
    draw_text_ex(
        format!("H{elevation}"),
        top[3].x + 3.0,
        top[3].y + 10.0,
        TextStyle::new(9.5, Color::new(0.48, 0.72, 0.68, 0.88)).params(),
    );
}

fn terrain_art_index(ctx: &UiContext<'_>, position: TilePos, blocked: bool, cost: u8) -> usize {
    if blocked {
        return 3;
    }
    if cost > 1 {
        return 1;
    }
    if (position.x + position.y) % 4 == 0 {
        ctx.visuals
            .faction_terrain_cell(&ctx.mission.hostile_faction)
    } else {
        0
    }
}

fn draw_tile_contents(ctx: &UiContext<'_>, view: GridView, position: TilePos) {
    let rect = view.tile_rect(position);
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
    let canopy_tiles = [TilePos::new(6, 2), TilePos::new(7, 2)];
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
        let bounds = view.tile_rect(tile);
        ctx.visuals.draw_atlas_cell(
            ctx.assets,
            &ctx.visuals.terrain,
            7,
            Rect::new(
                bounds.x - 6.0,
                bounds.y - bounds.w * 1.15,
                bounds.w + 12.0,
                bounds.w * 1.55,
            ),
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
    ctx.visuals.draw_atlas_cell(
        ctx.assets,
        &ctx.visuals.effects,
        7,
        Rect::new(center.x - 24.0, center.y - 44.0, 48.0, 56.0),
        Color::new(1.0, 1.0, 1.0, 0.86),
    );
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

fn handle_click(ctx: &UiContext<'_>, view: GridView, mouse: Vec2, actions: &mut Vec<UiAction>) {
    if !is_mouse_button_released(MouseButton::Left) {
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
        let action = target.and_then(|target| match targeting {
            TargetingView::Equipment {
                unit_id,
                equipment_id,
            } if ctx
                .session
                .can_use_equipment(unit_id, equipment_id, &target.id) =>
            {
                Some(UiAction::UseEquipmentOn(target.id.clone()))
            }
            TargetingView::ClassAction { unit_id }
                if ctx.session.can_target_class_action(unit_id, &target.id) =>
            {
                Some(UiAction::UseClassActionOn(target.id.clone()))
            }
            _ => None,
        });
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
