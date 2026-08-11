use super::*;
use crate::state::TacticalPhase;

#[test]
fn replay_keeps_important_events_bounded_and_advances_by_time() {
    let mut events = (0..10)
        .map(|index| BattleEvent::UnitMoved {
            unit_id: format!("hostile_{}", index),
            path: Vec::new(),
            cost: 1,
        })
        .collect::<Vec<_>>();
    events.push(BattleEvent::PhaseStarted {
        phase: TacticalPhase::Player,
        round: 2,
    });
    let mut replay = PhaseReplay::default();
    replay.start(&events);

    assert_eq!(replay.beats.len(), MAX_BEATS);
    assert!(replay.beats.last().unwrap().contains("PLAYER PHASE"));
    assert!(replay.is_active());
    replay.update(BEAT_SECONDS + 0.01);
    assert_eq!(replay.current, 1);
    replay.clear();
    assert!(!replay.is_active());
}
