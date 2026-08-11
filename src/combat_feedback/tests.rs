use super::*;
use crate::state::StatusKind;

#[test]
fn feedback_tracks_only_damage_healing_and_status_events_for_a_bounded_time() {
    let mut feedback = CombatFeedback::default();
    feedback.record(&[
        BattleEvent::DamageApplied {
            target_id: "kira_voss".to_owned(),
            amount: 3,
            remaining: 6,
        },
        BattleEvent::UnitHealed {
            unit_id: "kira_voss".to_owned(),
            amount: 2,
            remaining: 8,
        },
        BattleEvent::StatusApplied {
            unit_id: "kira_voss".to_owned(),
            status: StatusKind::Guarded,
        },
        BattleEvent::PhaseStarted {
            phase: crate::state::TacticalPhase::Player,
            round: 2,
        },
    ]);

    assert_eq!(feedback.callouts.len(), 3);
    assert_eq!(feedback.callouts[0].label, "-3");
    assert_eq!(feedback.callouts[1].label, "+2");
    assert_eq!(feedback.callouts[2].label, "GUARDED");
    feedback.update(1.6);
    assert!(feedback.callouts.is_empty());
}
