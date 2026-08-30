use super::*;

#[test]
fn each_colony_npc_has_three_progression_beats() {
    for character_id in [
        "mara_venn",
        "ilya_reed",
        "sol_cairn",
        "nadi_vale",
        "veya_orn",
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
fn each_identity_building_has_one_path_specific_colony_voice() {
    for (path_id, character_id, beat_id) in [
        ("human_redoubt", "mara_venn", "identity_redoubt_arsenal"),
        ("living_commonwealth", "nadi_vale", "identity_choir_garden"),
        ("open_threshold", "sol_cairn", "identity_threshold_spire"),
    ] {
        assert_eq!(identity_npc(path_id), Some(character_id));
        let beat = identity_beat(path_id, character_id).unwrap();
        assert_eq!(beat.id, beat_id);
        assert!(!beat.title.is_empty());
        assert!(!beat.text.is_empty());
        assert!(
            identity_beat(path_id, "ilya_reed").is_none_or(|candidate| candidate.id != beat_id)
        );
    }
    assert!(identity_npc("unknown_path").is_none());
    assert!(identity_beat("unknown_path", "mara_venn").is_none());
}
