use crate::campaign::CampaignState;
use crate::data::{GameData, HazardKind};
use crate::state::GameSession;
use crate::tactical::{BattleEvent, HazardTile, StatusKind};
use macroquad_toolkit::grid::TilePos;

fn session_with_skill(skill_id: &str) -> GameSession {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let roster = campaign.deployment_roster(&data, &data.mission);
    let mut session = GameSession::new(&data.config, &data.mission, &roster);
    let kira = unit_mut(&mut session, "kira_voss");
    kira.active_skills.push(skill_id.to_owned());
    session.tactical.selected_unit = Some("kira_voss".to_owned());
    session
}

fn place_for_attack(session: &mut GameSession) {
    unit_mut(session, "kira_voss").position = TilePos::new(8, 2);
    let target = unit_mut(session, "brood_stalker_a");
    target.position = TilePos::new(10, 2);
    target.health = 20;
    target.max_health = 20;
    target.armour = 3;
}

fn unit_mut<'a>(session: &'a mut GameSession, id: &str) -> &'a mut crate::state::UnitState {
    session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == id)
        .unwrap()
}

#[test]
fn controlled_burst_emits_two_shots_and_spends_the_weapon_cost_twice() {
    let mut session = session_with_skill("controlled_burst");
    place_for_attack(&mut session);
    let events = session
        .activate_skill_on("kira_voss", "controlled_burst", "brood_stalker_a")
        .unwrap();
    assert_eq!(
        events
            .iter()
            .filter(|event| matches!(event, BattleEvent::AttackRolled { .. }))
            .count(),
        2
    );
    assert_eq!(session.unit("kira_voss").unwrap().action_points, 1);
}

#[test]
fn armour_drill_is_consumed_by_the_next_attack() {
    let mut session = session_with_skill("armour_drill");
    place_for_attack(&mut session);
    session.activate_selected_skill("armour_drill").unwrap();
    assert!(
        session
            .unit("kira_voss")
            .unwrap()
            .next_attack_ignores_armour
    );
    session.attack_selected("brood_stalker_a").unwrap();
    assert!(
        !session
            .unit("kira_voss")
            .unwrap()
            .next_attack_ignores_armour
    );
}

#[test]
fn spotters_mark_improves_one_allied_attack_then_clears() {
    let mut session = session_with_skill("spotters_mark");
    place_for_attack(&mut session);
    session
        .activate_skill_on("kira_voss", "spotters_mark", "brood_stalker_a")
        .unwrap();
    assert!(session
        .unit("brood_stalker_a")
        .unwrap()
        .has_status(StatusKind::Marked));
    session.attack_selected("brood_stalker_a").unwrap();
    assert!(!session
        .unit("brood_stalker_a")
        .unwrap()
        .has_status(StatusKind::Marked));
}

#[test]
fn interpose_protects_the_ally_and_the_defender() {
    let mut session = session_with_skill("interpose");
    unit_mut(&mut session, "kira_voss").position = TilePos::new(4, 19);
    unit_mut(&mut session, "mara_venn").position = TilePos::new(5, 19);
    session
        .activate_skill_on("kira_voss", "interpose", "mara_venn")
        .unwrap();
    assert!(session
        .unit("kira_voss")
        .unwrap()
        .has_status(StatusKind::Guarded));
    assert!(session
        .unit("mara_venn")
        .unwrap()
        .has_status(StatusKind::Guarded));
}

#[test]
fn anchor_point_uses_the_same_guarded_phase_window() {
    let mut session = session_with_skill("anchor_point");
    session.activate_selected_skill("anchor_point").unwrap();
    assert!(session
        .unit("kira_voss")
        .unwrap()
        .has_status(StatusKind::Guarded));
}

#[test]
fn slipstep_reaches_a_hazard_without_triggering_its_landing_effect() {
    let mut session = session_with_skill("slipstep");
    let start = TilePos::new(4, 19);
    let destination = TilePos::new(5, 19);
    unit_mut(&mut session, "kira_voss").position = start;
    session.tactical.hazards.push(HazardTile {
        position: destination,
        kind: HazardKind::SporeBloom,
    });
    let events = session
        .activate_skill_on_tile("kira_voss", "slipstep", destination)
        .unwrap();
    assert_eq!(session.unit("kira_voss").unwrap().position, destination);
    assert_eq!(session.unit("kira_voss").unwrap().action_points, 3);
    assert!(events
        .iter()
        .any(|event| matches!(event, BattleEvent::UnitMoved { .. })));
    assert!(!events
        .iter()
        .any(|event| matches!(event, BattleEvent::HazardTriggered { .. })));
}
