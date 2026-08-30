use super::identity::identity_beat;
use super::*;

#[test]
fn each_colony_npc_has_progression_and_finale_beats() {
    for character_id in [
        "mara_venn",
        "ilya_reed",
        "sol_cairn",
        "nadi_vale",
        "veya_orn",
        "sedge",
    ] {
        let arrival = current_beat(character_id, 0, None, None).unwrap();
        let first = current_beat(character_id, 1, Some(true), None).unwrap();
        let second = current_beat(character_id, 2, Some(true), Some(true)).unwrap();
        assert_ne!(arrival.id, first.id);
        assert_ne!(first.id, second.id);
        assert!(!arrival.text.is_empty());
        assert!(!first.text.is_empty());
        assert!(!second.text.is_empty());
    }

    for path_id in ["human_redoubt", "living_commonwealth", "open_threshold"] {
        for character_id in [
            "kira_voss",
            "mara_venn",
            "ilya_reed",
            "sol_cairn",
            "nadi_vale",
            "veya_orn",
            "sedge",
        ] {
            let story = ColonyStoryState::default();
            let finale = finale_beat(path_id, character_id, true, &story).unwrap();
            assert!(finale.id.starts_with("finale_"));
            assert!(!finale.text.is_empty());

            let mut archived = story;
            archived.acknowledge(finale.id);
            assert!(finale_beat(path_id, character_id, true, &archived).is_none());
            assert!(
                finale_beat(path_id, character_id, false, &ColonyStoryState::default()).is_none()
            );
        }
    }
}

#[test]
fn operation_outcome_selects_a_distinct_aftermath() {
    let victory = current_beat("mara_venn", 1, Some(true), None).unwrap();
    let defeat = current_beat("mara_venn", 1, Some(false), None).unwrap();
    assert_eq!(victory.id, "mara_first_victory");
    assert_eq!(defeat.id, "mara_first_defeat");
}

#[test]
fn missing_legacy_outcome_uses_a_neutral_aftermath() {
    let first = current_beat("mara_venn", 1, None, None).unwrap();
    let second = current_beat("mara_venn", 2, Some(true), None).unwrap();
    assert_eq!(first.id, "mara_first_aftermath");
    assert_eq!(second.id, "mara_second_aftermath");
}

#[test]
fn acknowledgement_is_idempotent_and_preserves_order() {
    let mut story = ColonyStoryState::default();
    story.acknowledge("mara_arrival");
    story.acknowledge("mara_arrival");
    story.acknowledge("ilya_arrival");
    assert_eq!(
        story.heard_beats,
        &["mara_arrival".to_owned(), "ilya_arrival".to_owned()]
    );
    assert!(story.has_heard("mara_arrival"));
}

#[test]
fn missing_serialized_story_uses_an_empty_ledger() {
    let story: ColonyStoryState = serde_json::from_str("{}").unwrap();
    assert_eq!(story, ColonyStoryState::default());
}

#[test]
fn commons_meal_unlocks_an_authored_note_for_each_colony_voice() {
    for character_id in [
        "kira_voss",
        "mara_venn",
        "ilya_reed",
        "sol_cairn",
        "nadi_vale",
        "veya_orn",
        "sedge",
    ] {
        let beat = commons_meal_beat(character_id).unwrap();
        assert!(beat.id.starts_with("commons_meal_"));
        assert!(!beat.title.is_empty());
        assert!(!beat.text.is_empty());
    }
}

#[test]
fn unknown_colony_voice_has_no_commons_meal_note() {
    assert!(commons_meal_beat("unknown_colonist").is_none());
}

#[test]
fn phase_beats_give_each_middle_campaign_phase_a_specific_voice() {
    for (phase_id, character_id, beat_id) in [
        ("contact", "kira_voss", "phase_contact_kira"),
        ("adaptation", "mara_venn", "phase_adaptation_mara"),
        ("adaptation", "ilya_reed", "phase_adaptation_ilya"),
        ("adaptation", "sol_cairn", "phase_adaptation_sol"),
        ("adaptation", "nadi_vale", "phase_adaptation_nadi"),
        ("adaptation", "sedge", "phase_adaptation_sedge"),
        ("escalation", "mara_venn", "phase_escalation_mara"),
        ("escalation", "ilya_reed", "phase_escalation_ilya"),
        ("escalation", "sol_cairn", "phase_escalation_sol"),
        ("escalation", "nadi_vale", "phase_escalation_nadi"),
        ("escalation", "veya_orn", "phase_escalation_veya"),
        ("escalation", "sedge", "phase_escalation_sedge"),
    ] {
        let beat = phase_beat(phase_id, character_id).unwrap();
        assert_eq!(beat.id, beat_id);
        assert!(!beat.title.is_empty());
        assert!(!beat.text.is_empty());
    }
    assert!(phase_beat("contact", "ilya_reed").is_none());
    assert!(phase_beat("adaptation", "kira_voss").is_none());
    assert!(phase_beat("escalation", "kira_voss").is_none());
    assert!(phase_beat("isolation", "kira_voss").is_none());
    assert!(phase_beat("unknown_phase", "kira_voss").is_none());
}

