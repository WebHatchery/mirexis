//! Application integration for serialized first-hour guidance.

use super::{AppState, Game};
use crate::ui::{self, UiAction};
use macroquad_toolkit::ui::VirtualUi;

impl Game {
    pub(super) fn draw_first_hour(&self, ui: &VirtualUi, actions: &mut Vec<UiAction>) {
        if self.state != AppState::Title {
            crate::first_hour_ui::draw(
                &self.campaign.first_hour,
                ui::pointer_position(ui),
                actions,
            );
        }
    }

    pub(super) fn apply_first_hour_action(&mut self, action: &UiAction) -> bool {
        let save_message = match action {
            UiAction::AdvanceFirstHour => {
                self.campaign.first_hour.advance_arrival();
                "First-hour goal autosaved"
            }
            UiAction::AcknowledgeColonist(character_id) => {
                self.campaign.first_hour.acknowledge_colonist(character_id);
                "Colony introduction autosaved"
            }
            UiAction::ToggleFirstHourHelp => {
                self.campaign.first_hour.help_open = !self.campaign.first_hour.help_open;
                return true;
            }
            UiAction::SkipFirstHourTutorial => {
                self.campaign.first_hour.guidance_enabled = false;
                self.campaign.first_hour.help_open = false;
                "Tutorial prompts skipped; campaign goals preserved"
            }
            UiAction::RestartFirstHourTutorial => {
                self.campaign
                    .first_hour
                    .restart(self.campaign.operations_completed);
                "First-hour guide restarted"
            }
            _ => return false,
        };
        self.autosave_campaign_only(save_message);
        true
    }

    pub(super) fn record_first_hour_outcome(&mut self) {
        let won = self
            .last_outcome
            .as_ref()
            .is_some_and(|outcome| outcome.result == crate::state::ObjectiveState::Victory);
        self.campaign
            .first_hour
            .operation_resolved(self.campaign.operations_completed, won);
    }

    pub(super) fn first_hour_ability_success(
        &mut self,
        events: Vec<crate::tactical::BattleEvent>,
        fallback: &str,
    ) {
        self.campaign.first_hour.used_ability();
        self.notifications.success(
            events
                .first()
                .map(crate::ui_widgets::event_summary)
                .unwrap_or_else(|| fallback.to_owned()),
        );
    }
}
