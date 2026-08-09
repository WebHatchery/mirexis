//! Embedded content schemas and registry validation.

use macroquad_toolkit::assets::TextureConfig;
use macroquad_toolkit::data_loader::{load_embedded_json, load_embedded_json_labeled};
use serde::{Deserialize, Serialize};

const GAME_CONFIG_JSON: &str = include_str!("../assets/data/game_config.json");
const MISSION_JSON: &str = include_str!("../assets/data/mission.json");
const ROSTER_JSON: &str = include_str!("../assets/data/roster.json");
const CHARACTERS_JSON: &str = include_str!("../assets/data/characters.json");
const CLASSES_JSON: &str = include_str!("../assets/data/classes.json");
const MUTATIONS_JSON: &str = include_str!("../assets/data/mutations.json");
const EQUIPMENT_JSON: &str = include_str!("../assets/data/equipment.json");
const TEXTURE_MANIFEST_JSON: &str = include_str!("../assets/data/texture_manifest.json");

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
    pub round_limit: u32,
    pub materials_reward: i32,
    pub blocked_tiles: Vec<[i32; 2]>,
    pub objective_tile: [i32; 2],
    #[serde(default)]
    pub terrain_costs: Vec<TerrainCostDef>,
    #[serde(default)]
    pub cover_edges: Vec<CoverEdgeDef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerrainCostDef {
    pub position: [i32; 2],
    pub cost: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EdgeDirection {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub mutation: String,
    pub team: Team,
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
    pub primary_aptitude: String,
    pub health_bonus: i32,
    pub accuracy_bonus: i32,
    pub move_bonus: i8,
    pub skill_slots: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutationDef {
    pub id: String,
    pub name: String,
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
    pub slot: String,
    pub accuracy: i32,
    pub armour: i32,
    pub health: i32,
    pub damage: i32,
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
        let texture_manifest = load_embedded_json(TEXTURE_MANIFEST_JSON)?;

        let data = Self {
            config,
            mission,
            roster,
            characters,
            classes,
            mutations,
            equipment,
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
        ensure_unique("class", self.classes.iter().map(|entry| entry.id.as_str()))?;
        ensure_unique(
            "mutation",
            self.mutations.iter().map(|entry| entry.id.as_str()),
        )?;
        ensure_unique(
            "equipment",
            self.equipment.iter().map(|entry| entry.id.as_str()),
        )?;
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
mod tests {
    use super::*;

    #[test]
    fn complete_content_registry_is_valid() {
        let data = GameData::load().unwrap();

        assert_eq!(data.config.game_name, "mirexis");
        assert!(data.roster.iter().any(|unit| unit.team == Team::Colony));
        assert!(data.roster.iter().any(|unit| unit.team == Team::Hostile));
        assert!(data.mission.round_limit > 0);
        assert!(data.mission.terrain_costs.iter().all(|tile| tile.cost > 0));
        assert!(data.roster.iter().all(|unit| unit.weapon_ap_cost > 0));
        assert_eq!(
            data.roster
                .iter()
                .filter(|unit| unit.team == Team::Colony)
                .count(),
            4
        );
        let hostile_roles = data
            .roster
            .iter()
            .filter(|unit| unit.team == Team::Hostile)
            .map(|unit| unit.role.as_str())
            .collect::<std::collections::HashSet<_>>();
        assert!(hostile_roles.len() >= 2);
        assert_eq!(data.characters.len(), 4);
        assert!(data.classes.len() >= 7);
        assert!(data.mutations.len() >= 5);
    }
}
