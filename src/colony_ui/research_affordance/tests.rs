use super::*;

#[test]
fn research_button_names_the_material_gate() {
    assert_eq!(
        button_label("Field Fortifications", 30, 30),
        "RESEARCH Field Fortifications // 30 MAT"
    );
    assert_eq!(
        button_label("Field Fortifications", 29, 30),
        "RESEARCH Field Fortifications // NEED 30 MAT"
    );
    assert_eq!(
        button_label("Salvage Doctrine", 5, 5),
        "RESEARCH Salvage Doctrine // 5 MAT"
    );
}
