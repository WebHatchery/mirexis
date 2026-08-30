use super::*;
use crate::colony::{
    BuildingKind, EVOLUTION_CHAMBER_UPGRADE, HOT_CORE_UPGRADE, SIGNAL_CARTOGRAPHY_UPGRADE,
};
use crate::state::GameSession;

#[test]
fn legacy_save_without_facility_upgrade_fields_defaults_to_empty_state() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    let plant_id = campaign
        .colony
        .buildings
        .iter()
        .find(|building| building.kind == BuildingKind::PowerPlant)
        .unwrap()
        .id
        .clone();
    campaign
        .colony
        .queue_facility_upgrade(&plant_id, HOT_CORE_UPGRADE)
        .unwrap();
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let mut legacy = serde_json::to_value(session.to_save("1.77.0", &campaign)).unwrap();
    let current =
        migrate_save_value(Some(data.config.version.clone()), legacy.clone(), &data).unwrap();
    assert_eq!(
        current.campaign.colony.facility_upgrade_queue[0].upgrade_id,
        HOT_CORE_UPGRADE
    );
    assert_eq!(
        current.campaign.colony.facility_upgrade_queue[0].operations_remaining,
        1
    );
    let colony = legacy
        .get_mut("campaign")
        .and_then(serde_json::Value::as_object_mut)
        .and_then(|campaign| campaign.get_mut("colony"))
        .and_then(serde_json::Value::as_object_mut)
        .unwrap();
    colony.remove("facility_upgrade_queue");
    colony.remove("facility_upgrades");

    let migrated = migrate_save_value(Some("1.77.0".to_owned()), legacy, &data).unwrap();

    assert!(migrated.campaign.colony.facility_upgrade_queue.is_empty());
    assert!(migrated.campaign.colony.facility_upgrades.is_empty());
    assert_eq!(migrated.version, data.config.version);
}

#[test]
fn version_177_save_preserves_workshop_upgrade_queue() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    let workshop_id = campaign
        .colony
        .buildings
        .iter()
        .find(|building| building.kind == BuildingKind::Workshop)
        .unwrap()
        .id
        .clone();
    campaign
        .colony
        .queue_facility_upgrade(&workshop_id, crate::colony::PRECISION_BENCH_UPGRADE)
        .unwrap();
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let legacy = serde_json::to_value(session.to_save("1.77.0", &campaign)).unwrap();

    let migrated = migrate_save_value(Some("1.77.0".to_owned()), legacy, &data).unwrap();

    assert_eq!(migrated.version, data.config.version);
    assert_eq!(
        migrated.campaign.colony.facility_upgrade_queue[0].upgrade_id,
        crate::colony::PRECISION_BENCH_UPGRADE
    );
}

#[test]
fn version_101_save_preserves_gene_lab_upgrade_queue() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign.colony.ensure_gene_lab();
    campaign.colony.resources.power += 2;
    let gene_lab_id = campaign
        .colony
        .buildings
        .iter()
        .find(|building| building.kind == BuildingKind::GeneLab)
        .unwrap()
        .id
        .clone();
    campaign
        .colony
        .queue_facility_upgrade(&gene_lab_id, EVOLUTION_CHAMBER_UPGRADE)
        .unwrap();
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let legacy = serde_json::to_value(session.to_save("1.101.0", &campaign)).unwrap();

    let migrated = migrate_save_value(Some("1.101.0".to_owned()), legacy, &data).unwrap();

    assert_eq!(migrated.version, data.config.version);
    assert_eq!(
        migrated.campaign.colony.facility_upgrade_queue[0].upgrade_id,
        EVOLUTION_CHAMBER_UPGRADE
    );
}

#[test]
fn version_102_save_preserves_command_centre_upgrade_queue() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    let command_centre_id = campaign
        .colony
        .buildings
        .iter()
        .find(|building| building.kind == BuildingKind::CommandCentre)
        .unwrap()
        .id
        .clone();
    campaign
        .colony
        .queue_facility_upgrade(&command_centre_id, SIGNAL_CARTOGRAPHY_UPGRADE)
        .unwrap();
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let legacy = serde_json::to_value(session.to_save("1.102.0", &campaign)).unwrap();

    let migrated = migrate_save_value(Some("1.102.0".to_owned()), legacy, &data).unwrap();

    assert_eq!(migrated.version, data.config.version);
    assert_eq!(
        migrated.campaign.colony.facility_upgrade_queue[0].upgrade_id,
        SIGNAL_CARTOGRAPHY_UPGRADE
    );
}
