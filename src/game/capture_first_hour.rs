//! Deterministic first-hour capture states kept apart from the general scene registry.

use super::{AppState, Game};

impl Game {
    pub(super) fn capture_first_hour_guide(&mut self) {
        self.state = AppState::Colony;
        self.campaign.first_hour.stage = crate::first_hour::FirstHourStage::SecondOperation;
        self.campaign.first_hour.help_open = true;
        let metrics = &mut self.campaign.first_hour.metrics;
        metrics.elapsed_millis = 2_846_000;
        metrics.first_city_move_millis = Some(18_000);
        metrics.first_city_interaction_millis = Some(46_000);
        metrics.first_tactical_attack_millis = Some(612_000);
        metrics.operation_one_duration_millis = Some(934_000);
        metrics.operation_one_rounds = Some(5);
        metrics.invalid_commands = 3;
        metrics.guide_opens = 2;
    }

    pub(super) fn capture_first_hour_return(&mut self) {
        self.state = AppState::Colony;
        self.colony_explorer.reset();
        self.campaign.operations_completed = 1;
        self.campaign.first_hour.stage = crate::first_hour::FirstHourStage::FirstReturnColony;
        self.campaign.first_hour.first_outcome_won = Some(false);
    }

    pub(super) fn capture_first_hour_dialogue(&mut self) {
        self.state = AppState::Colony;
        self.colony_explorer.reset();
        self.campaign.first_hour.stage = crate::first_hour::FirstHourStage::MeetCoordinator;
        let position = crate::colony_exploration::npc_position(&self.campaign, "mara_venn")
            .expect("dialogue capture includes Mara");
        self.colony_explorer
            .request_approach("mara_venn", position, &self.campaign.colony);
    }

    pub(super) fn capture_first_hour_promise(&mut self) {
        self.state = AppState::Colony;
        self.colony_explorer.reset();
        self.campaign.operations_completed = 2;
        self.campaign.first_hour.stage = crate::first_hour::FirstHourStage::Promise;
        self.campaign.first_hour.first_outcome_won = Some(true);
        self.campaign.first_hour.second_outcome_won = Some(false);
    }

    pub(super) fn capture_first_hour_operations(&mut self) {
        self.state = AppState::Colony;
        self.campaign.operations_completed = 1;
        self.campaign.first_hour.stage = crate::first_hour::FirstHourStage::SecondOperation;
        self.campaign.colony.resources.materials -=
            crate::first_hour_investment_ui::INVESTMENT_COST;
        self.campaign.first_hour.investment_name = "survey_uplink".to_owned();
        self.colony_operations_open = true;
    }
}
