//! Central dispatch for actions emitted by the game UI.

use super::*;

impl Game {
    pub fn apply_action(&mut self, action: UiAction) {
        if self.apply_audio_action(&action) {
            return;
        }
        if self.apply_first_hour_action(&action) {
            return;
        }
        if self.apply_field_notes_action(&action) {
            return;
        }
        if self.apply_memorial_action(&action) {
            return;
        }
        if self.apply_colony_action(&action) {
            return;
        }
        if self.guard_destructive_action(&action) {
            return;
        }
        let audio_event_count = self.session.tactical.event_log.len();
        let audio_action = action.clone();
        if !matches!(&action, UiAction::EndPhase) {
            self.end_phase_armed = false;
        }
        if self.apply_skill_action(&action, audio_event_count) {
            return;
        }
        if self.apply_class_action(&action, audio_event_count) {
            return;
        }
        if self.apply_facility_upgrade_action(&action) {
            return;
        }
        if self.apply_salvage_action(&action) {
            return;
        }
        let handled = apply_action_group_a(self, action.clone())
            || apply_action_group_b(self, action.clone())
            || apply_action_group_c(self, action.clone())
            || apply_action_group_d(self, action.clone())
            || apply_action_group_e(self, action.clone())
            || apply_action_group_f(self, action.clone())
            || apply_action_group_g(self, action.clone());
        if !handled {
            unreachable!("unhandled application action reached the game state match");
        }
        self.finish_action_audio(&audio_action, audio_event_count);
        self.enter_debrief_if_finished();
    }
}

fn apply_action_group_a(game: &mut Game, action: UiAction) -> bool {
    match action {
        UiAction::StartMission => {
            {
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
            }
            true
        }
        UiAction::OpenMissionBriefing => {
            {
                game.campaign
                    .first_hour
                    .opened_briefing(game.campaign.operations_completed);
                game.normalize_deployment_formation();
                game.active_mission = game
                    .campaign
                    .strategy
                    .materialize_selected(&game.data, &game.campaign.colony);
                game.state = AppState::MissionBriefing;
            }
            true
        }
        UiAction::SelectMission(mission_id) => {
            {
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
            }
            true
        }
        UiAction::CompleteResearch(research_id) => {
            {
                match game.campaign.complete_research(&research_id) {
                    Ok(name) => {
                        game.campaign.first_hour.invested(name.clone());
                        game.notifications
                            .success(format!("Research complete: {}", name));
                        game.autosave_campaign_only("Research autosaved");
                    }
                    Err(err) => game.notifications.warning(err),
                }
            }
            true
        }
        UiAction::ChooseContactProtocol(protocol_id) => {
            {
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
            }
            true
        }
        UiAction::ChooseEscalationResponse(response_id) => {
            {
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
            }
            true
        }
        _ => false,
    }
}

