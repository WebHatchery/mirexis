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
            UiAction::ChooseFirstHourInvestment(investment_id) => {
                if self.campaign.first_hour.stage
                    != crate::first_hour::FirstHourStage::MakeInvestment
                {
                    self.notifications
                        .warning("The preparation window is closed");
                    return true;
                }
                let cost = crate::first_hour_investment_ui::INVESTMENT_COST;
                if self.campaign.colony.resources.materials < cost {
                    self.notifications
                        .warning("Recover 24 materials before choosing a preparation");
                    return true;
                }
                let label = crate::first_hour_investment_ui::label(investment_id);
                if label == "Unknown preparation" {
                    self.notifications.warning("Unknown first-hour preparation");
                    return true;
                }
                self.campaign.colony.resources.materials -= cost;
                self.campaign.first_hour.invested(investment_id.clone());
                self.notifications
                    .success(format!("{label} prepared for the second operation"));
                "First-hour investment autosaved"
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

    pub(super) fn ensure_first_hour_recovery_reserve(&mut self) {
        if self.campaign.first_hour.stage == crate::first_hour::FirstHourStage::FirstReturn
            && self.campaign.colony.resources.materials
                < crate::first_hour_investment_ui::INVESTMENT_COST
        {
            self.campaign.colony.resources.materials =
                crate::first_hour_investment_ui::INVESTMENT_COST;
            self.notifications
                .info("Emergency stores restored 24 materials for one viable preparation");
        }
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
