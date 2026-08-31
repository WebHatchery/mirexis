use super::*;
use crate::data::{Team, UnitDef};

fn test_unit(action_points: u8) -> UnitState {
    let definition = UnitDef {
        id: "objective_label_test".to_owned(),
        name: "Objective Label Test".to_owned(),
        role: "Test".to_owned(),
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
    UnitState::from_def(&definition, action_points)
}

#[test]
fn interaction_label_names_objective_recovery() {
    assert_eq!(
        interaction_label(
            ObjectiveKind::SecureAndClear,
            ObjectiveState::Active,
            None,
            false,
            false,
        ),
        "SELECT UNIT"
    );

    let mut unit = test_unit(2);
    assert_eq!(
        interaction_label(
            ObjectiveKind::SecureAndClear,
            ObjectiveState::Active,
            Some(&unit),
            false,
            false,
        ),
        "MOVE TO OBJECTIVE"
    );
    unit.action_points = 0;
    assert_eq!(
        interaction_label(
            ObjectiveKind::SecureAndClear,
            ObjectiveState::Active,
            Some(&unit),
            true,
            false,
        ),
        "NO AP"
    );
    unit.action_points = 2;
    unit.incapacitated = true;
    assert_eq!(
        interaction_label(
            ObjectiveKind::SecureAndClear,
            ObjectiveState::Active,
            Some(&unit),
            true,
            false,
        ),
        "INCAPACITATED"
    );
    unit.incapacitated = false;
    assert_eq!(
        interaction_label(
            ObjectiveKind::SecureAndClear,
            ObjectiveState::Active,
            Some(&unit),
            true,
            true,
        ),
        "SECURE OBJECTIVE"
    );
    assert_eq!(
        interaction_label(
            ObjectiveKind::SignalTrace,
            ObjectiveState::Secured,
            Some(&unit),
            true,
            false,
        ),
        "RELAY ACTIVE"
    );
    assert_eq!(
        interaction_label(
            ObjectiveKind::Holdout,
            ObjectiveState::Active,
            Some(&unit),
            false,
            false,
        ),
        "WAIT FOR DEADLINE"
    );
}
