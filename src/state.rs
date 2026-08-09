//! Deterministic tactical state and persistence model.

use crate::data::{GameConfig, MissionDef, Team, UnitDef};
use macroquad_toolkit::grid::{FlatGrid, FogState, TilePos};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TacticalPhase {
    Player,
    Enemy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitState {
    pub id: String,
    pub name: String,
    pub role: String,
    pub mutation: String,
    pub team: Team,
    pub position: TilePos,
    pub health: i32,
    pub max_health: i32,
    pub move_range: u8,
    pub action_points: u8,
}

impl UnitState {
    fn from_def(def: &UnitDef, action_points: u8) -> Self {
        Self {
            id: def.id.clone(),
            name: def.name.clone(),
            role: def.role.clone(),
            mutation: def.mutation.clone(),
            team: def.team,
            position: TilePos::new(def.position[0], def.position[1]),
            health: def.max_health,
            max_health: def.max_health,
            move_range: def.move_range,
            action_points,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TacticalState {
    pub fog: FlatGrid<FogState>,
    pub blocked: HashSet<TilePos>,
    pub units: Vec<UnitState>,
    pub selected_unit: Option<String>,
    pub selected_tile: TilePos,
    pub phase: TacticalPhase,
    pub round: u32,
    pub materials: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveData {
    pub version: String,
    pub tactical: TacticalState,
}

#[derive(Debug, Clone)]
pub struct GameSession {
    pub tactical: TacticalState,
}

impl GameSession {
    pub fn new(config: &GameConfig, mission: &MissionDef, roster: &[UnitDef]) -> Self {
        let fog = FlatGrid::new(config.world_width, config.world_height, FogState::Visible);
        let blocked = mission
            .blocked_tiles
            .iter()
            .map(|position| TilePos::new(position[0], position[1]))
            .collect();
        let units = roster
            .iter()
            .map(|unit| UnitState::from_def(unit, config.max_action_points))
            .collect::<Vec<_>>();
        let selected_unit = units
            .iter()
            .find(|unit| unit.team == Team::Colony)
            .map(|unit| unit.id.clone());
        let selected_tile = units
            .iter()
            .find(|unit| Some(&unit.id) == selected_unit.as_ref())
            .map(|unit| unit.position)
            .unwrap_or(TilePos::new(0, 0));

        Self {
            tactical: TacticalState {
                fog,
                blocked,
                units,
                selected_unit,
                selected_tile,
                phase: TacticalPhase::Player,
                round: 1,
                materials: 20,
            },
        }
    }

    pub fn from_save(save: SaveData) -> Self {
        Self {
            tactical: save.tactical,
        }
    }

    pub fn to_save(&self, version: &str) -> SaveData {
        SaveData {
            version: version.to_owned(),
            tactical: self.tactical.clone(),
        }
    }

    pub fn selected_unit(&self) -> Option<&UnitState> {
        let selected = self.tactical.selected_unit.as_ref()?;
        self.tactical.units.iter().find(|unit| &unit.id == selected)
    }

    pub fn select_tile(&mut self, tile: TilePos) {
        if !self.tactical.fog.is_valid(tile) {
            return;
        }
        self.tactical.selected_tile = tile;
        if let Some(unit) = self
            .tactical
            .units
            .iter()
            .find(|unit| unit.position == tile && unit.team == Team::Colony)
        {
            self.tactical.selected_unit = Some(unit.id.clone());
        }
    }

    pub fn move_selection(&mut self, dx: i32, dy: i32) {
        let tile = TilePos::new(
            self.tactical.selected_tile.x + dx,
            self.tactical.selected_tile.y + dy,
        );
        self.select_tile(tile);
    }

    pub fn can_move_selected_to(&self, tile: TilePos) -> bool {
        if self.tactical.phase != TacticalPhase::Player
            || !self.tactical.fog.is_valid(tile)
            || self.tactical.blocked.contains(&tile)
            || self.tactical.units.iter().any(|unit| unit.position == tile)
        {
            return false;
        }
        let Some(unit) = self.selected_unit() else {
            return false;
        };
        let distance = manhattan(unit.position, tile);
        distance > 0 && distance <= unit.move_range as i32 && distance <= unit.action_points as i32
    }

    pub fn move_selected_to(&mut self, tile: TilePos) -> bool {
        if !self.can_move_selected_to(tile) {
            return false;
        }
        let selected = self.tactical.selected_unit.clone().unwrap();
        let unit = self
            .tactical
            .units
            .iter_mut()
            .find(|unit| unit.id == selected)
            .unwrap();
        let cost = manhattan(unit.position, tile) as u8;
        unit.position = tile;
        unit.action_points -= cost;
        self.tactical.selected_tile = tile;
        true
    }

    pub fn end_player_phase(&mut self, config: &GameConfig) {
        self.tactical.phase = TacticalPhase::Enemy;
        self.advance_hostiles();
        self.tactical.round += 1;
        self.tactical.phase = TacticalPhase::Player;
        for unit in &mut self.tactical.units {
            if unit.team == Team::Colony {
                unit.action_points = config.max_action_points;
            }
        }
    }

    fn advance_hostiles(&mut self) {
        let colonists = self
            .tactical
            .units
            .iter()
            .filter(|unit| unit.team == Team::Colony)
            .map(|unit| unit.position)
            .collect::<Vec<_>>();
        let mut occupied = self
            .tactical
            .units
            .iter()
            .map(|unit| unit.position)
            .collect::<HashSet<_>>();

        for unit in self
            .tactical
            .units
            .iter_mut()
            .filter(|unit| unit.team == Team::Hostile)
        {
            let Some(target) = colonists
                .iter()
                .min_by_key(|target| manhattan(unit.position, **target))
            else {
                continue;
            };
            let dx = (target.x - unit.position.x).signum();
            let dy = (target.y - unit.position.y).signum();
            let candidates = [
                TilePos::new(unit.position.x + dx, unit.position.y),
                TilePos::new(unit.position.x, unit.position.y + dy),
            ];
            occupied.remove(&unit.position);
            if let Some(next) = candidates.into_iter().find(|candidate| {
                self.tactical.fog.is_valid(*candidate)
                    && !self.tactical.blocked.contains(candidate)
                    && !occupied.contains(candidate)
            }) {
                unit.position = next;
            }
            occupied.insert(unit.position);
        }
    }
}

fn manhattan(a: TilePos, b: TilePos) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

pub fn migrate_save_value(
    detected_version: Option<String>,
    value: Value,
    config: &GameConfig,
) -> Result<SaveData, String> {
    let payload = value.get("data").cloned().unwrap_or(value);
    let mut save = serde_json::from_value::<SaveData>(payload)
        .map_err(|err| format!("Unsupported Mirexis save {:?}: {}", detected_version, err))?;
    save.version = config.version.clone();
    Ok(save)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> (GameConfig, MissionDef, Vec<UnitDef>) {
        let data = crate::data::GameData::load().unwrap();
        (data.config, data.mission, data.roster)
    }

    #[test]
    fn selected_colonist_spends_action_points_to_move() {
        let (config, mission, roster) = fixture();
        let mut session = GameSession::new(&config, &mission, &roster);
        let start = session.selected_unit().unwrap().position;
        let target = TilePos::new(start.x + 1, start.y);

        assert!(session.move_selected_to(target));
        assert_eq!(
            session.selected_unit().unwrap().action_points,
            config.max_action_points - 1
        );
    }

    #[test]
    fn blocked_tiles_reject_movement() {
        let (config, mission, roster) = fixture();
        let mut session = GameSession::new(&config, &mission, &roster);
        let blocked = *session.tactical.blocked.iter().next().unwrap();

        assert!(!session.move_selected_to(blocked));
    }

    #[test]
    fn ending_phase_advances_round_and_refreshes_colonists() {
        let (config, mission, roster) = fixture();
        let mut session = GameSession::new(&config, &mission, &roster);
        session.tactical.selected_unit = None;
        session.end_player_phase(&config);

        assert_eq!(session.tactical.round, 2);
        assert!(session
            .tactical
            .units
            .iter()
            .filter(|unit| unit.team == Team::Colony)
            .all(|unit| unit.action_points == config.max_action_points));
    }
}
