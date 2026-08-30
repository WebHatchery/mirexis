//! Application flow for immediate and targeted class actions.

use super::{Game, TacticalTargeting};
use crate::ui::UiAction;

impl Game {
    pub(super) fn apply_class_action(&mut self, action: &UiAction, before_events: usize) -> bool {
        let handled = match action {
            UiAction::ActivateClassAction => {
                match self.session.activate_selected_class_action() {
                    Ok(events) => self.first_hour_ability_success(events, "Class action activated"),
                    Err(_) => self.notifications.warning("Class action is unavailable"),
                }
                true
            }
            UiAction::ArmClassAction => {
                if let Some(unit_id) = self.session.tactical.selected_unit.clone() {
                    if let Some(target_kind) = self
                        .session
                        .unit(&unit_id)
                        .and_then(|unit| crate::class_actions::target_kind(&unit.class_id))
                    {
                        self.targeting = Some(TacticalTargeting::ClassAction {
                            unit_id,
                            target_kind,
                        });
                        self.notifications.info(
                            if target_kind == crate::data::TechniqueTarget::Tile {
                                "Choose a highlighted class-action tile"
                            } else {
                                "Choose a highlighted class-action target"
                            },
                        );
                    }
                }
                true
            }
            UiAction::UseClassActionOn(target_id) => {
                let result = match self.targeting.take() {
                    Some(TacticalTargeting::ClassAction {
                        unit_id,
                        target_kind,
                    }) if target_kind != crate::data::TechniqueTarget::Tile => self
                        .session
                        .activate_class_action_on(&unit_id, target_id)
                        .map_err(|_| ()),
                    _ => Err(()),
                };
                match result {
                    Ok(events) => self.first_hour_ability_success(events, "Class action activated"),
                    Err(()) => self
                        .notifications
                        .warning("Class-action target is no longer valid"),
                }
                true
            }
            UiAction::UseClassActionOnTile(tile) => {
                let result = match self.targeting.take() {
                    Some(TacticalTargeting::ClassAction {
                        unit_id,
                        target_kind: crate::data::TechniqueTarget::Tile,
                    }) => self
                        .session
                        .activate_class_action_on_tile(&unit_id, *tile)
                        .map_err(|_| ()),
                    _ => Err(()),
                };
                match result {
                    Ok(events) => self.first_hour_ability_success(events, "Class action activated"),
                    Err(()) => self
                        .notifications
                        .warning("Class-action tile is no longer valid"),
                }
                true
            }
            _ => false,
        };
        if handled {
            self.finish_action_audio(action, before_events);
            self.enter_debrief_if_finished();
        }
        handled
    }
}
