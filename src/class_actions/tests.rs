use super::*;
use crate::campaign::CampaignState;
use crate::data::GameData;
use crate::state::{GameSession, HazardTile, UnitState};
use macroquad_toolkit::grid::TilePos;

fn session() -> GameSession {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let roster = campaign.deployment_roster(&data, &data.mission);
    GameSession::new(&data.config, &data.mission, &roster)
}

fn engineer_session() -> GameSession {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign.toggle_deployment("kira_voss").unwrap();
    campaign.toggle_deployment("sol_cairn").unwrap();
    let roster = campaign.deployment_roster(&data, &data.mission);
    GameSession::new(&data.config, &data.mission, &roster)
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
fn starting_classes_each_change_the_battle_state() {
    let mut scout = session();
    let before_ap = scout.unit("kira_voss").unwrap().action_points;
    scout.tactical.selected_unit = Some("kira_voss".into());
    scout.activate_selected_class_action().unwrap();
    let kira = scout.unit("kira_voss").unwrap();
    assert_eq!(kira.action_points, before_ap + 2);
    assert!(kira.has_status(StatusKind::Quickened));
    assert!(!scout.can_activate_selected_class_action());

    let mut defender = session();
    let base_armour = defender.unit("mara_venn").unwrap().effective_armour();
    defender.tactical.selected_unit = Some("mara_venn".into());
    defender.activate_selected_class_action().unwrap();
    assert_eq!(
        defender.unit("mara_venn").unwrap().effective_armour(),
        base_armour + 2
    );
    let config = GameData::load().unwrap().config;
    defender.end_player_phase(&config);
    assert!(defender
        .unit("mara_venn")
        .unwrap()
        .has_status(StatusKind::Guarded));

    let mut medic = session();
    medic
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == "mara_venn")
        .unwrap()
        .health = 3;
    medic.tactical.selected_unit = Some("ilya_reed".into());
    medic
        .activate_class_action_on("ilya_reed", "mara_venn")
        .unwrap();
    assert_eq!(medic.unit("mara_venn").unwrap().health, 7);

    let mut engineer = engineer_session();
    engineer
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == "sol_cairn")
        .unwrap()
        .position = TilePos::new(7, 2);
    engineer
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == "brood_stalker_a")
        .unwrap()
        .position = TilePos::new(8, 2);
    let before_health = engineer.unit("brood_stalker_a").unwrap().health;
    engineer.tactical.selected_unit = Some("sol_cairn".into());
    engineer
        .activate_class_action_on("sol_cairn", "brood_stalker_a")
        .unwrap();
    assert_eq!(
        engineer.unit("brood_stalker_a").unwrap().health,
        before_health - 3
    );
}

#[test]
fn retrained_classes_expose_their_own_status_actions() {
    for (class_id, expected_status) in [
        ("soldier", StatusKind::Focused),
        ("psionic", StatusKind::Disrupted),
        ("biotech", StatusKind::Regenerating),
    ] {
        let mut session = session();
        let kira = session
            .tactical
            .units
            .iter_mut()
            .find(|unit| unit.id == "kira_voss")
            .unwrap();
        kira.class_id = class_id.to_owned();
        kira.position = TilePos::new(7, 2);
        session
            .tactical
            .units
            .iter_mut()
            .find(|unit| unit.id == "brood_stalker_a")
            .unwrap()
            .position = TilePos::new(8, 2);
        session.tactical.selected_unit = Some("kira_voss".into());
        if requires_target(class_id) {
            session
                .activate_class_action_on("kira_voss", "brood_stalker_a")
                .unwrap();
        } else {
            session.activate_selected_class_action().unwrap();
        }
        assert!(session
            .tactical
            .units
            .iter()
            .any(|unit| unit.has_status(expected_status)));
    }
}

