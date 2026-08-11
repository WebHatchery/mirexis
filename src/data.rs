//! Embedded content schemas and registry validation.

use macroquad_toolkit::assets::TextureConfig;
use macroquad_toolkit::data_loader::{load_embedded_json, load_embedded_json_labeled};
use serde::{Deserialize, Serialize};

const GAME_CONFIG_JSON: &str =
    macroquad_toolkit::include_json_str!("../assets/data/game_config.json");
const MISSION_JSON: &str = macroquad_toolkit::include_json_str!("../assets/data/mission.json");
const ROSTER_JSON: &str = macroquad_toolkit::include_json_str!("../assets/data/roster.json");
const CHARACTERS_JSON: &str =
    macroquad_toolkit::include_json_str!("../assets/data/characters.json");
const CLASSES_JSON: &str = macroquad_toolkit::include_json_str!("../assets/data/classes.json");
const MUTATIONS_JSON: &str = macroquad_toolkit::include_json_str!("../assets/data/mutations.json");
const EQUIPMENT_JSON: &str = macroquad_toolkit::include_json_str!("../assets/data/equipment.json");
const CAMPAIGN_JSON: &str = macroquad_toolkit::include_json_str!("../assets/data/campaign.json");
const TEXTURE_MANIFEST_JSON: &str =
    macroquad_toolkit::include_json_str!("../assets/data/texture_manifest.json");

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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationModifier {
    #[default]
    None,
    DirectorateFireControl,
    BroodFrenzy,
    AscendantInterference,
    EscalationCrossfire,
}

impl OperationModifier {
    pub fn label(self) -> &'static str {
        match self {
            Self::None => "NO ESCALATION",
            Self::DirectorateFireControl => "DIRECTORATE FIRE-CONTROL",
            Self::BroodFrenzy => "BROOD FRENZY",
            Self::AscendantInterference => "ASCENDANT INTERFERENCE",
            Self::EscalationCrossfire => "THREE-POWER CROSSFIRE",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::None => "No faction-pressure modifier is active.",
            Self::DirectorateFireControl => "Hostile attacks gain 10 accuracy.",
            Self::BroodFrenzy => "Hostile units gain 1 movement.",
            Self::AscendantInterference => "Colonist attacks lose 10 accuracy.",
            Self::EscalationCrossfire => {
                "Hostiles gain 5 accuracy and 1 movement; colonists lose 5 accuracy."
            }
        }
    }
}

fn default_cover_integrity() -> i32 {
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
    pub mission_templates: Vec<MissionTemplateDef>,
    pub map_recipes: Vec<MapRecipeDef>,
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
    pub characters: Vec<CharacterDef>,
    pub classes: Vec<ClassDef>,
    pub mutations: Vec<MutationDef>,
    pub equipment: Vec<EquipmentDef>,
    pub campaign: CampaignDef,
    pub texture_manifest: Vec<TextureConfig>,
}

impl GameData {
    pub fn load() -> Result<Self, String> {
        let config = load_embedded_json_labeled("game_config", GAME_CONFIG_JSON)?;
        let mission = load_embedded_json_labeled("mission", MISSION_JSON)?;
        let roster = load_embedded_json_labeled("roster", ROSTER_JSON)?;
        let characters = load_embedded_json_labeled("characters", CHARACTERS_JSON)?;
        let classes = load_embedded_json_labeled("classes", CLASSES_JSON)?;
        let mutations = load_embedded_json_labeled("mutations", MUTATIONS_JSON)?;
        let equipment = load_embedded_json_labeled("equipment", EQUIPMENT_JSON)?;
        let campaign = load_embedded_json_labeled("campaign", CAMPAIGN_JSON)?;
        let texture_manifest = load_embedded_json(TEXTURE_MANIFEST_JSON)?;

        let data = Self {
            config,
            mission,
            roster,
            characters,
            classes,
            mutations,
            equipment,
            campaign,
            texture_manifest,
        };
        data.validate_registry()?;
        Ok(data)
    }

