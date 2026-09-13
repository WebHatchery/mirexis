//! Campaign navigation, deployment, and screen transition actions.

use super::*;

pub(super) fn apply_navigation_action(game: &mut Game, action: UiAction) -> bool {
    match action {
        UiAction::DeployMission => {
            match game.campaign.prepare_deployment(&game.data) {
                Ok(food_cost) => {
                    game.campaign
                        .first_hour
                        .deployed(game.campaign.operations_completed);
                    game.normalize_deployment_formation();
                    let mut roster = game
                        .campaign
                        .deployment_roster(&game.data, &game.active_mission);
                    crate::formation::apply(
                        &mut roster,
                        &game.active_mission,
                        &game.data.config,
                        game.deployment_formation,
                    );
                    game.session = game.create_first_hour_session(&roster);
                    game.reset_tactical_transients(true);
                    game.tactical_camera =
                        WorldCamera::tactical_start(game.session.tactical.selected_tile);
                    game.state = AppState::Tactical;
                    game.autosave_current("Deployment autosaved");
                    game.notifications.success(format!(
                        "{} deployed · {} food committed",
                        game.active_mission.name, food_cost
                    ));
                }
                Err(err) => game.notifications.warning(err),
            }
            true
        }
        UiAction::CycleFormation => {
            game.cycle_deployment_formation();
            true
        }
        UiAction::ToggleDeployment(character_id) => {
            let _ = game.campaign.select_character(&character_id);
            match game.campaign.toggle_deployment(&character_id) {
                Ok(selected) => {
                    game.notifications.info(if selected {
                        "Colonist assigned to the deployment squad"
                    } else {
                        "Colonist moved to reserve"
                    });
                    game.autosave_campaign_only("Squad selection autosaved");
                }
                Err(err) => game.notifications.warning(err),
            }
            true
        }
        UiAction::Continue => {
            game.load_game();
            true
        }
        UiAction::ReturnToTitle => {
            game.reset_tactical_transients(false);
            game.facility_upgrade_open = false;
            game.salvage_open = false;
            game.show_memorial = false;
            game.memorial_page = 0;
            game.roster_inspection_id = None;
            game.state = AppState::Title;
            true
        }
        UiAction::ReturnToColony => {
            game.reset_tactical_transients(false);
            game.facility_upgrade_open = false;
            game.salvage_open = false;
            game.show_memorial = false;
            game.memorial_page = 0;
            game.roster_inspection_id = None;
            game.state = if crate::demo::campaign_is_complete(game.campaign.operations_completed) {
                AppState::DemoComplete
            } else {
                AppState::Colony
            };
            game.ensure_first_hour_recovery_reserve();
            game.campaign.first_hour.returned_to_colony();
            game.autosave_campaign_only("Colony entry autosaved");
            true
        }
        UiAction::ToggleOperations => {
            game.colony_operations_open = !game.colony_operations_open;
            true
        }
        UiAction::OpenRoster => {
            game.state = AppState::Roster;
            true
        }
        _ => false,
    }
}