fn apply_action_group_b(game: &mut Game, action: UiAction) -> bool {
    match action {
        UiAction::ChooseMirexisPath(path_id) => {
            {
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
            }
            true
        }
        UiAction::ChooseMutationEvolution(character_id, evolution_id) => {
            {
                match game.campaign.choose_mutation_evolution(
                    &character_id,
                    &evolution_id,
                    &game.data,
                ) {
                    Ok(name) => {
                        game.notifications
                            .success(format!("Mutation evolved: {}", name));
                        game.autosave_campaign_only("Mutation evolution autosaved");
                    }
                    Err(err) => game.notifications.warning(err),
                }
            }
            true
        }
        UiAction::ResolveCharacterEvent(character_id) => {
            {
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
            {
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

fn apply_action_group_c(game: &mut Game, action: UiAction) -> bool {
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
            {
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
            }
            true
        }
        UiAction::Continue => {
            game.load_game();
            true
        }
        UiAction::ReturnToTitle => {
            {
                game.reset_tactical_transients(false);
                game.facility_upgrade_open = false;
                game.salvage_open = false;
                game.show_memorial = false;
                game.memorial_page = 0;
                game.roster_inspection_id = None;
                game.state = AppState::Title;
            }
            true
        }
        UiAction::ReturnToColony => {
            {
                game.reset_tactical_transients(false);
                game.facility_upgrade_open = false;
                game.salvage_open = false;
                game.show_memorial = false;
                game.memorial_page = 0;
                game.roster_inspection_id = None;
                game.state =
                    if crate::demo::campaign_is_complete(game.campaign.operations_completed) {
                        AppState::DemoComplete
                    } else {
                        AppState::Colony
                    };
                game.ensure_first_hour_recovery_reserve();
                game.campaign.first_hour.returned_to_colony();
                game.autosave_campaign_only("Colony entry autosaved");
            }
            true
        }
        UiAction::ToggleOperations => {
            {
                game.colony_operations_open = !game.colony_operations_open;
            }
            true
        }
        UiAction::OpenRoster => {
            game.state = AppState::Roster;
            true
        }
        _ => false,
    }
}

fn apply_action_group_d(game: &mut Game, action: UiAction) -> bool {
    match action {
        UiAction::OpenGeneLab => {
            {
                if game
                    .campaign
                    .colony
                    .has_facility(crate::colony::BuildingKind::GeneLab)
                {
                    game.state = AppState::GeneLab;
                } else {
                    game.notifications.warning("The Gene Lab is offline");
                }
            }
            true
        }
        UiAction::SelectColonist(character_id) => {
            {
                match game.campaign.select_character(&character_id) {
                    Ok(()) => {
                        game.roster_inspection_id = None;
                        game.autosave_campaign_only("Roster selection autosaved");
                    }
                    Err(err) => game.notifications.warning(err),
                }
            }
            true
        }
        UiAction::InspectRosterEquipment(equipment_id) => {
            {
                game.roster_inspection_id = Some(equipment_id);
            }
            true
        }
        UiAction::SelectConstruction(kind) => {
            {
                if kind == crate::colony::BuildingKind::GeneLab
                    && !game.campaign.strategy.contact_complete
                {
                    game.notifications
                        .warning("The Gene Lab unlocks in Adaptation");
                    return true;
                }
                if kind == crate::colony::BuildingKind::Waystation
                    && !game.campaign.waystation_unlocked()
                {
                    game.notifications
                        .warning("The Waystation unlocks through Contact or Adaptation");
                    return true;
                }
                match game.campaign.colony.select_construction(kind) {
                    Ok(()) => game.notifications.info(format!("Planning {}", kind.name())),
                    Err(err) => game.notifications.warning(err),
                }
            }
            true
        }
        UiAction::ConstructBuilding(kind, position) => {
            {
                if kind == crate::colony::BuildingKind::GeneLab
                    && !game.campaign.strategy.contact_complete
                {
                    game.notifications
                        .warning("The Gene Lab unlocks in Adaptation");
                    return true;
                }
                if kind == crate::colony::BuildingKind::Waystation
                    && !game.campaign.waystation_unlocked()
                {
                    game.notifications
                        .warning("The Waystation unlocks through Contact or Adaptation");
                    return true;
                }
                match game.campaign.colony.place_construction(kind, position) {
                    Ok(_) => {
                        game.campaign.first_hour.invested(kind.name());
                        game.notifications
                            .success(format!("{} construction planned", kind.name()));
                        game.autosave_campaign_only("Construction plan autosaved");
                    }
                    Err(err) => game.notifications.warning(err),
                }
            }
            true
        }
        UiAction::RepairBuilding(building_id) => {
            {
                match game.campaign.colony.repair_building(&building_id) {
                    Ok(repair) => {
                        game.notifications.success(format!(
                            "{} repaired for {} materials",
                            repair.building_name, repair.materials_spent
                        ));
                        game.autosave_campaign_only("Colony repair autosaved");
                    }
                    Err(err) => game.notifications.warning(err),
                }
            }
            true
        }
        _ => false,
    }
}

fn apply_action_group_e(game: &mut Game, action: UiAction) -> bool {
    match action {
        UiAction::TrainSelected(class_id) => {
            {
                let character_id = game.campaign.selected_character_id.clone();
                match game
                    .campaign
                    .train_character(&character_id, &class_id, &game.data)
                {
                    Ok(cost) => {
                        game.campaign.first_hour.invested("squad training");
                        game.notifications
                            .success(format!("Training complete · {} materials", cost));
                        game.autosave_campaign_only("Training autosaved");
                    }
                    Err(err) => game.notifications.warning(err),
                }
            }
            true
        }
        UiAction::TreatInjury => {
            match game.campaign.treat_first_injury() {
                Ok(name) => {
                    game.campaign.first_hour.invested("priority treatment");
                    game.notifications
                        .success(format!("{} received priority treatment", name));
                    game.autosave_campaign_only("Treatment autosaved");
                }
                Err(err) => game.notifications.warning(err),
            }
            true
        }
        UiAction::CraftSelected(equipment_id) => {
            {
                let character_id = game.campaign.selected_character_id.clone();
                match game
                    .campaign
                    .craft_equipment(&character_id, &equipment_id, &game.data)
                {
                    Ok(cost) => {
                        game.campaign.first_hour.invested("field equipment");
                        game.notifications.success(if cost == 0 {
                            "Equipment issued · salvage prototype consumed".to_owned()
                        } else {
                            format!("Equipment issued · {} materials", cost)
                        });
                        game.autosave_campaign_only("Workshop change autosaved");
                    }
                    Err(err) => game.notifications.warning(err),
                }
            }
            true
        }
        UiAction::SelectTile(tile) => {
            {
                game.session.select_tile(tile);
                if game
                    .session
                    .selected_unit()
                    .is_some_and(|unit| unit.position == tile)
                {
                    game.campaign.first_hour.selected();
                }
            }
            true
        }
        UiAction::SelectNextReady => {
            {
                if !game.select_next_ready_for_first_hour() {
                    game.notifications.info("No colonist has actions remaining");
                }
            }
            true
        }
        UiAction::MoveSelected(tile) => {
            {
                if game.session.move_selected_to(tile) {
                    game.record_first_hour_move(tile);
                    game.notifications.info("Colonist repositioned");
                } else {
                    game.notifications
                        .warning("Tile is outside the valid movement envelope");
                }
            }
            true
        }
        UiAction::AttackSelected(target_id) => {
            match game.session.attack_selected(&target_id) {
                Ok(events) => {
                    game.campaign.first_hour.attacked();
                    game.notifications.info(format!(
                        "Attack resolved — {} tactical events",
                        events.len()
                    ));
                }
                Err(_) => game.notifications.warning("No valid firing solution"),
            }
            true
        }
        _ => false,
    }
}

fn apply_action_group_f(game: &mut Game, action: UiAction) -> bool {
    match action {
        UiAction::AttackCover(position) => {
            match game.session.attack_selected_cover(position) {
                Ok(events) => game.notifications.info(
                    events
                        .last()
                        .map(crate::ui_widgets::event_summary)
                        .unwrap_or_else(|| "Cover struck".to_owned()),
                ),
                Err(_) => game
                    .notifications
                    .warning("Cover is outside the firing solution"),
            }
            true
        }
        UiAction::InteractObjective => {
            game.handle_objective_interaction();
            true
        }
        UiAction::ActivateMutation => {
            match game.session.activate_selected_mutation() {
                Ok(events) => {
                    game.campaign.first_hour.used_ability();
                    game.notifications.success(
                        events
                            .first()
                            .map(crate::ui_widgets::event_summary)
                            .unwrap_or_else(|| "Mutation gift activated".to_owned()),
                    );
                }
                Err(_) => game.notifications.warning("Mutation gift is unavailable"),
            }
            true
        }
        UiAction::SetOverwatch => {
            match game.session.set_selected_overwatch() {
                Ok(_) => game
                    .notifications
                    .success("Overwatch armed · first hostile movement in range draws fire"),
                Err(_) => game
                    .notifications
                    .warning("Selected colonist cannot enter overwatch"),
            }
            true
        }
        UiAction::ArmEquipment(equipment_id) => {
            {
                if let Some(unit_id) = game.session.tactical.selected_unit.clone() {
                    game.targeting = Some(TacticalTargeting::Equipment {
                        unit_id,
                        equipment_id,
                    });
                    game.notifications
                        .info("Choose a highlighted equipment target");
                }
            }
            true
        }
        UiAction::CancelTargeting => {
            {
                game.targeting = None;
                game.notifications.info("Tactical targeting cancelled");
            }
            true
        }
        UiAction::UseEquipmentOn(target_id) => {
            {
                let result = match game.targeting.take() {
                    Some(TacticalTargeting::Equipment {
                        unit_id,
                        equipment_id,
                    }) => game
                        .session
                        .use_equipment(&unit_id, &equipment_id, &target_id)
                        .map_err(|_| ()),
                    _ => Err(()),
                };
                match result {
                    Ok(events) => game.first_hour_ability_success(events, "Field equipment used"),
                    Err(()) => game
                        .notifications
                        .warning("Equipment target is no longer valid"),
                }
            }
            true
        }
        _ => false,
    }
}

fn apply_action_group_g(game: &mut Game, action: UiAction) -> bool {
    match action {
        UiAction::EndPhase => {
            {
                let ready = crate::phase_readiness::ready_count(&game.session);
                if ready > 0 && !game.end_phase_armed {
                    game.end_phase_armed = true;
                    game.notifications.warning(format!(
                        "{} colonist{} still ready · end phase again to confirm",
                        ready,
                        if ready == 1 { "" } else { "s" }
                    ));
                    return true;
                }
                game.end_phase_armed = false;
                game.targeting = None;
                let first_event = game.session.tactical.event_log.len();
                game.session.end_player_phase(&game.data.config);
                game.campaign.first_hour.ended_phase();
                game.phase_replay
                    .start(&game.session.tactical.event_log[first_event..]);
                game.notifications.info(format!(
                    "Enemy activity resolved — round {}",
                    game.session.tactical.round
                ));
            }
            true
        }
        UiAction::SkipPhaseReplay => {
            game.phase_replay.clear();
            true
        }
        UiAction::ToggleTacticalHelp => {
            {
                game.show_tactical_help = !game.show_tactical_help;
                game.show_battle_log = false;
                game.campaign.first_hour.help_open = false;
                game.targeting = None;
            }
            true
        }
        UiAction::ToggleBattleLog => {
            {
                game.show_battle_log = !game.show_battle_log;
                game.show_tactical_help = false;
                game.campaign.first_hour.help_open = false;
                game.targeting = None;
            }
            true
        }
        UiAction::ToggleTacticalPanel => {
            {
                game.tactical_panel_open = !game.tactical_panel_open;
                if !game.tactical_panel_open {
                    game.targeting = None;
                }
            }
            true
        }
        UiAction::SetBattleLogFilter(filter) => {
            game.battle_log_filter = filter;
            true
        }
        UiAction::Save => {
            game.save_game();
            true
        }
        UiAction::Load => {
            game.load_game();
            true
        }
        _ => false,
    }
}
