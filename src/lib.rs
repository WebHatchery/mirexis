//! Public logic boundary for the Mirexis game.
//!
//! The binary owns the window and frame loop; this library owns the game
//! state, authored data, rules, UI intents, and rendering coordination so
//! integration tests can exercise the same code path as the executable.

pub mod action_preview;
pub mod action_preview_ui;
pub mod audio;
pub mod battle_log_ui;
pub mod briefing_deployment_ui;
pub mod briefing_intel_ui;
pub mod briefing_loadout_ui;
pub mod camera_controls;
pub mod campaign;
pub mod class_action_ui;
pub mod class_actions;
pub mod class_training;
pub mod colony;
pub mod colony_decision_ui;
pub mod colony_exploration;
pub mod colony_header_ui;
pub mod colony_map_ui;
pub mod colony_story;
pub mod colony_ui;
pub mod combat_feedback;
pub mod cover_actions;
pub mod cover_rules;
pub mod cover_ui;
pub mod danger_rating;
pub mod data;
pub mod defense_objective;
pub mod demo;
pub mod demo_ui;
pub mod enemy_abilities;
pub mod enemy_intent;
pub mod enemy_intent_ui;
pub mod epilogue;
pub mod equipment_actions;
pub mod equipment_catalog;
pub mod equipment_ui;
pub mod field_notes_ui;
pub mod first_hour;
pub mod first_hour_colony_ui;
pub mod first_hour_consequences_ui;
pub mod first_hour_investment_ui;
pub mod first_hour_metrics;
pub mod first_hour_tactical_ui;
pub mod first_hour_ui;
pub mod formation;
pub mod game;
pub mod gene_lab_ui;
pub mod grid_ui;
pub mod hazard_ui;
pub mod hazards;
pub mod help_ui;
pub mod map_variants;
pub mod memorial_ui;
pub mod objective_ui;
pub mod operation_modifiers;
pub mod outsider_ui;
pub mod overwatch;
pub mod persistence;
pub mod phase_readiness;
pub mod phase_refresh;
pub mod phase_replay;
pub mod portrait_ui;
pub mod reinforcement_ui;
pub mod reinforcements;
pub mod relationships;
pub mod roster_ui;
pub mod settings_ui;
pub mod skill_training;
pub mod skill_ui;
pub mod skills;
pub mod state;
pub mod strategy;
pub mod strategy_choices;
pub mod strategy_events;
pub mod strategy_rewards;
pub mod tactical;
pub mod tactical_ai;
pub mod tactical_hud;
pub mod tactical_map_ui;
pub mod tactical_unit_ui;
pub mod title_scene_ui;
pub mod trauma;
pub mod ui;
pub mod ui_action;
pub mod ui_debrief;
pub mod ui_widgets;
pub mod visual_assets;
pub mod world_art;

/// Shared imports for migrated rule and layout tests.
///
/// These are intentionally narrow: they expose the stable data, state, grid,
/// and rendering primitives that the integration suites previously received
/// from their parent implementation module's private imports.
#[doc(hidden)]
pub mod test_prelude {
    pub use crate::campaign::OutsiderChoice;
    pub use crate::campaign::SQUAD_LIMIT;
    pub use crate::campaign::{Availability, CampaignState};
    pub use crate::colony::{
        BuildingKind, ColonyState, Resources, COLONY_HEIGHT, COLONY_WIDTH, SETTLEMENT_CENTER,
    };
    pub use crate::colony_exploration::{INTERACTION_DISTANCE, PLAYER_START};
    pub use crate::colony_map_ui::{COLONY_HALF_HEIGHT, COLONY_HALF_WIDTH};
    pub use crate::data::*;
    pub use crate::defense_objective::{OBJECTIVE_TARGET_ID, STARTING_INTEGRITY};
    pub use crate::first_hour::*;
    pub use crate::first_hour_colony_ui::{BRIEFING_BUTTON, INVESTMENT_CHOICES, OPERATIONS_BUTTON};
    pub use crate::grid_ui::{
        GridView, WorldCamera, CANOPY_ART_PIVOT, CANOPY_ART_SCALE, TERRAIN_ART_PIVOT,
        TERRAIN_ART_SCALE,
    };
    pub use crate::map_variants::{AUTHORED_MAX_Y, BATTLEFIELD_Y_OFFSET, BATTLEFIELD_Y_STRIDE};
    pub use crate::phase_replay::{BEAT_SECONDS, MAX_BEATS};
    pub use crate::relationships::BONDED_BOND;
    pub use crate::state::*;
    pub use crate::tactical::*;
    pub use crate::ui::TargetingView;
    pub use crate::ui::LOGICAL_WIDTH;
    pub use crate::ui_action::*;
    pub use macroquad::prelude::*;
    pub use macroquad_toolkit::grid::TilePos;
    pub use macroquad_toolkit::rng::SeededRng;
    pub use macroquad_toolkit::ui::VirtualUi;
    pub use std::collections::{BTreeMap, HashSet};
}
