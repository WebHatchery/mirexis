use super::*;

#[test]
fn upgrade_buttons_name_the_first_blocker_and_exact_cost() {
    assert_eq!(
        upgrade_button_state(2, false, true, false, 55, 55),
        ("LEVEL 2 ACTIVE".to_owned(), false)
    );
    assert_eq!(
        upgrade_button_state(1, true, true, false, 55, 55),
        ("REPAIR FIRST".to_owned(), false)
    );
    assert_eq!(
        upgrade_button_state(1, false, true, true, 55, 55),
        ("PROJECT QUEUED".to_owned(), false)
    );
    assert_eq!(
        upgrade_button_state(1, false, false, false, 55, 55),
        ("NEED POWER".to_owned(), false)
    );
    assert_eq!(
        upgrade_button_state(1, false, true, false, 54, 55),
        ("NEED 55 MAT".to_owned(), false)
    );
    assert_eq!(
        upgrade_button_state(1, false, true, false, 55, 55),
        ("QUEUE // 55 MAT".to_owned(), true)
    );
}