#[test]
fn targeted_class_action_changes_only_the_chosen_valid_unit() {
    let mut medic = session();
    for id in ["kira_voss", "mara_venn"] {
        medic
            .tactical
            .units
            .iter_mut()
            .find(|unit| unit.id == id)
            .unwrap()
            .health -= 4;
    }
    let kira_before = medic.unit("kira_voss").unwrap().health;
    let mara_before = medic.unit("mara_venn").unwrap().health;
    medic
        .activate_class_action_on("ilya_reed", "kira_voss")
        .unwrap();
    assert_eq!(medic.unit("kira_voss").unwrap().health, kira_before + 4);
    assert_eq!(medic.unit("mara_venn").unwrap().health, mara_before);
    assert!(!medic.can_target_class_action("ilya_reed", "mara_venn"));
}

#[test]
fn advanced_actions_create_hybrid_tactical_roles() {
    let mut vanguard = session();
    vanguard
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == "mara_venn")
        .unwrap()
        .class_id = "vanguard".to_owned();
    vanguard
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == "kira_voss")
        .unwrap()
        .position = TilePos::new(4, 18);
    vanguard.tactical.selected_unit = Some("mara_venn".into());
    vanguard.activate_selected_class_action().unwrap();
    assert!(vanguard
        .unit("mara_venn")
        .unwrap()
        .has_status(StatusKind::Guarded));
    assert!(vanguard
        .unit("kira_voss")
        .unwrap()
        .has_status(StatusKind::Guarded));

    let mut lifewright = session();
    lifewright
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == "ilya_reed")
        .unwrap()
        .class_id = "lifewright".to_owned();
    lifewright
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == "mara_venn")
        .unwrap()
        .health -= 4;
    let before = lifewright.unit("mara_venn").unwrap().health;
    lifewright
        .activate_class_action_on("ilya_reed", "mara_venn")
        .unwrap();
    assert_eq!(lifewright.unit("mara_venn").unwrap().health, before + 3);
    assert!(lifewright
        .unit("mara_venn")
        .unwrap()
        .has_status(StatusKind::Regenerating));

    let mut null_adept = session();
    let kira = null_adept
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == "kira_voss")
        .unwrap();
    kira.class_id = "null_adept".to_owned();
    kira.position = TilePos::new(7, 2);
    null_adept
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == "brood_stalker_a")
        .unwrap()
        .position = TilePos::new(8, 2);
    let before = null_adept.unit("brood_stalker_a").unwrap().health;
    null_adept
        .activate_class_action_on("kira_voss", "brood_stalker_a")
        .unwrap();
    assert_eq!(
        null_adept.unit("brood_stalker_a").unwrap().health,
        before - 2
    );
    assert!(null_adept
        .unit("brood_stalker_a")
        .unwrap()
        .has_status(StatusKind::Disrupted));
}

#[test]
fn breacher_crosses_a_short_route_breaks_a_hostile_and_opens_its_lane() {
    let mut session = session();
    unit_mut(&mut session, "mara_venn").class_id = "breacher".to_owned();
    unit_mut(&mut session, "mara_venn").position = TilePos::new(7, 2);
    unit_mut(&mut session, "brood_stalker_a").position = TilePos::new(10, 2);
    let before_health = session.unit("brood_stalker_a").unwrap().health;

    let events = session
        .activate_class_action_on_tile("mara_venn", TilePos::new(10, 2))
        .unwrap();

    assert_eq!(
        session.unit("mara_venn").unwrap().position,
        TilePos::new(9, 2)
    );
    assert_eq!(
        session.unit("brood_stalker_a").unwrap().health,
        before_health - 2
    );
    assert!(session
        .unit("brood_stalker_a")
        .unwrap()
        .has_status(StatusKind::Marked));
    assert!(events.iter().any(|event| matches!(
        event,
        BattleEvent::UnitMoved {
            unit_id,
            cost: 0,
            ..
        } if unit_id == "mara_venn"
    )));
}

