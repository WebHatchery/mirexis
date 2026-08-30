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
