use super::*;

#[test]
fn gene_lab_save_gains_maras_unevolved_paths() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign.strategy.contact_complete = true;
    campaign.strategy.phase_id = "adaptation".to_owned();
    campaign.colony.ensure_gene_lab();
    campaign.roster[0].mutation_evolution_id = "expanded_cortex".to_owned();
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let legacy = serde_json::to_value(session.to_save("1.14.0", &campaign)).unwrap();
    let migrated = migrate_save_value(Some("1.14.0".to_owned()), legacy, &data).unwrap();
    let mara = migrated
        .campaign
        .roster
        .iter()
        .find(|character| character.id == "mara_venn")
        .unwrap();
    assert!(mara.mutation_evolution_id.is_empty());
    assert_eq!(
        data.mutations
            .iter()
            .find(|mutation| mutation.id == mara.mutation_id)
            .unwrap()
            .evolutions
            .len(),
        2
    );
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

#[test]
fn mixed_power_save_gains_escalation_completion_gates() {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let mut legacy = serde_json::to_value(session.to_save("1.22.0", &campaign)).unwrap();
    let strategy = legacy["campaign"]["strategy"].as_object_mut().unwrap();
    strategy.remove("escalation_branch_completed");
    strategy.remove("escalation_complete");
    let migrated = migrate_save_value(Some("1.22.0".to_owned()), legacy, &data).unwrap();

    assert_eq!(migrated.version, data.config.version);
    assert!(!migrated.campaign.strategy.escalation_branch_completed);
    assert!(!migrated.campaign.strategy.escalation_complete);
}

#[test]
fn phase_five_save_gains_an_uncommitted_mirexis_path() {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let mut legacy = serde_json::to_value(session.to_save("1.23.0", &campaign)).unwrap();
    legacy["campaign"]["strategy"]
        .as_object_mut()
        .unwrap()
        .remove("mirexis_path_id");
    let migrated = migrate_save_value(Some("1.23.0".to_owned()), legacy, &data).unwrap();

    assert_eq!(migrated.version, data.config.version);
    assert!(migrated.campaign.strategy.mirexis_path_id.is_empty());
}

#[test]
fn mirexis_path_save_gains_its_phase_five_operation() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign.strategy.phase_id = "mirexis".to_owned();
    campaign.strategy.escalation_complete = true;
    campaign.strategy.mirexis_path_id = "open_threshold".to_owned();
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let legacy = serde_json::to_value(session.to_save("1.24.0", &campaign)).unwrap();
    let migrated = migrate_save_value(Some("1.24.0".to_owned()), legacy, &data).unwrap();

    assert_eq!(migrated.version, data.config.version);
    assert!(migrated
        .campaign
        .strategy
        .mission_offers
        .iter()
        .any(|mission| mission.template_id == "mirexis_threshold_door_of_light"));
}

#[test]
fn phase_five_operation_save_gains_campaign_completion_gates() {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let mut legacy = serde_json::to_value(session.to_save("1.25.0", &campaign)).unwrap();
    let strategy = legacy["campaign"]["strategy"].as_object_mut().unwrap();
    strategy.remove("mirexis_operation_completed");
    strategy.remove("campaign_complete");
    let migrated = migrate_save_value(Some("1.25.0".to_owned()), legacy, &data).unwrap();

    assert_eq!(migrated.version, data.config.version);
    assert!(!migrated.campaign.strategy.mirexis_operation_completed);
    assert!(!migrated.campaign.strategy.campaign_complete);
}

#[test]
fn completed_campaign_save_gains_nadi_as_an_unevolved_reserve() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign
        .roster
        .retain(|character| character.id != "nadi_vale");
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let legacy = serde_json::to_value(session.to_save("1.26.0", &campaign)).unwrap();
    let migrated = migrate_save_value(Some("1.26.0".to_owned()), legacy, &data).unwrap();
    let nadi = migrated
        .campaign
        .roster
        .iter()
        .find(|character| character.id == "nadi_vale")
        .expect("Nadi joins an existing campaign as a reserve");

    assert_eq!(migrated.version, data.config.version);
    assert!(!nadi.deployment_selected);
    assert!(nadi.mutation_evolution_id.is_empty());
}

#[test]
fn nadi_roster_save_preserves_class_mastery_for_advanced_training() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    let mara = campaign
        .roster
        .iter_mut()
        .find(|character| character.id == "mara_venn")
        .unwrap();
    mara.level = 3;
    mara.class_history.push("soldier".to_owned());
    campaign.strategy.phase_id = "adaptation".to_owned();
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let legacy = serde_json::to_value(session.to_save("1.27.0", &campaign)).unwrap();
    let migrated = migrate_save_value(Some("1.27.0".to_owned()), legacy, &data).unwrap();
    let vanguard = data
        .classes
        .iter()
        .find(|class| class.id == "vanguard")
        .unwrap();

    assert_eq!(migrated.version, data.config.version);
    assert_eq!(
        migrated
            .campaign
            .class_training_lock_reason("mara_venn", vanguard),
        None
    );
}

