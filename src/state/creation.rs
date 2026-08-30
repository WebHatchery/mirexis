//! Construction of a fresh tactical session from a mission and deployment roster.

use super::{
    BattleEvent, DestructibleCover, GameSession, HazardTile, ObjectiveState, TacticalPhase,
    TacticalState, UnitState,
};
use crate::data::{GameConfig, MissionDef, Team, UnitDef};
use macroquad_toolkit::grid::{FlatGrid, FogState, TilePos};
use macroquad_toolkit::rng::SeededRng;

impl GameSession {
    pub fn new(config: &GameConfig, mission: &MissionDef, roster: &[UnitDef]) -> Self {
        let fog = FlatGrid::new(config.world_width, config.world_height, FogState::Visible);
        let mut units = roster
            .iter()
            .map(|unit| UnitState::from_def(unit, config.max_action_points))
            .collect::<Vec<_>>();
        for unit in &mut units {
            crate::operation_modifiers::apply(mission.operation_modifier, unit);
        }
        let selected_unit = units
            .iter()
            .find(|unit| unit.team == Team::Colony)
            .map(|unit| unit.id.clone());
        let selected_tile = units
            .iter()
            .find(|unit| Some(&unit.id) == selected_unit.as_ref())
            .map(|unit| unit.position)
            .unwrap_or(TilePos::new(0, 0));
        let reinforcement_waves = crate::reinforcements::create_waves(config, mission, &units);
        let defense_integrity = crate::defense_objective::initial_integrity(mission.objective_kind);

        Self {
            tactical: TacticalState {
                fog,
                blocked: mission
                    .blocked_tiles
                    .iter()
                    .map(|position| super::tile(*position))
                    .collect(),
                terrain_costs: mission
                    .terrain_costs
                    .iter()
                    .map(|entry| (super::tile(entry.position), entry.cost))
                    .collect(),
                hazards: mission
                    .hazards
                    .iter()
                    .map(|hazard| HazardTile {
                        position: super::tile(hazard.position),
                        kind: hazard.kind,
                    })
                    .collect(),
                cover_edges: mission.cover_edges.clone(),
                destructible_cover: mission
                    .blocked_tiles
                    .iter()
                    .map(|position| DestructibleCover {
                        position: super::tile(*position),
                        health: mission.cover_integrity,
                        max_health: mission.cover_integrity,
                    })
                    .collect(),
                units,
                selected_unit,
                selected_tile,
                objective_tile: super::tile(mission.objective_tile),
                objective_kind: mission.objective_kind,
                objective_state: ObjectiveState::Active,
                objective_integrity: defense_integrity,
                objective_max_integrity: defense_integrity,
                phase: TacticalPhase::Player,
                round: 1,
                round_limit: mission.round_limit,
                materials: 20,
                rng: SeededRng::new(mission.seed),
                event_log: vec![BattleEvent::PhaseStarted {
                    phase: TacticalPhase::Player,
                    round: 1,
                }],
                reinforcement_waves,
                obscuring_fields: Vec::new(),
            },
        }
    }
}
