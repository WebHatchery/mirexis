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

#[test]
fn contextual_button_labels_name_treatment_and_gene_lab_blockers() {
    assert_eq!(
        treatment_button_label_for_state(false, 12, 5),
        "TREAT // NEED INFIRMARY"
    );
    assert_eq!(
        treatment_button_label_for_state(true, 4, 5),
        "TREAT // NEED 5 BIO"
    );
    assert_eq!(
        gene_lab_button_label_for_state(false, false, None),
        "GENE LAB // NEED ADAPTATION"
    );
    assert_eq!(
        gene_lab_button_label_for_state(true, true, None),
        "GENE LAB // PROJECT QUEUED"
    );
    assert_eq!(
        gene_lab_button_label_for_state(true, false, Some(GeneLabState::Damaged)),
        "GENE LAB // REPAIR FIRST"
    );
    assert_eq!(
        gene_lab_button_label_for_state(true, false, Some(GeneLabState::Unpowered)),
        "GENE LAB // NEED POWER"
    );
    assert_eq!(
        gene_lab_button_label_for_state(true, false, Some(GeneLabState::Powered)),
        "ENTER GENE LAB"
    );
    assert_eq!(
        gene_lab_button_label_for_state(true, false, None),
        "GENE LAB // BUILD FIRST"
    );
}
