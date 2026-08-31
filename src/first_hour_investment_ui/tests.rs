use super::*;

#[test]
fn active_summary_names_each_preparation_and_squad_effect() {
    assert_eq!(
        active_summary("bastion_mesh"),
        Some("PREP // BASTION MESH // SQUAD +1 ARM")
    );
    assert_eq!(
        active_summary("survey_uplink"),
        Some("PREP // SURVEY UPLINK // SQUAD +8 ACC")
    );
    assert_eq!(
        active_summary("rapid_injectors"),
        Some("PREP // RAPID INJECTORS // SQUAD +1 MOV")
    );
}

#[test]
fn active_summary_rejects_unknown_preparations() {
    assert_eq!(active_summary("unknown"), None);
}

#[test]
fn investment_buttons_name_the_material_gate() {
    assert_eq!(
        button_label("BASTION MESH", INVESTMENT_COST),
        "BASTION MESH // 24 MAT"
    );
    assert_eq!(
        button_label("SURVEY UPLINK", INVESTMENT_COST - 1),
        "SURVEY UPLINK // NEED 24 MAT"
    );
}

#[test]
fn tactical_summary_only_surfaces_the_active_second_operation_preparation() {
    let mut progress = FirstHourProgress {
        investment_name: "survey_uplink".to_owned(),
        ..FirstHourProgress::default()
    };
    assert_eq!(tactical_summary(&progress), None);

    progress.stage = FirstHourStage::SecondOperation;
    assert_eq!(tactical_summary(&progress), None);

    progress.stage = FirstHourStage::SecondOperationTactical;
    assert_eq!(
        tactical_summary(&progress),
        Some("PREP // SURVEY UPLINK // SQUAD +8 ACC")
    );

    progress.stage = FirstHourStage::SecondReturn;
    assert_eq!(tactical_summary(&progress), None);
}
