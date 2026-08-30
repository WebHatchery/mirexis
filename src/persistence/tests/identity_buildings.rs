use super::*;
use crate::colony::BuildingKind;
use crate::state::GameSession;

#[test]
fn version_178_save_gains_the_chosen_identity_building() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign.strategy.mirexis_path_id = "human_redoubt".to_owned();
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let legacy = serde_json::to_value(session.to_save("1.78.0", &campaign)).unwrap();

    let migrated = migrate_save_value(Some("1.78.0".to_owned()), legacy, &data).unwrap();

    assert_eq!(migrated.version, data.config.version);
    assert_eq!(
        migrated
            .campaign
            .colony
            .buildings
            .iter()
            .filter(|building| building.kind.is_identity())
            .count(),
        1
    );
    assert!(migrated
        .campaign
        .colony
        .buildings
        .iter()
        .any(|building| building.kind == BuildingKind::RedoubtArsenal));
}
