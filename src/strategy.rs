//! Phase One campaign pressure, research, events, and mission generation.

use crate::colony::{BuildingKind, ColonyState, SIGNAL_CARTOGRAPHY_UPGRADE};
use crate::data::{GameData, MissionTemplateDef, ObjectiveKind, OperationModifier};
use crate::state::{MissionOutcome, ObjectiveState};
use crate::strategy_events::{
    from_definition as character_event_from_def, is_available as event_is_available,
};
use macroquad_toolkit::rng::SeededRng;
use serde::{Deserialize, Serialize};

pub mod materialization;
pub mod missions;
pub mod progression;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FactionPressure {
    pub id: String,
    pub name: String,
    pub attention: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TelegraphedThreat {
    pub id: String,
    pub faction_id: String,
    pub name: String,
    pub operations_until: u8,
    pub strength: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResearchOpportunity {
    pub id: String,
    pub name: String,
    pub description: String,
    pub materials_cost: i32,
    pub power_reward: i32,
    pub completed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CharacterEventState {
    pub id: String,
    pub title: String,
    pub description: String,
    pub participants: Vec<String>,
    pub food_cost: i32,
    pub attention_change: i32,
    #[serde(default)]
    pub attention_faction: String,
    #[serde(default)]
    pub legacy_name: String,
    #[serde(default)]
    pub legacy_character_id: String,
    #[serde(default)]
    pub legacy_stat: String,
    #[serde(default)]
    pub legacy_amount: i32,
    #[serde(default)]
    pub required_protocol: String,
    #[serde(default)]
    pub requires_contact_trace: bool,
    pub resolved: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MissionInstance {
    pub id: String,
    pub template_id: String,
    pub name: String,
    pub briefing: String,
    pub objective: String,
    #[serde(default)]
    pub objective_kind: ObjectiveKind,
    pub faction_id: String,
    #[serde(default)]
    pub hostile_unit_ids: Vec<String>,
    pub map_recipe: String,
    pub seed: u64,
    pub round_limit: u32,
    pub materials_reward: i32,
    #[serde(default)]
    pub biomass_reward: i32,
    #[serde(default)]
    pub power_reward: i32,
    #[serde(default)]
    pub operation_modifier: OperationModifier,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyState {
    pub phase_id: String,
    pub phase_name: String,
    pub phase_summary: String,
    pub factions: Vec<FactionPressure>,
    pub threats: Vec<TelegraphedThreat>,
    pub research: Vec<ResearchOpportunity>,
    pub character_events: Vec<CharacterEventState>,
    pub mission_offers: Vec<MissionInstance>,
    pub selected_mission_id: String,
    #[serde(default)]
    pub isolation_victories: u8,
    #[serde(default)]
    pub first_assault_repulsed: bool,
    #[serde(default)]
    pub isolation_complete: bool,
    #[serde(default)]
    pub contact_protocol_id: String,
    #[serde(default)]
    pub contact_trace_completed: bool,
    #[serde(default)]
    pub contact_complete: bool,
    #[serde(default)]
    pub adaptation_operation_completed: bool,
    #[serde(default)]
    pub adaptation_complete: bool,
    #[serde(default)]
    pub escalation_operation_completed: bool,
    #[serde(default)]
    pub escalation_response_id: String,
    #[serde(default)]
    pub escalation_branch_completed: bool,
    #[serde(default)]
    pub escalation_complete: bool,
    #[serde(default)]
    pub mirexis_path_id: String,
    #[serde(default)]
    pub mirexis_operation_completed: bool,
    #[serde(default)]
    pub campaign_complete: bool,
    #[serde(default)]
    pub post_campaign_operations_completed: u32,
    rng: SeededRng,
}
