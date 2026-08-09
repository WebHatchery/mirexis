//! Compact hover and keyboard-focus preview for player tactical commands.

use crate::action_preview::{self, ActionPreview};
use crate::data::HazardKind;
use crate::state::GameSession;
use macroquad::prelude::*;
use macroquad_toolkit::prelude::{dark, draw_surface, SurfaceStyle, TextStyle};

pub(crate) fn draw(session: &GameSession, tile: macroquad_toolkit::grid::TilePos, panel: Rect) {
    let Some(preview) = action_preview::for_tile(session, tile) else {
        return;
    };
    let label = match preview {
        ActionPreview::Move { cost, hazard } => match hazard {
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

fn hazard_effect(kind: HazardKind) -> &'static str {
    match kind {
        HazardKind::FireLane => "FIRE LANE: 2 DAMAGE",
        HazardKind::SporeBloom => "SPORE BLOOM: 1 DAMAGE + HINDERED",
        HazardKind::StaticRift => "STATIC RIFT: DISRUPTED",
    }
}
