use super::*;

#[test]
fn settings_overlay_consumes_physical_input_except_close() {
    assert_eq!(settings_overlay_input(true, false, false), None);
    assert_eq!(
        settings_overlay_input(true, true, false),
        Some(UiAction::ToggleSettings)
    );
    assert_eq!(
        settings_overlay_input(true, false, true),
        Some(UiAction::ToggleSettings)
    );
}

#[test]
fn settings_input_does_not_intercept_the_normal_game() {
    assert_eq!(settings_overlay_input(false, true, true), None);
}

#[test]
fn first_hour_help_overlay_consumes_physical_input_except_close() {
    assert_eq!(first_hour_help_overlay_input(true, false, false), None);
    assert_eq!(
        first_hour_help_overlay_input(true, true, false),
        Some(UiAction::ToggleFirstHourHelp)
    );
    assert_eq!(
        first_hour_help_overlay_input(true, false, true),
        Some(UiAction::ToggleFirstHourHelp)
    );
}

#[test]
fn first_hour_help_input_does_not_intercept_the_normal_game() {
    assert_eq!(first_hour_help_overlay_input(false, true, true), None);
}

#[test]
fn colony_modal_consumes_physical_input_except_close() {
    assert_eq!(colony_modal_input(true, false, false, false), None);
    assert_eq!(
        colony_modal_input(true, false, true, false),
        Some(UiAction::CloseFacilityUpgrade)
    );
    assert_eq!(
        colony_modal_input(false, true, false, true),
        Some(UiAction::CloseSalvage)
    );
}

#[test]
fn colony_modal_input_does_not_intercept_the_normal_colony() {
    assert_eq!(colony_modal_input(false, false, true, true), None);
}

#[test]
fn invalid_controller_target_does_not_cancel_targeting() {
    let data = crate::data::GameData::load().unwrap();
    let campaign = crate::campaign::CampaignState::new(&data);
    let roster = campaign.deployment_roster(&data, &data.mission);
    let session = crate::state::GameSession::new(&data.config, &data.mission, &roster);
    let empty_tile = session
        .tactical
        .fog
        .iter_with_pos()
        .map(|(tile, _)| tile)
        .find(|tile| {
            !session
                .tactical
                .units
                .iter()
                .any(|unit| unit.position == *tile)
        })
        .unwrap();
    let targeting = crate::game::TacticalTargeting::Equipment {
        unit_id: "ilya_reed".to_owned(),
        equipment_id: "field_medkit".to_owned(),
    };

    assert_eq!(
        targeted_confirm_action(&session, &targeting, empty_tile),
        None
    );
}

#[test]
fn valid_controller_target_keeps_the_equipment_intent() {
    let data = crate::data::GameData::load().unwrap();
    let campaign = crate::campaign::CampaignState::new(&data);
    let roster = campaign.deployment_roster(&data, &data.mission);
    let mut session = crate::state::GameSession::new(&data.config, &data.mission, &roster);
    let kira_position = session.unit("kira_voss").unwrap().position;
    session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == "brood_stalker_a")
        .unwrap()
        .position = macroquad_toolkit::grid::TilePos::new(kira_position.x + 1, kira_position.y);
    let target_tile = session.unit("brood_stalker_a").unwrap().position;
    let targeting = crate::game::TacticalTargeting::Equipment {
        unit_id: "kira_voss".to_owned(),
        equipment_id: "survey_harness".to_owned(),
    };

    assert_eq!(
        targeted_confirm_action(&session, &targeting, target_tile),
        Some(UiAction::UseEquipmentOn("brood_stalker_a".to_owned()))
    );
}
