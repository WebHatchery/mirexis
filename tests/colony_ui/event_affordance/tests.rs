use super::*;

#[test]
fn event_buttons_name_the_food_gate_without_hiding_the_affordable_effect() {
    assert_eq!(
        button_label("Mara", 1, "armour", 2, 2),
        "CHOOSE MARA // +1 ARMOUR IN FUTURE BATTLES"
    );
    assert_eq!(
        button_label("Ilya", 1, "armour", 2, 1),
        "CHOOSE ILYA // NEED 2 FOOD"
    );
}
