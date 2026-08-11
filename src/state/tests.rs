use super::*;
use crate::data::OperationModifier;

fn session() -> (GameConfig, GameSession) {
    let data = crate::data::GameData::load().unwrap();
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    (data.config, session)
}

fn unit_mut<'a>(session: &'a mut GameSession, id: &str) -> &'a mut UnitState {
    session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == id)
        .unwrap()
}

#[test]
fn weighted_path_spends_terrain_cost() {
    let (_, mut session) = session();
    let unit_id = session.tactical.selected_unit.clone().unwrap();
    session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == unit_id)
        .unwrap()
        .position = TilePos::new(2, 2);
    session.tactical.terrain_costs.push((TilePos::new(3, 2), 2));
    let events = session
        .execute(Command::Move {
            unit_id,
            to: TilePos::new(3, 2),
        })
        .unwrap();
    assert!(matches!(events[0], BattleEvent::UnitMoved { cost: 2, .. }));
}

#[test]
fn operation_pressure_modifiers_change_their_intended_team() {
    let data = crate::data::GameData::load().unwrap();
    let colony = data
        .roster
        .iter()
        .find(|unit| unit.team == Team::Colony)
        .unwrap();
    let hostile = data
        .roster
        .iter()
        .find(|unit| unit.team == Team::Hostile)
        .unwrap();
    let mut mission = data.mission.clone();

    mission.operation_modifier = OperationModifier::DirectorateFireControl;
    let session = GameSession::new(&data.config, &mission, &data.roster);
    assert_eq!(
        session.unit(&hostile.id).unwrap().accuracy,
        hostile.accuracy + 10
    );
    assert_eq!(session.unit(&colony.id).unwrap().accuracy, colony.accuracy);

    mission.operation_modifier = OperationModifier::BroodFrenzy;
    let session = GameSession::new(&data.config, &mission, &data.roster);
    assert_eq!(
        session.unit(&hostile.id).unwrap().move_range,
        hostile.move_range + 1
    );

    mission.operation_modifier = OperationModifier::AscendantInterference;
    let session = GameSession::new(&data.config, &mission, &data.roster);
    assert_eq!(
        session.unit(&colony.id).unwrap().accuracy,
        colony.accuracy - 10
    );
    assert_eq!(
        session.unit(&hostile.id).unwrap().accuracy,
        hostile.accuracy
    );

    mission.operation_modifier = OperationModifier::EscalationCrossfire;
    let session = GameSession::new(&data.config, &mission, &data.roster);
    assert_eq!(
        session.unit(&colony.id).unwrap().accuracy,
        colony.accuracy - 5
    );
    assert_eq!(
        session.unit(&hostile.id).unwrap().accuracy,
        hostile.accuracy + 5
    );
    assert_eq!(
        session.unit(&hostile.id).unwrap().move_range,
        hostile.move_range + 1
    );
}

#[test]
fn attacks_are_deterministic_and_emit_ordered_events() {
    let data = crate::data::GameData::load().unwrap();
    let mut a = GameSession::new(&data.config, &data.mission, &data.roster);
    let mut b = a.clone();
    for session in [&mut a, &mut b] {
        session
            .tactical
            .units
            .iter_mut()
            .find(|unit| unit.id == "kira_voss")
            .unwrap()
            .position = TilePos::new(8, 2);
        unit_mut(session, "brood_stalker_a").position = TilePos::new(10, 2);
    }
    let command = Command::Attack {
        attacker_id: "kira_voss".into(),
        target_id: "brood_stalker_a".into(),
    };
    assert_eq!(
        a.execute(command.clone()).unwrap(),
        b.execute(command).unwrap()
    );
}

#[test]
fn edge_cover_reduces_accuracy_from_its_facing_direction() {
    let (_, session) = session();
    let attacker = UnitState {
        position: TilePos::new(17, 21),
        ..session.unit("kira_voss").unwrap().clone()
    };
    let target = UnitState {
        position: TilePos::new(18, 21),
        ..session.unit("brood_stalker_a").unwrap().clone()
    };
    assert_eq!(
        session.cover_against(target.position, attacker.position),
        25
    );
    assert!(session.hit_chance(&attacker, &target) < attacker.accuracy as u8);
}

