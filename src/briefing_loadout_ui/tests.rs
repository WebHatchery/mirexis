use super::*;

#[test]
fn selected_loadout_uses_the_same_derived_profile_as_deployment() {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let selected = campaign.selected_character().unwrap();
    let unit = campaign
        .derived_character_unit(&selected.id, &data)
        .unwrap();
    let deployed = campaign
        .deployment_roster(&data, &data.mission)
        .into_iter()
        .find(|candidate| candidate.id == selected.id)
        .unwrap();

    assert_eq!(unit.max_health, deployed.max_health);
    assert_eq!(unit.weapon_damage, deployed.weapon_damage);
    assert_eq!(unit.equipment_ids, deployed.equipment_ids);
}
