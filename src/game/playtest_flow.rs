//! Runtime sampling for persisted Phase 1 playtest evidence.

use super::{AppState, Game};
use crate::first_hour::FirstHourStage;

impl Game {
    pub(super) fn update_playtest_metrics(&mut self, dt: f32) {
        if self.state == AppState::Title
            || self.campaign.first_hour.stage == FirstHourStage::Complete
        {
            return;
        }
        self.campaign.first_hour.metrics.tick(dt);
        if self.state == AppState::Colony && self.colony_explorer.has_left_start() {
            self.campaign.first_hour.metrics.city_moved();
        }
    }
}
