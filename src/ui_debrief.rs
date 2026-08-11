//! Operation-result and campaign-finale presentation.

use crate::campaign::CampaignState;
use crate::data::MissionDef;
use crate::state::{MissionOutcome, ObjectiveState};
use crate::ui::{UiAction, LOGICAL_HEIGHT, LOGICAL_WIDTH};
use crate::ui_widgets::button;
use crate::visual_assets::VisualCatalog;
use macroquad::prelude::*;
use macroquad_toolkit::assets::AssetManager;
use macroquad_toolkit::prelude::*;
use macroquad_toolkit::ui::VirtualUi;

pub fn draw_debrief(
    mission: &MissionDef,
    outcome: &MissionOutcome,
    campaign: &CampaignState,
    assets: &AssetManager,
    visuals: &VisualCatalog,
    ui: &VirtualUi,
) -> Vec<UiAction> {
    let mut actions = Vec::new();
    let mouse = crate::ui::pointer_position(ui);
    draw_rectangle(
        0.0,
        0.0,
        LOGICAL_WIDTH,
        LOGICAL_HEIGHT,
        Color::new(0.025, 0.04, 0.055, 1.0),
    );
    let won = outcome.result == ObjectiveState::Victory;
    let panel = Rect::new(140.0, 76.0, 1000.0, 568.0);
    draw_surface(
        panel,
        &SurfaceStyle::new(Color::new(0.045, 0.065, 0.075, 0.98))
            .with_border(1.0, if won { dark::POSITIVE } else { dark::NEGATIVE }),
    );
    draw_rectangle(
        panel.x,
        panel.y,
        panel.w,
        48.0,
        Color::new(0.07, 0.12, 0.13, 1.0),
    );
    draw_text(
        "OPERATION DEBRIEF // CONSEQUENCE RECORD",
        430.0,
        108.0,
        18.0,
        dark::TEXT,
    );
    draw_text(
        if won {
            "OPERATION SUCCESS"
        } else {
            "OPERATION FAILED"
        },
        200.0,
        174.0,
        36.0,
        if won { dark::POSITIVE } else { dark::NEGATIVE },
    );
    if campaign.strategy.campaign_complete {
        draw_text(
            &campaign.strategy.phase_name,
            200.0,
            207.0,
            20.0,
            dark::ACCENT,
        );
    }
    let injuries = if outcome.colonists_incapacitated.is_empty() {
        "No squad incapacitations".to_owned()
    } else {
        format!(
            "Incapacitated: {}",
            outcome
                .colonists_incapacitated
                .iter()
                .map(|entry| entry.name.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        )
    };
    let mut report = vec![
        format!("Operation: {}", mission.name),
        format!(
            "Squad returned: {}/{}",
            outcome.colonists_deployed - outcome.colonists_incapacitated.len(),
            outcome.colonists_deployed
        ),
        injuries,
        format!("Hostiles neutralised: {}", outcome.hostiles_neutralised),
        format!(
            "Recovered: {} materials · {} biomass · {} power",
            outcome.materials_awarded, outcome.biomass_awarded, outcome.power_awarded
        ),
    ];
    if won && outcome.colonists_deployed > 1 {
        report.push("Shared victory strengthened squad relationships".to_owned());
    }
    if !outcome.colonists_incapacitated.is_empty() {
        report.push("Incapacitation left a lasting tradeoff scar".to_owned());
    }
    for (index, line) in report.iter().enumerate() {
        draw_text(line, 200.0, 258.0 + index as f32 * 27.0, 19.0, dark::TEXT);
    }
    draw_field_record(mission, outcome, assets, visuals, won);
    draw_squad_tableau(campaign, outcome, assets, visuals, won);
    if button(
        Rect::new(860.0, 574.0, 220.0, 48.0),
        "RETURN TO COLONY",
        true,
        mouse,
    ) {
        actions.push(UiAction::ReturnToColony);
    }
    draw_text(
        "ENTER / SPACE / PAD A // RETURN TO COLONY",
        438.0,
        672.0,
        13.0,
        dark::TEXT_DIM,
    );
    actions
}

fn draw_field_record(
    mission: &MissionDef,
    outcome: &MissionOutcome,
    assets: &AssetManager,
    visuals: &VisualCatalog,
    won: bool,
) {
    let panel = Rect::new(752.0, 148.0, 338.0, 260.0);
    draw_surface(
        panel,
        &SurfaceStyle::new(Color::new(0.028, 0.052, 0.058, 0.98))
            .with_border(1.0, if won { dark::POSITIVE } else { dark::NEGATIVE }),
    );
    let (unit_id, unit_name, terrain, effect, accent, faction) =
        if mission.hostile_faction.contains("directorate") {
            (
                "directorate_rifleman",
                "Directorate Rifleman",
                10,
                4,
                Color::new(1.0, 0.34, 0.22, 1.0),
                "DIRECTORATE",
            )
        } else if mission.hostile_faction.contains("ascendant") {
            (
                "ascendant_sentinel",
                "Ascendant Sentinel",
                11,
                6,
                Color::new(0.72, 0.48, 1.0, 1.0),
                "ASCENDANT",
            )
        } else {
            (
                "brood_stalker",
                "Brood Stalker",
                9,
                5,
                Color::new(0.96, 0.30, 0.34, 1.0),
                "BROOD",
            )
        };
    draw_text(
        "FIELD RECORD // HOSTILE POWER",
        panel.x + 18.0,
        panel.y + 28.0,
        14.0,
        dark::TEXT_DIM,
    );
    visuals.draw_atlas_cell(
        assets,
        &visuals.terrain,
        terrain,
        Rect::new(panel.x + 16.0, panel.y + 44.0, 144.0, 96.0),
        WHITE,
    );
    visuals.draw_portrait(
        assets,
        unit_id,
        unit_name,
        Rect::new(panel.x + 176.0, panel.y + 44.0, 72.0, 96.0),
        accent,
    );
    visuals.draw_atlas_cell(
        assets,
        &visuals.effects,
        effect,
        Rect::new(panel.x + 244.0, panel.y + 50.0, 78.0, 82.0),
        WHITE,
    );
    draw_text(faction, panel.x + 18.0, panel.y + 162.0, 18.0, accent);
    draw_text(
        if won {
            "THREAT NEUTRALISED // FIELD SECURED"
        } else {
            "THREAT ACTIVE // COLONY EXPOSED"
        },
        panel.x + 18.0,
        panel.y + 184.0,
        13.0,
        if won { dark::POSITIVE } else { dark::NEGATIVE },
    );
    draw_text(
        format!(
            "SALVAGE  M{:02}  B{:02}  P{:02}    LOSSES {}",
            outcome.materials_awarded,
            outcome.biomass_awarded,
            outcome.power_awarded,
            outcome.colonists_incapacitated.len()
        ),
        panel.x + 18.0,
        panel.y + 218.0,
        14.0,
        dark::TEXT_BRIGHT,
    );
    draw_rectangle(
        panel.x + 18.0,
        panel.y + 232.0,
        panel.w - 36.0,
        4.0,
        if won { dark::POSITIVE } else { dark::NEGATIVE },
    );
}

fn draw_squad_tableau(
    campaign: &CampaignState,
    outcome: &MissionOutcome,
    assets: &AssetManager,
    visuals: &VisualCatalog,
    won: bool,
) {
    draw_text(
        "SQUAD STATE // RETURNED / SCARRED / RECOVERING",
        200.0,
        438.0,
        15.0,
        dark::ACCENT,
    );
    for (index, character) in campaign.roster.iter().take(5).enumerate() {
        let incapacitated = outcome
            .colonists_incapacitated
            .iter()
            .any(|entry| entry.name == character.name);
        let rect = Rect::new(200.0 + index as f32 * 158.0, 458.0, 132.0, 108.0);
        crate::portrait_ui::draw_character_portrait(
            assets,
            visuals,
            Rect::new(rect.x, rect.y, 74.0, 84.0),
            &character.id,
            &character.name,
            if incapacitated {
                dark::NEGATIVE
            } else if won {
                dark::POSITIVE
            } else {
                dark::WARNING
            },
        );
        draw_text(
            &character.name,
            rect.x,
            rect.bottom() - 8.0,
            14.0,
            dark::TEXT,
        );
        draw_text(
            if incapacitated { "SCAR" } else { "RETURNED" },
            rect.x + 80.0,
            rect.y + 30.0,
            11.0,
            if incapacitated {
                dark::NEGATIVE
            } else {
                dark::POSITIVE
            },
        );
    }
}
