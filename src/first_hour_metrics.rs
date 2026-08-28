//! Persisted quantitative evidence for Phase 1 playtest sessions.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub(crate) struct FirstHourMetrics {
    #[serde(default)]
    pub elapsed_millis: u64,
    #[serde(default)]
    pub first_city_move_millis: Option<u64>,
    #[serde(default)]
    pub first_city_interaction_millis: Option<u64>,
    #[serde(default)]
    pub first_tactical_attack_millis: Option<u64>,
    #[serde(default)]
    pub operation_one_duration_millis: Option<u64>,
    #[serde(default)]
    pub operation_one_rounds: Option<u32>,
    #[serde(default)]
    pub operation_two_duration_millis: Option<u64>,
    #[serde(default)]
    pub operation_two_rounds: Option<u32>,
    #[serde(default)]
    pub invalid_commands: u32,
    #[serde(default)]
    pub guide_opens: u32,
    #[serde(default)]
    operation_started_millis: Option<u64>,
}

impl FirstHourMetrics {
    pub(crate) fn tick(&mut self, dt: f32) {
        let frame_millis = (dt.max(0.0) * 1_000.0).round() as u64;
        self.elapsed_millis = self.elapsed_millis.saturating_add(frame_millis);
    }

    pub(crate) fn city_moved(&mut self) {
        self.first_city_move_millis
            .get_or_insert(self.elapsed_millis);
    }

    pub(crate) fn city_interacted(&mut self) {
        self.first_city_interaction_millis
            .get_or_insert(self.elapsed_millis);
    }

    pub(crate) fn tactical_attack(&mut self) {
        self.first_tactical_attack_millis
            .get_or_insert(self.elapsed_millis);
    }

    pub(crate) fn operation_started(&mut self) {
        self.operation_started_millis = Some(self.elapsed_millis);
    }

    pub(crate) fn operation_resolved(&mut self, operation_number: u32, rounds: u32) {
        let duration = self
            .operation_started_millis
            .take()
            .map(|start| self.elapsed_millis.saturating_sub(start));
        match operation_number {
            1 => {
                self.operation_one_duration_millis = duration;
                self.operation_one_rounds = Some(rounds);
            }
            2 => {
                self.operation_two_duration_millis = duration;
                self.operation_two_rounds = Some(rounds);
            }
            _ => {}
        }
    }

    pub(crate) fn invalid_command(&mut self) {
        self.invalid_commands = self.invalid_commands.saturating_add(1);
    }

    pub(crate) fn opened_guide(&mut self) {
        self.guide_opens = self.guide_opens.saturating_add(1);
    }
}

#[cfg(test)]
mod tests;
