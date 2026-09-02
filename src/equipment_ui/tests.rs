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
fn equipment_button_label_names_unavailable_states() {
    let mut unit = test_unit();
    unit.equipment_ids = vec!["field_medkit".to_owned()];

    assert_eq!(
        equipment_button_label("FIELD PATCH", None, None, false, false),
        "SELECT UNIT"
    );
    assert_eq!(
        equipment_button_label(
            "FIELD PATCH",
            Some(&unit),
            Some("field_medkit"),
            false,
            true,
        ),
        "FIELD PATCH"
    );
    unit.incapacitated = true;
    assert_eq!(
        equipment_button_label(
            "FIELD PATCH",
            Some(&unit),
            Some("field_medkit"),
            false,
            false,
        ),
        "INCAPACITATED"
    );
    unit.incapacitated = false;
    unit.action_points = 0;
    assert_eq!(
        equipment_button_label(
            "FIELD PATCH",
            Some(&unit),
            Some("field_medkit"),
            false,
            false,
        ),
        "NO AP"
    );
    unit.action_points = 2;
    assert_eq!(
        equipment_button_label(
            "FIELD PATCH",
            Some(&unit),
            Some("field_medkit"),
            false,
            false,
        ),
        "NO TARGET"
    );
    unit.used_equipment_ids.push("field_medkit".to_owned());
    assert_eq!(
        equipment_button_label("FIELD ITEM", Some(&unit), None, false, false),
        "SPENT"
    );
    unit.equipment_ids.clear();
    unit.used_equipment_ids.clear();
    assert_eq!(
        equipment_button_label("FIELD ITEM", Some(&unit), None, false, false),
        "NO FIELD ITEM"
    );
    assert_eq!(
        equipment_button_label(
            "FIELD PATCH",
            Some(&unit),
            Some("field_medkit"),
            true,
            false,
        ),
        "CANCEL"
    );
}

#[test]
fn equipment_action_is_enabled_for_cancellation_or_a_valid_target() {
    for (armed, has_valid_target, expected) in [
        (true, false, true),
        (true, true, true),
        (false, true, true),
        (false, false, false),
    ] {
        assert_eq!(
            action_button_enabled(armed, has_valid_target),
            expected,
            "armed={armed}, has_valid_target={has_valid_target}"
        );
    }
}
