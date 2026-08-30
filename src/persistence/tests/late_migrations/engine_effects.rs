use crate::campaign::CampaignState;
use crate::data::{GameData, OperationModifier};
use crate::persistence::migrate_save_value;
use crate::state::{GameSession, SaveData};

#[test]
fn previous_path_save_refreshes_the_engine_effect_on_migration() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign.strategy.phase_id = "mirexis".to_owned();
    campaign.strategy.escalation_complete = true;
    campaign.strategy.mirexis_path_id = "open_threshold".to_owned();
    campaign.strategy.mirexis_operation_completed = true;
    campaign.strategy.campaign_complete = true;
    campaign.strategy.regenerate_missions(&data);
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let mut legacy = serde_json::to_value(session.to_save("1.95.0", &campaign)).unwrap();
    for offer in legacy["campaign"]["strategy"]["mission_offers"]
        .as_array_mut()
        .unwrap()
    {
        offer["operation_modifier"] = serde_json::json!("none");
    }

    let migrated = migrate_save_value(Some("1.95.0".to_owned()), legacy, &data).unwrap();
    let selected = migrated.campaign.strategy.selected_mission().unwrap();
    assert_eq!(selected.template_id, "epilogue_threshold_return");
    assert_eq!(
        selected.operation_modifier,
        OperationModifier::MirexisThreshold
    );
}

#[test]
fn previous_save_without_epilogue_work_defaults_to_zero() {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let save = SaveData::campaign_only("1.96.0", &campaign);
    let mut value = serde_json::to_value(save).unwrap();
    value["campaign"]["strategy"]
        .as_object_mut()
        .unwrap()
        .remove("post_campaign_operations_completed");

    let migrated = migrate_save_value(Some("1.96.0".to_owned()), value, &data).unwrap();
    assert_eq!(
        migrated
            .campaign
            .strategy
            .post_campaign_operations_completed,
        0
    );
}
