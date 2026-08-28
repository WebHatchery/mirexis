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
    progress.operation_resolved(1, false);
    assert_eq!(progress.stage, FirstHourStage::FirstReturn);
    progress.returned_to_colony();
    progress.invested("priority treatment");
    progress.operation_resolved(2, false);
    assert_eq!(progress.stage, FirstHourStage::Promise);
    assert_eq!(progress.second_outcome_won, Some(false));
}

#[test]
fn missing_serialized_fields_use_safe_tutorial_defaults() {
    let progress: FirstHourProgress = serde_json::from_str("{}").unwrap();
    assert_eq!(progress, FirstHourProgress::default());
}
