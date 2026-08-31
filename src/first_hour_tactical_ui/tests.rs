use super::*;
use crate::data::{GameData, Team};
use crate::first_hour::{FirstHourStage, TacticalLesson};
use macroquad_toolkit::grid::TilePos;

fn session() -> GameSession {
    let data = GameData::load().unwrap();
    GameSession::new(&data.config, &data.mission, &data.roster)
}

fn progress(lesson: TacticalLesson) -> FirstHourProgress {
    FirstHourProgress {
        stage: FirstHourStage::FirstOperation,
        lesson,
        ..FirstHourProgress::default()
    }
}

#[test]
fn tactical_focus_is_gated_to_guided_tactical_stages() {
    let session = session();
    let mut guided = progress(TacticalLesson::Select);
    assert!(map_focus_tile(&guided, &session).is_some());

    guided.guidance_enabled = false;
    assert_eq!(map_focus_tile(&guided, &session), None);

    guided.guidance_enabled = true;
    guided.stage = FirstHourStage::FirstBriefing;
    assert_eq!(map_focus_tile(&guided, &session), None);

    guided.stage = FirstHourStage::SecondOperationTactical;
    guided.lesson = TacticalLesson::ApplyLearning;
    assert!(map_focus_tile(&guided, &session).is_some());
}

#[test]
fn tactical_focus_tracks_cover_objective_and_hostile_targets() {
    let mut session = session();
    let selected = session.selected_unit().unwrap().position;
    session
        .tactical
        .cover_edges
        .push(crate::data::CoverEdgeDef {
            position: [6, 18],
            direction: crate::data::EdgeDirection::North,
            strength: 25,
        });

    let move_target = map_focus_tile(&progress(TacticalLesson::MoveToCover), &session).unwrap();
    assert_ne!(move_target, selected);
    assert!(session.can_move_selected_to(move_target));
    assert!(crate::cover_rules::is_cover_position(
        &session.tactical.cover_edges,
        move_target
    ));

    assert_eq!(
        map_focus_tile(&progress(TacticalLesson::Objective), &session),
        Some(session.tactical.objective_tile)
    );

    let attack_target = map_focus_tile(&progress(TacticalLesson::Attack), &session).unwrap();
    assert!(session
        .tactical
        .units
        .iter()
        .any(|unit| unit.team == Team::Hostile && unit.position == attack_target));
}

#[test]
fn ability_focus_chooses_the_first_visible_action_that_can_resolve() {
    let mut session = session();
    assert_eq!(
        ability_focus_slot(&session),
        Some(AbilityFocusSlot::Mutation)
    );

    session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == "kira_voss")
        .unwrap()
        .mutation_gift_used = true;
    session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == "kira_voss")
        .unwrap()
        .class_id = "scout".to_owned();
    assert_eq!(
        ability_focus_slot(&session),
        Some(AbilityFocusSlot::ClassAction)
    );
}

#[test]
fn apply_learning_focus_points_to_a_remaining_hostile() {
    let mut session = session();
    let selected = session.selected_unit().unwrap().position;
    let (hostile_id, hostile_tile) = {
        let hostile = session
            .tactical
            .units
            .iter_mut()
            .find(|unit| unit.team == Team::Hostile)
            .unwrap();
        hostile.position = TilePos::new(selected.x + 4, selected.y);
        (hostile.id.clone(), hostile.position)
    };

    assert_eq!(
        map_focus_tile(
            &FirstHourProgress {
                stage: FirstHourStage::SecondOperationTactical,
                lesson: TacticalLesson::ApplyLearning,
                ..FirstHourProgress::default()
            },
            &session,
        ),
        Some(hostile_tile)
    );
    assert!(session.can_attack_selected(&hostile_id));
}

#[test]
fn attack_focus_waits_for_a_valid_forecast() {
    let mut session = session();
    assert_eq!(attack_focus_rect(&session), None);

    let selected = session.selected_unit().unwrap().position;
    let hostile_tile = TilePos::new(selected.x + 4, selected.y);
    session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.team == Team::Hostile)
        .unwrap()
        .position = hostile_tile;
    session.tactical.selected_tile = hostile_tile;

    let focus = attack_focus_rect(&session).expect("valid forecast should focus its attack button");
    assert_eq!(focus, Rect::new(689.0, 610.0, 126.0, 40.0));
}

#[test]
fn objective_focus_waits_until_the_selected_colonist_can_interact() {
    let mut session = session();
    assert_eq!(objective_focus_rect(&session), None);

    let objective = session.tactical.objective_tile;
    let selected_id = session
        .tactical
        .selected_unit
        .clone()
        .expect("default session selects a colonist");
    session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == selected_id)
        .expect("selected colonist exists")
        .position = objective;

    let focus = objective_focus_rect(&session)
        .expect("an adjacent selected colonist should focus objective interaction");
    assert_eq!(focus, Rect::new(938.0, 450.0, 314.0, 34.0));
}

#[test]
fn replay_focus_targets_the_visible_skip_control() {
    assert_eq!(replay_focus_target(), Rect::new(650.0, 560.0, 168.0, 36.0));
    assert_eq!(
        replay_focus_target(),
        crate::phase_replay::skip_button_bounds()
    );
}
