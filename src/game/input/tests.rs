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
