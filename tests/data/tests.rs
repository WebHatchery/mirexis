use super::*;

#[test]
fn complete_content_registry_is_valid() {
    let data = GameData::load().unwrap();

    assert_eq!(data.config.game_name, "mirexis");
    assert!(data.roster.iter().any(|unit| unit.team == Team::Colony));
    assert!(data.roster.iter().any(|unit| unit.team == Team::Hostile));
    assert_eq!(data.recruitable_roster.len(), 3);
    assert!(data.mission.round_limit > 0);
    assert!(data.mission.terrain_costs.iter().all(|tile| tile.cost > 0));
    assert!(data.roster.iter().all(|unit| unit.weapon_ap_cost > 0));
    assert_eq!(
        data.roster
            .iter()
            .filter(|unit| unit.team == Team::Colony)
            .count(),
        5
    );
    let hostile_roles = data
        .roster
        .iter()
        .filter(|unit| unit.team == Team::Hostile)
        .map(|unit| unit.role.as_str())
        .collect::<std::collections::HashSet<_>>();
    assert!(hostile_roles.len() >= 2);
    assert_eq!(data.characters.len(), 8);
    assert!(data.classes.len() >= 7);
    assert_eq!(
        data.classes.iter().filter(|class| class.advanced).count(),
        8
    );
    assert!(data.mutations.len() >= 5);
    assert_eq!(data.campaign.phase_id, "isolation");
    assert!(data.campaign.mission_templates.len() >= 4);
    let objective_kinds = data
        .campaign
        .mission_templates
        .iter()
        .map(|mission| mission.objective_kind)
        .collect::<std::collections::HashSet<_>>();
    assert_eq!(objective_kinds.len(), 6);
}
