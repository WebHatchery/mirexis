use super::*;
use crate::campaign::CampaignState;
use crate::data::GameData;
use crate::state::GameSession;
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
