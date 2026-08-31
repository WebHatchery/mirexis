use super::*;
use crate::state::{CharacterConsequence, ObjectiveState};

fn outcome(result: ObjectiveState, colonists_deployed: usize) -> MissionOutcome {
    MissionOutcome {
        result,
        colonists_deployed,
        colonists_incapacitated: Vec::new(),
        hostiles_neutralised: 0,
        materials_awarded: 0,
        biomass_awarded: 0,
        power_awarded: 0,
    }
}

#[test]
fn experience_summary_discloses_victory_award_per_deployed_colonist() {
    assert_eq!(
        experience_summary(&outcome(ObjectiveState::Victory, 3)),
        "Field experience: +20 XP each // 3 deployed colonists"
    );
}

#[test]
fn experience_summary_discloses_defeat_award_per_deployed_colonist() {
    assert_eq!(
        experience_summary(&outcome(ObjectiveState::Failed, 2)),
        "Field experience: +8 XP each // 2 deployed colonists"
    );
}

#[test]
fn completed_campaign_debrief_hides_early_operation_voice() {
    let data = crate::data::GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign.operations_completed = 2;
    assert!(show_early_debrief_voice(&campaign));

    campaign.strategy.campaign_complete = true;
    assert!(!show_early_debrief_voice(&campaign));
}

#[test]
fn squad_status_distinguishes_fielded_reserve_and_incapacitated_colonists() {
    let deployed_ids = vec!["kira_voss".to_owned()];
    let incapacitated = vec![CharacterConsequence {
        id: "kira_voss".to_owned(),
        name: "Kira Voss".to_owned(),
    }];

    assert_eq!(
        squad_status("mara_venn", &deployed_ids, &incapacitated),
        SquadStatus::Reserve
    );
    assert_eq!(
        squad_status("kira_voss", &deployed_ids, &[]),
        SquadStatus::Returned
    );
    assert_eq!(
        squad_status("kira_voss", &deployed_ids, &incapacitated),
        SquadStatus::Scar
    );
}

#[test]
fn debrief_roster_prioritizes_fielded_recruits_before_reserves() {
    let data = crate::data::GameData::load().unwrap();
    let campaign = crate::campaign::CampaignState::new(&data);
    let mut roster = campaign.roster.clone();
    let mut recruit = roster[0].clone();
    recruit.id = "sedge_drift".to_owned();
    recruit.name = "Sedge Drift".to_owned();
    roster.push(recruit);

    let deployed_ids = vec!["sedge_drift".to_owned()];
    let indices = debrief_roster_indices(&roster, &deployed_ids);

    assert_eq!(indices.first(), Some(&(roster.len() - 1)));
    assert_eq!(indices.len(), 5);
    assert_eq!(
        debrief_roster_indices(
            &campaign.roster,
            &campaign
                .roster
                .iter()
                .take(3)
                .map(|character| character.id.clone())
                .collect::<Vec<_>>()
        ),
        vec![0, 1, 2, 3, 4]
    );
}
