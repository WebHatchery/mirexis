//! Embedded, data-driven Phase 0 content.

use macroquad_toolkit::assets::TextureConfig;
use macroquad_toolkit::data_loader::{load_embedded_json, load_embedded_json_labeled};
use serde::{Deserialize, Serialize};

const GAME_CONFIG_JSON: &str = include_str!("../assets/data/game_config.json");
const MISSION_JSON: &str = include_str!("../assets/data/mission.json");
const ROSTER_JSON: &str = include_str!("../assets/data/roster.json");
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
}

#[derive(Debug, Clone)]
pub struct GameData {
    pub config: GameConfig,
    pub mission: MissionDef,
    pub roster: Vec<UnitDef>,
    pub texture_manifest: Vec<TextureConfig>,
}

impl GameData {
    pub fn load() -> Result<Self, String> {
        let config = load_embedded_json_labeled("game_config", GAME_CONFIG_JSON)?;
        let mission = load_embedded_json_labeled("mission", MISSION_JSON)?;
        let roster = load_embedded_json_labeled("roster", ROSTER_JSON)?;
        let texture_manifest = load_embedded_json(TEXTURE_MANIFEST_JSON)?;

        Ok(Self {
            config,
            mission,
            roster,
            texture_manifest,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phase_zero_content_is_valid() {
        let data = GameData::load().unwrap();

        assert_eq!(data.config.game_name, "mirexis");
        assert!(data.roster.iter().any(|unit| unit.team == Team::Colony));
        assert!(data.roster.iter().any(|unit| unit.team == Team::Hostile));
        assert!(data.mission.round_limit > 0);
        assert!(data.mission.terrain_costs.iter().all(|tile| tile.cost > 0));
        assert!(data.roster.iter().all(|unit| unit.weapon_ap_cost > 0));
    }
}
