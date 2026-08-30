use super::*;
use crate::colony::BuildingState;

fn directorate_waystation_campaign(data: &GameData) -> CampaignState {
    let mut campaign = CampaignState::new(data);
    campaign.strategy.contact_protocol_id = "directorate_requisition".to_owned();
    campaign.colony.buildings.push(BuildingState {
        id: "waystation_test".to_owned(),
        kind: BuildingKind::Waystation,
        position: [2, 10],
        level: 1,
        damaged: false,
    });
    campaign
}

#[test]
fn route_exclusive_outsider_requires_the_matching_operational_waystation() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    assert!(!campaign
        .roster
        .iter()
        .any(|character| character.id == "veya_orn"));
    assert!(!campaign.outsider_recruit_available(&data));
    assert!(campaign.recruit_outsider(&data).is_err());

    campaign.strategy.contact_protocol_id = "brood_cultivation".to_owned();
    campaign.colony.buildings.push(BuildingState {
        id: "waystation_wrong_route".to_owned(),
        kind: BuildingKind::Waystation,
        position: [2, 10],
        level: 1,
        damaged: false,
    });
    assert!(!campaign.outsider_recruit_available(&data));
    assert!(campaign.recruit_outsider(&data).is_err());
}

#[test]
fn outsider_recruitment_adds_a_reserve_with_a_distinct_origin_profile() {
    let data = GameData::load().unwrap();
    let mut campaign = directorate_waystation_campaign(&data);
    let materials_before = campaign.colony.resources.materials;

    assert!(campaign.outsider_recruit_available(&data));
    assert_eq!(campaign.recruit_outsider(&data).unwrap(), "Veya Orn");
    let veya = campaign
        .roster
        .iter()
        .find(|character| character.id == "veya_orn")
        .unwrap();
    assert!(!veya.deployment_selected);
    assert_eq!(veya.origin, "Directorate Exile");
    assert_eq!(campaign.colony.resources.materials, materials_before - 30);
    let unit = campaign.derived_character_unit("veya_orn", &data).unwrap();
    let class_bonus = data
        .classes
        .iter()
        .find(|class| class.id == "soldier")
        .unwrap()
        .accuracy_bonus;
    assert_eq!(
        unit.accuracy,
        data.recruitable_roster[0].accuracy + class_bonus + 4
    );
    campaign.roster[0].deployment_selected = false;
    campaign
        .roster
        .iter_mut()
        .find(|character| character.id == "veya_orn")
        .unwrap()
        .deployment_selected = true;
    assert!(campaign
        .deployment_roster(&data, &data.mission)
        .iter()
        .any(|unit| unit.id == "veya_orn"));
    assert!(campaign.outsider_arc_available());
}

#[test]
fn outsider_arc_persists_two_disagreements_and_a_third_closing_beat() {
    let data = GameData::load().unwrap();
    let mut campaign = directorate_waystation_campaign(&data);
    campaign.recruit_outsider(&data).unwrap();

    campaign.resolve_outsider_beat(0, "shelter_cipher").unwrap();
    assert_eq!(campaign.outsider_arc_stage, 1);
    assert_eq!(campaign.outsider_disagreements, 1);
    assert_eq!(campaign.roster[5].event_legacies[0].stat, "accuracy");

    campaign.operations_completed = 1;
    campaign
        .resolve_outsider_beat(1, "kira_walks_range")
        .unwrap();
    assert_eq!(campaign.outsider_arc_stage, 2);
    assert_eq!(campaign.outsider_disagreements, 2);

    campaign.operations_completed = 2;
    campaign.resolve_outsider_beat(2, "stay_on_line").unwrap();
    assert_eq!(campaign.outsider_arc_stage, 3);
    assert_eq!(campaign.outsider_final_choice, "stay_on_line");
    assert!(!campaign.outsider_arc_available());

    let saved = serde_json::to_value(&campaign).unwrap();
    let restored: CampaignState = serde_json::from_value(saved).unwrap();
    assert_eq!(restored.outsider_disagreements, 2);
    assert_eq!(restored.outsider_final_choice, "stay_on_line");
    assert_eq!(restored.roster.len(), 6);
}
