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
fn skill_button_label_names_unavailable_states() {
    let mut unit = test_unit();
    assert_eq!(
        skill_button_label("ARMOUR DRILL", &unit, "armour_drill", false, false, true),
        "ARMOUR DRILL"
    );

    unit.used_skill_ids.push("armour_drill".to_owned());
    assert_eq!(
        skill_button_label("ARMOUR DRILL", &unit, "armour_drill", false, false, false),
        "SPENT"
    );
    unit.used_skill_ids.clear();
    unit.action_points = 0;
    assert_eq!(
        skill_button_label("ARMOUR DRILL", &unit, "armour_drill", false, false, false),
        "NO AP"
    );
    unit.action_points = 2;
    unit.incapacitated = true;
    assert_eq!(
        skill_button_label("ARMOUR DRILL", &unit, "armour_drill", false, false, false),
        "INCAPACITATED"
    );
    unit.incapacitated = false;
    assert_eq!(
        skill_button_label(
            "CONTROLLED BURST",
            &unit,
            "controlled_burst",
            false,
            true,
            false
        ),
        "NO TARGET"
    );
    assert_eq!(
        skill_button_label("ARMOUR DRILL", &unit, "armour_drill", false, false, false),
        "UNAVAILABLE"
    );
    assert_eq!(
        skill_button_label(
            "CONTROLLED BURST",
            &unit,
            "controlled_burst",
            true,
            true,
            false
        ),
        "CANCEL"
    );
}
