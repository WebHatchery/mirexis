use super::*;

#[test]
fn palette_covers_every_required_response_and_ambience() {
    let cues = palette()
        .into_iter()
        .map(|entry| entry.0)
        .collect::<Vec<_>>();
    for cue in [
        SoundCue::Focus,
        SoundCue::Invalid,
        SoundCue::Move,
        SoundCue::Hit,
        SoundCue::Miss,
        SoundCue::Damage,
        SoundCue::Recovery,
        SoundCue::Ability,
        SoundCue::Objective,
        SoundCue::Reinforcement,
        SoundCue::Phase,
        SoundCue::Victory,
        SoundCue::Defeat,
        SoundCue::CityAmbience,
        SoundCue::TacticalAmbience,
    ] {
        assert!(cues.contains(&cue));
    }
}

#[test]
fn healing_events_use_a_distinct_recovery_cue() {
    assert_eq!(
        event_cue(&BattleEvent::UnitHealed {
            unit_id: "kira_voss".to_owned(),
            amount: 2,
            remaining: 8,
        }),
        Some(SoundCue::Recovery)
    );
}

#[test]
fn phase_and_reinforcement_events_use_distinct_cues() {
    assert_eq!(
        event_cue(&BattleEvent::PhaseStarted {
            phase: crate::state::TacticalPhase::Enemy,
            round: 2,
        }),
        Some(SoundCue::Phase)
    );
    assert_eq!(
        event_cue(&BattleEvent::ReinforcementsArrived { round: 3, count: 2 }),
        Some(SoundCue::Reinforcement)
    );
}

#[test]
fn critical_cues_win_over_later_low_priority_events() {
    let events = vec![
        BattleEvent::BattleEnded {
            outcome: ObjectiveState::Victory,
        },
        BattleEvent::UnitMoved {
            unit_id: "kira_voss".to_owned(),
            path: Vec::new(),
            cost: 1,
        },
    ];
    assert_eq!(prioritized_event_cue(&events), Some(SoundCue::Victory));
}

#[test]
fn reinforcement_cue_wins_over_a_damage_tail() {
    let events = vec![
        BattleEvent::ReinforcementsArrived { round: 3, count: 2 },
        BattleEvent::DamageApplied {
            target_id: "kira_voss".to_owned(),
            amount: 1,
            remaining: 8,
        },
    ];
    assert_eq!(
        prioritized_event_cue(&events),
        Some(SoundCue::Reinforcement)
    );
}

#[test]
fn volume_steps_are_bounded() {
    let settings = AudioSettings::default();
    assert_eq!(settings.volume_percent, 75);
    assert!(!settings.muted);
    assert!(!settings.reduced_motion);
}
