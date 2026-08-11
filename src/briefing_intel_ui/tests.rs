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
