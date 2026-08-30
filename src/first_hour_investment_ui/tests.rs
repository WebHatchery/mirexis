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
