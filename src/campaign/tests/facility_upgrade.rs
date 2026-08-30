use super::*;
use crate::colony::{
    BuildingKind, BuildingState, COMMUNITY_KITCHEN_UPGRADE, DRONE_BAY_UPGRADE,
    PRECISION_BENCH_UPGRADE,
};

#[test]
fn community_kitchen_makes_the_commons_meal_more_affordable() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign.colony.buildings.push(BuildingState {
        id: "commons_test".to_owned(),
        kind: BuildingKind::Commons,
        position: [2, 10],
        level: 1,
        damaged: false,
    });
    let hydroponics_id = campaign
        .colony
        .buildings
        .iter()
        .find(|building| building.kind == BuildingKind::Hydroponics)
        .unwrap()
        .id
        .clone();
    campaign
        .colony
        .queue_facility_upgrade(&hydroponics_id, COMMUNITY_KITCHEN_UPGRADE)
        .unwrap();
    campaign.colony.advance_operation();

    let food_before = campaign.colony.resources.food;
    assert_eq!(campaign.commons_meal_food_cost(), 2);
    campaign.host_commons_meal().unwrap();
    assert_eq!(campaign.colony.resources.food, food_before - 2);
}

#[test]
fn hot_core_adds_attention_to_the_most_visible_faction_after_an_operation() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    let plant_id = campaign
        .colony
        .buildings
        .iter()
        .find(|building| building.kind == crate::colony::BuildingKind::PowerPlant)
        .unwrap()
        .id
        .clone();
    campaign
        .colony
        .queue_facility_upgrade(&plant_id, crate::colony::HOT_CORE_UPGRADE)
        .unwrap();
    campaign.colony.advance_operation();

    let highest_id = campaign
        .strategy
        .factions
        .iter()
        .max_by_key(|faction| (faction.attention, faction.id.clone()))
        .unwrap()
        .id
        .clone();
    let before = campaign
        .strategy
        .factions
        .iter()
        .map(|faction| (faction.id.clone(), faction.attention))
        .collect::<Vec<_>>();
    let mission = campaign.strategy.selected_mission().unwrap().clone();
    let mission_faction = mission.faction_id.clone();
    let outcome = MissionOutcome {
        result: ObjectiveState::Victory,
        colonists_deployed: 0,
        colonists_incapacitated: Vec::new(),
        hostiles_neutralised: 0,
        materials_awarded: 0,
        biomass_awarded: 0,
        power_awarded: 0,
    };

    campaign.apply_mission_outcome(&outcome, &mission, &data);

    for faction in &campaign.strategy.factions {
        let initial = before
            .iter()
            .find(|(id, _)| id == &faction.id)
            .map(|(_, attention)| *attention)
            .unwrap();
        let expected = initial
            + i32::from(faction.id == highest_id)
            + i32::from(faction.id == mission_faction) * 8;
        assert_eq!(faction.attention, expected.min(100));
    }
}

#[test]
fn precision_bench_reduces_weapon_fabrication_cost() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    let workshop_id = campaign
        .colony
        .buildings
        .iter()
        .find(|building| building.kind == BuildingKind::Workshop)
        .unwrap()
        .id
        .clone();
    campaign
        .colony
        .queue_facility_upgrade(&workshop_id, PRECISION_BENCH_UPGRADE)
        .unwrap();
    campaign.colony.advance_operation();

    campaign.colony.resources.materials = 30;
    let cost = campaign
        .craft_equipment("kira_voss", "service_pistol", &data)
        .unwrap();
    assert_eq!(cost, 25);
    assert_eq!(campaign.colony.resources.materials, 5);
}

#[test]
fn drone_bay_reduces_repairs_while_the_workshop_is_online() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    let workshop_id = campaign
        .colony
        .buildings
        .iter()
        .find(|building| building.kind == BuildingKind::Workshop)
        .unwrap()
        .id
        .clone();
    campaign
        .colony
        .queue_facility_upgrade(&workshop_id, DRONE_BAY_UPGRADE)
        .unwrap();
    campaign.colony.advance_operation();
    campaign
        .colony
        .buildings
        .iter_mut()
        .find(|building| building.kind == BuildingKind::Hydroponics)
        .unwrap()
        .damaged = true;
    campaign.colony.resources.materials = 20;

    let (_, cost) = campaign.colony.repair_building("hydroponics").unwrap();
    assert_eq!(cost, 15);
    assert!(campaign
        .colony
        .buildings
        .iter()
        .find(|building| building.kind == BuildingKind::Hydroponics)
        .is_some_and(|building| !building.damaged));
}
