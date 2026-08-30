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
