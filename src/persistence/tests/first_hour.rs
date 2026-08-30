use super::*;
use crate::first_hour::{FirstHourProgress, FirstHourStage};
use crate::state::{GameSession, SaveData};

#[test]
fn previous_save_without_first_hour_fields_gains_safe_guidance() {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let save = SaveData::campaign_only("1.63.0", &campaign);
    let mut value = serde_json::to_value(save).unwrap();
    value
        .get_mut("campaign")
        .and_then(serde_json::Value::as_object_mut)
        .unwrap()
        .remove("first_hour");
    value["campaign"]["strategy"]
        .as_object_mut()
        .unwrap()
        .remove("post_campaign_operations_completed");

    let migrated = migrate_save_value(Some("1.63.0".to_owned()), value, &data).unwrap();

    assert_eq!(migrated.version, "1.120.0");
    assert_eq!(migrated.campaign.first_hour, FirstHourProgress::default());
    assert_eq!(migrated.campaign.first_hour.stage, FirstHourStage::Arrival);
    assert_eq!(
        migrated
            .campaign
            .strategy
            .post_campaign_operations_completed,
        0
    );
}

#[test]
fn previous_save_without_colony_story_fields_gains_an_empty_ledger() {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let save = SaveData::campaign_only("1.65.0", &campaign);
    let mut value = serde_json::to_value(save).unwrap();
    value
        .get_mut("campaign")
        .and_then(serde_json::Value::as_object_mut)
        .unwrap()
        .remove("colony_story");

    let migrated = migrate_save_value(Some("1.65.0".to_owned()), value, &data).unwrap();

    assert_eq!(migrated.version, "1.120.0");
    assert_eq!(
        migrated.campaign.colony_story,
        crate::colony_story::ColonyStoryState::default()
    );
}

#[test]
fn version_164_keeps_exact_onboarding_progress_and_defaults_metrics() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign.first_hour.stage = FirstHourStage::SecondReturn;
    campaign.first_hour.investment_name = "survey_uplink".to_owned();
    campaign.operations_completed = 2;
    let save = SaveData::campaign_only("1.64.0", &campaign);
    let mut value = serde_json::to_value(save).unwrap();
    value
        .get_mut("campaign")
        .and_then(serde_json::Value::as_object_mut)
        .and_then(|campaign| campaign.get_mut("first_hour"))
        .and_then(serde_json::Value::as_object_mut)
        .unwrap()
        .remove("metrics");

    let migrated = migrate_save_value(Some("1.64.0".to_owned()), value, &data).unwrap();

    assert_eq!(migrated.version, "1.120.0");
    assert_eq!(
        migrated.campaign.first_hour.stage,
        FirstHourStage::SecondReturn
    );
    assert_eq!(
        migrated.campaign.first_hour.investment_name,
        "survey_uplink"
    );
    assert_eq!(
        migrated.campaign.first_hour.metrics,
        crate::first_hour_metrics::FirstHourMetrics::default()
    );
}

#[test]
fn established_previous_campaign_does_not_restart_the_arrival_guide() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign.operations_completed = 5;
    let save = SaveData::campaign_only("1.63.0", &campaign);
    let mut value = serde_json::to_value(save).unwrap();
    value
        .get_mut("campaign")
        .and_then(serde_json::Value::as_object_mut)
        .unwrap()
        .remove("first_hour");

    let migrated = migrate_save_value(Some("1.63.0".to_owned()), value, &data).unwrap();

    assert_eq!(migrated.campaign.first_hour.stage, FirstHourStage::Complete);
    assert!(!migrated.campaign.first_hour.guidance_enabled);
}

#[test]
fn version_117_tactical_save_resumes_second_operation_guidance() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign.operations_completed = 1;
    campaign.first_hour.stage = FirstHourStage::SecondOperation;
    let mission = campaign
        .strategy
        .materialize_selected(&data, &campaign.colony);
    let session = GameSession::new(
        &data.config,
        &mission,
        &campaign.deployment_roster(&data, &mission),
    );
    let save = session.to_save("1.117.0", &campaign);
    let value = serde_json::to_value(save).unwrap();

    let migrated = migrate_save_value(Some("1.117.0".to_owned()), value, &data).unwrap();

    assert_eq!(
        migrated.campaign.first_hour.stage,
        FirstHourStage::SecondOperationTactical
    );
    assert_eq!(
        migrated.campaign.first_hour.visible_goal(),
        "Tap a remaining hostile, review the forecast, then tap ATTACK."
    );
}

#[test]
fn version_117_campaign_only_save_keeps_second_operation_preparation() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign.operations_completed = 1;
    campaign.first_hour.stage = FirstHourStage::SecondOperation;
    let save = SaveData::campaign_only("1.117.0", &campaign);
    let value = serde_json::to_value(save).unwrap();

    let migrated = migrate_save_value(Some("1.117.0".to_owned()), value, &data).unwrap();

    assert_eq!(
        migrated.campaign.first_hour.stage,
        FirstHourStage::SecondOperation
    );
}
