//! Campaign and Waystation action handlers.

use super::*;

pub(super) fn apply_campaign_action(game: &mut Game, action: UiAction) -> bool {
    match action {
        UiAction::StartMission => {
            game.reset_tactical_transients(false);
            game.campaign = CampaignState::new(&game.data);
            game.active_mission = game
                .campaign
                .strategy
                .materialize_selected(&game.data, &game.campaign.colony);
            game.colony_camera = WorldCamera::colony_start(crate::colony::SETTLEMENT_CENTER);
            game.colony_explorer.reset();
            game.colony_operations_open = false;
            game.facility_upgrade_open = false;
            game.salvage_open = false;
            game.show_memorial = false;
            game.memorial_page = 0;
            game.state = AppState::Colony;
            game.last_outcome = None;
            game.autosave_campaign_only("New colony autosaved");
            true
        }
        UiAction::OpenMissionBriefing => {
            game.campaign
                .first_hour
                .opened_briefing(game.campaign.operations_completed);
            game.normalize_deployment_formation();
            game.active_mission = game
                .campaign
                .strategy
                .materialize_selected(&game.data, &game.campaign.colony);
            game.state = AppState::MissionBriefing;
            true
        }
        UiAction::SelectMission(mission_id) => {
            match game.campaign.strategy.select_mission(&mission_id) {
                Ok(()) => {
                    game.active_mission = game
                        .campaign
                        .strategy
                        .materialize_selected(&game.data, &game.campaign.colony);
                    game.notifications.success("Mission selected");
                    game.autosave_campaign_only("Mission selection autosaved");
                }
                Err(err) => game.notifications.warning(err),
            }
            true
        }
        UiAction::CompleteResearch(research_id) => {
            match game.campaign.complete_research(&research_id) {
                Ok(name) => {
                    game.campaign.first_hour.invested(name.clone());
                    game.notifications
                        .success(format!("Research complete: {}", name));
                    game.autosave_campaign_only("Research autosaved");
                }
                Err(err) => game.notifications.warning(err),
            }
            true
        }
        UiAction::ChooseContactProtocol(protocol_id) => {
            match game.campaign.strategy.choose_contact_protocol(
                &protocol_id,
                &mut game.campaign.colony,
                &game.data,
            ) {
                Ok(name) => {
                    game.notifications
                        .success(format!("Contact protocol active: {}", name));
                    game.autosave_campaign_only("Contact protocol autosaved");
                }
                Err(err) => game.notifications.warning(err),
            }
            true
        }
        UiAction::ChooseEscalationResponse(response_id) => {
            match game.campaign.strategy.choose_escalation_response(
                &response_id,
                &mut game.campaign.colony,
                &game.data,
            ) {
                Ok(name) => {
                    game.notifications
                        .success(format!("Escalation response committed: {}", name));
                    game.autosave_campaign_only("Escalation response autosaved");
                }
                Err(err) => game.notifications.warning(err),
            }
            true
        }
        UiAction::ChooseMirexisPath(path_id) => {
            match game.campaign.strategy.choose_mirexis_path(
                &path_id,
                &mut game.campaign.colony,
                &game.data,
            ) {
                Ok(name) => {
                    game.notifications
                        .success(format!("Mirexis path committed: {}", name));
                    game.autosave_campaign_only("Mirexis path autosaved");
                }
                Err(err) => game.notifications.warning(err),
            }
            true
        }
        UiAction::ChooseMutationEvolution(character_id, evolution_id) => {
            match game
                .campaign
                .choose_mutation_evolution(&character_id, &evolution_id, &game.data)
            {
                Ok(name) => {
                    game.notifications
                        .success(format!("Mutation evolved: {}", name));
                    game.autosave_campaign_only("Mutation evolution autosaved");
                }
                Err(err) => game.notifications.warning(err),
            }
            true
        }
        UiAction::ResolveCharacterEvent(character_id) => {
            match game
                .campaign
                .resolve_first_character_event_for(&character_id, &game.data)
            {
                Ok(title) => {
                    game.notifications
                        .success(format!("Event resolved: {}", title));
                    game.autosave_campaign_only("Character event autosaved");
                }
                Err(err) => game.notifications.warning(err),
            }
            true
        }
        UiAction::RecruitOutsider => {
            match game.campaign.recruit_outsider(&game.data) {
                Ok(name) => {
                    game.notifications
                        .success(format!("{} joined the colony roster", name));
                    game.autosave_campaign_only("Outsider recruitment autosaved");
                }
                Err(err) => game.notifications.warning(err),
            }
            true
        }
        UiAction::ResolveOutsiderBeat(stage, choice_id) => {
            match game
                .campaign
                .resolve_outsider_beat(&game.data, stage, &choice_id)
            {
                Ok(summary) => {
                    game.notifications.success(summary);
                    game.autosave_campaign_only("Outsider arc autosaved");
                }
                Err(err) => game.notifications.warning(err),
            }
            true
        }
        UiAction::HostCommonsMeal => {
            game.handle_commons_meal();
            true
        }
        UiAction::RunRelayScan => {
            game.handle_relay_scan();
            true
        }
        UiAction::RunIdentityStewardship => {
            game.handle_identity_stewardship();
            true
        }
        UiAction::PrepareIdentityBuilding => {
            game.handle_identity_preparation();
            true
        }
        _ => false,
    }
}
