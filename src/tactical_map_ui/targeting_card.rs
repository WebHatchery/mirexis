//! Targeting-card and tile-preview presentation for tactical actions.

use crate::data::Team;
use crate::grid_ui::GridView;
use crate::ui::{draw_ui_text_ex, TargetingView, UiAction, UiContext};
use macroquad::prelude::*;
use macroquad_toolkit::grid::TilePos;
use macroquad_toolkit::prelude::{dark, draw_surface, SurfaceStyle, TextStyle};

pub(super) fn card_bounds(panel: Rect) -> Rect {
    Rect::new(panel.x + 16.0, panel.bottom() - 94.0, panel.w - 32.0, 80.0)
}

pub(super) fn cancel_bounds(card: Rect) -> Rect {
    Rect::new(card.right() - 232.0, card.bottom() - 36.0, 146.0, 28.0)
}

pub(crate) fn draw_skill_target_tiles(ctx: &UiContext<'_>, view: GridView) {
    let Some(TargetingView::Skill { unit_id, skill_id }) = ctx.targeting else {
        return;
    };
    if crate::skills::target_kind(skill_id) != Some(crate::data::TechniqueTarget::Tile) {
        return;
    }
    for (position, _) in ctx.session.tactical.fog.iter_with_pos() {
        if crate::skills::can_target_tile(ctx.session, unit_id, skill_id, position) {
            super::draw_diamond_fill(view.diamond(position), Color::new(0.95, 0.65, 0.20, 0.25));
            super::draw_diamond_outline(
                view.diamond(position),
                Color::new(1.0, 0.78, 0.30, 1.0),
                3.0,
            );
        }
    }
}

pub(crate) fn draw(
    ctx: &UiContext<'_>,
    tile: TilePos,
    panel: Rect,
    mouse: Vec2,
    actions: &mut Vec<UiAction>,
) {
    let card = card_bounds(panel);
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
        }) => draw_equipment(ctx, card, target, unit_id, equipment_id),
        Some(TargetingView::ClassAction { unit_id }) => {
            draw_class_action(ctx, card, target, unit_id)
        }
        Some(TargetingView::Skill { unit_id, skill_id }) => {
            draw_skill(ctx, card, target, tile, unit_id, skill_id)
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
    let instruction = match ctx.targeting {
        Some(TargetingView::Skill { skill_id, .. })
            if crate::skills::target_kind(skill_id) == Some(crate::data::TechniqueTarget::Tile) =>
        {
            "TAP HIGHLIGHTED TILE / A CONFIRM"
        }
        _ => "TAP HIGHLIGHTED UNIT / A CONFIRM",
    };
    draw_ui_text_ex(
        instruction,
        card.x + 82.0,
        card.bottom() - 9.0,
        TextStyle::new(10.0, dark::TEXT_DIM).params(),
    );
    if crate::ui_widgets::button(cancel_bounds(card), "CANCEL TARGET", true, mouse) {
        actions.push(UiAction::CancelTargeting);
    }
}

fn draw_equipment(
    ctx: &UiContext<'_>,
    card: Rect,
    target: Option<&crate::state::UnitState>,
    unit_id: &str,
    equipment_id: &str,
) {
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
            validity_label(valid)
        ),
        card.x + 82.0,
        card.y + 49.0,
        TextStyle::new(13.0, validity_color(valid)).params(),
    );
}

fn draw_class_action(
    ctx: &UiContext<'_>,
    card: Rect,
    target: Option<&crate::state::UnitState>,
    unit_id: &str,
) {
    let user = ctx.session.unit(unit_id);
    let valid =
        target.is_some_and(|target| ctx.session.can_target_class_action(unit_id, &target.id));
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
            "{} // {}",
            target.map_or("NO UNIT SELECTED", |unit| unit.name.as_str()),
            validity_label(valid)
        ),
        card.x + 82.0,
        card.y + 49.0,
        TextStyle::new(13.0, validity_color(valid)).params(),
    );
}

fn draw_skill(
    ctx: &UiContext<'_>,
    card: Rect,
    target: Option<&crate::state::UnitState>,
    tile: TilePos,
    unit_id: &str,
    skill_id: &str,
) {
    let user = ctx.session.unit(unit_id);
    let technique = user.and_then(|user| {
        ctx.data
            .classes
            .iter()
            .find(|class| class.id == user.class_id)
            .and_then(|class| {
                class
                    .techniques
                    .iter()
                    .find(|technique| technique.id == skill_id)
            })
    });
    let tile_target =
        crate::skills::target_kind(skill_id) == Some(crate::data::TechniqueTarget::Tile);
    let valid = if tile_target {
        crate::skills::can_target_tile(ctx.session, unit_id, skill_id, tile)
    } else {
        target.is_some_and(|target| {
            crate::skills::can_target_unit(ctx.session, unit_id, skill_id, &target.id)
        })
    };
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
        &format!(
            "TECHNIQUE TARGETING // {}",
            technique.map_or(skill_id, |technique| technique.name.as_str())
        ),
        card.x + 82.0,
        card.y + 25.0,
        TextStyle::new(16.0, dark::WARNING).params(),
    );
    draw_ui_text_ex(
        &format!(
            "{} // {}",
            if tile_target {
                format!("TILE {},{}", tile.x, tile.y)
            } else {
                target.map_or("NO UNIT SELECTED".to_owned(), |unit| unit.name.clone())
            },
            validity_label(valid)
        ),
        card.x + 82.0,
        card.y + 49.0,
        TextStyle::new(13.0, validity_color(valid)).params(),
    );
}

fn validity_label(valid: bool) -> &'static str {
    if valid {
        "VALID TARGET"
    } else {
        "INVALID TARGET"
    }
}

fn validity_color(valid: bool) -> Color {
    if valid {
        dark::POSITIVE
    } else {
        dark::NEGATIVE
    }
}
