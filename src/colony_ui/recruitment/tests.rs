use super::*;

#[test]
fn recruit_button_names_the_route_resource_gate() {
    assert_eq!(
        recruit_button_label("Veya Orn", 30, "materials", 30),
        "RECRUIT VEYA ORN // 30 MATERIALS"
    );
    assert_eq!(
        recruit_button_label("Veya Orn", 30, "materials", 29),
        "RECRUIT VEYA ORN // NEED 30 MATERIALS"
    );
    assert_eq!(
        recruit_button_label("Sedge", 8, "biomass", 7),
        "RECRUIT SEDGE // NEED 8 BIOMASS"
    );
}
