use super::*;
use crate::campaign::CampaignState;
use crate::data::GameData;
use macroquad_toolkit::grid::TilePos;

fn session() -> GameSession {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let roster = campaign.deployment_roster(&data, &data.mission);
    GameSession::new(&data.config, &data.mission, &roster)
}

#[test]
fn carried_field_items_target_each_team_and_are_spent_once() {
    let mut session = session();
    let hostile = "brood_stalker_a";
    let kira = session.unit("kira_voss").unwrap().position;
    session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == hostile)
        .unwrap()
        .position = TilePos::new(kira.x + 1, kira.y);
    assert!(validate(&session, "kira_voss", "survey_harness", hostile).is_ok());
    execute(&mut session, "kira_voss", "survey_harness", hostile);
    assert!(session
        .unit(hostile)
        .unwrap()
        .has_status(StatusKind::Disrupted));
    assert_eq!(
        validate(&session, "kira_voss", "survey_harness", hostile),
        Err(RuleError::EquipmentUnavailable)
    );

    let ally = session.unit("mara_venn").unwrap().position;
    session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == "ilya_reed")
        .unwrap()
        .position = TilePos::new(ally.x + 1, ally.y);
    let mara = session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == "mara_venn")
        .unwrap();
    mara.health -= 5;
    let before = mara.health;
    assert!(validate(&session, "ilya_reed", "field_medkit", "mara_venn").is_ok());
    execute(&mut session, "ilya_reed", "field_medkit", "mara_venn");
    assert_eq!(session.unit("mara_venn").unwrap().health, before + 5);
}

#[test]
fn exile_cipher_scrambles_a_hostile_movement_envelope() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign.strategy.contact_protocol_id = "directorate_requisition".to_owned();
    campaign
        .colony
        .buildings
        .push(crate::colony::BuildingState {
            id: "waystation_test".to_owned(),
            kind: crate::colony::BuildingKind::Waystation,
            position: [2, 10],
            level: 1,
            damaged: false,
        });
    campaign.recruit_outsider(&data).unwrap();
    let mut roster = data.roster.clone();
    roster.push(campaign.derived_character_unit("veya_orn", &data).unwrap());
    let mut session = GameSession::new(&data.config, &data.mission, &roster);
    let veya = session.unit("veya_orn").unwrap().position;
    session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == "directorate_rifle_a")
        .unwrap()
        .position = TilePos::new(veya.x + 2, veya.y);

    assert!(validate(
        &session,
        "veya_orn",
        "directorate_cipher",
        "directorate_rifle_a"
    )
    .is_ok());
    execute(
        &mut session,
        "veya_orn",
        "directorate_cipher",
        "directorate_rifle_a",
    );
    assert!(session
        .unit("directorate_rifle_a")
        .unwrap()
        .has_status(StatusKind::Hindered));
}

#[test]
fn mireborn_sense_braces_sedge_and_disrupts_a_hostile() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign.strategy.phase_id = "adaptation".to_owned();
    campaign.strategy.contact_complete = true;
    campaign
        .colony
        .buildings
        .push(crate::colony::BuildingState {
            id: "waystation_sedge_test".to_owned(),
            kind: crate::colony::BuildingKind::Waystation,
            position: [2, 10],
            level: 1,
            damaged: false,
        });
    campaign.recruit_outsider(&data).unwrap();
    let mut roster = data.roster.clone();
    roster.push(campaign.derived_character_unit("sedge", &data).unwrap());
    let mut session = GameSession::new(&data.config, &data.mission, &roster);
    let sedge = session.unit("sedge").unwrap().position;
    let hostile = session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == "brood_stalker_a")
        .unwrap();
    hostile.position = TilePos::new(sedge.x + 2, sedge.y);

    assert!(validate(&session, "sedge", "mireborn_sense", "brood_stalker_a").is_ok());
    execute(&mut session, "sedge", "mireborn_sense", "brood_stalker_a");
    assert!(session
        .unit("sedge")
        .unwrap()
        .has_status(StatusKind::Guarded));
    assert!(session
        .unit("brood_stalker_a")
        .unwrap()
        .has_status(StatusKind::Disrupted));
}
