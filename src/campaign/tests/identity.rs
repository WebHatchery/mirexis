use super::*;
use crate::colony::BuildingKind;

fn campaign_with_identity(data: &GameData, path_id: &str) -> CampaignState {
    let mut campaign = CampaignState::new(data);
    campaign.strategy.campaign_complete = true;
    campaign.strategy.mirexis_path_id = path_id.to_owned();
    campaign.colony.ensure_identity_building(path_id).unwrap();
    if path_id == "open_threshold" {
        campaign.colony.resources.power += 6;
    }
    campaign
}

#[test]
fn each_identity_building_can_trade_its_resource_for_less_faction_pressure() {
    let data = GameData::load().unwrap();
    for (path_id, building, faction_id, resource) in [
        (
            "human_redoubt",
            BuildingKind::RedoubtArsenal,
            "directorate",
            "materials",
        ),
        (
            "living_commonwealth",
            BuildingKind::ChoirGarden,
            "brood",
            "biomass",
        ),
        (
            "open_threshold",
            BuildingKind::ThresholdSpire,
            "ascendants",
            "power",
        ),
    ] {
        let mut campaign = campaign_with_identity(&data, path_id);
        let materials = campaign.colony.resources.materials;
        let biomass = campaign.colony.resources.biomass;
        let power = campaign.colony.resources.power;
        let attention = campaign
            .strategy
            .factions
            .iter()
            .find(|faction| faction.id == faction_id)
            .unwrap()
            .attention;

        assert!(campaign.identity_stewardship_available());
        let summary = campaign.run_identity_stewardship().unwrap();

        assert!(summary.contains("ATTENTION -5"));
        assert_eq!(campaign.identity_stewardship_completed, 1);
        assert_eq!(campaign.identity_stewardship_operation, Some(0));
        assert!(!campaign.identity_stewardship_available());
        assert_eq!(
            campaign
                .strategy
                .factions
                .iter()
                .find(|faction| faction.id == faction_id)
                .unwrap()
                .attention,
            (attention - 5).max(0)
        );
        match resource {
            "materials" => assert_eq!(campaign.colony.resources.materials, materials - 4),
            "biomass" => assert_eq!(campaign.colony.resources.biomass, biomass - 2),
            "power" => assert_eq!(campaign.colony.resources.power, power - 2),
            _ => unreachable!(),
        }
        assert_eq!(
            campaign.identity_stewardship_definition().unwrap().building,
            building
        );
    }
}

#[test]
fn identity_stewardship_is_available_again_after_the_next_operation() {
    let data = GameData::load().unwrap();
    let mut campaign = campaign_with_identity(&data, "open_threshold");
    campaign.run_identity_stewardship().unwrap();
    campaign.operations_completed = 1;

    assert!(campaign.identity_stewardship_available());
    campaign.run_identity_stewardship().unwrap();
    assert_eq!(campaign.identity_stewardship_completed, 2);
    assert_eq!(campaign.identity_stewardship_operation, Some(1));
}

#[test]
fn damaged_identity_buildings_cannot_be_stewarded() {
    let data = GameData::load().unwrap();
    let mut campaign = campaign_with_identity(&data, "human_redoubt");
    campaign
        .colony
        .buildings
        .iter_mut()
        .find(|building| building.kind == BuildingKind::RedoubtArsenal)
        .unwrap()
        .damaged = true;

    assert!(!campaign.identity_stewardship_available());
    assert!(campaign.run_identity_stewardship().is_err());
}