#[test]
fn each_contact_route_has_a_witness_contradiction_and_aftermath() {
    for (protocol_id, characters, ids) in [
        (
            "directorate_requisition",
            ["kira_voss", "sol_cairn", "mara_venn"],
            [
                "contact_directorate_witness",
                "contact_directorate_contradiction",
                "contact_directorate_aftermath",
            ],
        ),
        (
            "brood_cultivation",
            ["nadi_vale", "ilya_reed", "mara_venn"],
            [
                "contact_brood_witness",
                "contact_brood_contradiction",
                "contact_brood_aftermath",
            ],
        ),
        (
            "ascendant_capacitor",
            ["sol_cairn", "kira_voss", "nadi_vale"],
            [
                "contact_ascendant_witness",
                "contact_ascendant_contradiction",
                "contact_ascendant_aftermath",
            ],
        ),
    ] {
        for (stage, (character_id, beat_id)) in characters.iter().zip(ids).enumerate() {
            let beat =
                contact_route_beat(protocol_id, character_id, stage >= 1, stage >= 2).unwrap();
            assert_eq!(beat.id, beat_id);
            assert!(!beat.text.is_empty());
        }
        assert!(contact_route_beat(protocol_id, "veya_orn", false, false).is_none());
    }
}

#[test]
fn each_identity_building_has_one_path_specific_colony_voice() {
    for (path_id, character_id, beat_id) in [
        ("human_redoubt", "mara_venn", "identity_redoubt_arsenal"),
        ("living_commonwealth", "nadi_vale", "identity_choir_garden"),
        ("open_threshold", "sol_cairn", "identity_threshold_spire"),
    ] {
        assert_eq!(identity_npc(path_id), Some(character_id));
        let beat = identity_beat(path_id, character_id, false).unwrap();
        assert_eq!(beat.id, beat_id);
        assert!(!beat.title.is_empty());
        assert!(!beat.text.is_empty());
        let ending = identity_beat(path_id, character_id, true).unwrap();
        assert!(ending.id.ends_with("_ending"));
        assert_ne!(ending.id, beat.id);
        assert_ne!(ending.text, beat.text);
        assert!(identity_beat(path_id, "ilya_reed", false)
            .is_none_or(|candidate| candidate.id != beat_id));
    }
    assert!(identity_npc("unknown_path").is_none());
    assert!(identity_beat("unknown_path", "mara_venn", false).is_none());
}

#[test]
fn identity_building_failures_open_a_repair_and_power_recovery_arc() {
    for (path_id, character_id, damage_id, repair_id, power_failure_id, power_restored_id) in [
        (
            "human_redoubt",
            "mara_venn",
            "civic_redoubt_damage",
            "civic_redoubt_repair",
            "civic_redoubt_power_failure",
            "civic_redoubt_power_restored",
        ),
        (
            "living_commonwealth",
            "nadi_vale",
            "civic_commonwealth_damage",
            "civic_commonwealth_repair",
            "civic_commonwealth_power_failure",
            "civic_commonwealth_power_restored",
        ),
        (
            "open_threshold",
            "sol_cairn",
            "civic_threshold_damage",
            "civic_threshold_repair",
            "civic_threshold_power_failure",
            "civic_threshold_power_restored",
        ),
    ] {
        let mut story = ColonyStoryState::default();
        let identity = identity_arc_beat(
            path_id,
            character_id,
            false,
            0,
            IdentityArcProgress::default(),
            &story,
            None,
        )
        .expect("identity establishment note");
        story.acknowledge(identity.id);

        let damaged = identity_arc_beat(
            path_id,
            character_id,
            false,
            0,
            IdentityArcProgress::default(),
            &story,
            Some(IdentityBuildingState {
                damaged: true,
                powered: true,
            }),
        )
        .expect("damaged identity note");
        assert_eq!(damaged.id, damage_id);
        story.acknowledge(damaged.id);
        let repaired = identity_arc_beat(
            path_id,
            character_id,
            false,
            0,
            IdentityArcProgress::default(),
            &story,
            Some(IdentityBuildingState {
                damaged: false,
                powered: true,
            }),
        )
        .expect("repair identity note");
        assert_eq!(repaired.id, repair_id);
        story.acknowledge(repaired.id);
        assert!(identity_arc_beat(
            path_id,
            character_id,
            false,
            0,
            IdentityArcProgress::default(),
            &story,
            Some(IdentityBuildingState {
                damaged: false,
                powered: true,
            }),
        )
        .is_none());

        let power_failure = identity_arc_beat(
            path_id,
            character_id,
            false,
            0,
            IdentityArcProgress::default(),
            &story,
            Some(IdentityBuildingState {
                damaged: false,
                powered: false,
            }),
        )
        .expect("power failure identity note");
        assert_eq!(power_failure.id, power_failure_id);
        story.acknowledge(power_failure.id);
        let power_restored = identity_arc_beat(
            path_id,
            character_id,
            false,
            0,
            IdentityArcProgress::default(),
            &story,
            Some(IdentityBuildingState {
                damaged: false,
                powered: true,
            }),
        )
        .expect("power restoration identity note");
        assert_eq!(power_restored.id, power_restored_id);
    }
}

