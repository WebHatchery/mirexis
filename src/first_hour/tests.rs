use super::*;

#[test]
fn opening_advances_only_after_mara_is_acknowledged() {
    let mut progress = FirstHourProgress::default();
    progress.advance_arrival();
    assert_eq!(progress.stage, FirstHourStage::MeetCoordinator);
    progress.acknowledge_colonist("ilya_reed");
    assert_eq!(progress.stage, FirstHourStage::MeetCoordinator);
    progress.acknowledge_colonist("mara_venn");
    assert_eq!(progress.stage, FirstHourStage::PrepareFirstOperation);
}

#[test]
fn tactical_lessons_follow_the_authored_order() {
    let mut progress = FirstHourProgress::default();
    progress.deployed(0);
    progress.attacked();
    assert_eq!(progress.lesson, TacticalLesson::Select);
    progress.selected();
    progress.moved();
    progress.attacked();
    progress.ended_phase();
    progress.touched_objective();
    progress.used_ability();
    assert_eq!(progress.lesson, TacticalLesson::ApplyLearning);
}

#[test]
fn defeat_still_reaches_return_and_promise_beats() {
    let mut progress = FirstHourProgress::default();
    progress.operation_resolved(1, false, 5);
    assert_eq!(progress.stage, FirstHourStage::FirstReturn);
    progress.returned_to_colony();
    progress.invested("priority treatment");
    progress.operation_resolved(2, false, 6);
    assert_eq!(progress.stage, FirstHourStage::SecondReturn);
    progress.returned_to_colony();
    assert_eq!(progress.stage, FirstHourStage::Promise);
    assert_eq!(progress.second_outcome_won, Some(false));
    assert_eq!(progress.metrics.operation_one_rounds, Some(5));
    assert_eq!(progress.metrics.operation_two_rounds, Some(6));
}

#[test]
fn first_investment_changes_only_the_second_deployment() {
    let data = crate::data::GameData::load().unwrap();
    let campaign = crate::campaign::CampaignState::new(&data);
    let mission = campaign
        .strategy
        .materialize_selected(&data, &campaign.colony);
    let mut units = campaign.deployment_roster(&data, &mission);
    let base = units[0].accuracy;
    let mut progress = FirstHourProgress::default();
    progress.investment_name = "survey_uplink".to_owned();
    progress.apply_second_operation_bonus(0, &mut units);
    assert_eq!(units[0].accuracy, base);
    progress.apply_second_operation_bonus(1, &mut units);
    assert_eq!(units[0].accuracy, base + 8);
}

#[test]
fn missing_serialized_fields_use_safe_tutorial_defaults() {
    let progress: FirstHourProgress = serde_json::from_str("{}").unwrap();
    assert_eq!(progress, FirstHourProgress::default());
}
