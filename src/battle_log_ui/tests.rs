use super::*;
use crate::state::{BattleEvent, TacticalPhase};

fn attack_event() -> BattleEvent {
    BattleEvent::AttackRolled {
        attacker_id: "mara_venn".to_owned(),
        target_id: "brood_stalker_a".to_owned(),
        roll: 42,
        hit_chance: 75,
    }
}

#[test]
fn battle_log_filters_group_events_by_player_meaning() {
    assert_eq!(event_filter(&attack_event()), BattleLogFilter::Combat);
    assert_eq!(
        event_filter(&BattleEvent::UnitMoved {
            unit_id: "mara_venn".to_owned(),
            path: Vec::new(),
            cost: 1,
        }),
        BattleLogFilter::Ground
    );
    assert_eq!(
        event_filter(&BattleEvent::PhaseStarted {
            phase: TacticalPhase::Player,
            round: 1,
        }),
        BattleLogFilter::System
    );
}

#[test]
fn all_filter_keeps_every_event_and_specific_filters_reject_other_groups() {
    let attack = attack_event();
    let phase = BattleEvent::PhaseStarted {
        phase: TacticalPhase::Enemy,
        round: 2,
    };

    assert!(filter_matches(BattleLogFilter::All, &attack));
    assert!(filter_matches(BattleLogFilter::Combat, &attack));
    assert!(!filter_matches(BattleLogFilter::Ground, &attack));
    assert!(filter_matches(BattleLogFilter::System, &phase));
}

#[test]
fn a_new_battle_history_view_starts_unfiltered() {
    assert_eq!(BattleLogFilter::default(), BattleLogFilter::All);
}
