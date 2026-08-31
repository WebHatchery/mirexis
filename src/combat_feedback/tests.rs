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
    assert_eq!(feedback.callouts[0].unit_id, "kira_voss");
    assert_eq!(
        feedback.callouts[0].fallback_unit_id.as_deref(),
        Some("brood_stalker_a")
    );
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
            "EXTRACTED"
        ]
    );
    assert_eq!(feedback.callouts[0].tone, FeedbackTone::Hit);
    assert_eq!(feedback.impacts.len(), 1);
}
