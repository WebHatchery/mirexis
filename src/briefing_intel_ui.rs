//! Materialized mission threat intelligence for deployment planning.

use crate::campaign::CampaignState;
use crate::colony::{BuildingKind, SIGNAL_CARTOGRAPHY_UPGRADE};
use crate::data::{GameData, HazardKind, MissionDef, ObjectiveKind, Team};
use macroquad::prelude::*;
use macroquad_toolkit::prelude::{dark, fit_text_to_box_ex, TextLayoutResult, TextStyle};

const INTEL_TEXT_WIDTH: f32 = 232.0;
const INTEL_LINE_HEIGHT: f32 = 17.0;
const INTEL_MIN_FONT_SIZE: f32 = 8.5;

pub(crate) fn draw(campaign: &CampaignState, data: &GameData, mission: &MissionDef, origin: Vec2) {
    let danger = crate::danger_rating::for_mission(mission, data);
    draw_text_ex(
        format!("THREAT INTELLIGENCE // {} {}", danger.label(), danger.score),
        origin.x,
        origin.y,
        TextStyle::new(15.0, Color::new(0.96, 0.55, 0.35, 1.0)).params(),
    );
    let cartography_active = campaign
        .colony
        .has_active_upgrade(BuildingKind::CommandCentre, SIGNAL_CARTOGRAPHY_UPGRADE);
    let lines = if cartography_active {
        intel_lines_with_cartography(data, mission, true)
    } else {
        intel_lines(data, mission)
    };
    for (index, line) in lines.iter().enumerate() {
        draw_intel_line(line, origin.x, origin.y + 24.0 + index as f32 * 17.0);
    }
}

fn intel_line_layout(line: &str) -> TextLayoutResult {
    fit_text_to_box_ex(
        line,
        INTEL_TEXT_WIDTH,
        INTEL_LINE_HEIGHT,
        TextStyle::new(12.5, dark::TEXT_DIM),
        INTEL_MIN_FONT_SIZE,
    )
}

fn draw_intel_line(line: &str, x: f32, y: f32) {
    let layout = intel_line_layout(line);
    let content = layout.lines.first().map(String::as_str).unwrap_or("");
    draw_text_ex(
        content,
        x,
        y,
        TextStyle::new(layout.font_size, dark::TEXT_DIM).params(),
    );
}

fn intel_lines(data: &GameData, mission: &MissionDef) -> Vec<String> {
    intel_lines_with_cartography(data, mission, true)
}

fn intel_lines_with_cartography(
    data: &GameData,
    mission: &MissionDef,
    cartography_active: bool,
) -> Vec<String> {
    let hostiles = data
        .roster
        .iter()
        .filter(|unit| {
            unit.team == Team::Hostile
                && if mission.hostile_unit_ids.is_empty() {
                    unit.faction.as_deref() == Some(&mission.hostile_faction)
                } else {
                    mission.hostile_unit_ids.contains(&unit.id)
                }
        })
        .collect::<Vec<_>>();
    let roles = hostiles.iter().fold(Vec::new(), |mut roles, unit| {
        let role = unit.role.to_uppercase();
        if !roles.contains(&role) {
            roles.push(role);
        }
        roles
    });
    let roles = roles.join(" · ");
    let ability = crate::enemy_abilities::ability_name(Some(&mission.hostile_faction))
        .unwrap_or("MIXED POWER RESPONSE");
    let hazards = [
        (HazardKind::FireLane, "FIRE LANE"),
        (HazardKind::SporeBloom, "SPORE BLOOM"),
        (HazardKind::StaticRift, "STATIC RIFT"),
    ]
    .into_iter()
    .filter_map(|(kind, name)| {
        let count = mission
            .hazards
            .iter()
            .filter(|hazard| hazard.kind == kind)
            .count();
        (count > 0).then(|| format!("{}x {}", count, name))
    })
    .collect::<Vec<_>>()
    .join(" · ");
    vec![
        format!(
            "CONTRACT // {} · {} ROUNDS",
            objective_label(mission.objective_kind),
            mission.round_limit
        ),
        format!("HOSTILES // {}", hostiles.len()),
        if roles.is_empty() {
            "ROLES // UNKNOWN".to_owned()
        } else {
            format!("ROLES // {}", roles)
        },
        format!("ABILITY // {}", ability),
        if hazards.is_empty() {
            "HAZARDS // NONE KNOWN".to_owned()
        } else {
            format!("HAZARDS // {}", hazards)
        },
        {
            let wave_forecast = if cartography_active {
                crate::reinforcements::briefing_forecast(data, mission)
            } else {
                crate::reinforcements::briefing_forecast_with_detail(data, mission, false)
            };
            wave_forecast.map_or_else(
                || "WAVES // NONE".to_owned(),
                |wave| format!("WAVES // {}", wave),
            )
        },
    ]
}

fn objective_label(kind: ObjectiveKind) -> &'static str {
    match kind {
        ObjectiveKind::SecureAndClear => "SECURE + CLEAR",
        ObjectiveKind::EliminateAll => "ELIMINATE",
        ObjectiveKind::Holdout => "HOLDOUT",
        ObjectiveKind::Extraction => "EXTRACTION",
        ObjectiveKind::SignalTrace => "SIGNAL TRACE",
        ObjectiveKind::DefendAsset => "DEFEND ASSET",
    }
}

#[cfg(test)]
mod tests;
