use super::*;
use crate::campaign::CampaignState;
use crate::data::GameData;
use crate::state::{CommandCost, GameSession, RuleError};
use macroquad_toolkit::grid::TilePos;

fn unit_mut<'a>(session: &'a mut GameSession, id: &str) -> &'a mut crate::state::UnitState {
    session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == id)
        .unwrap()
}

#[test]
fn targeting_status_keeps_rule_reasons_visible() {
    for (error, label) in [
        (RuleError::WrongTeam, "WRONG TEAM"),
        (RuleError::Incapacitated, "UNIT INCAPACITATED"),
        (RuleError::OutOfRange, "OUT OF RANGE"),
        (RuleError::NoLineOfFire, "NO LINE OF FIRE"),
        (RuleError::InsufficientActionPoints, "INSUFFICIENT AP"),
    ] {
        assert_eq!(
            status_from_validation(Err::<CommandCost, _>(error)).label,
            label
        );
    }
}

#[test]
fn targeting_status_uses_the_authoritative_skill_command() {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let roster = campaign.deployment_roster(&data, &data.mission);
    let mut session = GameSession::new(&data.config, &data.mission, &roster);
    unit_mut(&mut session, "kira_voss")
        .active_skills
        .push("spotters_mark".to_owned());
    unit_mut(&mut session, "kira_voss").position = TilePos::new(4, 19);
    unit_mut(&mut session, "brood_stalker_a").position = TilePos::new(20, 19);
    let tile = TilePos::new(20, 19);
    let command = targeting_command(
        TargetingView::Skill {
            unit_id: "kira_voss",
            skill_id: "spotters_mark",
        },
        tile,
        &session,
    )
    .expect("targeting command should resolve the hostile on the tile");

    assert_eq!(
        status_from_validation(session.validate(&command)).label,
        "OUT OF RANGE"
    );
}
