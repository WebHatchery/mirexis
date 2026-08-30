use super::*;
use crate::colony::BuildingKind;

#[test]
fn version_174_save_migrates_to_the_watchtower_schema() {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let legacy = serde_json::to_value(session.to_save("1.74.0", &campaign)).unwrap();

    let migrated = migrate_save_value(Some("1.74.0".to_owned()), legacy, &data).unwrap();

    assert_eq!(migrated.version, data.config.version);
    assert_eq!(
        migrated.campaign.colony.planned_construction,
        BuildingKind::Barricade
    );
}
