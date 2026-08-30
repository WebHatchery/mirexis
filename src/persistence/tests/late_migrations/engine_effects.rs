use crate::campaign::CampaignState;
use crate::data::{GameData, OperationModifier};
use crate::persistence::migrate_save_value;
use crate::state::GameSession;

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
