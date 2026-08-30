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
