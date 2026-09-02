use super::*;
use crate::data::GameData;

#[test]
fn pending_research_tracks_incomplete_doctrines_until_all_are_finished() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);

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

    for research in &mut campaign.strategy.research {
        research.completed = true;
    }
    assert!(!has_pending(&campaign));
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
