//! Embedded JSON loading for the authored Mirexis content registry.

use super::*;
use macroquad_toolkit::data_loader::load_embedded_json_labeled;

pub const GAME_CONFIG_JSON: &str =
    macroquad_toolkit::include_json_str!("../../assets/data/game_config.json");
pub const MISSION_JSON: &str =
    macroquad_toolkit::include_json_str!("../../assets/data/mission.json");
pub const ROSTER_JSON: &str = macroquad_toolkit::include_json_str!("../../assets/data/roster.json");
pub const RECRUITABLE_ROSTER_JSON: &str =
    macroquad_toolkit::include_json_str!("../../assets/data/recruitable_roster.json");
pub const CHARACTERS_JSON: &str =
    macroquad_toolkit::include_json_str!("../../assets/data/characters.json");
pub const CLASSES_JSON: &str =
    macroquad_toolkit::include_json_str!("../../assets/data/classes.json");
pub const MUTATIONS_JSON: &str =
    macroquad_toolkit::include_json_str!("../../assets/data/mutations.json");
pub const EQUIPMENT_JSON: &str =
    macroquad_toolkit::include_json_str!("../../assets/data/equipment.json");
pub const CAMPAIGN_JSON: &str =
    macroquad_toolkit::include_json_str!("../../assets/data/campaign.json");
pub const TEXTURE_MANIFEST_JSON: &str =
    macroquad_toolkit::include_json_str!("../../assets/data/texture_manifest.json");

impl GameData {
    pub fn load() -> Result<Self, String> {
        let config = load_embedded_json_labeled("game_config", GAME_CONFIG_JSON)?;
        let mission = load_embedded_json_labeled("mission", MISSION_JSON)?;
        let roster = load_embedded_json_labeled("roster", ROSTER_JSON)?;
        let recruitable_roster =
            load_embedded_json_labeled("recruitable_roster", RECRUITABLE_ROSTER_JSON)?;
        let characters = load_embedded_json_labeled("characters", CHARACTERS_JSON)?;
        let classes = load_embedded_json_labeled("classes", CLASSES_JSON)?;
        let mutations = load_embedded_json_labeled("mutations", MUTATIONS_JSON)?;
        let equipment = load_embedded_json_labeled("equipment", EQUIPMENT_JSON)?;
        let campaign = load_embedded_json_labeled("campaign", CAMPAIGN_JSON)?;
        let texture_manifest =
            load_embedded_json_labeled("texture_manifest", TEXTURE_MANIFEST_JSON)?;

        let data = Self {
            config,
            mission,
            roster,
            recruitable_roster,
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
}