    fn validate_registry(&self) -> Result<(), String> {
        ensure_unique(
            "character",
            self.characters.iter().map(|entry| entry.id.as_str()),
        )?;
        for template in &self.campaign.mission_templates {
            if !template.required_response.is_empty()
                && !self
                    .campaign
                    .escalation_responses
                    .iter()
                    .any(|response| response.id == template.required_response)
            {
                return Err(format!(
                    "Mission template {} references missing Escalation response {}",
                    template.id, template.required_response
                ));
            }
            for unit_id in &template.hostile_unit_ids {
                if !self
                    .roster
                    .iter()
                    .any(|unit| unit.id == *unit_id && unit.team == Team::Hostile)
                {
                    return Err(format!(
                        "Mission template {} references missing hostile unit {}",
                        template.id, unit_id
                    ));
                }
            }
            if !template.required_mirexis_path.is_empty()
                && !self
                    .campaign
                    .mirexis_paths
                    .iter()
                    .any(|path| path.id == template.required_mirexis_path)
            {
                return Err(format!(
                    "Mission template {} references missing Mirexis path {}",
                    template.id, template.required_mirexis_path
                ));
            }
        }
        for equipment in &self.equipment {
            if !equipment.required_protocol.is_empty()
                && !self
                    .campaign
                    .contact_protocols
                    .iter()
                    .any(|protocol| protocol.id == equipment.required_protocol)
            {
                return Err(format!(
                    "Equipment {} references missing Contact protocol {}",
                    equipment.id, equipment.required_protocol
                ));
            }
        }
        ensure_unique(
            "faction",
            self.campaign.factions.iter().map(|entry| entry.id.as_str()),
        )?;
        ensure_unique(
            "research",
            self.campaign.research.iter().map(|entry| entry.id.as_str()),
        )?;
        ensure_unique(
            "contact protocol",
            self.campaign
                .contact_protocols
                .iter()
                .map(|entry| entry.id.as_str()),
        )?;
        ensure_unique(
            "escalation response",
            self.campaign
                .escalation_responses
                .iter()
                .map(|entry| entry.id.as_str()),
        )?;
        if self.campaign.escalation_responses.iter().any(|response| {
            response.materials_cost < 0 || response.biomass_cost < 0 || response.power_cost < 0
        }) {
            return Err("Escalation responses cannot refund their selection cost".to_owned());
        }
        ensure_unique(
            "Mirexis path",
            self.campaign
                .mirexis_paths
                .iter()
                .map(|entry| entry.id.as_str()),
        )?;
        if self
            .campaign
            .mirexis_paths
            .iter()
            .any(|path| path.materials_cost < 0 || path.biomass_cost < 0 || path.power_cost < 0)
        {
            return Err("Mirexis paths cannot refund their selection cost".to_owned());
        }
        ensure_unique(
            "campaign event",
            self.campaign.events.iter().map(|entry| entry.id.as_str()),
        )?;
        ensure_unique(
            "mission template",
            self.campaign
                .mission_templates
                .iter()
                .map(|entry| entry.id.as_str()),
        )?;
        ensure_unique(
            "map recipe",
            self.campaign
                .map_recipes
                .iter()
                .map(|entry| entry.id.as_str()),
        )?;
        ensure_unique("class", self.classes.iter().map(|entry| entry.id.as_str()))?;
        crate::class_training::validate_definitions(&self.classes)?;
        ensure_unique(
            "mutation",
            self.mutations.iter().map(|entry| entry.id.as_str()),
        )?;
        for mutation in &self.mutations {
            ensure_unique(
                "mutation evolution",
                mutation.evolutions.iter().map(|entry| entry.id.as_str()),
            )?;
            if mutation
                .evolutions
                .iter()
                .any(|evolution| evolution.biomass_cost <= 0)
            {
                return Err(format!("Mutation {} has a free evolution", mutation.id));
            }
        }
        ensure_unique(
            "equipment",
            self.equipment.iter().map(|entry| entry.id.as_str()),
        )?;
        crate::equipment_catalog::validate_definitions(&self.equipment)?;
        for character in &self.characters {
            if !self
                .classes
                .iter()
                .any(|entry| entry.id == character.initial_class)
            {
                return Err(format!(
                    "Character {} references missing class {}",
                    character.id, character.initial_class
                ));
            }
            if !self
                .mutations
                .iter()
                .any(|entry| entry.id == character.mutation)
            {
                return Err(format!(
                    "Character {} references missing mutation {}",
                    character.id, character.mutation
                ));
            }
            for equipment_id in &character.equipment {
                if !self.equipment.iter().any(|entry| &entry.id == equipment_id) {
                    return Err(format!(
                        "Character {} references missing equipment {}",
                        character.id, equipment_id
                    ));
                }
            }
            if character
                .aptitudes
                .values()
                .any(|rating| !(1..=5).contains(rating))
            {
                return Err(format!(
                    "Character {} has an aptitude outside 1..=5",
                    character.id
                ));
            }
        }
        for unit in self.roster.iter().filter(|unit| unit.team == Team::Colony) {
            if !self
                .characters
                .iter()
                .any(|character| character.id == unit.id)
            {
                return Err(format!(
                    "Deployed colonist {} has no persistent character record",
                    unit.id
                ));
            }
        }
        for template in &self.campaign.mission_templates {
            if !self
                .campaign
                .factions
                .iter()
                .any(|faction| faction.id == template.faction)
            {
                return Err(format!(
                    "Mission template {} references missing faction {}",
                    template.id, template.faction
                ));
            }
            if !template.required_protocol.is_empty()
                && !self
                    .campaign
                    .contact_protocols
                    .iter()
                    .any(|protocol| protocol.id == template.required_protocol)
            {
                return Err(format!(
                    "Mission template {} references missing Contact protocol {}",
                    template.id, template.required_protocol
                ));
            }
            if !template.required_phase.is_empty()
                && ![
                    "isolation",
                    "contact",
                    "adaptation",
                    "escalation",
                    "mirexis",
                ]
                .contains(&template.required_phase.as_str())
            {
                return Err(format!(
                    "Mission template {} references unknown phase {}",
                    template.id, template.required_phase
                ));
            }
            if !self
                .campaign
                .map_recipes
                .iter()
                .any(|recipe| recipe.id == template.map_recipe)
            {
                return Err(format!(
                    "Mission template {} references missing map recipe {}",
                    template.id, template.map_recipe
                ));
            }
        }
        for protocol in &self.campaign.contact_protocols {
            if !self
                .campaign
                .factions
                .iter()
                .any(|faction| faction.id == protocol.faction)
            {
                return Err(format!(
                    "Contact protocol {} references missing faction {}",
                    protocol.id, protocol.faction
                ));
            }
            if protocol.alien_components_cost <= 0
                || [
                    protocol.materials_bonus,
                    protocol.biomass_bonus,
                    protocol.power_bonus,
                ]
                .into_iter()
                .filter(|bonus| *bonus > 0)
                .count()
                    != 1
            {
                return Err(format!(
                    "Contact protocol {} must have a cost and one positive reward bonus",
                    protocol.id
                ));
            }
        }
        for faction in &self.campaign.factions {
            if !self.roster.iter().any(|unit| {
                unit.team == Team::Hostile && unit.faction.as_deref() == Some(&faction.id)
            }) {
                return Err(format!("Faction {} has no hostile units", faction.id));
            }
        }
        for recipe in &self.campaign.map_recipes {
            let positions = recipe
                .blocked_tiles
                .iter()
                .chain(std::iter::once(&recipe.objective_tile))
                .chain(recipe.terrain_costs.iter().map(|entry| &entry.position))
                .chain(recipe.hazards.iter().map(|entry| &entry.position));
            if positions.into_iter().any(|position| {
                position[0] < 0
                    || position[1] < 0
                    || position[0] >= self.config.world_width as i32
                    || position[1] >= self.config.world_height as i32
            }) {
                return Err(format!(
                    "Map recipe {} contains an out-of-bounds tile",
                    recipe.id
                ));
            }
            if recipe.blocked_tiles.contains(&recipe.objective_tile) {
                return Err(format!("Map recipe {} blocks its objective", recipe.id));
            }
            if recipe
                .hazards
                .iter()
                .any(|hazard| recipe.blocked_tiles.contains(&hazard.position))
            {
                return Err(format!("Map recipe {} blocks a hazard", recipe.id));
            }
        }
        for template in &self.campaign.mission_templates {
            let recipe = self
                .campaign
                .map_recipes
                .iter()
                .find(|recipe| recipe.id == template.map_recipe)
                .expect("map recipe references were validated above");
            if self.roster.iter().any(|unit| {
                unit.team == Team::Hostile
                    && unit.faction.as_deref() == Some(&template.faction)
                    && recipe.blocked_tiles.contains(&unit.position)
            }) {
                return Err(format!(
                    "Mission template {} blocks a hostile spawn",
                    template.id
                ));
            }
        }
        crate::relationships::validate_event_definitions(self)?;
        Ok(())
    }
}

fn ensure_unique<'a>(label: &str, ids: impl Iterator<Item = &'a str>) -> Result<(), String> {
    let mut seen = std::collections::HashSet::new();
    for id in ids {
        if !seen.insert(id) {
            return Err(format!("Duplicate {} id: {}", label, id));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;
