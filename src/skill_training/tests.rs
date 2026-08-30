use crate::campaign::CampaignState;
use crate::data::GameData;

#[test]
fn support_and_combat_classes_expose_two_data_backed_techniques() {
    let data = GameData::load().unwrap();
    for class_id in ["soldier", "defender", "scout", "medic", "engineer"] {
        let class = data
            .classes
            .iter()
            .find(|class| class.id == class_id)
            .unwrap();
        assert_eq!(class.techniques.len(), 2);
        assert_eq!(class.technique_slots, 1);
        assert!(class.techniques.iter().all(|technique| {
            !technique.description.is_empty() && technique.experience_required > 0
        }));
    }
}

#[test]
fn deployed_xp_unlocks_and_loadout_swaps_a_class_technique() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    let kira = campaign
        .roster
        .iter_mut()
        .find(|character| character.id == "kira_voss")
        .unwrap();
    kira.experience = 20;

    let learned = campaign
        .learn_skill("kira_voss", "slipstep", &data)
        .unwrap();
    assert_eq!(learned, "Slipstep");
    assert!(campaign
        .roster
        .iter()
        .find(|character| character.id == "kira_voss")
        .unwrap()
        .active_skills
        .contains(&"slipstep".to_owned()));

    campaign
        .roster
        .iter_mut()
        .find(|character| character.id == "kira_voss")
        .unwrap()
        .experience = 40;
    campaign
        .learn_skill("kira_voss", "spotters_mark", &data)
        .unwrap();
    assert!(!campaign
        .roster
        .iter()
        .find(|character| character.id == "kira_voss")
        .unwrap()
        .active_skills
        .contains(&"spotters_mark".to_owned()));
    campaign
        .toggle_skill("kira_voss", "slipstep", &data)
        .unwrap();
    campaign
        .toggle_skill("kira_voss", "spotters_mark", &data)
        .unwrap();
    assert!(!campaign
        .roster
        .iter()
        .find(|character| character.id == "kira_voss")
        .unwrap()
        .active_skills
        .contains(&"slipstep".to_owned()));
    assert!(campaign
        .roster
        .iter()
        .find(|character| character.id == "kira_voss")
        .unwrap()
        .active_skills
        .contains(&"spotters_mark".to_owned()));
}

#[test]
fn a_technique_cannot_be_learned_before_its_xp_threshold() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign.roster[0].experience = 19;
    let error = campaign
        .learn_skill("kira_voss", "slipstep", &data)
        .unwrap_err();
    assert_eq!(error, "REQUIRES 20 XP");
}