#[test]
fn identity_preparation_unlocks_one_path_specific_reflection() {
    for (path_id, character_id, preparation_id) in [
        ("human_redoubt", "mara_venn", "preparation_redoubt_mara"),
        (
            "living_commonwealth",
            "nadi_vale",
            "preparation_commonwealth_nadi",
        ),
        ("open_threshold", "sol_cairn", "preparation_threshold_sol"),
    ] {
        let mut story = ColonyStoryState::default();
        let identity = identity_arc_beat(
            path_id,
            character_id,
            false,
            0,
            IdentityArcProgress::default(),
            &story,
            None,
        )
        .expect("identity establishment note");
        story.acknowledge(identity.id);

        let preparation = identity_arc_beat(
            path_id,
            character_id,
            false,
            0,
            IdentityArcProgress {
                preparations_completed: 1,
                stewardship_completed: 0,
            },
            &story,
            None,
        )
        .expect("identity preparation note");
        assert_eq!(preparation.id, preparation_id);
        assert!(!preparation.text.is_empty());
        story.acknowledge(preparation.id);
        assert!(identity_arc_beat(
            path_id,
            character_id,
            false,
            0,
            IdentityArcProgress {
                preparations_completed: 1,
                stewardship_completed: 0
            },
            &story,
            None
        )
        .is_none());
    }
}

#[test]
fn post_ending_identity_scene_unlocks_after_the_final_reflection() {
    for (path_id, character_id, post_id, operation_id) in [
        (
            "human_redoubt",
            "mara_venn",
            "post_ending_redoubt_mara",
            "epilogue_operation_redoubt_mara",
        ),
        (
            "living_commonwealth",
            "nadi_vale",
            "post_ending_commonwealth_nadi",
            "epilogue_operation_commonwealth_nadi",
        ),
        (
            "open_threshold",
            "sol_cairn",
            "post_ending_threshold_sol",
            "epilogue_operation_threshold_sol",
        ),
    ] {
        let story = ColonyStoryState::default();
        let ending = identity_arc_beat(
            path_id,
            character_id,
            true,
            0,
            IdentityArcProgress::default(),
            &story,
            None,
        )
        .unwrap();
        assert!(ending.id.ends_with("_ending"));

        let mut story = story;
        story.acknowledge(ending.id);
        let post = identity_arc_beat(
            path_id,
            character_id,
            true,
            0,
            IdentityArcProgress::default(),
            &story,
            None,
        )
        .unwrap();
        assert_eq!(post.id, post_id);
        assert!(!post.title.is_empty());
        assert!(!post.text.is_empty());
        story.acknowledge(post.id);
        assert!(identity_arc_beat(
            path_id,
            character_id,
            true,
            0,
            IdentityArcProgress::default(),
            &story,
            None
        )
        .is_none());
        let stewardship = identity_arc_beat(
            path_id,
            character_id,
            true,
            0,
            IdentityArcProgress {
                preparations_completed: 0,
                stewardship_completed: 1,
            },
            &story,
            None,
        )
        .unwrap();
        assert!(stewardship.id.starts_with("stewardship_"));
        story.acknowledge(stewardship.id);
        assert!(identity_arc_beat(
            path_id,
            character_id,
            true,
            0,
            IdentityArcProgress {
                preparations_completed: 0,
                stewardship_completed: 1
            },
            &story,
            None
        )
        .is_none());
        let operation = identity_arc_beat(
            path_id,
            character_id,
            true,
            1,
            IdentityArcProgress {
                preparations_completed: 0,
                stewardship_completed: 1,
            },
            &story,
            None,
        )
        .unwrap();
        assert_eq!(operation.id, operation_id);
        assert!(identity_arc_beat(
            path_id,
            "ilya_reed",
            true,
            1,
            IdentityArcProgress {
                preparations_completed: 0,
                stewardship_completed: 1
            },
            &story,
            None
        )
        .is_none());
    }
}
