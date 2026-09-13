//! Focused late_migrations integration tests.

use super::*;

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
        .filter(|unit| unit.team == mirexis::data::Team::Hostile)
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
    session
        .tactical
        .hazards
        .push(mirexis::tactical::HazardTile {
            position: macroquad_toolkit::grid::TilePos::new(2, 2),
            kind: mirexis::data::HazardKind::StaticRift,
        });
    let legacy = serde_json::to_value(session.to_save("1.35.0", &campaign)).unwrap();
    let migrated = migrate_save_value(Some("1.35.0".to_owned()), legacy, &data).unwrap();

    assert_eq!(migrated.version, data.config.version);
    assert_eq!(migrated.tactical.unwrap().hazards, session.tactical.hazards);
}
