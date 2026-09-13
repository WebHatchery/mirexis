//! Typed authored data schemas for the Mirexis registry.

use super::operation::OperationModifier;
use super::techniques::TechniqueDef;
use macroquad_toolkit::assets::TextureConfig;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    pub game_name: String,
    pub display_name: String,
    pub save_slot: String,
    pub version: String,
    pub world_width: usize,
    pub world_height: usize,
    pub max_action_points: u8,
    pub battle_seed: u64,
    pub starting_resources: StartingResources,
    pub equipment_costs: std::collections::BTreeMap<String, u32>,
    pub exploration: ExplorationBalance,
    pub tutorial: TutorialCopy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartingResources {
    pub materials: i32,
    pub power: i32,
    pub food: i32,
    pub biomass: i32,
    pub alien_components: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplorationBalance {
    pub player_start: [f32; 2],
    pub walk_speed: f32,
    pub player_radius: f32,
    pub interaction_distance: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TutorialCopy {
    pub primary_goal_label: String,
    pub first_hour_help_title: String,
    pub first_hour_current_goal_label: String,
    pub first_hour_colony_lines: Vec<String>,
    pub first_hour_tactical_lines: Vec<String>,
    pub first_hour_skip_notice: String,
    pub tactical_manual_title: String,
    pub tactical_manual_paused: String,
    pub tactical_read_field_lines: Vec<String>,
    pub tactical_spend_phase_lines: Vec<String>,
    pub tactical_survive_ground_lines: Vec<String>,
    pub tactical_win_contract_lines: Vec<String>,
    pub tactical_touch_controls: String,
    pub tactical_optional_controls: String,
    pub first_hour_goals: FirstHourGoals,
    pub first_hour_help_button: String,
    pub first_hour_help_return_button: String,
    pub first_hour_help_restart_button: String,
    pub first_hour_help_skip_button: String,
    pub first_hour_help_skipped_button: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirstHourGoals {
    pub arrival: String,
    pub meet_coordinator: String,
    pub prepare_first_operation: String,
    pub first_briefing: String,
    pub first_return: String,
    pub first_return_colony: String,
    pub make_investment: String,
    pub second_operation: String,
    pub second_return: String,
    pub promise: String,
    pub complete: String,
    pub unguided_first_operation: String,
    pub unguided_second_operation: String,
    pub lessons: FirstHourLessons,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirstHourLessons {
    pub select: String,
    pub move_to_cover: String,
    pub attack: String,
    pub enemy_phase: String,
    pub objective: String,
    pub ability: String,
    pub apply_learning: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionDef {
    pub id: String,
    pub name: String,
    pub briefing: String,
    pub objective: String,
    #[serde(default)]
    pub objective_kind: ObjectiveKind,
    pub hostile_faction: String,
    #[serde(default)]
    pub hostile_unit_ids: Vec<String>,
    pub round_limit: u32,
    pub materials_reward: i32,
    #[serde(default)]
    pub biomass_reward: i32,
    #[serde(default)]
    pub power_reward: i32,
    #[serde(default)]
    pub operation_modifier: OperationModifier,
    #[serde(default = "default_cover_integrity")]
    pub cover_integrity: i32,
    pub seed: u64,
    pub blocked_tiles: Vec<[i32; 2]>,
    pub objective_tile: [i32; 2],
    #[serde(default)]
    pub terrain_costs: Vec<TerrainCostDef>,
    #[serde(default)]
    pub hazards: Vec<HazardDef>,
    #[serde(default)]
    pub cover_edges: Vec<CoverEdgeDef>,
}

pub fn default_cover_integrity() -> i32 {
    6
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObjectiveKind {
    #[default]
    SecureAndClear,
    EliminateAll,
    Holdout,
    Extraction,
    SignalTrace,
    DefendAsset,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerrainCostDef {
    pub position: [i32; 2],
    pub cost: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HazardKind {
    FireLane,
    SporeBloom,
    StaticRift,
}

impl HazardKind {
    pub fn label(self) -> &'static str {
        match self {
            Self::FireLane => "DIRECTORATE FIRE LANE",
            Self::SporeBloom => "BROOD SPORE BLOOM",
            Self::StaticRift => "ASCENDANT STATIC RIFT",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HazardDef {
    pub position: [i32; 2],
    pub kind: HazardKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EdgeDirection {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoverEdgeDef {
    pub position: [i32; 2],
    pub direction: EdgeDirection,
    pub strength: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Team {
    Colony,
    Hostile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitDef {
    pub id: String,
    pub name: String,
    pub role: String,
    #[serde(default)]
    pub class_id: String,
    pub mutation: String,
    pub team: Team,
    #[serde(default)]
    pub faction: Option<String>,
    #[serde(default)]
    pub equipment_ids: Vec<String>,
    #[serde(default)]
    pub learned_skills: Vec<String>,
    #[serde(default)]
    pub active_skills: Vec<String>,
    pub position: [i32; 2],
    pub max_health: i32,
    pub move_range: u8,
    pub armour: i32,
    pub accuracy: i32,
    pub weapon_range: u8,
    pub weapon_damage: i32,
    pub weapon_ap_cost: u8,
    #[serde(default)]
    pub round_regeneration: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterDef {
    pub id: String,
    pub name: String,
    pub biography: String,
    pub aptitudes: std::collections::BTreeMap<String, u8>,
    pub initial_class: String,
    pub mutation: String,
    pub equipment: Vec<String>,
    #[serde(default)]
    pub recruitment_protocol: String,
    #[serde(default)]
    pub recruitment_cost: i32,
    #[serde(default)]
    pub recruitment_resource: String,
    #[serde(default)]
    pub recruitment_phase: String,
    #[serde(default)]
    pub origin: String,
    #[serde(default)]
    pub origin_description: String,
    #[serde(default)]
    pub origin_accuracy_bonus: i32,
    #[serde(default)]
    pub origin_move_bonus: i8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassDef {
    pub id: String,
    pub name: String,
    pub description: String,
    pub primary_aptitude: String,
    pub health_bonus: i32,
    pub accuracy_bonus: i32,
    pub move_bonus: i8,
    pub skill_slots: u8,
    #[serde(default)]
    pub technique_slots: u8,
    #[serde(default)]
    pub techniques: Vec<TechniqueDef>,
    #[serde(default)]
    pub advanced: bool,
    #[serde(default)]
    pub required_level: u8,
    #[serde(default)]
    pub required_phase: String,
    #[serde(default)]
    pub prerequisite_classes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutationDef {
    pub id: String,
    pub name: String,
    pub gift: Vec<StatModifier>,
    pub complication: Vec<StatModifier>,
    #[serde(default)]
    pub evolutions: Vec<MutationEvolutionDef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutationEvolutionDef {
    pub id: String,
    pub name: String,
    pub description: String,
    pub biomass_cost: i32,
    pub gift: Vec<StatModifier>,
    pub complication: Vec<StatModifier>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatModifier {
    pub stat: String,
    pub amount: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquipmentDef {
    pub id: String,
    pub name: String,
    pub description: String,
    pub slot: String,
    pub accuracy: i32,
    pub armour: i32,
    pub health: i32,
    pub damage: i32,
    #[serde(default)]
    pub move_bonus: i8,
    #[serde(default)]
    pub weapon_range_override: u8,
    #[serde(default)]
    pub weapon_ap_cost_override: u8,
    #[serde(default)]
    pub required_protocol: String,
    #[serde(default)]
    pub requires_contact_trace: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignDef {
    pub phase_id: String,
    pub phase_name: String,
    pub summary: String,
    pub factions: Vec<FactionDef>,
    pub research: Vec<ResearchDef>,
    pub contact_protocols: Vec<ContactProtocolDef>,
    pub escalation_responses: Vec<EscalationResponseDef>,
    pub mirexis_paths: Vec<MirexisPathDef>,
    pub events: Vec<CharacterEventDef>,
    #[serde(default)]
    pub outsider_beats: Vec<OutsiderBeatDef>,
    pub mission_templates: Vec<MissionTemplateDef>,
    pub map_recipes: Vec<MapRecipeDef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutsiderBeatDef {
    pub outsider_id: String,
    pub outsider_name: String,
    pub attention_faction: String,
    pub stage: u8,
    pub title: String,
    pub description: String,
    pub choices: Vec<OutsiderChoiceDef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutsiderChoiceDef {
    pub id: String,
    pub label: String,
    pub description: String,
    pub materials_cost: i32,
    pub food_cost: i32,
    pub power_cost: i32,
    pub biomass_cost: i32,
    pub attention_change: i32,
    pub relationship_partner: String,
    pub disagreement: bool,
    pub legacy_name: String,
    pub legacy_stat: String,
    pub legacy_amount: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationResponseDef {
    pub id: String,
    pub name: String,
    pub description: String,
    pub materials_cost: i32,
    pub biomass_cost: i32,
    pub power_cost: i32,
    pub attention_change_all: i32,
    pub threat_delay: u8,
    pub materials_bonus: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MirexisPathDef {
    pub id: String,
    pub name: String,
    pub description: String,
    pub materials_cost: i32,
    pub biomass_cost: i32,
    pub power_cost: i32,
    pub defense_cover_bonus: i32,
    pub deployment_food_discount: i32,
    pub power_bonus: i32,
    pub ending_title: String,
    pub revelation: String,
    pub legacy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactProtocolDef {
    pub id: String,
    pub name: String,
    pub faction: String,
    pub description: String,
    pub alien_components_cost: i32,
    pub materials_bonus: i32,
    pub biomass_bonus: i32,
    pub power_bonus: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactionDef {
    pub id: String,
    pub name: String,
    pub initial_attention: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchDef {
    pub id: String,
    pub name: String,
    pub description: String,
    pub materials_cost: i32,
    pub power_reward: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterEventDef {
    pub id: String,
    pub title: String,
    pub description: String,
    pub participants: Vec<String>,
    pub food_cost: i32,
    pub attention_change: i32,
    #[serde(default)]
    pub attention_faction: String,
    pub legacy_name: String,
    pub legacy_character_id: String,
    pub legacy_stat: String,
    pub legacy_amount: i32,
    #[serde(default)]
    pub required_protocol: String,
    #[serde(default)]
    pub requires_contact_trace: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionTemplateDef {
    pub id: String,
    pub name: String,
    pub objective: String,
    pub objective_kind: ObjectiveKind,
    pub faction: String,
    #[serde(default)]
    pub hostile_unit_ids: Vec<String>,
    pub map_recipe: String,
    #[serde(default)]
    pub required_protocol: String,
    #[serde(default)]
    pub required_phase: String,
    #[serde(default)]
    pub required_response: String,
    #[serde(default)]
    pub required_mirexis_path: String,
    #[serde(default)]
    pub post_campaign: bool,
    pub materials_reward: i32,
    #[serde(default)]
    pub biomass_reward: i32,
    #[serde(default)]
    pub power_reward: i32,
    pub round_limit: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapRecipeDef {
    pub id: String,
    pub blocked_tiles: Vec<[i32; 2]>,
    pub objective_tile: [i32; 2],
    pub terrain_costs: Vec<TerrainCostDef>,
    #[serde(default)]
    pub hazards: Vec<HazardDef>,
    pub cover_edges: Vec<CoverEdgeDef>,
}

#[derive(Debug, Clone)]
pub struct GameData {
    pub config: GameConfig,
    pub mission: MissionDef,
    pub roster: Vec<UnitDef>,
    pub recruitable_roster: Vec<UnitDef>,
    pub characters: Vec<CharacterDef>,
    pub classes: Vec<ClassDef>,
    pub mutations: Vec<MutationDef>,
    pub equipment: Vec<EquipmentDef>,
    pub campaign: CampaignDef,
    pub texture_manifest: Vec<TextureConfig>,
}
