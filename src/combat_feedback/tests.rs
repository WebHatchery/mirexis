use super::*;
use crate::state::StatusKind;

#[test]
fn feedback_callouts_stack_clear_of_the_unit_effect_lane() {
    let rect = Rect::new(100.0, 200.0, 48.0, 48.0);

    assert_eq!(callout_y(rect, 0), 142.0);
    assert_eq!(callout_y(rect, 1), 124.0);
    assert!(callout_y(rect, 0) < rect.y - 37.0);
    assert_eq!(callout_y(Rect::new(100.0, 50.0, 48.0, 48.0), 4), 24.0);
}

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

#[test]
fn feedback_marks_missed_attacks_at_the_target_with_an_attacker_fallback() {
    let mut feedback = CombatFeedback::default();
    feedback.record_for(
        &[BattleEvent::AttackRolled {
            attacker_id: "brood_stalker_a".to_owned(),
            target_id: "kira_voss".to_owned(),
            roll: 96,
            hit_chance: 72,
        }],
        0.8,
    );

    assert_eq!(feedback.callouts.len(), 1);
    assert!(matches!(
        &feedback.callouts[0].anchor,
        FeedbackAnchor::Unit {
            unit_id,
            fallback_unit_id: Some(fallback),
        } if unit_id == "kira_voss" && fallback == "brood_stalker_a"
    ));
    assert_eq!(feedback.callouts[0].label, "MISS");
    assert_eq!(feedback.callouts[0].tone, FeedbackTone::Miss);
    feedback.update(0.81);
    assert!(feedback.callouts.is_empty());
}

#[test]
fn feedback_names_successful_hits_incapacitations_and_completion() {
    let mut feedback = CombatFeedback::default();
    feedback.record_for(
        &[
            BattleEvent::AttackRolled {
                attacker_id: "brood_stalker_a".to_owned(),
                target_id: "kira_voss".to_owned(),
                roll: 4,
                hit_chance: 72,
            },
            BattleEvent::DamageApplied {
                target_id: "kira_voss".to_owned(),
                amount: 3,
                remaining: 0,
            },
            BattleEvent::UnitIncapacitated {
                unit_id: "kira_voss".to_owned(),
            },
            BattleEvent::ObjectiveSecured {
                unit_id: "mara_venn".to_owned(),
            },
            BattleEvent::ExtractionCompleted {
                unit_id: "ilya_reed".to_owned(),
            },
            BattleEvent::ObjectiveDamaged {
                amount: 2,
                remaining: 5,
            },
            BattleEvent::ObjectiveDestroyed,
            BattleEvent::CoverDamaged {
                position: macroquad_toolkit::grid::TilePos::new(5, 4),
                amount: 3,
                remaining: 4,
            },
            BattleEvent::CoverDestroyed {
                position: macroquad_toolkit::grid::TilePos::new(5, 4),
            },
        ],
        0.8,
    );

    assert_eq!(
        feedback
            .callouts
            .iter()
            .map(|callout| callout.label.as_str())
            .collect::<Vec<_>>(),
        [
            "HIT",
            "-3",
            "INCAPACITATED",
            "OBJECTIVE SECURED",
            "EXTRACTED",
            "OBJECTIVE -2",
            "OBJECTIVE DESTROYED",
            "COVER -3",
            "COVER DESTROYED"
        ]
    );
    assert_eq!(feedback.callouts[0].tone, FeedbackTone::Hit);
    assert_eq!(feedback.impacts.len(), 1);
    assert!(matches!(
        &feedback.callouts[5].anchor,
        FeedbackAnchor::Objective
    ));
    assert!(matches!(
        &feedback.callouts[7].anchor,
        FeedbackAnchor::Tile(position) if *position == macroquad_toolkit::grid::TilePos::new(5, 4)
    ));
}
