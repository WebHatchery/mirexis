use super::*;

#[test]
fn armed_equipment_can_always_be_cancelled() {
    assert!(action_button_enabled(true, false));
    assert!(action_button_enabled(true, true));
}

#[test]
fn unarmed_equipment_still_requires_a_valid_target() {
    assert!(action_button_enabled(false, true));
    assert!(!action_button_enabled(false, false));
}
