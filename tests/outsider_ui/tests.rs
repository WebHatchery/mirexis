use super::*;

fn test_choice() -> OutsiderChoice {
    OutsiderChoice {
        id: "test_choice".to_owned(),
        label: "TEST CHOICE".to_owned(),
        description: "Test choice".to_owned(),
        materials_cost: 12,
        food_cost: 2,
        power_cost: 1,
        biomass_cost: 3,
        attention_change: 0,
        relationship_partner: "test_partner".to_owned(),
        disagreement: false,
        legacy_name: "Test Legacy".to_owned(),
        legacy_stat: "health".to_owned(),
        legacy_amount: 1,
    }
}

#[test]
fn choice_cost_label_names_missing_resources() {
    let resources = Resources {
        materials: 0,
        power: 0,
        food: 0,
        biomass: 0,
        alien_components: 0,
    };
    assert_eq!(
        cost_label(&resources, &test_choice()),
        "NEEDS 12 MAT // 2 FOOD // 1 POWER // 3 BIOMASS"
    );
}

#[test]
fn affordable_choice_cost_label_keeps_the_price() {
    let resources = Resources {
        materials: 12,
        power: 1,
        food: 2,
        biomass: 3,
        alien_components: 3,
    };
    assert_eq!(
        cost_label(&resources, &test_choice()),
        "12 MAT // 2 FOOD // 1 POWER // 3 BIOMASS"
    );
}
