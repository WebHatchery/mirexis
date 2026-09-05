use super::*;

#[test]
fn overlays_only_translate_close_input_while_open() {
    type OverlayInput = fn(bool, bool, bool) -> Option<UiAction>;
    let overlays: [(&str, OverlayInput, UiAction); 3] = [
        ("settings", settings_overlay_input, UiAction::ToggleSettings),
        (
            "field notes",
            field_notes_overlay_input,
            UiAction::ToggleFieldNotes,
        ),
        (
            "first-hour help",
            first_hour_help_overlay_input,
            UiAction::ToggleFirstHourHelp,
        ),
    ];

    for (name, translate, close_action) in overlays {
        assert_eq!(translate(true, false, false), None, "{name}: no input");
        assert_eq!(
            translate(true, true, false),
            Some(close_action.clone()),
            "{name}: escape"
        );
        assert_eq!(
            translate(true, false, true),
            Some(close_action),
            "{name}: controller cancel"
        );
        assert_eq!(translate(false, true, true), None, "{name}: closed overlay");
    }
}

#[test]
fn colony_modals_only_translate_close_input_while_open() {
    assert_eq!(colony_modal_input(true, false, false, false), None);
    assert_eq!(
        colony_modal_input(true, false, true, false),
        Some(UiAction::CloseFacilityUpgrade)
    );
    assert_eq!(
        colony_modal_input(false, true, false, true),
        Some(UiAction::CloseSalvage)
    );
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
