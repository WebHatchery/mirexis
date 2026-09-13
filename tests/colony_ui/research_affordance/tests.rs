use super::*;

#[test]
fn compact_choice_label_keeps_the_effect_line_clear() {
    assert_eq!(choice_button_label(40, 40), "COMPLETE // 40 MAT");
    assert_eq!(choice_button_label(39, 40), "NEED 40 MAT");
}
