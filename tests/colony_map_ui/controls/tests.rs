use super::*;

#[test]
fn build_buttons_name_material_and_unique_project_gates() {
    assert_eq!(
        build_button_label(BuildingKind::Barricade, false, 20, false, false),
        "BARRICADE // 20 MAT"
    );
    assert_eq!(
        build_button_label(BuildingKind::Barricade, false, 19, false, false),
        "BARRICADE // NEED 20 MAT"
    );
    assert_eq!(
        build_button_label(BuildingKind::ResearchAnnex, false, 60, true, false),
        "RESEARCH ANNEX // ALREADY BUILT"
    );
    assert_eq!(
        build_button_label(BuildingKind::PowerPlant, false, 45, false, false),
        "POWER PLANT // 45 MAT"
    );
    assert_eq!(
        build_button_label(BuildingKind::SalvageYard, true, 55, false, true),
        "> SALVAGE YARD // PROJECT QUEUED"
    );
}
