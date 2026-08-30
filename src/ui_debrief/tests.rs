use super::*;
use crate::state::ObjectiveState;

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
