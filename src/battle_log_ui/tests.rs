use super::*;
use crate::state::{BattleEvent, StatusKind, TacticalPhase};

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
fn filter_counts_advertise_the_events_available_in_each_category() {
    let data = crate::data::GameData::load().unwrap();
    let mut session = GameSession::new(&data.config, &data.mission, &data.roster);
    session.tactical.event_log = vec![
        attack_event(),
        BattleEvent::UnitMoved {
            unit_id: "mara_venn".to_owned(),
            path: Vec::new(),
            cost: 1,
        },
        BattleEvent::PhaseStarted {
            phase: TacticalPhase::Player,
            round: 1,
        },
    ];

    assert_eq!(filtered_event_count(&session, BattleLogFilter::All), 3);
    assert_eq!(filtered_event_count(&session, BattleLogFilter::Combat), 1);
    assert_eq!(filtered_event_count(&session, BattleLogFilter::Ground), 1);
    assert_eq!(filtered_event_count(&session, BattleLogFilter::System), 1);
}

#[test]
fn battle_log_groups_secondary_outcomes_and_names_their_event_kinds() {
    let recovery = BattleEvent::UnitHealed {
        unit_id: "kira_voss".to_owned(),
        amount: 2,
        remaining: 8,
    };
    let status = BattleEvent::StatusApplied {
        unit_id: "kira_voss".to_owned(),
        status: StatusKind::Guarded,
    };
    let ability = BattleEvent::EnemyAbilityActivated {
        unit_id: "brood_stalker_a".to_owned(),
        ability: "Predatory Surge".to_owned(),
    };
    let objective = BattleEvent::ObjectiveDamaged {
        amount: 2,
        remaining: 5,
    };
    let cover = BattleEvent::CoverDestroyed {
        position: macroquad_toolkit::grid::TilePos::new(5, 4),
    };

    assert_eq!(event_filter(&recovery), BattleLogFilter::Combat);
    assert_eq!(event_filter(&status), BattleLogFilter::Combat);
    assert_eq!(event_filter(&ability), BattleLogFilter::Combat);
    assert_eq!(event_filter(&objective), BattleLogFilter::Ground);
    assert_eq!(event_filter(&cover), BattleLogFilter::Ground);
    assert_eq!(event_kind(&recovery), "RECOVERY");
    assert_eq!(event_kind(&status), "STATUS");
    assert_eq!(event_kind(&ability), "ABILITY");
    assert_eq!(event_kind(&objective), "OBJECTIVE");
    assert_eq!(event_kind(&cover), "COVER");
}
