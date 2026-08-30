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
fn colony_guidance_targets_arrival_and_first_return_conversations() {
    let mut progress = FirstHourProgress::default();
    assert_eq!(progress.colony_guidance_target(), None);

    progress.advance_arrival();
    assert_eq!(progress.colony_guidance_target(), Some("mara_venn"));
    progress.acknowledge_colonist("mara_venn");
    progress.operation_resolved(1, true, 4);
    assert_eq!(progress.stage, FirstHourStage::FirstReturn);
    assert_eq!(progress.colony_guidance_target(), None);
    assert_eq!(
        progress.primary_goal(),
        "Tap RETURN TO COLONY, then tap Ilya Reed's marker."
    );
    progress.returned_to_colony();
    assert_eq!(progress.stage, FirstHourStage::FirstReturnColony);
    assert_eq!(progress.colony_guidance_target(), Some("ilya_reed"));
    assert_eq!(
        progress.primary_goal(),
        "Tap Ilya Reed's speech marker, then tap CONTINUE."
    );

    progress.guidance_enabled = false;
    assert_eq!(progress.colony_guidance_target(), None);
}

#[test]
fn tactical_lessons_follow_the_authored_order() {
    let mut progress = FirstHourProgress::default();
    progress.deployed(0);
    progress.attacked();
    assert_eq!(progress.lesson, TacticalLesson::Select);
    progress.selected();
    progress.moved(false);
    assert_eq!(progress.lesson, TacticalLesson::MoveToCover);
    progress.moved(true);
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
    assert_eq!(progress.stage, FirstHourStage::FirstReturnColony);
    progress.acknowledge_colonist("ilya_reed");
    assert_eq!(progress.stage, FirstHourStage::MakeInvestment);
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
    let progress = FirstHourProgress {
        investment_name: "survey_uplink".to_owned(),
        ..FirstHourProgress::default()
    };
    progress.apply_second_operation_bonus(0, &mut units);
    assert_eq!(units[0].accuracy, base);
    progress.apply_second_operation_bonus(1, &mut units);
    assert_eq!(units[0].accuracy, base + 8);
}

#[test]
fn second_deployment_enters_a_distinct_tactical_guidance_stage() {
    let mut progress = FirstHourProgress {
        stage: FirstHourStage::MakeInvestment,
        ..FirstHourProgress::default()
    };
    progress.invested("survey_uplink");
    assert_eq!(progress.stage, FirstHourStage::SecondOperation);

    progress.deployed(1);

    assert_eq!(progress.stage, FirstHourStage::SecondOperationTactical);
    assert_eq!(progress.lesson, TacticalLesson::ApplyLearning);
    assert_eq!(
        progress.primary_goal(),
        "Tap a remaining hostile, review the forecast, then tap ATTACK."
    );
}

#[test]
fn enemy_phase_prompt_explains_the_conditional_confirmation() {
    let progress = FirstHourProgress {
        stage: FirstHourStage::FirstOperation,
        lesson: TacticalLesson::EnemyPhase,
        ..FirstHourProgress::default()
    };

    assert_eq!(
        progress.primary_goal(),
        "Tap END PHASE; if a READY warning appears, tap CONFIRM END again to watch the hostile response."
    );
}

#[test]
fn missing_serialized_fields_use_safe_tutorial_defaults() {
    let progress: FirstHourProgress = serde_json::from_str("{}").unwrap();
    assert_eq!(progress, FirstHourProgress::default());
}
