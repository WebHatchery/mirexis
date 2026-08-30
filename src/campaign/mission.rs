//! Campaign-owned mission offer context derived from operational facilities.

use super::CampaignState;
use crate::colony::{BuildingKind, SIGNAL_CARTOGRAPHY_UPGRADE};
use crate::data::GameData;

impl CampaignState {
    pub(crate) fn refresh_mission_offers(&mut self, data: &GameData) {
        let offer_limit = self.mission_offer_limit();
        self.strategy
            .regenerate_missions_with_offer_limit(data, offer_limit);
    }

    pub(super) fn mission_offer_limit(&self) -> usize {
        if self
            .colony
            .has_active_upgrade(BuildingKind::CommandCentre, SIGNAL_CARTOGRAPHY_UPGRADE)
        {
            3
        } else {
            2
        }
    }
}
