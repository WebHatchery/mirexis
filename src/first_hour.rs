//! Persistent first-session direction and contextual tactical teaching.

use crate::data::{FirstHourGoals, FirstHourLessons};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum FirstHourStage {
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
pub enum TacticalLesson {
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
pub struct FirstHourProgress {
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

pub fn enabled() -> bool {
    true
}

impl FirstHourProgress {
    pub fn primary_goal<'a>(&self, copy: &'a FirstHourGoals) -> &'a str {
        match self.stage {
            FirstHourStage::Arrival => &copy.arrival,
            FirstHourStage::MeetCoordinator => &copy.meet_coordinator,
            FirstHourStage::PrepareFirstOperation => &copy.prepare_first_operation,
            FirstHourStage::FirstBriefing => &copy.first_briefing,
            FirstHourStage::FirstOperation | FirstHourStage::SecondOperationTactical => {
                lesson_prompt(self.lesson, &copy.lessons)
            }
            FirstHourStage::FirstReturn => &copy.first_return,
            FirstHourStage::FirstReturnColony => &copy.first_return_colony,
            FirstHourStage::MakeInvestment => &copy.make_investment,
            FirstHourStage::SecondOperation => &copy.second_operation,
            FirstHourStage::SecondReturn => &copy.second_return,
            FirstHourStage::Promise => &copy.promise,
            FirstHourStage::Complete => &copy.complete,
        }
    }

    pub fn visible_goal<'a>(&self, copy: &'a FirstHourGoals) -> &'a str {
        if self.guidance_enabled {
            return self.primary_goal(copy);
        }
        match self.stage {
            FirstHourStage::FirstOperation => &copy.unguided_first_operation,
            FirstHourStage::SecondOperationTactical => &copy.unguided_second_operation,
            _ => self.primary_goal(copy),
        }
    }

    pub fn colony_guidance_target(&self) -> Option<&'static str> {
        if !self.guidance_enabled {
            return None;
        }
        match self.stage {
            FirstHourStage::MeetCoordinator => Some("mara_venn"),
            FirstHourStage::FirstReturnColony => Some("ilya_reed"),
            _ => None,
        }
    }

    pub fn advance_arrival(&mut self) {
        if self.stage == FirstHourStage::Arrival {
            self.stage = FirstHourStage::MeetCoordinator;
        } else if self.stage == FirstHourStage::Promise {
            self.stage = FirstHourStage::Complete;
            self.help_open = false;
        }
    }

    pub fn acknowledge_colonist(&mut self, id: &str) {
        self.metrics.city_interacted();
        if self.stage == FirstHourStage::MeetCoordinator && id == "mara_venn" {
            self.stage = FirstHourStage::PrepareFirstOperation;
        } else if self.stage == FirstHourStage::FirstReturnColony && id == "ilya_reed" {
            self.stage = FirstHourStage::MakeInvestment;
        }
    }

    pub fn deployed(&mut self, operations_completed: u32) {
        self.metrics.operation_started();
        if operations_completed == 0 {
            self.stage = FirstHourStage::FirstOperation;
            self.lesson = TacticalLesson::Select;
        } else if operations_completed == 1 {
            self.entered_second_operation_tactical();
        }
    }

    pub fn entered_second_operation_tactical(&mut self) {
        if self.stage == FirstHourStage::SecondOperation {
            self.stage = FirstHourStage::SecondOperationTactical;
            self.lesson = TacticalLesson::ApplyLearning;
        }
    }

    pub fn is_tactical_stage(&self) -> bool {
        matches!(
            self.stage,
            FirstHourStage::FirstOperation | FirstHourStage::SecondOperationTactical
        )
    }

    pub fn opened_briefing(&mut self, operations_completed: u32) {
        if operations_completed == 0 && self.stage == FirstHourStage::PrepareFirstOperation {
            self.stage = FirstHourStage::FirstBriefing;
        }
    }

    pub fn selected(&mut self) {
        self.advance_lesson(TacticalLesson::Select, TacticalLesson::MoveToCover);
    }

    pub fn moved(&mut self, on_cover: bool) {
        if on_cover {
            self.advance_lesson(TacticalLesson::MoveToCover, TacticalLesson::Attack);
        }
    }

    pub fn attacked(&mut self) {
        self.metrics.tactical_attack();
        self.advance_lesson(TacticalLesson::Attack, TacticalLesson::EnemyPhase);
    }

    pub fn ended_phase(&mut self) {
        self.advance_lesson(TacticalLesson::EnemyPhase, TacticalLesson::Objective);
    }

    pub fn touched_objective(&mut self) {
        self.advance_lesson(TacticalLesson::Objective, TacticalLesson::Ability);
    }

    pub fn used_ability(&mut self) {
        self.advance_lesson(TacticalLesson::Ability, TacticalLesson::ApplyLearning);
    }

    pub fn operation_resolved(&mut self, operation_number: u32, won: bool, rounds: u32) {
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

    pub fn returned_to_colony(&mut self) {
        if self.stage == FirstHourStage::FirstReturn {
            self.stage = FirstHourStage::FirstReturnColony;
        } else if self.stage == FirstHourStage::SecondReturn {
            self.stage = FirstHourStage::Promise;
        }
    }

    pub fn invested(&mut self, name: impl Into<String>) {
        if self.stage == FirstHourStage::MakeInvestment {
            self.investment_name = name.into();
            self.stage = FirstHourStage::SecondOperation;
        }
    }

    pub fn apply_second_operation_bonus(
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

    pub fn restart(&mut self) {
        self.guidance_enabled = true;
        self.help_open = false;
        match self.stage {
            FirstHourStage::FirstOperation => self.lesson = TacticalLesson::Select,
            FirstHourStage::SecondOperationTactical => self.lesson = TacticalLesson::ApplyLearning,
            _ => {}
        }
    }

    pub fn migrate_from_operations(&mut self, operations_completed: u32) {
        self.stage = match operations_completed {
            0 => FirstHourStage::Arrival,
            1 => FirstHourStage::MakeInvestment,
            _ => FirstHourStage::Complete,
        };
        if self.stage == FirstHourStage::Complete {
            self.guidance_enabled = false;
        }
    }

    pub fn advance_lesson(&mut self, expected: TacticalLesson, next: TacticalLesson) {
        if self.stage == FirstHourStage::FirstOperation && self.lesson == expected {
            self.lesson = next;
        }
    }
}

pub fn lesson_prompt(lesson: TacticalLesson, copy: &FirstHourLessons) -> &str {
    match lesson {
        TacticalLesson::Select => &copy.select,
        TacticalLesson::MoveToCover => &copy.move_to_cover,
        TacticalLesson::Attack => &copy.attack,
        TacticalLesson::EnemyPhase => &copy.enemy_phase,
        TacticalLesson::Objective => &copy.objective,
        TacticalLesson::Ability => &copy.ability,
        TacticalLesson::ApplyLearning => &copy.apply_learning,
    }
}
