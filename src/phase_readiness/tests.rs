use super::*;
use crate::data::GameData;

#[test]
fn readiness_counts_only_active_colonists_with_unspent_actions() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data.config, &data.mission, &data.roster);
    assert_eq!(ready_count(&session), 5);
    for unit in &mut session.tactical.units {
        if unit.team == Team::Colony {
            unit.action_points = 0;
        }
    }
    session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == "kira_voss")
        .unwrap()
        .action_points = 1;
    assert_eq!(ready_count(&session), 1);
    session.tactical.selected_unit = Some("mara_venn".to_owned());
    assert_eq!(select_next(&mut session).as_deref(), Some("kira_voss"));
    assert_eq!(
        session.tactical.selected_tile,
        session.unit("kira_voss").unwrap().position
    );
    session.unit("kira_voss").unwrap();
    session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == "kira_voss")
        .unwrap()
        .action_points = 0;
    assert!(select_next(&mut session).is_none());
}
