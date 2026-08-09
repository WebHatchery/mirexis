//! Operation-result and campaign-finale presentation.

use crate::campaign::CampaignState;
use crate::data::MissionDef;
use crate::state::{MissionOutcome, ObjectiveState};
use crate::ui::{UiAction, LOGICAL_HEIGHT, LOGICAL_WIDTH};
use crate::ui_widgets::button;
use macroquad::prelude::*;
use macroquad_toolkit::prelude::*;
use macroquad_toolkit::ui::VirtualUi;

pub fn draw_debrief(
    mission: &MissionDef,
    outcome: &MissionOutcome,
    campaign: &CampaignState,
    ui: &VirtualUi,
) -> Vec<UiAction> {
    let mut actions = Vec::new();
    let mouse = ui.mouse_position();
    draw_rectangle(
        0.0,
        0.0,
        LOGICAL_WIDTH,
        LOGICAL_HEIGHT,
        Color::new(0.025, 0.04, 0.055, 1.0),
    );
    let won = outcome.result == ObjectiveState::Victory;
    let panel = Rect::new(220.0, 100.0, 840.0, 500.0);
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
    draw_text("OPERATION DEBRIEF", 535.0, 132.0, 18.0, dark::TEXT);
    draw_text(
        if won {
            "OPERATION SUCCESS"
        } else {
            "OPERATION FAILED"
        },
        280.0,
        220.0,
        36.0,
        if won { dark::POSITIVE } else { dark::NEGATIVE },
    );
    if campaign.strategy.campaign_complete {
        draw_text(
            &campaign.strategy.phase_name,
            280.0,
            255.0,
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
        draw_text(line, 280.0, 315.0 + index as f32 * 30.0, 21.0, dark::TEXT);
    }
    if button(
        Rect::new(780.0, 510.0, 220.0, 48.0),
        "RETURN TO COLONY",
        true,
        mouse,
    ) {
        actions.push(UiAction::ReturnToColony);
    }
    actions
}
