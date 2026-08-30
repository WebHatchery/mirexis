//! Application-state types shared by the game coordinator and child flows.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum AppState {
    Title,
    Colony,
    Roster,
    GeneLab,
    MissionBriefing,
    Tactical,
    Debrief,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum TacticalTargeting {
    Equipment {
        unit_id: String,
        equipment_id: String,
    },
    ClassAction {
        unit_id: String,
    },
    Skill {
        unit_id: String,
        skill_id: String,
    },
}
