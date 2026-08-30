use super::*;
use crate::data::GameData;
use crate::state::ObjectiveState;

fn session() -> GameSession {
    let data = GameData::load().unwrap();
    GameSession::new(&data.config, &data.mission, &data.roster)
}

#[test]
fn unfinished_tactical_saves_resume_in_battle() {
    assert_eq!(resume_state(&session()), AppState::Tactical);
}

#[test]
fn finished_tactical_saves_resume_in_the_debrief() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data.config, &data.mission, &data.roster);
    session.tactical.objective_state = ObjectiveState::Victory;

    assert_eq!(resume_state(&session), AppState::Debrief);
    assert_eq!(
        restored_outcome(&session, &data.mission)
            .expect("finished sessions restore a debrief outcome")
            .result,
        ObjectiveState::Victory
    );
}

#[test]
fn restoring_state_closes_the_persisted_first_hour_field_guide() {
    let mut progress = crate::first_hour::FirstHourProgress {
        help_open: true,
        ..crate::first_hour::FirstHourProgress::default()
    };

    clear_first_hour_help(&mut progress);

    assert!(!progress.help_open);
}
