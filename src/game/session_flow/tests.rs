use super::*;
use crate::state::{BattleEvent, TacticalPhase};

#[test]
fn tactical_transition_clears_presentation_and_keeps_existing_audio_aligned() {
    let mut feedback = CombatFeedback::default();
    feedback.record(&[BattleEvent::UnitHealed {
        unit_id: "kira_voss".to_owned(),
        amount: 2,
        remaining: 12,
    }]);
    let mut replay = PhaseReplay::default();
    replay.start(&[BattleEvent::PhaseStarted {
        phase: TacticalPhase::Enemy,
        round: 2,
    }]);
    let mut observed_event_count = 99;
    let mut end_phase_armed = true;

    reset_tactical_presentation(
        &mut feedback,
        &mut replay,
        &mut observed_event_count,
        &mut end_phase_armed,
        17,
        false,
    );

    assert!(feedback.is_empty());
    assert!(!replay.is_active());
    assert_eq!(observed_event_count, 17);
    assert!(!end_phase_armed);
}

#[test]
fn fresh_tactical_transition_leaves_the_opening_phase_for_audio() {
    let mut feedback = CombatFeedback::default();
    let mut replay = PhaseReplay::default();
    let mut observed_event_count = 99;
    let mut end_phase_armed = true;

    reset_tactical_presentation(
        &mut feedback,
        &mut replay,
        &mut observed_event_count,
        &mut end_phase_armed,
        1,
        true,
    );

    assert_eq!(observed_event_count, 0);
    assert!(!end_phase_armed);
}
