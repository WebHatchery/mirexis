//! Shared imports for integration tests.

pub use macroquad::prelude::*;
pub use macroquad_toolkit::grid::TilePos;
pub use macroquad_toolkit::rng::SeededRng;
pub use macroquad_toolkit::ui::VirtualUi;
pub use mirexis::campaign::SQUAD_LIMIT;
pub use mirexis::campaign::{Availability, CampaignState, OutsiderChoice};
pub use mirexis::colony::{
    BuildingKind, ColonyState, Resources, COLONY_HEIGHT, COLONY_WIDTH, SETTLEMENT_CENTER,
};
pub use mirexis::colony_exploration::{INTERACTION_DISTANCE, PLAYER_START};
pub use mirexis::colony_map_ui::{COLONY_HALF_HEIGHT, COLONY_HALF_WIDTH};
pub use mirexis::combat_feedback::CombatFeedback;
pub use mirexis::data::*;
pub use mirexis::defense_objective::{OBJECTIVE_TARGET_ID, STARTING_INTEGRITY};
pub use mirexis::first_hour::*;
pub use mirexis::first_hour_colony_ui::{BRIEFING_BUTTON, INVESTMENT_CHOICES, OPERATIONS_BUTTON};
pub use mirexis::game::{AppState, TacticalTargeting};
pub use mirexis::grid_ui::{
    GridView, WorldCamera, CANOPY_ART_PIVOT, CANOPY_ART_SCALE, TERRAIN_ART_PIVOT, TERRAIN_ART_SCALE,
};
pub use mirexis::map_variants::{AUTHORED_MAX_Y, BATTLEFIELD_Y_OFFSET, BATTLEFIELD_Y_STRIDE};
pub use mirexis::phase_replay::{PhaseReplay, BEAT_SECONDS, MAX_BEATS};
pub use mirexis::relationships::BONDED_BOND;
pub use mirexis::state::*;
pub use mirexis::tactical::*;
pub use mirexis::ui::TargetingView;
pub use mirexis::ui::LOGICAL_WIDTH;
pub use mirexis::ui_action::*;
pub use std::collections::{BTreeMap, HashSet};
