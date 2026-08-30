//! Class-technique schema shared by campaign data and tactical presentation.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TechniqueTarget {
    #[serde(rename = "self")]
    SelfTarget,
    Ally,
    Hostile,
    Tile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechniqueDef {
    pub id: String,
    pub name: String,
    pub description: String,
    pub target: TechniqueTarget,
    pub action_points: u8,
    pub experience_required: u32,
}