#[test]
fn solid_obstacles_block_line_of_fire() {
    let (_, mut session) = session();
    unit_mut(&mut session, "kira_voss").position = TilePos::new(8, 2);
    unit_mut(&mut session, "brood_stalker_a").position = TilePos::new(10, 2);
    session.tactical.blocked.insert(TilePos::new(9, 2));
    assert_eq!(
        session.validate(&Command::Attack {
            attacker_id: "kira_voss".into(),
            target_id: "brood_stalker_a".into(),
        }),
        Err(RuleError::NoLineOfFire)
    );
}

#[test]
fn each_colonist_mutation_gift_changes_its_tactical_options() {
    let (_, base) = session();

    let mut neural = base.clone();
    neural.tactical.selected_unit = Some("kira_voss".into());
    neural.activate_selected_mutation().unwrap();
    assert_eq!(neural.unit("kira_voss").unwrap().temporary_accuracy, 20);

    let mut chitin = base.clone();
    chitin.tactical.selected_unit = Some("mara_venn".into());
    chitin.activate_selected_mutation().unwrap();
    assert_eq!(chitin.unit("mara_venn").unwrap().temporary_armour, 3);

    let mut regen = base.clone();
    regen.tactical.selected_unit = Some("ilya_reed".into());
    unit_mut(&mut regen, "ilya_reed").health = 4;
    regen.activate_selected_mutation().unwrap();
    assert_eq!(regen.unit("ilya_reed").unwrap().health, 7);

    let mut healthy_regen = base.clone();
    healthy_regen.tactical.selected_unit = Some("ilya_reed".into());
    assert!(!healthy_regen.can_activate_selected_mutation());

    let mut elastic = base;
    elastic.tactical.selected_unit = Some("sol_cairn".into());
    let before = elastic.unit("sol_cairn").unwrap().action_points;
    elastic.activate_selected_mutation().unwrap();
    let sol = elastic.unit("sol_cairn").unwrap();
    assert_eq!(sol.action_points, before + 2);
    assert_eq!(sol.temporary_move_range, 3);
    assert!(!elastic.can_activate_selected_mutation());
}

#[test]
fn objective_requires_adjacency_and_one_action_point() {
    let (_, mut session) = session();
    let id = session.tactical.selected_unit.clone().unwrap();
    assert_eq!(
        session.validate(&Command::Interact {
            unit_id: id.clone()
        }),
        Err(RuleError::ObjectiveUnavailable)
    );
    session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == id)
        .unwrap()
        .position = TilePos::new(29, 21);
    assert!(session.execute(Command::Interact { unit_id: id }).is_ok());
    assert_eq!(session.tactical.objective_state, ObjectiveState::Secured);
}

