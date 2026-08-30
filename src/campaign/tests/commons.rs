use super::*;
use crate::colony::{BuildingKind, BuildingState};

#[test]
fn commons_meal_turns_a_powered_city_space_into_bond_progress() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign.colony.buildings.push(BuildingState {
        id: "commons_test".to_owned(),
        kind: BuildingKind::Commons,
        position: [2, 10],
        level: 1,
        damaged: false,
    });
    let food_before = campaign.colony.resources.food;

    assert!(campaign.commons_meal_available());
    assert_eq!(
        campaign.host_commons_meal().unwrap(),
        "COMMONS MEAL // 3 colonists shared the table"
    );
    assert_eq!(campaign.colony.resources.food, food_before - 4);
    assert_eq!(campaign.commons_meals_hosted, 1);
    assert_eq!(campaign.commons_meal_operation, Some(0));
    assert!(!campaign.commons_meal_available());
    assert_eq!(campaign.relationships.len(), 3);
    assert!(campaign
        .relationships
        .iter()
        .all(|relationship| relationship.bond == 2 && relationship.shared_victories == 0));
}

#[test]
fn commons_meal_can_be_hosted_again_after_the_next_operation() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign.colony.buildings.push(BuildingState {
        id: "commons_test".to_owned(),
        kind: BuildingKind::Commons,
        position: [2, 10],
        level: 1,
        damaged: false,
    });
    campaign.host_commons_meal().unwrap();
    campaign.operations_completed = 1;

    assert!(campaign.commons_meal_available());
    campaign.host_commons_meal().unwrap();
    assert_eq!(campaign.commons_meals_hosted, 2);
    assert_eq!(campaign.commons_meal_operation, Some(1));
}
