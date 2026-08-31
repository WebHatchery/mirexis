use super::*;

#[test]
fn version_215_save_gains_field_note_archive_defaults() {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let mut legacy = serde_json::to_value(session.to_save("1.215.0", &campaign)).unwrap();
    legacy["campaign"]["colony_story"]
        .as_object_mut()
        .unwrap()
        .remove("archived_notes");

    let migrated = migrate_save_value(Some("1.215.0".to_owned()), legacy, &data).unwrap();

    assert!(migrated.campaign.colony_story.archived_notes().is_empty());
    assert_eq!(migrated.version, data.config.version);
}
