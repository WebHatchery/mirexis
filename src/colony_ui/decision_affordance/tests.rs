use super::*;

fn resources(materials: i32, biomass: i32, power: i32) -> Resources {
    Resources {
        materials,
        biomass,
        power,
        food: 0,
        alien_components: 0,
    }
}

#[test]
fn decision_buttons_name_resource_shortfalls_without_hiding_effects() {
    assert_eq!(
        button_label("Living Decoy", "8 BIO // ATTENTION -8", true, ""),
        "LIVING DECOY // 8 BIO // ATTENTION -8"
    );
    assert_eq!(
        button_label("Living Decoy", "8 BIO // ATTENTION -8", false, "8 BIO"),
        "LIVING DECOY // NEED 8 BIO"
    );
}

#[test]
fn decision_shortfalls_include_every_missing_resource() {
    assert_eq!(
        resource_shortfalls(&resources(29, 7, 3), 30, 8, 4),
        "30 MAT // 8 BIO // 4 PWR"
    );
    assert_eq!(resource_shortfalls(&resources(30, 8, 4), 30, 8, 4), "");
}
