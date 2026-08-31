use super::validate;
use crate::campaign::CampaignState;
use crate::data::{EdgeDirection, GameData, HazardKind};
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

fn session_with_unit_skill(unit_id: &str, skill_id: &str) -> GameSession {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let mut roster = campaign.deployment_roster(&data, &data.mission);
    if !roster.iter().any(|unit| unit.id == unit_id) {
        roster.push(
            campaign
                .derived_character_unit(unit_id, &data)
                .expect("test unit has a derived character profile"),
        );
    }
    let mut session = GameSession::new(&data.config, &data.mission, &roster);
    unit_mut(&mut session, unit_id)
        .active_skills
        .push(skill_id.to_owned());
    session.tactical.selected_unit = Some(unit_id.to_owned());
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

#[test]
fn kinetic_draw_pulls_a_hostile_into_the_nearest_clear_tile() {
    let mut session = session_with_unit_skill("mara_venn", "kinetic_draw");
    unit_mut(&mut session, "mara_venn").position = TilePos::new(8, 2);
    unit_mut(&mut session, "brood_stalker_a").position = TilePos::new(10, 2);

    let events = session
        .activate_skill_on("mara_venn", "kinetic_draw", "brood_stalker_a")
        .unwrap();

    assert_eq!(
        session.unit("brood_stalker_a").unwrap().position,
        TilePos::new(9, 2)
    );
    assert_eq!(session.unit("mara_venn").unwrap().action_points, 4);
    assert!(events.iter().any(|event| matches!(
        event,
        BattleEvent::UnitMoved {
            unit_id,
            cost: 0,
            ..
        } if unit_id == "brood_stalker_a"
    )));
}

#[test]
fn premonition_softens_a_hostiles_accuracy_for_the_current_phase() {
    let mut session = session_with_unit_skill("mara_venn", "premonition");
    unit_mut(&mut session, "mara_venn").position = TilePos::new(8, 2);
    unit_mut(&mut session, "brood_stalker_a").position = TilePos::new(10, 2);
    let before = session
        .unit("brood_stalker_a")
        .unwrap()
        .effective_accuracy();

    session
        .activate_skill_on("mara_venn", "premonition", "brood_stalker_a")
        .unwrap();

    let target = session.unit("brood_stalker_a").unwrap();
    assert!(target.has_status(StatusKind::Disrupted));
    assert_eq!(target.effective_accuracy(), before - 20);
}

#[test]
fn adaptive_secretion_resists_the_nearest_visible_hazard() {
    let mut session = session_with_unit_skill("nadi_vale", "adaptive_secretion");
    unit_mut(&mut session, "nadi_vale").position = TilePos::new(7, 20);
    unit_mut(&mut session, "kira_voss").position = TilePos::new(6, 20);
    session.tactical.hazards.push(HazardTile {
        position: TilePos::new(6, 21),
        kind: HazardKind::StaticRift,
    });

    session
        .activate_skill_on("nadi_vale", "adaptive_secretion", "kira_voss")
        .unwrap();

    let target = session.unit("kira_voss").unwrap();
    assert_eq!(target.hazard_resistance, Some(HazardKind::StaticRift));
    assert!(target.has_status(StatusKind::Adapted));
    let health = target.health;
    unit_mut(&mut session, "kira_voss").position = TilePos::new(6, 21);
    let events = crate::hazards::resolve_after_move(&mut session, "kira_voss");
    assert_eq!(session.unit("kira_voss").unwrap().health, health);
    assert!(!events
        .iter()
        .any(|event| matches!(event, BattleEvent::HazardTriggered { .. })));
}

#[test]
fn spore_veil_adds_a_bounded_obscuring_field_used_by_attack_preview() {
    let mut session = session_with_unit_skill("nadi_vale", "spore_veil");
    unit_mut(&mut session, "nadi_vale").position = TilePos::new(7, 20);
    unit_mut(&mut session, "kira_voss").position = TilePos::new(6, 19);
    unit_mut(&mut session, "brood_stalker_a").position = TilePos::new(8, 19);
    let attacker = session.unit("kira_voss").unwrap().clone();
    let target = session.unit("brood_stalker_a").unwrap().clone();
    let before = session.hit_chance(&attacker, &target);

    session
        .activate_skill_on_tile("nadi_vale", "spore_veil", TilePos::new(7, 19))
        .unwrap();

    assert_eq!(session.tactical.obscuring_fields.len(), 1);
    assert_eq!(session.tactical.obscuring_penalty(target.position), 15);
    assert_eq!(
        before.saturating_sub(session.hit_chance(&attacker, &target)),
        15
    );
    session.advance_obscuring_fields();
    assert!(session.tactical.obscuring_fields.is_empty());
}

#[test]
fn stabilise_revives_an_incapacitated_ally_without_granting_actions() {
    let mut session = session_with_unit_skill("ilya_reed", "stabilise");
    unit_mut(&mut session, "ilya_reed").position = TilePos::new(5, 20);
    let target = unit_mut(&mut session, "mara_venn");
    target.position = TilePos::new(6, 20);
    target.health = 0;
    target.incapacitated = true;
    target.action_points = 0;

    let events = session
        .activate_skill_on("ilya_reed", "stabilise", "mara_venn")
        .unwrap();

    let target = session.unit("mara_venn").unwrap();
    assert_eq!(target.health, 1);
    assert!(!target.incapacitated);
    assert_eq!(target.action_points, 0);
    assert!(events.iter().any(|event| matches!(
        event,
        BattleEvent::UnitHealed {
            unit_id,
            amount: 1,
            remaining: 1
        } if unit_id == "mara_venn"
    )));
}

#[test]
fn combat_stimulant_trades_two_actions_for_a_visible_hindered_window() {
    let mut session = session_with_unit_skill("ilya_reed", "combat_stimulant");
    unit_mut(&mut session, "ilya_reed").position = TilePos::new(5, 20);
    let target = unit_mut(&mut session, "mara_venn");
    target.position = TilePos::new(6, 20);
    target.action_points = 1;

    session
        .activate_skill_on("ilya_reed", "combat_stimulant", "mara_venn")
        .unwrap();

    let target = session.unit("mara_venn").unwrap();
    assert_eq!(target.action_points, 3);
    assert_eq!(
        target
            .statuses
            .iter()
            .find(|status| status.kind == StatusKind::Hindered)
            .unwrap()
            .remaining_phases,
        2
    );
}

#[test]
fn portable_cover_blocks_an_empty_adjacent_tile_with_directional_protection() {
    let mut session = session_with_unit_skill("sol_cairn", "portable_cover");
    let position = TilePos::new(6, 19);
    let cover_tile = TilePos::new(7, 19);
    unit_mut(&mut session, "sol_cairn").position = position;

    session
        .activate_skill_on_tile("sol_cairn", "portable_cover", cover_tile)
        .unwrap();

    assert!(session.tactical.blocked.contains(&cover_tile));
    assert_eq!(
        session
            .tactical
            .destructible_cover
            .iter()
            .find(|cover| cover.position == cover_tile)
            .unwrap()
            .health,
        6
    );
    assert!(session.tactical.cover_edges.iter().any(|edge| {
        edge.position == [7, 19] && edge.direction == EdgeDirection::West && edge.strength == 18
    }));
}

#[test]
fn overcharge_improves_the_next_equipment_action_and_is_consumed() {
    let mut session = session_with_unit_skill("sol_cairn", "overcharge");
    unit_mut(&mut session, "sol_cairn").position = TilePos::new(6, 19);
    unit_mut(&mut session, "kira_voss").position = TilePos::new(7, 19);

    session.activate_selected_skill("overcharge").unwrap();
    assert!(
        session
            .unit("sol_cairn")
            .unwrap()
            .next_equipment_overcharged
    );
    session
        .use_equipment("sol_cairn", "field_toolkit", "kira_voss")
        .unwrap();

    assert!(
        !session
            .unit("sol_cairn")
            .unwrap()
            .next_equipment_overcharged
    );
    assert_eq!(
        session
            .unit("kira_voss")
            .unwrap()
            .statuses
            .iter()
            .find(|status| status.kind == StatusKind::Guarded)
            .unwrap()
            .remaining_phases,
        3
    );
}

#[test]
fn targeted_techniques_explain_team_and_range_failures() {
    let mut session = session_with_skill("spotters_mark");
    assert_eq!(
        validate(
            &session,
            "kira_voss",
            "spotters_mark",
            Some("mara_venn"),
            None,
        ),
        Err(crate::state::RuleError::WrongTeam)
    );

    unit_mut(&mut session, "brood_stalker_a").position = TilePos::new(20, 19);
    assert_eq!(
        validate(
            &session,
            "kira_voss",
            "spotters_mark",
            Some("brood_stalker_a"),
            None,
        ),
        Err(crate::state::RuleError::OutOfRange)
    );
}