#[test]
fn typed_objectives_resolve_with_distinct_victory_rules() {
    let (_, base) = session();

    let mut eliminate = base.clone();
    eliminate.tactical.objective_kind = ObjectiveKind::EliminateAll;
    for hostile in eliminate
        .tactical
        .units
        .iter_mut()
        .filter(|unit| unit.team == Team::Hostile)
    {
        hostile.incapacitated = true;
    }
    let mut events = Vec::new();
    eliminate.check_outcome(&mut events);
    assert_eq!(eliminate.tactical.objective_state, ObjectiveState::Victory);

    let mut secure = base.clone();
    for hostile in secure
        .tactical
        .units
        .iter_mut()
        .filter(|unit| unit.team == Team::Hostile)
    {
        hostile.incapacitated = true;
    }
    secure.check_outcome(&mut Vec::new());
    assert_eq!(secure.tactical.objective_state, ObjectiveState::Active);

    let mut extraction = base.clone();
    extraction.tactical.objective_kind = ObjectiveKind::Extraction;
    let colonist_id = extraction.tactical.selected_unit.clone().unwrap();
    extraction
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == colonist_id)
        .unwrap()
        .position = TilePos::new(
        extraction.tactical.objective_tile.x - 1,
        extraction.tactical.objective_tile.y,
    );
    let events = extraction
        .execute(Command::Interact {
            unit_id: colonist_id,
        })
        .unwrap();
    assert_eq!(extraction.tactical.objective_state, ObjectiveState::Victory);
    assert!(extraction
        .tactical
        .units
        .iter()
        .any(|unit| unit.team == Team::Hostile && !unit.incapacitated));
    assert!(events
        .iter()
        .any(|event| matches!(event, BattleEvent::ExtractionCompleted { .. })));

    let (config, mut holdout) = session();
    holdout.tactical.objective_kind = ObjectiveKind::Holdout;
    holdout.tactical.round_limit = 1;
    for colonist in holdout
        .tactical
        .units
        .iter_mut()
        .filter(|unit| unit.team == Team::Colony)
    {
        colonist.health = 100;
        colonist.max_health = 100;
    }
    holdout.end_player_phase(&config);
    assert_eq!(holdout.tactical.objective_state, ObjectiveState::Victory);
}

#[test]
fn signal_trace_must_be_activated_before_the_squad_can_outlast_it() {
    let (config, base) = session();
    let mut failed = base.clone();
    failed.tactical.objective_kind = ObjectiveKind::SignalTrace;
    failed.tactical.round_limit = 1;
    failed.end_player_phase(&config);
    assert_eq!(failed.tactical.objective_state, ObjectiveState::Failed);

    let mut traced = base;
    traced.tactical.objective_kind = ObjectiveKind::SignalTrace;
    traced.tactical.round_limit = 1;
    let colonist_id = traced.tactical.selected_unit.clone().unwrap();
    traced
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == colonist_id)
        .unwrap()
        .position = TilePos::new(
        traced.tactical.objective_tile.x - 1,
        traced.tactical.objective_tile.y,
    );
    for colonist in traced
        .tactical
        .units
        .iter_mut()
        .filter(|unit| unit.team == Team::Colony)
    {
        colonist.health = 100;
        colonist.max_health = 100;
    }
    traced
        .execute(Command::Interact {
            unit_id: colonist_id,
        })
        .unwrap();
    assert_eq!(traced.tactical.objective_state, ObjectiveState::Secured);
    traced.end_player_phase(&config);
    assert_eq!(traced.tactical.objective_state, ObjectiveState::Victory);
}

#[test]
fn ending_phase_runs_deterministic_enemy_ai_and_refreshes_colonists() {
    let (config, mut session) = session();
    let before = session.tactical.event_log.len();
    session.end_player_phase(&config);
    assert_eq!(session.tactical.round, 2);
    assert!(session.tactical.event_log.len() > before);
    assert!(session
        .tactical
        .units
        .iter()
        .filter(|unit| unit.team == Team::Colony && !unit.incapacitated)
        .all(|unit| unit.action_points == config.max_action_points));
}

#[test]
fn hostile_activations_spend_their_available_attack_economy() {
    let (config, mut session) = session();
    let kira_position = session.unit("kira_voss").unwrap().position;
    let stalker = session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == "brood_stalker_a")
        .unwrap();
    stalker.position = TilePos::new(kira_position.x + 1, kira_position.y);
    session.end_player_phase(&config);
    let attacks = session
        .tactical
        .event_log
        .iter()
        .filter(|event| {
            matches!(
                event,
                BattleEvent::AttackRolled { attacker_id, .. }
                    if attacker_id == "brood_stalker_a"
            )
        })
        .count();
    assert_eq!(attacks, 2);
}

