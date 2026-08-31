use super::*;

#[test]
fn relay_button_names_the_first_scan_blocker() {
    assert_eq!(
        relay_button_label(false, false, 2, true),
        "RELAY // 2 PWR +4 ATT"
    );
    assert_eq!(
        relay_button_label(false, false, 1, true),
        "RELAY // NEED 2 PWR"
    );
    assert_eq!(
        relay_button_label(false, false, 2, false),
        "RELAY // NO SIGNAL"
    );
    assert_eq!(relay_button_label(false, true, 2, true), "RELAY // SCANNED");
    assert_eq!(
        relay_button_label(true, true, 2, true),
        "RELAY // ROUTE CLOSED"
    );
}
