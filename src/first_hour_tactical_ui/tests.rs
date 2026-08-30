use super::*;
use crate::data::{GameData, Team};
use crate::first_hour::{FirstHourStage, TacticalLesson};

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
fn tactical_focus_is_gated_to_guided_first_operation() {
    let session = session();
    let mut guided = progress(TacticalLesson::Select);
    assert!(map_focus_tile(&guided, &session).is_some());

    guided.guidance_enabled = false;
    assert_eq!(map_focus_tile(&guided, &session), None);

    guided.guidance_enabled = true;
    guided.stage = FirstHourStage::FirstBriefing;
    assert_eq!(map_focus_tile(&guided, &session), None);
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