#[test]
fn fortifier_places_a_stronger_directional_bastion_on_a_valid_tile() {
    let mut session = session();
    unit_mut(&mut session, "mara_venn").class_id = "fortifier".to_owned();
    unit_mut(&mut session, "mara_venn").position = TilePos::new(6, 19);
    let tile = TilePos::new(8, 19);

    session
        .activate_class_action_on_tile("mara_venn", tile)
        .unwrap();

    assert!(session.tactical.blocked.contains(&tile));
    assert_eq!(
        session
            .tactical
            .destructible_cover
            .iter()
            .find(|cover| cover.position == tile)
            .unwrap()
            .health,
        8
    );
    assert!(session.tactical.cover_edges.iter().any(|edge| {
        edge.position == [8, 19] && edge.direction == EdgeDirection::West && edge.strength == 25
    }));
}

#[test]
fn rescue_specialist_moves_a_pair_away_from_pressure_and_revives_the_ally() {
    let mut session = session();
    session.tactical.blocked.clear();
    session.tactical.hazards.clear();
    unit_mut(&mut session, "mara_venn").class_id = "rescue_specialist".to_owned();
    unit_mut(&mut session, "mara_venn").position = TilePos::new(11, 10);
    unit_mut(&mut session, "kira_voss").position = TilePos::new(12, 10);
    unit_mut(&mut session, "kira_voss").health = 0;
    unit_mut(&mut session, "kira_voss").incapacitated = true;
    unit_mut(&mut session, "brood_stalker_a").position = TilePos::new(14, 10);
    let events = session
        .activate_class_action_on("mara_venn", "kira_voss")
        .unwrap();

    let mara = session.unit("mara_venn").unwrap();
    let kira = session.unit("kira_voss").unwrap();
    assert_eq!(kira.health, 1);
    assert!(!kira.incapacitated);
    assert!(mara.position != TilePos::new(11, 10));
    assert!(kira.position != TilePos::new(12, 10));
    assert_eq!(manhattan(mara.position, kira.position), 1);
    assert!(manhattan(kira.position, TilePos::new(14, 10)) > 2);
    assert!(mara.has_status(StatusKind::Guarded));
    assert!(kira.has_status(StatusKind::Guarded));
    assert!(
        events
            .iter()
            .filter(|event| matches!(event, BattleEvent::UnitMoved { cost: 0, .. }))
            .count()
            >= 2
    );
}

#[test]
fn chorus_warden_converts_a_hazard_and_shares_a_short_regeneration_field() {
    let mut session = session();
    session.tactical.blocked.clear();
    session.tactical.cover_edges.clear();
    session.tactical.destructible_cover.clear();
    session.tactical.obscuring_fields.clear();
    session.tactical.hazards = vec![HazardTile {
        position: TilePos::new(9, 2),
        kind: crate::data::HazardKind::SporeBloom,
    }];
    unit_mut(&mut session, "mara_venn").class_id = "chorus_warden".to_owned();
    unit_mut(&mut session, "mara_venn").position = TilePos::new(7, 2);
    unit_mut(&mut session, "kira_voss").position = TilePos::new(8, 2);

    let events = session
        .activate_class_action_on_tile("mara_venn", TilePos::new(9, 2))
        .unwrap();

    assert!(session
        .tactical
        .hazards
        .iter()
        .all(|hazard| hazard.position != TilePos::new(9, 2)));
    assert!(session.tactical.obscuring_fields.iter().any(|field| {
        field.center == TilePos::new(9, 2) && field.radius == 1 && field.remaining_phases == 1
    }));
    assert!(session
        .unit("mara_venn")
        .unwrap()
        .has_status(StatusKind::Hindered));
    assert!(session
        .unit("kira_voss")
        .unwrap()
        .has_status(StatusKind::Regenerating));
    assert!(events.iter().any(|event| matches!(
        event,
        BattleEvent::HazardConverted {
            position,
            kind: crate::data::HazardKind::SporeBloom,
            ..
        } if *position == TilePos::new(9, 2)
    )));
}
