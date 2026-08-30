use super::*;
use crate::campaign::CampaignState;
use crate::data::GameData;
use crate::first_hour::FirstHourStage;
use crate::formation::FormationKind;
use crate::state::GameSession;
use macroquad_toolkit::grid::TilePos;

#[test]
fn first_hour_teaching_lane_is_attackable_after_the_cover_move() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign.first_hour.stage = FirstHourStage::FirstOperation;
    let mission = campaign
        .strategy
        .materialize_selected(&data, &campaign.colony);
    let mut roster = campaign.deployment_roster(&data, &mission);
    crate::formation::apply(&mut roster, &mission, &data.config, FormationKind::Wedge);
    let mut session = GameSession::new(&data.config, &mission, &roster);

    prepare_first_hour_tactical_session(&mut session, &mission);

    let cover_position = TilePos::new(FIRST_HOUR_COVER_POSITION[0], FIRST_HOUR_COVER_POSITION[1]);
    assert!(session.can_move_selected_to(cover_position));
    assert!(session.move_selected_to(cover_position));
    let target = session
        .tactical
        .units
        .iter()
        .find(|unit| unit.team == crate::data::Team::Hostile)
        .unwrap();
    assert_eq!(target.position, first_hour_attack_position());
    assert!(session.can_attack_selected(&target.id));
    assert_eq!(
        session.tactical.objective_tile,
        first_hour_objective_position()
    );
    let objective_approach = TilePos::new(
        session.tactical.objective_tile.x - 1,
        session.tactical.objective_tile.y,
    );
    let route = session
        .movement_path("kira_voss", objective_approach)
        .unwrap();
    assert_eq!(
        crate::tactical::path_cost(&route, &session.tactical.terrain_costs),
        15
    );
}

#[test]
fn first_hour_ability_step_reselects_a_colonist_with_a_visible_action() {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let mission = campaign
        .strategy
        .materialize_selected(&data, &campaign.colony);
    let mut session = GameSession::new(&data.config, &mission, &data.roster);
    let ilya_position = session.unit("ilya_reed").unwrap().position;
    session.tactical.selected_unit = Some("ilya_reed".to_owned());
    session.tactical.selected_tile = ilya_position;

    prepare_first_hour_ability_selection(&mut session);

    assert_eq!(session.tactical.selected_unit.as_deref(), Some("kira_voss"));
    assert!(session.can_activate_selected_mutation());
    assert_eq!(
        session.tactical.selected_tile,
        session.unit("kira_voss").unwrap().position
    );
}

#[test]
fn first_hour_help_closes_other_tactical_modal_layers() {
    assert_eq!(
        first_hour_help_overlay_state(false, true, true),
        (true, false, false)
    );
    assert_eq!(
        first_hour_help_overlay_state(true, true, true),
        (false, false, false)
    );
}

#[test]
fn next_ready_selection_advances_the_first_hour_select_lesson() {
    let data = crate::data::GameData::load().unwrap();
    let campaign = crate::campaign::CampaignState::new(&data);
    let mission = campaign
        .strategy
        .materialize_selected(&data, &campaign.colony);
    let mut session = crate::state::GameSession::new(&data.config, &mission, &data.roster);
    let mut progress = crate::first_hour::FirstHourProgress {
        stage: FirstHourStage::FirstOperation,
        lesson: crate::first_hour::TacticalLesson::Select,
        ..crate::first_hour::FirstHourProgress::default()
    };
    let mut targeting = Some(TacticalTargeting::Equipment {
        unit_id: "kira_voss".to_owned(),
        equipment_id: "field_medkit".to_owned(),
    });

    assert!(advance_first_hour_selection(
        &mut session,
        &mut progress,
        &mut targeting
    ));
    assert!(targeting.is_none());
    assert_eq!(
        progress.lesson,
        crate::first_hour::TacticalLesson::MoveToCover
    );
}

#[test]
fn first_hour_guide_actions_keep_tactical_save_context_when_needed() {
    assert!(first_hour_action_needs_current_save(AppState::Tactical));
    assert!(first_hour_action_needs_current_save(AppState::Debrief));
    assert!(!first_hour_action_needs_current_save(AppState::Colony));
    assert!(!first_hour_action_needs_current_save(
        AppState::MissionBriefing
    ));
}
