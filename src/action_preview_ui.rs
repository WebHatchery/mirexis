//! Compact hover and keyboard-focus preview for player tactical commands.

use crate::action_preview::{self, ActionPreview};
use crate::data::HazardKind;
use crate::grid_ui::GridView;
use crate::state::GameSession;
use crate::tactical::terrain_cost;
use macroquad::prelude::*;
use macroquad_toolkit::prelude::{dark, draw_surface, SurfaceStyle, TextStyle};

pub(crate) fn draw(session: &GameSession, tile: macroquad_toolkit::grid::TilePos, panel: Rect) {
    let Some(preview) = action_preview::for_tile(session, tile) else {
        return;
    };
    let label = match preview {
        ActionPreview::Move { cost, hazard, .. } => match hazard {
            Some(kind) => format!("MOVE // {} AP // {}", cost, hazard_effect(kind)),
            None => format!("MOVE // {} AP", cost),
        },
        ActionPreview::Attack {
            target_name,
            cost,
            hit_chance,
            damage,
            critical_damage,
        } => format!(
            "ATTACK {} // {}% // {}-{} DMG // {} AP",
            target_name.to_uppercase(),
            hit_chance,
            damage,
            critical_damage,
            cost
        ),
    };
    let rect = Rect::new(panel.x + 18.0, panel.bottom() - 34.0, panel.w - 36.0, 28.0);
    draw_surface(
        rect,
        &SurfaceStyle::new(Color::new(0.06, 0.10, 0.11, 0.96))
            .with_border(1.0, Color::new(0.48, 0.84, 0.63, 0.9)),
    );
    draw_text_ex(
        label,
        rect.x + 10.0,
        rect.y + 19.0,
        TextStyle::new(13.0, dark::TEXT_BRIGHT).params(),
    );
}

pub(crate) fn draw_route(
    session: &GameSession,
    tile: macroquad_toolkit::grid::TilePos,
    view: GridView,
) {
    let Some(ActionPreview::Move { path, .. }) = action_preview::for_tile(session, tile) else {
        return;
    };
    let mut previous: Option<Vec2> = None;
    for (index, position) in path.iter().copied().enumerate() {
        let rect = view.tile_rect(position);
        let center = vec2(rect.x + rect.w * 0.5, rect.y + rect.h * 0.5);
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
