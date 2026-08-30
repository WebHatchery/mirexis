use super::*;

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
