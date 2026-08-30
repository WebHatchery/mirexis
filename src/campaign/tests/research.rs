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
    assert_eq!(
        campaign.complete_research("field_fortifications").unwrap(),
        "Field Fortifications"
    );
    assert_eq!(campaign.colony.resources.materials, 0);

    campaign
        .colony
        .buildings
        .iter_mut()
        .find(|building| building.kind == BuildingKind::ResearchAnnex)
        .unwrap()
        .damaged = true;
    assert_eq!(campaign.research_material_cost(30), 30);
}
