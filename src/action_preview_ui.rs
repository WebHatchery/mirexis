//! Compact hover and keyboard-focus preview for player tactical commands.

use crate::action_preview::{self, ActionPreview};
use crate::data::HazardKind;
use crate::grid_ui::GridView;
use crate::state::GameSession;
use crate::state::RuleError;
use crate::tactical::terrain_cost;
use crate::visual_assets::VisualCatalog;
use macroquad::prelude::*;
use macroquad_toolkit::assets::AssetManager;
use macroquad_toolkit::prelude::{dark, draw_surface, SurfaceStyle, TextStyle};

pub(crate) fn draw(
    session: &GameSession,
    tile: macroquad_toolkit::grid::TilePos,
    panel: Rect,
    assets: &AssetManager,
    visuals: &VisualCatalog,
) {
    let Some(preview) = action_preview::for_tile(session, tile) else {
        return;
    };
    let invalid = matches!(preview, ActionPreview::Invalid { .. });
    if let ActionPreview::Attack {
        target_name,
        target_position,
        cost,
        hit_chance,
        cover_penalty,
        damage,
        critical_damage,
        ..
    } = &preview
    {
        draw_attack_comparison(
            session,
            *target_position,
            target_name,
            *cost,
            *hit_chance,
            *cover_penalty,
            *damage,
            *critical_damage,
            panel,
            assets,
            visuals,
        );
        return;
    }
    let label = match preview {
        ActionPreview::Move { cost, hazard, .. } => match hazard {
            Some(kind) => format!("MOVE // {} AP // {}", cost, hazard_effect(kind)),
            None => format!("MOVE // {} AP", cost),
        },
        ActionPreview::Attack {
            target_name,
            cost,
            hit_chance,
            cover_penalty,
            damage,
            critical_damage,
            ..
        } => format!(
            "ATTACK {} // {}%{} // {}-{} DMG // {} AP",
            target_name.to_uppercase(),
            hit_chance,
            if cover_penalty > 0 {
                format!(" // COVER -{cover_penalty}")
            } else {
                String::new()
            },
            damage,
            critical_damage,
            cost
        ),
        ActionPreview::Invalid { action, reason } => {
            format!("{action} BLOCKED // {}", rule_error_label(&reason))
        }
    };
    let rect = Rect::new(panel.x + 18.0, panel.bottom() - 34.0, panel.w - 36.0, 28.0);
    draw_surface(
        rect,
        &SurfaceStyle::new(Color::new(0.06, 0.10, 0.11, 0.96)).with_border(
            1.0,
            if invalid {
                Color::new(0.92, 0.42, 0.28, 0.95)
            } else {
                Color::new(0.48, 0.84, 0.63, 0.9)
            },
        ),
    );
    draw_text_ex(
        label,
        rect.x + 10.0,
        rect.y + 19.0,
        TextStyle::new(13.0, dark::TEXT_BRIGHT).params(),
    );
}

#[allow(clippy::too_many_arguments)]
fn draw_attack_comparison(
    session: &GameSession,
    target_position: macroquad_toolkit::grid::TilePos,
    target_name: &str,
    cost: u8,
    hit_chance: u8,
    cover_penalty: i32,
    damage: i32,
    critical_damage: i32,
    panel: Rect,
    assets: &AssetManager,
    visuals: &VisualCatalog,
) {
    let Some(attacker) = session.selected_unit() else {
        return;
    };
    let Some(target) = session
        .tactical
        .units
        .iter()
        .find(|unit| unit.position == target_position)
    else {
        return;
    };
    let rect = Rect::new(panel.x + 16.0, panel.bottom() - 94.0, panel.w - 32.0, 84.0);
    draw_surface(
        rect,
        &SurfaceStyle::new(Color::new(0.025, 0.052, 0.058, 0.98))
            .with_border(2.0, Color::new(0.86, 0.50, 0.24, 0.96)),
    );
    visuals.draw_portrait(
        assets,
        &attacker.id,
        &attacker.name,
        Rect::new(rect.x + 7.0, rect.y + 7.0, 60.0, 70.0),
        Color::new(0.24, 0.90, 0.76, 1.0),
    );
    visuals.draw_portrait(
        assets,
        &target.id,
        &target.name,
        Rect::new(rect.right() - 67.0, rect.y + 7.0, 60.0, 70.0),
        Color::new(0.98, 0.28, 0.24, 1.0),
    );
    let x = rect.x + 78.0;
    draw_text_ex(
        format!(
            "{}  >  {}",
            attacker.name.to_uppercase(),
            target_name.to_uppercase()
        ),
        x,
        rect.y + 22.0,
        TextStyle::new(14.0, dark::TEXT_BRIGHT).params(),
    );
    draw_text_ex(
        format!(
            "HIT {hit_chance}%  //  DAMAGE {damage}  //  CRITICAL {critical_damage}  //  {cost} AP"
        ),
        x,
        rect.y + 45.0,
        TextStyle::new(14.0, Color::new(1.0, 0.72, 0.30, 1.0)).params(),
    );
    draw_text_ex(
        format!(
            "TARGET VITALITY {} > {}  //  ARMOUR {}  //  COVER -{}",
            target.health,
            (target.health - damage).max(0),
            target.effective_armour(),
            cover_penalty
        ),
        x,
        rect.y + 67.0,
        TextStyle::new(12.0, dark::TEXT_DIM).params(),
    );
}

