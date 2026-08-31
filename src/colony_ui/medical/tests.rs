use super::*;

#[test]
fn treatment_button_names_the_first_recovery_blocker() {
    assert_eq!(
        treatment_button_label_for_state(false, true, 5, 5),
        "TREAT // NEED INFIRMARY"
    );
    assert_eq!(
        treatment_button_label_for_state(true, false, 5, 5),
        "TREAT // NO INJURY"
    );
    assert_eq!(
        treatment_button_label_for_state(true, true, 4, 5),
        "TREAT // NEED 5 BIO"
    );
    assert_eq!(
        treatment_button_label_for_state(true, true, 5, 5),
        "TREAT // 5 BIO"
    );
    assert_eq!(
        treatment_button_label_for_state(true, true, 3, 3),
        "TREAT // 3 BIO"
    );
}
