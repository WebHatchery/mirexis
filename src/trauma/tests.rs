use super::*;
use crate::campaign::CampaignState;
use crate::data::{GameData, Team};

#[test]
fn repeated_incapacitation_adds_two_distinct_tradeoff_scars() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    let character = campaign
        .roster
        .iter_mut()
        .find(|character| character.id == "kira_voss")
        .unwrap();

    assert!(record_incapacitation(character, 1));
    assert!(record_incapacitation(character, 2));
    assert!(!record_incapacitation(character, 3));
    assert_eq!(character.traumas.len(), 2);
    assert_ne!(character.traumas[0].id, character.traumas[1].id);
}

#[test]
fn scar_effects_change_later_deployment_without_blocking_it() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    let before = campaign.deployment_roster(&data, &data.mission);
    let before = before
        .iter()
        .find(|unit| unit.id == "kira_voss" && unit.team == Team::Colony)
        .unwrap()
        .clone();
    let character = campaign
        .roster
        .iter_mut()
        .find(|character| character.id == "kira_voss")
        .unwrap();
    character.traumas.push(TraumaRecord {
        id: "reinforced_ribs".to_owned(),
        name: "Reinforced Ribs".to_owned(),
        effect: "+1 ARMOUR · -1 MOVE".to_owned(),
    });
    let deployment = campaign.deployment_roster(&data, &data.mission);
    let deployed = deployment
        .iter()
        .find(|unit| unit.id == "kira_voss" && unit.team == Team::Colony)
        .unwrap();

    assert_eq!(deployed.armour, before.armour + 1);
    assert_eq!(deployed.move_range + 1, before.move_range);
}

#[test]
fn trauma_ward_softens_persistent_scar_tradeoffs() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign
        .roster
        .iter_mut()
        .find(|character| character.id == "kira_voss")
        .unwrap()
        .traumas
        .push(TraumaRecord {
            id: "reinforced_ribs".to_owned(),
            name: "Reinforced Ribs".to_owned(),
            effect: "+1 ARMOUR · -1 MOVE".to_owned(),
        });
    let baseline = campaign.derived_character_unit("kira_voss", &data).unwrap();
    let infirmary_id = campaign
        .colony
        .buildings
        .iter()
        .find(|building| building.kind == crate::colony::BuildingKind::Infirmary)
        .unwrap()
        .id
        .clone();
    campaign
        .colony
        .queue_facility_upgrade(&infirmary_id, crate::colony::TRAUMA_WARD_UPGRADE)
        .unwrap();
    campaign.colony.advance_operation();

    let softened = campaign.derived_character_unit("kira_voss", &data).unwrap();
    assert_eq!(softened.armour, baseline.armour);
    assert_eq!(softened.move_range, baseline.move_range + 1);
}
