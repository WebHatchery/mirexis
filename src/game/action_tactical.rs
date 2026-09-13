//! Tactical movement, attack, targeting, and equipment actions.

use super::*;

pub(super) fn apply_tactical_action(game: &mut Game, action: UiAction) -> bool {
    match action {
        UiAction::SelectTile(tile) => {
            game.session.select_tile(tile);
            if game
                .session
                .selected_unit()
                .is_some_and(|unit| unit.position == tile)
            {
                game.campaign.first_hour.selected();
            }
            true
        }
        UiAction::SelectNextReady => {
            if !game.select_next_ready_for_first_hour() {
                game.notifications.info("No colonist has actions remaining");
            }
            true
        }
        UiAction::MoveSelected(tile) => {
            if game.session.move_selected_to(tile) {
                game.record_first_hour_move(tile);
                game.notifications.info("Colonist repositioned");
            } else {
                game.notifications
                    .warning("Tile is outside the valid movement envelope");
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
            if let Some(unit_id) = game.session.tactical.selected_unit.clone() {
                game.targeting = Some(TacticalTargeting::Equipment {
                    unit_id,
                    equipment_id,
                });
                game.notifications
                    .info("Choose a highlighted equipment target");
            }
            true
        }
        UiAction::CancelTargeting => {
            game.targeting = None;
            game.notifications.info("Tactical targeting cancelled");
            true
        }
        UiAction::UseEquipmentOn(target_id) => {
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
            true
        }
        _ => false,
    }
}
