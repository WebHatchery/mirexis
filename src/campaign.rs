//! Persistent character identity, progression, and deployment derivation.
pub mod state;

pub mod commons;
pub mod construction;
pub mod deployment;
pub mod derivation;
pub mod evolution;
pub mod identity;
pub mod medical;
pub mod mission;
pub mod modifiers;
pub mod outsider;
pub mod relay;
pub mod research;
pub mod salvage;
pub mod story;

// These re-exports form the campaign's public calculation seam for UI and
// integration tests; some consumers only use them through glob imports.
pub use derivation::derive_unit;
pub use derivation::derive_unit_with_evolution_options;
// Kept as a named public seam for authored mutation application.
pub use modifiers::apply_mutation;
pub use modifiers::derived_mutation_traits_with_options;
// Both direct campaign callers and the public campaign namespace use these
// helpers; retain the allowance while the feature modules remain split.
pub use modifiers::{derived_mutation_traits, equipment_cost};
pub use outsider::OutsiderChoice;
pub use relay::{RELAY_SCAN_POWER_COST, RELAY_SIGNAL_ATTENTION};
pub use salvage::{SalvageChoice, SALVAGE_MATERIALS_REWARD, SALVAGE_RESEARCH_INSIGHT};

use crate::colony::{
    BuildingKind, ColonyState, COUNTERINTELLIGENCE_CELL_UPGRADE, HOT_CORE_UPGRADE,
    PRECISION_BENCH_UPGRADE, STABILISATION_WING_UPGRADE, TRAUMA_WARD_UPGRADE,
};
use crate::data::{CharacterDef, EquipmentDef, GameData, Team, UnitDef};
use crate::relationships::RelationshipRecord;
use crate::state::{MissionOutcome, ObjectiveState};
use crate::strategy::{MissionInstance, StrategyState};
use crate::trauma::TraumaRecord;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const SQUAD_LIMIT: usize = 3;
pub const VICTORY_OPERATION_XP: u32 = 20;
pub const FAILED_OPERATION_XP: u32 = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContactCompletionProgress {
    pub trace_completed: bool,
    pub aftermath_resolved: bool,
    pub prototype_equipped: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdaptationCompletionProgress {
    pub operation_completed: bool,
    pub evolved_count: usize,
    pub gene_lab_ready: bool,
}

pub fn operation_experience(result: ObjectiveState) -> u32 {
    match result {
        ObjectiveState::Victory => VICTORY_OPERATION_XP,
        ObjectiveState::Failed => FAILED_OPERATION_XP,
        ObjectiveState::Active | ObjectiveState::Secured => 0,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Availability {
    Ready,
    Recovering,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InjuryRecord {
    pub id: String,
    pub name: String,
    pub recovery_operations: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CharacterLegacy {
    pub id: String,
    pub name: String,
    pub stat: String,
    pub amount: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LostObjectiveRecord {
    pub id: String,
    pub mission_name: String,
    pub objective: String,
    pub operation: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterRecord {
    pub id: String,
    pub name: String,
    pub biography: String,
    pub aptitudes: BTreeMap<String, u8>,
    pub experience: u32,
    pub level: u8,
    pub active_class: String,
    pub class_history: Vec<String>,
    pub learned_skills: Vec<String>,
    pub active_skills: Vec<String>,
    pub mutation_id: String,
    #[serde(default)]
    pub origin: String,
    #[serde(default)]
    pub origin_description: String,
    #[serde(default)]
    pub mutation_evolution_id: String,
    pub injuries: Vec<InjuryRecord>,
    pub availability: Availability,
    #[serde(default = "deployment_selected_default")]
    pub deployment_selected: bool,
    pub equipment_ids: Vec<String>,
    #[serde(default)]
    pub event_legacies: Vec<CharacterLegacy>,
    #[serde(default)]
    pub traumas: Vec<TraumaRecord>,
}

impl CharacterRecord {
    pub fn from_def(def: &CharacterDef) -> Self {
        let starter_skill = format!("{}_fundamentals", def.initial_class);
        Self {
            id: def.id.clone(),
            name: def.name.clone(),
            biography: def.biography.clone(),
            aptitudes: def.aptitudes.clone(),
            experience: 0,
            level: 1,
            active_class: def.initial_class.clone(),
            class_history: vec![def.initial_class.clone()],
            learned_skills: vec![starter_skill.clone()],
            active_skills: vec![starter_skill],
            mutation_id: def.mutation.clone(),
            origin: def.origin.clone(),
            origin_description: def.origin_description.clone(),
            mutation_evolution_id: String::new(),
            injuries: Vec::new(),
            availability: Availability::Ready,
            deployment_selected: true,
            equipment_ids: def.equipment.clone(),
            event_legacies: Vec::new(),
            traumas: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OutsiderArcState {
    #[serde(default)]
    pub stage: u8,
    #[serde(default)]
    pub disagreements: u8,
    #[serde(default)]
    pub final_choice: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignState {
    pub roster: Vec<CharacterRecord>,
    #[serde(default)]
    pub selected_character_id: String,
    pub colony: ColonyState,
    pub strategy: StrategyState,
    pub operations_completed: u32,
    #[serde(default)]
    pub relationships: Vec<RelationshipRecord>,
    #[serde(default)]
    pub lost_objectives: Vec<LostObjectiveRecord>,
    #[serde(default)]
    pub first_hour: crate::first_hour::FirstHourProgress,
    #[serde(default)]
    pub colony_story: crate::colony_story::ColonyStoryState,
    #[serde(default)]
    pub outsider_arc_stage: u8,
    #[serde(default)]
    pub outsider_disagreements: u8,
    #[serde(default)]
    pub outsider_final_choice: String,
    #[serde(default)]
    pub outsider_arc_states: BTreeMap<String, OutsiderArcState>,
    #[serde(default)]
    pub commons_meals_hosted: u32,
    #[serde(default)]
    pub commons_meal_operation: Option<u32>,
    #[serde(default)]
    pub relay_scans_used: u32,
    #[serde(default)]
    pub relay_scan_operation: Option<u32>,
    #[serde(default)]
    pub identity_stewardship_completed: u32,
    #[serde(default)]
    pub identity_stewardship_operation: Option<u32>,
    #[serde(default)]
    pub identity_preparations_completed: u32,
    #[serde(default)]
    pub identity_preparation_operation: Option<u32>,
    #[serde(default)]
    pub salvage_cache_count: u32,
    #[serde(default)]
    pub salvage_yard_operation: Option<u32>,
    #[serde(default)]
    pub research_insight: i32,
    #[serde(default)]
    pub salvage_prototypes: u32,
}

pub fn deployment_selected_default() -> bool {
    true
}
