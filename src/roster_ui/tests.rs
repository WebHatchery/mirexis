use super::*;

#[test]
fn full_recruited_roster_stays_above_relationships() {
    let (default_step, default_height) = character_list_row_layout(5);
    assert_eq!((default_step, default_height), (60.0, 54.0));

    let (step, height) = character_list_row_layout(7);
    let last_row_bottom = 150.0 + 6.0 * step + height;
    assert!(step < default_step);
    assert!(last_row_bottom <= 524.0);
}
