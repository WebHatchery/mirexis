//! Strategic Salvage Yard actions and their application-level persistence feedback.

use super::Game;
use crate::ui::UiAction;

impl Game {
    pub(super) fn apply_salvage_action(&mut self, action: &UiAction) -> bool {
        match action {
            UiAction::OpenSalvage => {
                if self
                    .campaign
                    .colony
                    .has_facility(crate::colony::BuildingKind::SalvageYard)
                {
                    self.salvage_open = true;
                } else {
                    self.notifications.warning("The Salvage Yard is offline");
                }
                true
            }
            UiAction::ProcessSalvage(choice) => {
                match self.campaign.process_salvage(*choice) {
                    Ok(summary) => {
                        self.salvage_open = false;
                        self.notifications.success(summary);
                        self.autosave_campaign_only("Salvage decision autosaved");
                    }
                    Err(err) => self.notifications.warning(err),
                }
                true
            }
            UiAction::CloseSalvage => {
                self.salvage_open = false;
                true
            }
            _ => false,
        }
    }
}
