use super::*;

#[test]
fn recruitment_button_label_names_the_resource_gate() {
    assert_eq!(
        recruitment_button_label(30, "materials", 30),
        "RECRUIT // 30 MATERIALS"
    );
    assert_eq!(
        recruitment_button_label(30, "power", 29),
        "RECRUIT // NEED 30 POWER"
    );
}
