use super::*;

#[test]
fn colony_map_input_is_enabled_only_without_blocking_overlays() {
    assert!(colony_map_input_enabled(
        false, false, false, false, false, false
    ));
    assert!(!colony_map_input_enabled(
        true, false, false, false, false, false
    ));
    assert!(!colony_map_input_enabled(
        false, true, false, false, false, false
    ));
    assert!(!colony_map_input_enabled(
        false, false, true, false, false, false
    ));
    assert!(!colony_map_input_enabled(
        false, false, false, true, false, false
    ));
    assert!(!colony_map_input_enabled(
        false, false, false, false, true, false
    ));
    assert!(!colony_map_input_enabled(
        false, false, false, false, false, true
    ));
}
