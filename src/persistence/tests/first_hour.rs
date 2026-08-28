use super::*;
use crate::first_hour::{FirstHourProgress, FirstHourStage};
use crate::state::SaveData;

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

    let migrated = migrate_save_value(Some("1.63.0".to_owned()), value, &data).unwrap();

    assert_eq!(migrated.version, "1.64.0");
    assert_eq!(migrated.campaign.first_hour, FirstHourProgress::default());
    assert_eq!(migrated.campaign.first_hour.stage, FirstHourStage::Arrival);
}
