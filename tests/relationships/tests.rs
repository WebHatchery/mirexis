use super::*;

#[test]
fn shared_victories_unlock_bounded_pair_bonuses() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    let baseline = campaign.deployment_roster(&data, &data.mission);
    let baseline_kira = baseline.iter().find(|unit| unit.id == "kira_voss").unwrap();
    let baseline_accuracy = baseline_kira.accuracy;
    let baseline_armour = baseline_kira.armour;
    let pair = vec!["kira_voss".to_owned(), "mara_venn".to_owned()];

    for _ in 0..3 {
        campaign.strengthen_shared_victory(&pair);
    }
    let trusted = campaign.deployment_roster(&data, &data.mission);
    let trusted_kira = trusted.iter().find(|unit| unit.id == "kira_voss").unwrap();
    assert_eq!(trusted_kira.accuracy, baseline_accuracy + 2);
    assert_eq!(trusted_kira.armour, baseline_armour);

    for _ in 0..4 {
        campaign.strengthen_shared_victory(&pair);
    }
    let bonded = campaign.deployment_roster(&data, &data.mission);
    let bonded_kira = bonded.iter().find(|unit| unit.id == "kira_voss").unwrap();
    assert_eq!(bonded_kira.accuracy, baseline_accuracy + 2);
    assert_eq!(bonded_kira.armour, baseline_armour + 1);
    assert_eq!(campaign.relationships[0].bond, BONDED_BOND);
    assert_eq!(campaign.relationships[0].shared_victories, 7);
}

#[test]
fn character_events_deepen_bonds_without_inventing_victories() {
    let mut campaign = CampaignState::new(&GameData::load().unwrap());
    campaign.strengthen_event_participants(&["ilya_reed".to_owned(), "mara_venn".to_owned()]);

    assert_eq!(campaign.relationships[0].bond, 2);
    assert_eq!(campaign.relationships[0].shared_victories, 0);
    assert_eq!(
        campaign.relationship_summaries("mara_venn"),
        vec!["FAMILIAR // Ilya Reed · BOND 2"]
    );
}
