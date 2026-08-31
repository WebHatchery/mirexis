//! Operation-result and campaign-finale presentation.

use crate::campaign::CampaignState;
use crate::data::MissionDef;
use crate::state::{CharacterConsequence, MissionOutcome, ObjectiveState};
use crate::ui::{UiAction, LOGICAL_HEIGHT, LOGICAL_WIDTH};
use crate::ui_widgets::button;
use crate::visual_assets::VisualCatalog;
use macroquad::prelude::*;
use macroquad_toolkit::assets::AssetManager;
use macroquad_toolkit::prelude::*;
use macroquad_toolkit::ui::VirtualUi;

#[cfg(test)]
mod tests;

fn experience_summary(outcome: &MissionOutcome) -> String {
    format!(
        "Field experience: +{} XP each // {} deployed colonists",
        crate::campaign::operation_experience(outcome.result),
        outcome.colonists_deployed
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SquadStatus {
    Returned,
    Scar,
    Reserve,
}

fn squad_status(
    character_id: &str,
    deployed_ids: &[String],
    incapacitated: &[CharacterConsequence],
) -> SquadStatus {
    if incapacitated.iter().any(|entry| entry.id == character_id) {
        SquadStatus::Scar
    } else if deployed_ids.iter().any(|id| id == character_id) {
        SquadStatus::Returned
    } else {
        SquadStatus::Reserve
    }
}

fn debrief_roster_indices(
    roster: &[crate::campaign::CharacterRecord],
    deployed_ids: &[String],
) -> Vec<usize> {
    let mut indices = Vec::new();
    for (index, character) in roster.iter().enumerate() {
        if indices.len() == 5 {
            break;
        }
        if deployed_ids.iter().any(|id| id == &character.id) {
            indices.push(index);
        }
    }
    for index in 0..roster.len() {
        if indices.len() == 5 {
            break;
        }
        if !indices.contains(&index) {
            indices.push(index);
        }
    }
    indices
}

pub fn draw_debrief(
    mission: &MissionDef,
    outcome: &MissionOutcome,
    deployed_ids: &[String],
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
        if let Some(dossier) = crate::epilogue::derive(campaign) {
            draw_text(dossier.debrief_line(), 200.0, 232.0, 12.0, dark::POSITIVE);
        }
    }
    if campaign.operations_completed <= 2 {
        draw_text(
            if won {
                "ILYA REED // Account for everyone before Mara spends the recovery."
            } else {
                "ILYA REED // Failure is a condition. We treat it, then move."
            },
            200.0,
            214.0,
            14.0,
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
        experience_summary(outcome),
    ];
    if won && outcome.colonists_deployed > 1 {
        report.push("Shared victory strengthened squad relationships".to_owned());
    }
    if campaign.last_operation_had_commons_meal() {
        report.push(if won {
            "Commons meal carried the squad's bond into the field".to_owned()
        } else {
            "Commons meal gave the squad a table to return to".to_owned()
        });
    }
    if !outcome.colonists_incapacitated.is_empty() {
        report.push("Incapacitation left a lasting tradeoff scar".to_owned());
    }
    for (index, line) in report.iter().enumerate() {
        draw_text(line, 200.0, 258.0 + index as f32 * 23.0, 17.0, dark::TEXT);
    }
    draw_field_record(mission, outcome, assets, visuals, won);
    draw_squad_tableau(campaign, outcome, deployed_ids, assets, visuals, won);
    if button(
        crate::first_hour_ui::debrief_return_button_bounds(),
        "RETURN TO COLONY",
        true,
        mouse,
    ) {
        actions.push(UiAction::ReturnToColony);
    }
    draw_text(
        "TAP RETURN TO COLONY // ENTER / SPACE / PAD A",
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
    deployed_ids: &[String],
    assets: &AssetManager,
    visuals: &VisualCatalog,
    won: bool,
) {
    draw_text(
        "SQUAD STATE // RETURNED / SCARRED / RESERVE",
        200.0,
        438.0,
        15.0,
        dark::ACCENT,
    );
    let roster_indices = debrief_roster_indices(&campaign.roster, deployed_ids);
    for (index, roster_index) in roster_indices.into_iter().enumerate() {
        let character = &campaign.roster[roster_index];
        let status = squad_status(
            &character.id,
            deployed_ids,
            &outcome.colonists_incapacitated,
        );
        let rect = Rect::new(200.0 + index as f32 * 158.0, 458.0, 132.0, 108.0);
        crate::portrait_ui::draw_character_portrait(
            assets,
            visuals,
            Rect::new(rect.x, rect.y, 74.0, 84.0),
            &character.id,
            &character.name,
            match status {
                SquadStatus::Scar => dark::NEGATIVE,
                SquadStatus::Returned if won => dark::POSITIVE,
                SquadStatus::Returned => dark::WARNING,
                SquadStatus::Reserve => dark::TEXT_DIM,
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
            match status {
                SquadStatus::Scar => "SCAR",
                SquadStatus::Returned => "RETURNED",
                SquadStatus::Reserve => "RESERVE",
            },
            rect.x + 80.0,
            rect.y + 30.0,
            11.0,
            match status {
                SquadStatus::Scar => dark::NEGATIVE,
                SquadStatus::Returned if won => dark::POSITIVE,
                SquadStatus::Returned => dark::WARNING,
                SquadStatus::Reserve => dark::TEXT_DIM,
            },
        );
    }
}