#[test]
fn specialist_enemy_hits_apply_role_statuses() {
    let (_, mut session) = session();
    let kira_position = session.unit("kira_voss").unwrap().position;
    let sporecaster = session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == "brood_stalker_b")
        .unwrap();
    sporecaster.position = TilePos::new(kira_position.x + 1, kira_position.y);
    sporecaster.accuracy = 100;
    session.tactical.phase = TacticalPhase::Enemy;
    let seed = (0..1000)
        .find(|seed| {
            let mut rng = SeededRng::new(*seed);
            rng.range_i32(1, 101) <= 95
        })
        .unwrap();
    session.tactical.rng = SeededRng::new(seed);
    session
        .execute(Command::Attack {
            attacker_id: "brood_stalker_b".into(),
            target_id: "kira_voss".into(),
        })
        .unwrap();
    assert!(session
        .unit("kira_voss")
        .unwrap()
        .has_status(StatusKind::Hindered));
}

#[test]
fn holdouts_wait_for_serialized_reinforcement_waves() {
    let data = crate::data::GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let mut mission = data.mission.clone();
    mission.objective_kind = ObjectiveKind::Holdout;
    mission.hostile_faction = "ascendants".to_owned();
    mission.round_limit = 6;
    let roster = campaign.deployment_roster(&data, &mission);
    let mut session = GameSession::new(&data.config, &mission, &roster);
    assert_eq!(session.tactical.reinforcement_waves.len(), 2);
    for unit in &mut session.tactical.units {
        if unit.team == Team::Hostile {
            unit.incapacitated = true;
            unit.health = 0;
        } else {
            unit.health = 100;
            unit.max_health = 100;
        }
    }
    session.check_outcome(&mut Vec::new());
    assert_eq!(session.tactical.objective_state, ObjectiveState::Active);
    for _ in 0..3 {
        session.end_player_phase(&data.config);
    }
    assert_eq!(session.tactical.reinforcement_waves.len(), 1);
    assert_eq!(
        session
            .tactical
            .units
            .iter()
            .filter(|unit| unit.team == Team::Hostile && !unit.incapacitated)
            .count(),
        2
    );
    assert!(session.tactical.event_log.iter().any(|event| matches!(
        event,
        BattleEvent::ReinforcementsArrived { round: 3, count: 2 }
    )));
}

#[test]
fn phase_zero_save_payload_migrates_to_battle_schema() {
    let data = crate::data::GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let current =
        GameSession::new(&data.config, &data.mission, &data.roster).to_save("0.2.0", &campaign);
    let mut legacy = serde_json::to_value(current).unwrap();
    legacy.as_object_mut().unwrap().remove("campaign");
    let tactical = legacy["tactical"].as_object_mut().unwrap();
    for key in [
        "terrain_costs",
        "cover_edges",
        "objective_tile",
        "objective_state",
        "round_limit",
        "rng",
        "event_log",
    ] {
        tactical.remove(key);
    }
    for unit in tactical["units"].as_array_mut().unwrap() {
        let unit = unit.as_object_mut().unwrap();
        for key in [
            "armour",
            "accuracy",
            "weapon_range",
            "weapon_damage",
            "weapon_ap_cost",
            "incapacitated",
            "round_regeneration",
        ] {
            unit.remove(key);
        }
    }
    let migrated =
        crate::persistence::migrate_save_value(Some("0.1.0".to_owned()), legacy, &data).unwrap();
    assert_eq!(migrated.version, data.config.version);
    assert!(!migrated.tactical.unwrap().units.is_empty());
}

#[test]
fn victorious_mission_outcome_carries_debrief_consequences() {
    let data = crate::data::GameData::load().unwrap();
    let mut session = GameSession::new(&data.config, &data.mission, &data.roster);
    session.tactical.objective_state = ObjectiveState::Victory;
    session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == "ilya_reed")
        .unwrap()
        .incapacitated = true;
    let outcome = session.mission_outcome(&data.mission).unwrap();
    assert_eq!(outcome.colonists_deployed, 5);
    assert_eq!(outcome.colonists_incapacitated[0].id, "ilya_reed");
    assert_eq!(outcome.materials_awarded, data.mission.materials_reward);
    assert_eq!(outcome.biomass_awarded, data.mission.biomass_reward);
    assert_eq!(outcome.power_awarded, data.mission.power_reward);
}
