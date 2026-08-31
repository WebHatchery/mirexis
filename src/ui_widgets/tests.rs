use super::*;
use crate::data::{Team, UnitDef};
use crate::state::StatusKind;
use crate::tactical::StatusEffect;
use macroquad_toolkit::grid::TilePos;

#[test]
fn action_button_label_explains_target_gates_without_hiding_cancel() {
    assert_eq!(
        action_button_label("FIELD PATCH", false, true, true),
        "FIELD PATCH"
    );
    assert_eq!(
        action_button_label("FIELD PATCH", false, true, false),
        "NO TARGET"
    );
    assert_eq!(
        action_button_label("FIELD PATCH", true, true, false),
        "CANCEL"
    );
    assert_eq!(
        action_button_label("ARMOUR DRILL", false, false, false),
        "ARMOUR DRILL"
    );
}

#[test]
fn attack_event_summaries_name_hit_and_miss_outcomes() {
    let hit = BattleEvent::AttackRolled {
        attacker_id: "mara_venn".to_owned(),
        target_id: "brood_stalker_a".to_owned(),
        roll: 41,
        hit_chance: 64,
    };
    let miss = BattleEvent::AttackRolled {
        attacker_id: "mara_venn".to_owned(),
        target_id: "brood_stalker_a".to_owned(),
        roll: 65,
        hit_chance: 64,
    };

    assert_eq!(event_summary(&hit), "Attack HIT · roll 41 · 64% target");
    assert_eq!(event_summary(&miss), "Attack MISS · roll 65 · 64% target");
}

#[test]
fn event_summaries_turn_internal_identifiers_into_display_names() {
    let moved = BattleEvent::UnitMoved {
        unit_id: "brood_stalker_a".to_owned(),
        path: Vec::new(),
        cost: 2,
    };
    let equipment = BattleEvent::EquipmentUsed {
        unit_id: "mara_venn".to_owned(),
        equipment_id: "field_medkit".to_owned(),
        target_id: "kira_voss".to_owned(),
    };

    assert_eq!(event_summary(&moved), "Brood Stalker A moved · 2 AP");
    assert_eq!(event_summary(&equipment), "Field Medkit used on Kira Voss");
}

#[test]
fn active_statuses_name_their_remaining_phase_duration() {
    let definition = UnitDef {
        id: "status_test".to_owned(),
        name: "Status Test".to_owned(),
        role: "Scout".to_owned(),
        class_id: "soldier".to_owned(),
        equipment_ids: Vec::new(),
        learned_skills: Vec::new(),
        active_skills: Vec::new(),
        mutation: "None".to_owned(),
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
    let mut unit = UnitState::from_def(&definition, 2);
    unit.position = TilePos::new(0, 0);
    unit.statuses = vec![
        StatusEffect {
            kind: StatusKind::Focused,
            remaining_phases: 1,
        },
        StatusEffect {
            kind: StatusKind::Guarded,
            remaining_phases: 2,
        },
    ];

    assert_eq!(
        action_status(&unit),
        "FOCUSED 1 PH / GUARDED 2 PH · CLASS READY"
    );
    assert_eq!(
        active_status_summary(&unit),
        "EFFECTS // FOCUSED 1 PH / GUARDED 2 PH"
    );

    unit.statuses.clear();
    assert_eq!(active_status_summary(&unit), "EFFECTS // NONE");
}
