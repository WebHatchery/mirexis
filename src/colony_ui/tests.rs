use super::*;

#[test]
fn completed_campaign_drops_the_obsolete_plan_line() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);

    assert!(should_draw_colony_plan(&campaign));

    campaign.strategy.campaign_complete = true;

    assert!(!should_draw_colony_plan(&campaign));
}

#[test]
fn event_card_stays_below_the_first_hour_briefing_focus() {
    let briefing = mission_briefing_bounds();
    let event = character_event_card_bounds();

    assert!(event.y >= briefing.y + briefing.h + 4.0);
    assert!(event.y + event.h <= 682.0);
}
