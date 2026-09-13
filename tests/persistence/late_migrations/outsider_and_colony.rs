//! Outsider, colony, and relationship save migrations.

use super::*;

#[test]
fn version_171_save_gains_outsider_runtime_defaults() {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let mut legacy = serde_json::to_value(session.to_save("1.71.0", &campaign)).unwrap();
    let campaign = legacy["campaign"].as_object_mut().unwrap();
    campaign.remove("outsider_arc_stage");
    campaign.remove("outsider_disagreements");
    campaign.remove("outsider_final_choice");
    campaign.remove("outsider_arc_states");
    campaign.remove("commons_meals_hosted");
    campaign.remove("commons_meal_operation");
    for character in campaign["roster"].as_array_mut().unwrap() {
        let character = character.as_object_mut().unwrap();
        character.remove("origin");
        character.remove("origin_description");
    }

    let migrated = migrate_save_value(Some("1.71.0".to_owned()), legacy, &data).unwrap();

    assert_eq!(migrated.version, data.config.version);
    assert_eq!(migrated.campaign.outsider_arc_stage, 0);
    assert_eq!(migrated.campaign.outsider_disagreements, 0);
    assert!(migrated.campaign.outsider_final_choice.is_empty());
    assert!(migrated.campaign.outsider_arc_states.is_empty());
    assert_eq!(migrated.campaign.commons_meals_hosted, 0);
    assert_eq!(migrated.campaign.commons_meal_operation, None);
    assert!(migrated
        .campaign
        .roster
        .iter()
        .all(|character| character.origin.is_empty() && character.origin_description.is_empty()));
}

#[test]
fn version_172_save_gains_commons_runtime_defaults() {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let mut legacy = serde_json::to_value(session.to_save("1.72.0", &campaign)).unwrap();
    let campaign = legacy["campaign"].as_object_mut().unwrap();
    campaign.remove("commons_meals_hosted");
    campaign.remove("commons_meal_operation");

    let migrated = migrate_save_value(Some("1.72.0".to_owned()), legacy, &data).unwrap();

    assert_eq!(migrated.version, data.config.version);
    assert_eq!(migrated.campaign.commons_meals_hosted, 0);
    assert_eq!(migrated.campaign.commons_meal_operation, None);
}

#[test]
fn version_173_save_gains_relay_runtime_defaults() {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let mut legacy = serde_json::to_value(session.to_save("1.73.0", &campaign)).unwrap();
    let campaign = legacy["campaign"].as_object_mut().unwrap();
    campaign.remove("relay_scans_used");
    campaign.remove("relay_scan_operation");

    let migrated = migrate_save_value(Some("1.73.0".to_owned()), legacy, &data).unwrap();

    assert_eq!(migrated.version, data.config.version);
    assert_eq!(migrated.campaign.relay_scans_used, 0);
    assert_eq!(migrated.campaign.relay_scan_operation, None);
}

#[test]
fn same_version_corner_colony_save_moves_into_the_centered_frontier() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    let legacy_founders = [
        ("command_centre", [3, 2]),
        ("barracks", [2, 3]),
        ("infirmary", [4, 3]),
        ("workshop", [3, 4]),
        ("hydroponics", [5, 3]),
        ("power_plant", [5, 4]),
    ];
    for (id, position) in legacy_founders {
        campaign
            .colony
            .buildings
            .iter_mut()
            .find(|building| building.id == id)
            .unwrap()
            .position = position;
    }
    let legacy =
        serde_json::to_value(SaveData::campaign_only(&data.config.version, &campaign)).unwrap();

    let migrated = migrate_save_value(Some(data.config.version.clone()), legacy, &data).unwrap();
    let command_centre = migrated
        .campaign
        .colony
        .buildings
        .iter()
        .find(|building| building.id == "command_centre")
        .unwrap();
    assert_eq!(command_centre.position, mirexis::colony::SETTLEMENT_CENTER);
    assert!(migrated.campaign.colony.buildings.iter().all(|building| {
        building.position[0] >= 0
            && building.position[1] >= 0
            && building.position[0] < mirexis::colony::COLONY_WIDTH
            && building.position[1] < mirexis::colony::COLONY_HEIGHT
    }));
}

#[test]
fn chitin_save_gains_ilyas_unevolved_paths() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign.strategy.contact_complete = true;
    campaign.strategy.phase_id = "adaptation".to_owned();
    campaign.colony.ensure_gene_lab();
    campaign.roster[1].mutation_evolution_id = "fortress_carapace".to_owned();
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let legacy = serde_json::to_value(session.to_save("1.15.0", &campaign)).unwrap();
    let migrated = migrate_save_value(Some("1.15.0".to_owned()), legacy, &data).unwrap();
    let ilya = migrated
        .campaign
        .roster
        .iter()
        .find(|character| character.id == "ilya_reed")
        .unwrap();
    assert!(ilya.mutation_evolution_id.is_empty());
    assert_eq!(
        data.mutations
            .iter()
            .find(|mutation| mutation.id == ilya.mutation_id)
            .unwrap()
            .evolutions
            .len(),
        2
    );
}

