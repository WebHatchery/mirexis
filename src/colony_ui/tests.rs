use super::*;

#[test]
fn completed_campaign_drops_the_obsolete_plan_line() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);

    assert!(should_draw_colony_plan(&campaign));

    campaign.strategy.campaign_complete = true;

    assert!(!should_draw_colony_plan(&campaign));
}
