use super::*;
use crate::data::GameData;

#[test]
fn a_new_campaign_keeps_every_doctrine_available() {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);

    assert!(has_pending(&campaign));
    assert_eq!(
        campaign
            .strategy
            .research
            .iter()
            .filter(|research| !research.completed)
            .count(),
        3
    );
}

#[test]
fn research_rows_are_separated_and_fit_the_operations_panel() {
    let first = research_row_bounds(0);
    let second = research_row_bounds(1);
    let third = research_row_bounds(2);

    assert!(second.y >= first.bottom() + 4.0);
    assert!(third.y >= second.bottom() + 4.0);
    assert!(third.bottom() <= 644.0);
}

#[test]
fn completing_one_doctrine_leaves_other_choices_open() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign.strategy.research[0].completed = true;

    assert!(has_pending(&campaign));
    assert_eq!(
        campaign
            .strategy
            .research
            .iter()
            .filter(|research| !research.completed)
            .count(),
        2
    );
}
