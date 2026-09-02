use super::*;

#[test]
fn deployment_button_label_names_selection_and_food_gates() {
    assert_eq!(deployment_button_label(0, 9, 0), "SELECT COLONISTS");
    assert_eq!(deployment_button_label(2, 1, 3), "NEED 3 FOOD");
    assert_eq!(deployment_button_label(2, 3, 3), "DEPLOY SQUAD · 3 FOOD");
}

#[test]
fn full_recruited_roster_stays_above_briefing_actions() {
    let (default_step, default_height) = deployment_row_layout(5);
    assert_eq!((default_step, default_height), (32.0, 29.0));

    let (step, height) = deployment_row_layout(7);
    let last_row_bottom = 384.0 + 6.0 * step + height;
    assert!(step < default_step);
    assert!(last_row_bottom <= 578.0);
}
