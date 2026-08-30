use super::*;
use crate::campaign::{SalvageChoice, SALVAGE_MATERIALS_REWARD, SALVAGE_RESEARCH_INSIGHT};
use crate::colony::{BuildingKind, BuildingState};
use crate::state::MissionOutcome;

fn campaign_with_salvage_yard(data: &GameData) -> CampaignState {
    let mut campaign = CampaignState::new(data);
    campaign.colony.buildings.push(BuildingState {
        id: "salvage_test".to_owned(),
        kind: BuildingKind::SalvageYard,
        position: [2, 10],
        level: 1,
        damaged: false,
    });
    campaign
}

#[test]
fn victories_store_recovered_objects_for_the_salvage_yard() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    let mission = campaign.strategy.selected_mission().unwrap().clone();
    let outcome = MissionOutcome {
        result: ObjectiveState::Victory,
        colonists_deployed: 3,
        colonists_incapacitated: Vec::new(),
        hostiles_neutralised: 2,
        materials_awarded: 7,
        biomass_awarded: 0,
        power_awarded: 0,
    };

    campaign.apply_mission_outcome(&outcome, &mission, &data);

    assert_eq!(campaign.salvage_cache_count, 1);
    assert_eq!(campaign.operations_completed, 1);

    let failed = MissionOutcome {
        result: ObjectiveState::Failed,
        colonists_deployed: 3,
        colonists_incapacitated: Vec::new(),
        hostiles_neutralised: 0,
        materials_awarded: 0,
        biomass_awarded: 0,
        power_awarded: 0,
    };
    campaign.apply_mission_outcome(&failed, &mission, &data);
    assert_eq!(campaign.salvage_cache_count, 1);
}

#[test]
fn salvage_materials_are_disclosed_and_limited_to_once_per_operation() {
    let data = GameData::load().unwrap();
    let mut campaign = campaign_with_salvage_yard(&data);
    campaign.salvage_cache_count = 1;
    let materials = campaign.colony.resources.materials;

    assert!(campaign.salvage_available());
    assert_eq!(
        campaign.process_salvage(SalvageChoice::Materials).unwrap(),
        format!("SALVAGE SORTED // +{} MATERIALS", SALVAGE_MATERIALS_REWARD)
    );
    assert_eq!(
        campaign.colony.resources.materials,
        materials + SALVAGE_MATERIALS_REWARD
    );
    assert!(!campaign.salvage_available());
    assert!(campaign.process_salvage(SalvageChoice::Prototype).is_err());
}

#[test]
fn salvage_insight_discounts_and_is_consumed_by_the_next_doctrine() {
    let data = GameData::load().unwrap();
    let mut campaign = campaign_with_salvage_yard(&data);
    campaign.salvage_cache_count = 1;
    let base_cost = campaign.strategy.research[0].materials_cost;

    campaign
        .process_salvage(SalvageChoice::ResearchInsight)
        .unwrap();
    assert_eq!(
        campaign.research_material_cost(base_cost),
        base_cost - SALVAGE_RESEARCH_INSIGHT
    );
    let cost = campaign.research_material_cost(base_cost);
    campaign.colony.resources.materials = cost;
    campaign
        .complete_research(&campaign.strategy.research[0].id.clone())
        .unwrap();
    assert_eq!(campaign.research_insight, 0);
    assert_eq!(campaign.colony.resources.materials, 0);
}

#[test]
fn salvage_prototype_crafts_standard_equipment_without_materials() {
    let data = GameData::load().unwrap();
    let mut campaign = campaign_with_salvage_yard(&data);
    campaign.salvage_cache_count = 1;
    campaign.process_salvage(SalvageChoice::Prototype).unwrap();
    campaign.colony.resources.materials = 0;

    assert_eq!(
        campaign.craft_equipment("ilya_reed", "frontier_rifle", &data),
        Ok(0)
    );
    assert_eq!(campaign.salvage_prototypes, 0);
    assert_eq!(campaign.colony.resources.materials, 0);
}