fn rule_error_label(error: &RuleError) -> &'static str {
    match error {
        RuleError::WrongPhase => "WRONG PHASE",
        RuleError::UnknownUnit => "UNKNOWN UNIT",
        RuleError::WrongTeam => "WRONG TEAM",
        RuleError::Incapacitated => "UNIT INCAPACITATED",
        RuleError::Occupied => "TILE OCCUPIED",
        RuleError::NoPath => "NO PATH",
        RuleError::NoLineOfFire => "NO LINE OF FIRE",
        RuleError::OutOfRange => "OUT OF RANGE",
        RuleError::InsufficientActionPoints => "INSUFFICIENT AP",
        RuleError::InvalidTarget => "INVALID TARGET",
        RuleError::ObjectiveUnavailable => "OBJECTIVE UNAVAILABLE",
        RuleError::MutationUnavailable => "MUTATION UNAVAILABLE",
        RuleError::ClassActionUnavailable => "CLASS ACTION UNAVAILABLE",
        RuleError::EquipmentUnavailable => "EQUIPMENT UNAVAILABLE",
        RuleError::CoverUnavailable => "COVER UNAVAILABLE",
        RuleError::OverwatchUnavailable => "OVERWATCH UNAVAILABLE",
        RuleError::EnemyAbilityUnavailable => "ABILITY UNAVAILABLE",
    }
}

pub(crate) fn draw_route(
    session: &GameSession,
    tile: macroquad_toolkit::grid::TilePos,
    view: GridView,
) {
    let Some(preview) = action_preview::for_tile(session, tile) else {
        return;
    };
    if let ActionPreview::Attack {
        attacker_position,
        target_position,
        ..
    } = preview
    {
        let from = tile_center(view, attacker_position);
        let to = tile_center(view, target_position);
        draw_line(from.x, from.y, to.x, to.y, 3.0, shot_color());
        for step in 1..6 {
            let amount = step as f32 / 6.0;
            let point = from.lerp(to, amount);
            draw_circle(point.x, point.y, 2.0, shot_color());
        }
        return;
    }
    let ActionPreview::Move { path, .. } = preview else {
        return;
    };
    let mut previous: Option<Vec2> = None;
    for (index, position) in path.iter().copied().enumerate() {
        let rect = view.tile_rect(position);
        let center = view.tile_center(position);
        if let Some(from) = previous {
            draw_line(from.x, from.y, center.x, center.y, 3.0, route_color());
        }
        if index > 0 {
            draw_circle(center.x, center.y, 5.0, route_color());
            draw_text_ex(
                format!(
                    "+{}",
                    terrain_cost(position, &session.tactical.terrain_costs)
                ),
                rect.x + rect.w - 17.0,
                rect.y + rect.h - 8.0,
                TextStyle::new(10.0, dark::TEXT_BRIGHT).params(),
            );
        }
        previous = Some(center);
    }
}

fn tile_center(view: GridView, tile: macroquad_toolkit::grid::TilePos) -> Vec2 {
    view.tile_center(tile)
}

fn shot_color() -> Color {
    Color::new(0.55, 0.94, 0.58, 0.95)
}

fn route_color() -> Color {
    Color::new(0.48, 0.90, 1.0, 0.9)
}

fn hazard_effect(kind: HazardKind) -> &'static str {
    match kind {
        HazardKind::FireLane => "FIRE LANE: 2 DAMAGE",
        HazardKind::SporeBloom => "SPORE BLOOM: 1 DAMAGE + HINDERED",
        HazardKind::StaticRift => "STATIC RIFT: DISRUPTED",
    }
}
