//! Illustrated campaign-decision context shown in place of routine mission offers.

use crate::{campaign::CampaignState, visual_assets::VisualCatalog};
use macroquad::prelude::*;
use macroquad_toolkit::{assets::AssetManager, prelude::dark, ui::draw_ui_text};

#[derive(Clone, Copy)]
pub(crate) enum DecisionKind {
    Contact,
    Escalation,
    Mirexis,
}

pub(crate) fn draw_decision_dossier(
    kind: DecisionKind,
    campaign: &CampaignState,
    assets: &AssetManager,
    visuals: &VisualCatalog,
) {
    let panel = Rect::new(878.0, 354.0, 362.0, 146.0);
    draw_rectangle(
        panel.x,
        panel.y,
        panel.w,
        panel.h,
        Color::new(0.02, 0.045, 0.05, 0.96),
    );
    draw_rectangle_lines(
        panel.x,
        panel.y,
        panel.w,
        panel.h,
        1.0,
        Color::new(0.12, 0.34, 0.34, 1.0),
    );
    let (title, subtitle, enemy_id, enemy_name, effect_cell, accent) = match kind {
        DecisionKind::Contact => (
            "CONTACT DOSSIER",
            "ALIEN SIGNAL // HUMAN RESPONSE",
            "brood_sporecaster",
            "Sporecaster",
            5,
            Color::new(0.62, 0.88, 0.35, 1.0),
        ),
        DecisionKind::Escalation => (
            "CONVERGENCE DOSSIER",
            "THREE POWERS // ONE COLONY",
            "directorate_rifleman",
            "Rifleman",
            4,
            dark::WARNING,
        ),
        DecisionKind::Mirexis => (
            "MIREXIS DOSSIER",
            "IDENTITY // SURVIVAL // ASCENSION",
            "ascendant_warden",
            "Warden",
            6,
            Color::new(0.72, 0.52, 1.0, 1.0),
        ),
    };
    draw_ui_text(title, 890.0, 374.0, 16.0, accent);
    draw_ui_text(subtitle, 890.0, 390.0, 10.0, dark::TEXT_DIM);

    if let Some(colonist) = campaign.roster.first() {
        visuals.draw_portrait(
            assets,
            &colonist.id,
            &colonist.name,
            Rect::new(890.0, 400.0, 70.0, 84.0),
            dark::ACCENT,
        );
    }
    visuals.draw_atlas_cell(
        assets,
        &visuals.effects,
        effect_cell,
        Rect::new(976.0, 407.0, 78.0, 70.0),
        WHITE,
    );
    visuals.draw_portrait(
        assets,
        enemy_id,
        enemy_name,
        Rect::new(1070.0, 400.0, 70.0, 84.0),
        accent,
    );
    draw_ui_text("COLONY", 898.0, 496.0, 9.0, dark::TEXT_DIM);
    draw_ui_text("DECIDE", 992.0, 496.0, 9.0, accent);
    draw_ui_text("OUTSIDE", 1081.0, 496.0, 9.0, dark::TEXT_DIM);
    draw_line(960.0, 442.0, 976.0, 442.0, 1.5, accent);
    draw_line(1054.0, 442.0, 1070.0, 442.0, 1.5, accent);
}
