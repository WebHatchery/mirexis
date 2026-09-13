use super::*;

#[test]
fn commons_button_names_the_first_meal_blocker() {
    assert_eq!(commons_button_label(false, 3, 8, 4), "HOST MEAL // 4 FOOD");
    assert_eq!(commons_button_label(false, 1, 8, 4), "MEAL // NEED 2 READY");
    assert_eq!(commons_button_label(false, 2, 3, 4), "MEAL // NEED 4 FOOD");
    assert_eq!(commons_button_label(true, 3, 8, 4), "MEAL // HOSTED");
}

#[test]
fn commons_button_uses_the_upgraded_food_cost() {
    assert_eq!(commons_button_label(false, 2, 2, 2), "HOST MEAL // 2 FOOD");
    assert_eq!(commons_button_label(false, 2, 1, 2), "MEAL // NEED 2 FOOD");
}
