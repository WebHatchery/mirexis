//! Deterministic tactical command simulation and persistence model.

use crate::campaign::CampaignState;
use crate::data::{GameConfig, MissionDef, ObjectiveKind, Team};
use crate::tactical::{line_between, manhattan, path_cost, UnitAnimationState, UnitFacing};
pub use crate::tactical::{
    BattleEvent, Command, CommandCost, DestructibleCover, HazardTile, ObjectiveState,
    ObscuringField, ReinforcementWave, RuleError, StatusKind, TacticalPhase, TacticalState,
    UnitState,
};
use macroquad_toolkit::grid::TilePos;
use serde::{Deserialize, Serialize};

pub mod combat;
pub mod creation;
pub mod pathfinding;
pub mod session;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveData {
    pub version: String,
    pub campaign: CampaignState,
    #[serde(default)]
    pub active_mission: Option<MissionDef>,
    pub tactical: Option<TacticalState>,
}

impl SaveData {
    pub fn campaign_only(version: &str, campaign: &CampaignState) -> Self {
        Self {
            version: version.to_owned(),
            campaign: campaign.clone(),
            active_mission: None,
            tactical: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MissionOutcome {
    pub result: ObjectiveState,
    pub colonists_deployed: usize,
    pub colonists_incapacitated: Vec<CharacterConsequence>,
    pub hostiles_neutralised: usize,
    pub materials_awarded: i32,
    pub biomass_awarded: i32,
    pub power_awarded: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterConsequence {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct GameSession {
    pub tactical: TacticalState,
}

pub fn tile(position: [i32; 2]) -> TilePos {
    TilePos::new(position[0], position[1])
}

pub fn mutation_gift_name(mutation: &str) -> Option<&'static str> {
    match mutation {
        "Neural Bloom" => Some("NEURAL FOCUS · +20 accuracy this round"),
        "Chitinous Growth" => Some("HARDEN CARAPACE · +3 armour this round"),
        "Regenerative Tissue" => Some("ACCELERATE TISSUE · restore 3 vitality"),
        "Elastic Musculature" => Some("COIL MUSCLE · +2 AP and +3 movement this round"),
        "Symbiotic Organism" => Some("FEEDING FRENZY · +2 weapon damage this round"),
        _ => None,
    }
}
