//! Persistent first-session direction and contextual tactical teaching.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub(crate) enum FirstHourStage {
    #[default]
    Arrival,
    MeetCoordinator,
    PrepareFirstOperation,
    FirstBriefing,
    FirstOperation,
    FirstReturn,
    FirstReturnColony,
    MakeInvestment,
    SecondOperation,
    SecondOperationTactical,
    SecondReturn,
    Promise,
    Complete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub(crate) enum TacticalLesson {
    #[default]
    Select,
    MoveToCover,
    Attack,
    EnemyPhase,
    Objective,
    Ability,
    ApplyLearning,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct FirstHourProgress {
    #[serde(default)]
    pub stage: FirstHourStage,
    #[serde(default)]
    pub lesson: TacticalLesson,
    #[serde(default = "enabled")]
    pub guidance_enabled: bool,
    #[serde(default)]
    pub help_open: bool,
    #[serde(default)]
    pub first_outcome_won: Option<bool>,
    #[serde(default)]
    pub investment_name: String,
    #[serde(default)]
    pub second_outcome_won: Option<bool>,
    #[serde(default)]
    pub metrics: crate::first_hour_metrics::FirstHourMetrics,
}

impl Default for FirstHourProgress {
    fn default() -> Self {
        Self {
            stage: FirstHourStage::Arrival,
            lesson: TacticalLesson::Select,
            guidance_enabled: true,
            help_open: false,
            first_outcome_won: None,
            investment_name: String::new(),
            second_outcome_won: None,
            metrics: crate::first_hour_metrics::FirstHourMetrics::default(),
        }
    }
}

fn enabled() -> bool {
    true
}

impl FirstHourProgress {
    pub(crate) fn primary_goal(&self) -> &'static str {
        match self.stage {
            FirstHourStage::Arrival => "Tap BEGIN ARRIVAL to answer the refuge distress call.",
            FirstHourStage::MeetCoordinator => "Tap Mara Venn's speech marker, then tap CONTINUE.",
            FirstHourStage::PrepareFirstOperation => {
                "Tap OPERATIONS, inspect GLASSROOT, then tap BRIEF SELECTED MISSION."
            }
            FirstHourStage::FirstBriefing => {
                "Tap colonists to choose up to three, then tap DEPLOY SQUAD."
            }
            FirstHourStage::FirstOperation | FirstHourStage::SecondOperationTactical => {
                lesson_prompt(self.lesson)
            }
            FirstHourStage::FirstReturn => "Tap RETURN TO COLONY, then tap Ilya Reed's marker.",
            FirstHourStage::FirstReturnColony => {
                "Tap Ilya Reed's speech marker, then tap CONTINUE."
            }
            FirstHourStage::MakeInvestment => {
                "Tap OPERATIONS and choose one affordable preparation investment."
            }
            FirstHourStage::SecondOperation => {
                "Tap OPERATIONS, choose the next mission, then tap BRIEF SELECTED MISSION."
            }
            FirstHourStage::SecondReturn => {
                "Tap RETURN TO COLONY to see what the two operations changed."
            }
            FirstHourStage::Promise => "Read the colony consequence, then tap CONTINUE CAMPAIGN.",
            FirstHourStage::Complete => {
                "Protect the colony before the Directorate assault reaches Mirexis."
            }
        }
    }

    pub(crate) fn visible_goal(&self) -> &'static str {
        if self.guidance_enabled {
            return self.primary_goal();
        }
        match self.stage {
            FirstHourStage::FirstOperation => {
                "Secure the refuge and neutralise the remaining Brood."
            }
            FirstHourStage::SecondOperationTactical => {
                "Complete the mission and neutralise remaining hostiles."
            }
            _ => self.primary_goal(),
        }
    }

    pub(crate) fn colony_guidance_target(&self) -> Option<&'static str> {
        if !self.guidance_enabled {
            return None;
        }
        match self.stage {
            FirstHourStage::MeetCoordinator => Some("mara_venn"),
            FirstHourStage::FirstReturnColony => Some("ilya_reed"),
            _ => None,
        }
    }

    pub(crate) fn advance_arrival(&mut self) {
        if self.stage == FirstHourStage::Arrival {
            self.stage = FirstHourStage::MeetCoordinator;
        } else if self.stage == FirstHourStage::Promise {
            self.stage = FirstHourStage::Complete;
            self.help_open = false;
        }
    }

    pub(crate) fn acknowledge_colonist(&mut self, id: &str) {
        self.metrics.city_interacted();
        if self.stage == FirstHourStage::MeetCoordinator && id == "mara_venn" {
            self.stage = FirstHourStage::PrepareFirstOperation;
        } else if self.stage == FirstHourStage::FirstReturnColony && id == "ilya_reed" {
            self.stage = FirstHourStage::MakeInvestment;
        }
    }

    pub(crate) fn deployed(&mut self, operations_completed: u32) {
        self.metrics.operation_started();
        if operations_completed == 0 {
            self.stage = FirstHourStage::FirstOperation;
            self.lesson = TacticalLesson::Select;
        } else if operations_completed == 1 {
            self.entered_second_operation_tactical();
        }
    }

    pub(crate) fn entered_second_operation_tactical(&mut self) {
        if self.stage == FirstHourStage::SecondOperation {
            self.stage = FirstHourStage::SecondOperationTactical;
            self.lesson = TacticalLesson::ApplyLearning;
        }
    }

    pub(crate) fn is_tactical_stage(&self) -> bool {
        matches!(
            self.stage,
            FirstHourStage::FirstOperation | FirstHourStage::SecondOperationTactical
        )
    }

    pub(crate) fn opened_briefing(&mut self, operations_completed: u32) {
        if operations_completed == 0 && self.stage == FirstHourStage::PrepareFirstOperation {
            self.stage = FirstHourStage::FirstBriefing;
        }
    }

    pub(crate) fn selected(&mut self) {
        self.advance_lesson(TacticalLesson::Select, TacticalLesson::MoveToCover);
    }

    pub(crate) fn moved(&mut self, on_cover: bool) {
        if on_cover {
            self.advance_lesson(TacticalLesson::MoveToCover, TacticalLesson::Attack);
        }
    }

    pub(crate) fn attacked(&mut self) {
        self.metrics.tactical_attack();
        self.advance_lesson(TacticalLesson::Attack, TacticalLesson::EnemyPhase);
    }

    pub(crate) fn ended_phase(&mut self) {
        self.advance_lesson(TacticalLesson::EnemyPhase, TacticalLesson::Objective);
    }

    pub(crate) fn touched_objective(&mut self) {
        self.advance_lesson(TacticalLesson::Objective, TacticalLesson::Ability);
    }

    pub(crate) fn used_ability(&mut self) {
        self.advance_lesson(TacticalLesson::Ability, TacticalLesson::ApplyLearning);
    }

    pub(crate) fn operation_resolved(&mut self, operation_number: u32, won: bool, rounds: u32) {
        self.metrics.operation_resolved(operation_number, rounds);
        match operation_number {
            1 => {
                self.first_outcome_won = Some(won);
                self.stage = FirstHourStage::FirstReturn;
            }
            2 => {
                self.second_outcome_won = Some(won);
                self.stage = FirstHourStage::SecondReturn;
            }
            _ => {}
        }
    }

    pub(crate) fn returned_to_colony(&mut self) {
        if self.stage == FirstHourStage::FirstReturn {
            self.stage = FirstHourStage::FirstReturnColony;
        } else if self.stage == FirstHourStage::SecondReturn {
            self.stage = FirstHourStage::Promise;
        }
    }

    pub(crate) fn invested(&mut self, name: impl Into<String>) {
        if self.stage == FirstHourStage::MakeInvestment {
            self.investment_name = name.into();
            self.stage = FirstHourStage::SecondOperation;
        }
    }

    pub(crate) fn apply_second_operation_bonus(
        &self,
        operations_completed: u32,
        units: &mut [crate::data::UnitDef],
    ) {
        if operations_completed != 1 {
            return;
        }
        for unit in units
            .iter_mut()
            .filter(|unit| unit.team == crate::data::Team::Colony)
        {
            match self.investment_name.as_str() {
                "bastion_mesh" => unit.armour += 1,
                "survey_uplink" => unit.accuracy += 8,
                "rapid_injectors" => unit.move_range = unit.move_range.saturating_add(1),
                _ => {}
            }
        }
    }

    pub(crate) fn restart(&mut self) {
        self.guidance_enabled = true;
        self.help_open = false;
        match self.stage {
            FirstHourStage::FirstOperation => self.lesson = TacticalLesson::Select,
            FirstHourStage::SecondOperationTactical => self.lesson = TacticalLesson::ApplyLearning,
            _ => {}
        }
    }

    pub(crate) fn migrate_from_operations(&mut self, operations_completed: u32) {
        self.stage = match operations_completed {
            0 => FirstHourStage::Arrival,
            1 => FirstHourStage::MakeInvestment,
            _ => FirstHourStage::Complete,
        };
        if self.stage == FirstHourStage::Complete {
            self.guidance_enabled = false;
        }
    }

    fn advance_lesson(&mut self, expected: TacticalLesson, next: TacticalLesson) {
        if self.stage == FirstHourStage::FirstOperation && self.lesson == expected {
            self.lesson = next;
        }
    }
}

fn lesson_prompt(lesson: TacticalLesson) -> &'static str {
    match lesson {
        TacticalLesson::Select => "Tap a colonist to select them.",
        TacticalLesson::MoveToCover => "Tap the gold-marked cover tile to move into cover.",
        TacticalLesson::Attack => "Tap a hostile, review the forecast, then tap ATTACK.",
        TacticalLesson::EnemyPhase => {
            "Tap END PHASE; if a READY warning appears, tap CONFIRM END again to watch the hostile response."
        }
        TacticalLesson::Objective => "Move onto the gold objective and tap SECURE OBJECTIVE.",
        TacticalLesson::Ability => "Tap a visible CLASS, MUTATION, or GEAR action.",
        TacticalLesson::ApplyLearning => {
            "Tap a remaining hostile, review the forecast, then tap ATTACK."
        }
    }
}

#[cfg(test)]
mod tests;
