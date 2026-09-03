//! Application-state types shared by the game coordinator and child flows.

use crate::data::TechniqueTarget;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum AppState {
    Title,
    Colony,
    Roster,
    GeneLab,
    MissionBriefing,
    Tactical,
    Debrief,
    DemoComplete,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum TacticalTargeting {
    Equipment {
        unit_id: String,
        equipment_id: String,
    },
    ClassAction {
        unit_id: String,
        target_kind: TechniqueTarget,
    },
    Skill {
        unit_id: String,
        skill_id: String,
    },
}
