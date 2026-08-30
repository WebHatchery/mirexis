use super::*;
use crate::data::GameData;

#[test]
fn formations_are_distinct_and_fall_back_around_unsafe_entry_cells() {
    let data = GameData::load().unwrap();
    let mut mission = data.mission.clone();
    mission.blocked_tiles.extend([[4, 19], [4, 20]]);
    let mut shapes = Vec::new();
    for formation in [
        FormationKind::Wedge,
        FormationKind::Line,
        FormationKind::Column,
    ] {
        let mut roster = data.roster.clone();
        apply(&mut roster, &mission, &data.config, formation);
        let positions = roster
            .iter()
            .filter(|unit| unit.team == Team::Colony)
            .take(3)
            .map(|unit| unit.position)
            .collect::<Vec<_>>();
        assert_eq!(positions.iter().collect::<HashSet<_>>().len(), 3);
        assert!(positions.iter().all(|position| is_safe(
            *position,
            &mission,
            &data.config,
            &HashSet::new()
        )));
        shapes.push(positions);
    }
    assert_ne!(shapes[0], shapes[1]);
    assert_ne!(shapes[1], shapes[2]);
}

#[test]
fn doctrine_yard_adds_a_rally_formation_without_changing_the_base_cycle() {
    assert_eq!(
        FormationKind::Column.next_with_doctrine_yard(false),
        FormationKind::Wedge
    );
    assert_eq!(
        FormationKind::Column.next_with_doctrine_yard(true),
        FormationKind::Rally
    );
    assert_eq!(
        FormationKind::Rally.next_with_doctrine_yard(true),
        FormationKind::Wedge
    );
    assert_eq!(
        FormationKind::Rally.without_doctrine_yard(),
        FormationKind::Wedge
    );
}
