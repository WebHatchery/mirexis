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

#[test]
fn replay_pairs_attacks_with_their_immediate_field_consequences() {
    let events = vec![
        BattleEvent::AttackRolled {
            attacker_id: "brood_stalker_a".to_owned(),
            target_id: "kira_voss".to_owned(),
            roll: 4,
            hit_chance: 72,
        },
        BattleEvent::DamageApplied {
            target_id: "kira_voss".to_owned(),
            amount: 3,
            remaining: 6,
        },
        BattleEvent::UnitIncapacitated {
            unit_id: "kira_voss".to_owned(),
        },
        BattleEvent::ObjectiveDamaged {
            amount: 2,
            remaining: 5,
        },
        BattleEvent::PhaseStarted {
            phase: TacticalPhase::Player,
            round: 2,
        },
    ];
    let mut replay = PhaseReplay::default();
    replay.start(&events);

    assert_eq!(replay.beats.len(), 3);
    assert!(replay.beats[0].contains("ATTACK HIT"));
    assert!(replay.beats[0].contains("3 DMG · 6 REMAIN"));
    assert!(replay.beats[0].contains("INCAPACITATED"));
    assert!(replay.beats[1].contains("FIELD ASSET TOOK 2 DAMAGE"));
    assert!(replay.beats[2].contains("PLAYER PHASE"));
}

#[test]
fn skip_control_is_visible_inside_the_replay_overlay() {
    let overlay = Rect::new(18.0, 544.0, 820.0, 82.0);
    let skip = skip_button_bounds();
    assert!(skip.x >= overlay.x);
    assert!(skip.right() <= overlay.right());
    assert!(skip.y >= overlay.y);
    assert!(skip.bottom() <= overlay.bottom());
}
