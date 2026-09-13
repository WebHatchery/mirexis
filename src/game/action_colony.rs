//! Colony construction, roster, training, treatment, and equipment actions.

use super::*;

pub(super) fn apply_colony_management_action(game: &mut Game, action: UiAction) -> bool {
    match action {
        UiAction::OpenGeneLab => {
            if game
                .campaign
                .colony
                .has_facility(crate::colony::BuildingKind::GeneLab)
            {
                game.state = AppState::GeneLab;
            } else {
                game.notifications.warning("The Gene Lab is offline");
            }
            true
        }
        UiAction::SelectColonist(character_id) => {
            match game.campaign.select_character(&character_id) {
                Ok(()) => {
                    game.roster_inspection_id = None;
                    game.autosave_campaign_only("Roster selection autosaved");
                }
                Err(err) => game.notifications.warning(err),
            }
            true
        }
        UiAction::InspectRosterEquipment(equipment_id) => {
            game.roster_inspection_id = Some(equipment_id);
            true
        }
        UiAction::SelectConstruction(kind) => {
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
            true
        }
        UiAction::ConstructBuilding(kind, position) => {
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
            true
        }
        UiAction::RepairBuilding(building_id) => {
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
            true
        }
        UiAction::TrainSelected(class_id) => {
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
            true
        }
        _ => false,
    }
}
