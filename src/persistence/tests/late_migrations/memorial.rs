use super::*;

#[test]
fn version_219_save_gains_lost_objective_defaults() {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let mut legacy = serde_json::to_value(session.to_save("1.219.0", &campaign)).unwrap();
    legacy["campaign"]
        .as_object_mut()
        .unwrap()
        .remove("lost_objectives");

    let migrated = migrate_save_value(Some("1.219.0".to_owned()), legacy, &data).unwrap();

    assert!(migrated.campaign.lost_objectives.is_empty());
    assert_eq!(migrated.version, data.config.version);
}
