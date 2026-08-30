use super::*;
use crate::campaign::CampaignState;
use crate::data::GameData;
use crate::first_hour::FirstHourStage;
use crate::formation::FormationKind;
use crate::state::GameSession;
use macroquad_toolkit::grid::TilePos;

#[test]
fn first_hour_teaching_lane_is_attackable_after_the_cover_move() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign.first_hour.stage = FirstHourStage::FirstOperation;
    let mission = campaign
        .strategy
        .materialize_selected(&data, &campaign.colony);
    let mut roster = campaign.deployment_roster(&data, &mission);
    crate::formation::apply(&mut roster, &mission, &data.config, FormationKind::Wedge);
    let mut session = GameSession::new(&data.config, &mission, &roster);

    prepare_first_hour_tactical_session(&mut session, &mission);

    let cover_position = TilePos::new(FIRST_HOUR_COVER_POSITION[0], FIRST_HOUR_COVER_POSITION[1]);
    assert!(session.can_move_selected_to(cover_position));
    assert!(session.move_selected_to(cover_position));
    let target = session
        .tactical
        .units
        .iter()
        .find(|unit| unit.team == crate::data::Team::Hostile)
        .unwrap();
    assert_eq!(target.position, first_hour_attack_position());
    assert!(session.can_attack_selected(&target.id));
    assert_eq!(
        session.tactical.objective_tile,
        first_hour_objective_position()
    );
    let objective_approach = TilePos::new(
        session.tactical.objective_tile.x - 1,
        session.tactical.objective_tile.y,
    );
    let route = session
        .movement_path("kira_voss", objective_approach)
        .unwrap();
    assert_eq!(
        crate::tactical::path_cost(&route, &session.tactical.terrain_costs),
        15
    );
}
