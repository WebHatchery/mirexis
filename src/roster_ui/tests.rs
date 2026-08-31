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

#[test]
fn training_labels_name_the_blocking_prerequisite() {
    assert_eq!(
        class_training_label(false, None, false, true, "Scout", 80),
        "SCOUT · REQUIRES BARRACKS"
    );
    assert_eq!(
        class_training_label(false, None, true, false, "Scout", 80),
        "SCOUT · NEEDS 80 MAT"
    );
    assert_eq!(
        class_training_label(false, Some("REQUIRES LEVEL 3"), true, true, "Vanguard", 110,),
        "VANGUARD · REQUIRES LEVEL 3"
    );
}

#[test]
fn technique_labels_name_barracks_xp_and_slot_gates() {
    assert_eq!(
        technique_label(false, false, false, true, true, "Stabilise", 20),
        "REQUIRES BARRACKS: Stabilise"
    );
    assert_eq!(
        technique_label(false, false, true, false, true, "Stabilise", 20),
        "NEEDS 20 XP: Stabilise"
    );
    assert_eq!(
        technique_label(false, true, true, true, false, "Stabilise", 20),
        "SLOTS FULL: Stabilise"
    );
}

#[test]
fn equipment_labels_name_workshop_and_material_gates() {
    assert_eq!(
        equipment_label(true, false, false, false, true, "Field Medkit", 20),
        "REQUIRES WORKSHOP: Field Medkit"
    );
    assert_eq!(
        equipment_label(true, false, false, true, false, "Field Medkit", 20),
        "NEEDS 20 MAT: Field Medkit"
    );
    assert_eq!(
        equipment_label(false, false, false, true, true, "Exile Cipher", 20),
        "LOCKED: Exile Cipher"
    );
}

#[test]
fn equipment_rows_clear_the_roster_footer() {
    let first_row = equipment_row_rect(0);
    let last_row = equipment_row_rect(13);

    assert_eq!(first_row.y, 548.0);
    assert_eq!(last_row.y, 648.0);
    assert!(last_row.y + last_row.h <= 676.0);
    assert!(last_row.y + last_row.h < 707.0);
}
