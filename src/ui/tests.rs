use super::*;
use crate::data::{Team, UnitDef};
use crate::tactical::UnitState;

fn test_unit(mutation: &str) -> UnitState {
    let definition = UnitDef {
        id: "mutation_label_test".to_owned(),
        name: "Mutation Label Test".to_owned(),
        role: "Test".to_owned(),
        class_id: "soldier".to_owned(),
        equipment_ids: Vec::new(),
        learned_skills: Vec::new(),
        active_skills: Vec::new(),
        mutation: mutation.to_owned(),
        team: Team::Colony,
        faction: None,
        position: [0, 0],
        max_health: 10,
        move_range: 5,
        armour: 1,
        accuracy: 70,
        weapon_range: 5,
        weapon_damage: 3,
        weapon_ap_cost: 1,
        round_regeneration: 0,
    };
    UnitState::from_def(&definition, 2)
}

#[test]
fn mutation_button_label_names_unavailable_states() {
    assert_eq!(mutation_button_label(None, false), "MUTATION GIFT");

    let mut unit = test_unit("Neural Bloom");
    assert_eq!(mutation_button_label(Some(&unit), true), "NEURAL FOCUS");
    unit.mutation_gift_used = true;
    assert_eq!(mutation_button_label(Some(&unit), false), "SPENT");
    unit.mutation_gift_used = false;
    unit.action_points = 0;
    assert_eq!(mutation_button_label(Some(&unit), false), "NO AP");
    unit.action_points = 2;
    unit.incapacitated = true;
    assert_eq!(mutation_button_label(Some(&unit), false), "INCAPACITATED");

    let unit = test_unit("Regenerative Tissue");
    assert_eq!(mutation_button_label(Some(&unit), false), "FULL HEALTH");
}

#[test]
fn logical_clip_rect_converts_to_letterboxed_physical_pixels() {
    let ui = VirtualUi {
        logical_width: 1280.0,
        logical_height: 720.0,
        scale: 0.8,
        offset: vec2(40.0, 12.0),
    };
    assert_eq!(
        ui_clip_pixels(&ui, Rect::new(18.0, 106.0, 884.0, 568.0), 1.5),
        (82, 145, 1061, 682)
    );
}

#[test]
fn map_drag_release_discards_actions_from_controls_under_the_pointer() {
    let mut actions = vec![UiAction::ReturnToTitle, UiAction::EndPhase];
    suppress_map_release_actions(&mut actions, false);
    assert_eq!(actions.len(), 2);
    suppress_map_release_actions(&mut actions, true);
    assert!(actions.is_empty());
}

#[test]
fn modal_tactical_layers_disable_world_input() {
    assert!(tactical_world_input_enabled(
        false, false, false, false, false
    ));
    for state in [
        (true, false, false, false, false),
        (false, true, false, false, false),
        (false, false, true, false, false),
        (false, false, false, true, false),
        (false, false, false, false, true),
        (true, true, true, true, true),
    ] {
        assert!(!tactical_world_input_enabled(
            state.0, state.1, state.2, state.3, state.4
        ));
    }
}

#[test]
fn capture_pointer_is_fixed_off_canvas() {
    let ui = VirtualUi {
        logical_width: 1280.0,
        logical_height: 720.0,
        scale: 0.8,
        offset: vec2(40.0, 12.0),
    };
    assert_eq!(
        pointer_position_for_capture(&ui, true),
        vec2(-1_000.0, -1_000.0)
    );
}
