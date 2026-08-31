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

#[test]
fn signal_cartography_reveals_reinforcement_roles_in_the_briefing() {
    let data = GameData::load().unwrap();
    let mut mission = data.mission.clone();
    mission.objective_kind = ObjectiveKind::Holdout;
    mission.hostile_faction = "brood".to_owned();
    mission.round_limit = 6;

    let basic = intel_lines_with_cartography(&data, &mission, false).join(" ");
    let mapped = intel_lines_with_cartography(&data, &mission, true).join(" ");

    assert!(basic.contains("WAVES // R3/R5 · UNKNOWN ROLES · EAST"));
    assert!(mapped.contains("WAVES // R3/R5 · "));
    assert!(!mapped.contains("UNKNOWN ROLES"));
}

#[test]
fn repeated_hostile_roles_are_collapsed_for_the_briefing_row() {
    let data = GameData::load().unwrap();
    let roles = intel_lines(&data, &data.mission)
        .into_iter()
        .find(|line| line.starts_with("ROLES //"))
        .expect("briefing includes a roles row");

    assert!(!roles.contains(" · ARTILLERY · ARTILLERY"));
}