#[test]
fn advanced_class_save_preserves_new_weapon_profiles() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    let kira = campaign
        .roster
        .iter_mut()
        .find(|character| character.id == "kira_voss")
        .unwrap();
    kira.equipment_ids.retain(|id| id != "frontier_rifle");
    kira.equipment_ids.push("needle_carbine".to_owned());
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let legacy = serde_json::to_value(session.to_save("1.28.0", &campaign)).unwrap();
    let migrated = migrate_save_value(Some("1.28.0".to_owned()), legacy, &data).unwrap();
    let kira = migrated
        .campaign
        .roster
        .iter()
        .find(|character| character.id == "kira_voss")
        .unwrap();

    assert_eq!(migrated.version, data.config.version);
    assert!(kira.equipment_ids.contains(&"needle_carbine".to_owned()));
}

#[test]
fn weapon_profile_save_gains_empty_relationship_history() {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let mut legacy = serde_json::to_value(session.to_save("1.29.0", &campaign)).unwrap();
    legacy["campaign"]
        .as_object_mut()
        .unwrap()
        .remove("relationships");
    let migrated = migrate_save_value(Some("1.29.0".to_owned()), legacy, &data).unwrap();

    assert_eq!(migrated.version, data.config.version);
    assert!(migrated.campaign.relationships.is_empty());
}

#[test]
fn relationship_save_gains_disarmed_tactical_overwatch() {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let mut legacy = serde_json::to_value(session.to_save("1.30.0", &campaign)).unwrap();
    for unit in legacy["tactical"]["units"].as_array_mut().unwrap() {
        unit.as_object_mut().unwrap().remove("overwatching");
    }
    let migrated = migrate_save_value(Some("1.30.0".to_owned()), legacy, &data).unwrap();

    assert_eq!(migrated.version, data.config.version);
    assert!(migrated
        .tactical
        .unwrap()
        .units
        .iter()
        .all(|unit| !unit.overwatching));
}

#[test]
fn overwatch_save_gains_empty_character_trauma_histories() {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let mut legacy = serde_json::to_value(session.to_save("1.31.0", &campaign)).unwrap();
    for character in legacy["campaign"]["roster"].as_array_mut().unwrap() {
        character.as_object_mut().unwrap().remove("traumas");
    }
    let migrated = migrate_save_value(Some("1.31.0".to_owned()), legacy, &data).unwrap();

    assert_eq!(migrated.version, data.config.version);
    assert!(migrated
        .campaign
        .roster
        .iter()
        .all(|character| character.traumas.is_empty()));
}

#[test]
fn trauma_save_gains_inactive_defense_asset_integrity() {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let mut legacy = serde_json::to_value(session.to_save("1.32.0", &campaign)).unwrap();
    let tactical = legacy["tactical"].as_object_mut().unwrap();
    tactical.remove("objective_integrity");
    tactical.remove("objective_max_integrity");
    let migrated = migrate_save_value(Some("1.32.0".to_owned()), legacy, &data).unwrap();
    let tactical = migrated.tactical.unwrap();

    assert_eq!(migrated.version, data.config.version);
    assert_eq!(tactical.objective_integrity, 0);
    assert_eq!(tactical.objective_max_integrity, 0);
}

#[test]
fn defense_objective_save_gains_hostile_factions_and_ready_abilities() {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let mut legacy = serde_json::to_value(session.to_save("1.33.0", &campaign)).unwrap();
    for unit in legacy["tactical"]["units"].as_array_mut().unwrap() {
        let unit = unit.as_object_mut().unwrap();
        unit.remove("faction");
        unit.remove("enemy_ability_used");
    }
    let migrated = migrate_save_value(Some("1.33.0".to_owned()), legacy, &data).unwrap();
    let tactical = migrated.tactical.unwrap();

    assert_eq!(migrated.version, data.config.version);
    assert!(tactical
        .units
        .iter()
        .filter(|unit| unit.team == crate::data::Team::Hostile)
        .all(|unit| unit.faction.is_some() && !unit.enemy_ability_used));
}

#[test]
fn faction_ability_save_gains_empty_realized_hazard_tiles() {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let mut legacy = serde_json::to_value(session.to_save("1.34.0", &campaign)).unwrap();
    legacy["tactical"]
        .as_object_mut()
        .unwrap()
        .remove("hazards");
    let migrated = migrate_save_value(Some("1.34.0".to_owned()), legacy, &data).unwrap();

    assert_eq!(migrated.version, data.config.version);
    assert!(migrated.tactical.unwrap().hazards.is_empty());
}

#[test]
fn hazard_save_keeps_realized_tiles_when_intent_preview_is_added() {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let mut session = GameSession::new(&data.config, &data.mission, &data.roster);
    session.tactical.hazards.push(crate::tactical::HazardTile {
        position: macroquad_toolkit::grid::TilePos::new(2, 2),
        kind: crate::data::HazardKind::StaticRift,
    });
    let legacy = serde_json::to_value(session.to_save("1.35.0", &campaign)).unwrap();
    let migrated = migrate_save_value(Some("1.35.0".to_owned()), legacy, &data).unwrap();

    assert_eq!(migrated.version, data.config.version);
    assert_eq!(migrated.tactical.unwrap().hazards, session.tactical.hazards);
}
