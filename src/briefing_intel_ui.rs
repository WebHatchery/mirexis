//! Materialized mission threat intelligence for deployment planning.

use crate::data::{GameData, HazardKind, MissionDef, ObjectiveKind, Team};
use macroquad::prelude::*;
use macroquad_toolkit::prelude::{dark, TextStyle};

pub(crate) fn draw(data: &GameData, mission: &MissionDef, origin: Vec2) {
    let danger = crate::danger_rating::for_mission(mission, data);
    draw_text_ex(
        format!("THREAT INTELLIGENCE // {} {}", danger.label(), danger.score),
        origin.x,
        origin.y,
        TextStyle::new(15.0, Color::new(0.96, 0.55, 0.35, 1.0)).params(),
    );
    let lines = intel_lines(data, mission);
    for (index, line) in lines.iter().enumerate() {
        draw_text_ex(
            line,
            origin.x,
            origin.y + 24.0 + index as f32 * 20.0,
            TextStyle::new(13.0, dark::TEXT_DIM).params(),
        );
    }
}

fn intel_lines(data: &GameData, mission: &MissionDef) -> Vec<String> {
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
    let roles = hostiles
        .iter()
        .map(|unit| unit.role.to_uppercase())
        .collect::<Vec<_>>()
        .join(" · ");
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
mod tests {
    use super::*;

    #[test]
    fn materialized_briefing_intel_names_contract_roles_ability_and_hazards() {
        let data = GameData::load().unwrap();
        let mut mission = data.mission.clone();
        mission.objective_kind = ObjectiveKind::Extraction;
        mission.hostile_faction = "brood".to_owned();
        mission.hazards = vec![crate::data::HazardDef {
            position: [5, 3],
            kind: HazardKind::SporeBloom,
        }];
        let lines = intel_lines(&data, &mission).join(" ");

        assert!(lines.contains("EXTRACTION"));
        assert!(lines.contains("PREDATORY SURGE"));
        assert!(lines.contains("SPORE BLOOM"));
    }
}
