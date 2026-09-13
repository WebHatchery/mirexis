use super::*;

#[test]
fn powered_research_annex_reduces_the_current_doctrine_cost() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign
        .colony
        .place_construction(BuildingKind::ResearchAnnex, [1, 1])
        .unwrap();
    campaign.colony.advance_operation();

    assert_eq!(campaign.research_material_cost(35), 30);
    campaign.colony.resources.materials = 30;
    assert!(campaign.can_complete_research("field_fortifications"));
    assert_eq!(
        campaign.complete_research("field_fortifications").unwrap(),
        "Field Fortifications"
    );
    assert_eq!(campaign.colony.resources.materials, 0);
    assert!(!campaign.can_complete_research("field_fortifications"));

    campaign
        .colony
        .buildings
        .iter_mut()
        .find(|building| building.kind == BuildingKind::ResearchAnnex)
        .unwrap()
        .damaged = true;
    assert_eq!(campaign.research_material_cost(30), 30);
}

#[test]
fn research_affordance_closes_when_materials_cannot_cover_the_effective_cost() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    let research = campaign
        .strategy
        .research
        .iter()
        .find(|research| !research.completed)
        .unwrap()
        .clone();
    let cost = campaign.research_material_cost(research.materials_cost);

    campaign.colony.resources.materials = cost - 1;
    assert!(!campaign.can_complete_research(&research.id));
    campaign.colony.resources.materials = cost;
    assert!(campaign.can_complete_research(&research.id));
    assert!(!campaign.can_complete_research("missing_research"));
}