#[test]
fn regeneration_save_gains_sols_unevolved_paths() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign.strategy.contact_complete = true;
    campaign.strategy.phase_id = "adaptation".to_owned();
    campaign.colony.ensure_gene_lab();
    campaign.roster[2].mutation_evolution_id = "clean_marrow".to_owned();
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let legacy = serde_json::to_value(session.to_save("1.16.0", &campaign)).unwrap();
    let migrated = migrate_save_value(Some("1.16.0".to_owned()), legacy, &data).unwrap();
    let sol = migrated
        .campaign
        .roster
        .iter()
        .find(|character| character.id == "sol_cairn")
        .unwrap();
    assert!(sol.mutation_evolution_id.is_empty());
    assert_eq!(
        data.mutations
            .iter()
            .find(|mutation| mutation.id == sol.mutation_id)
            .unwrap()
            .evolutions
            .len(),
        2
    );
}

#[test]
fn full_roster_save_gains_adaptation_completion_gates() {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let mut legacy = serde_json::to_value(session.to_save("1.17.0", &campaign)).unwrap();
    let strategy = legacy["campaign"]["strategy"].as_object_mut().unwrap();
    strategy.remove("adaptation_operation_completed");
    strategy.remove("adaptation_complete");
    let migrated = migrate_save_value(Some("1.17.0".to_owned()), legacy, &data).unwrap();
    assert_eq!(migrated.version, data.config.version);
    assert!(!migrated.campaign.strategy.adaptation_operation_completed);
    assert!(!migrated.campaign.strategy.adaptation_complete);
}

#[test]
fn adaptation_completion_save_gains_the_escalation_operation() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign.strategy.contact_complete = true;
    campaign.strategy.adaptation_operation_completed = true;
    campaign.strategy.adaptation_complete = true;
    campaign.strategy.phase_id = "escalation".to_owned();
    campaign.strategy.phase_name = "PHASE FOUR: ESCALATION".to_owned();
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let legacy = serde_json::to_value(session.to_save("1.18.0", &campaign)).unwrap();
    let migrated = migrate_save_value(Some("1.18.0".to_owned()), legacy, &data).unwrap();

    assert_eq!(migrated.version, data.config.version);
    assert!(migrated
        .campaign
        .strategy
        .mission_offers
        .iter()
        .any(|mission| mission.template_id == "escalation_three_knives"));
}

#[test]
fn escalation_operation_save_gains_response_state() {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let mut legacy = serde_json::to_value(session.to_save("1.19.0", &campaign)).unwrap();
    let strategy = legacy["campaign"]["strategy"].as_object_mut().unwrap();
    strategy.remove("escalation_operation_completed");
    strategy.remove("escalation_response_id");
    let migrated = migrate_save_value(Some("1.19.0".to_owned()), legacy, &data).unwrap();

    assert_eq!(migrated.version, data.config.version);
    assert!(!migrated.campaign.strategy.escalation_operation_completed);
    assert!(migrated.campaign.strategy.escalation_response_id.is_empty());
}

#[test]
fn convergence_response_save_gains_its_branch_operation() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign.strategy.phase_id = "escalation".to_owned();
    campaign.strategy.adaptation_complete = true;
    campaign.strategy.escalation_operation_completed = true;
    campaign.strategy.escalation_response_id = "bastion_beacon".to_owned();
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let legacy = serde_json::to_value(session.to_save("1.20.0", &campaign)).unwrap();
    let migrated = migrate_save_value(Some("1.20.0".to_owned()), legacy, &data).unwrap();

    assert_eq!(migrated.version, data.config.version);
    assert!(migrated
        .campaign
        .strategy
        .mission_offers
        .iter()
        .any(|mission| mission.template_id == "escalation_bastion_breakwater"));
}

#[test]
fn escalation_save_gains_mixed_power_deployment() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign.strategy.phase_id = "escalation".to_owned();
    campaign.strategy.adaptation_complete = true;
    campaign.strategy.regenerate_missions(&data);
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let mut legacy = serde_json::to_value(session.to_save("1.21.0", &campaign)).unwrap();
    for offer in legacy["campaign"]["strategy"]["mission_offers"]
        .as_array_mut()
        .unwrap()
    {
        offer.as_object_mut().unwrap().remove("hostile_unit_ids");
    }
    let migrated = migrate_save_value(Some("1.21.0".to_owned()), legacy, &data).unwrap();
    let three_knives = migrated
        .campaign
        .strategy
        .mission_offers
        .iter()
        .find(|mission| mission.template_id == "escalation_three_knives")
        .unwrap();

    assert_eq!(migrated.version, data.config.version);
    assert_eq!(three_knives.hostile_unit_ids.len(), 3);
}
