use super::*;
use crate::campaign::CampaignState;
use crate::data::GameData;

fn test_unit() -> UnitState {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let roster = campaign.deployment_roster(&data, &data.mission);
    let session = crate::state::GameSession::new(&data.config, &data.mission, &roster);
    session.unit("kira_voss").unwrap().clone()
}

#[test]
fn class_action_button_label_names_unavailable_states() {
    let mut unit = test_unit();
    assert_eq!(
        class_action_button_label("STEADY AIM", &unit, false, false, true),
        "STEADY AIM"
    );

    unit.class_action_used = true;
    assert_eq!(
        class_action_button_label("STEADY AIM", &unit, false, false, false),
        "SPENT"
    );
    unit.class_action_used = false;
    unit.action_points = 0;
    assert_eq!(
        class_action_button_label("STEADY AIM", &unit, false, false, false),
        "NO AP"
    );
    unit.action_points = 2;
    unit.incapacitated = true;
    assert_eq!(
        class_action_button_label("STEADY AIM", &unit, false, false, false),
        "INCAPACITATED"
    );
    unit.incapacitated = false;
    assert_eq!(
        class_action_button_label("FIELD DRESSING", &unit, false, true, false),
        "NO TARGET"
    );
    assert_eq!(
        class_action_button_label("FIELD DRESSING", &unit, true, true, false),
        "CANCEL"
    );
}
