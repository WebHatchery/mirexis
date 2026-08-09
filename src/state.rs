//! Deterministic tactical command simulation and persistence model.

use crate::data::{CoverEdgeDef, EdgeDirection, GameConfig, MissionDef, Team, UnitDef};
use macroquad_toolkit::grid::{FlatGrid, FogState, TilePos};
use macroquad_toolkit::pathfinding::{find_path_with, Heuristic, Pos};
use macroquad_toolkit::rng::SeededRng;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TacticalPhase {
    Player,
    Enemy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObjectiveState {
    Active,
    Secured,
    Victory,
    Failed,
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
    pub armour: i32,
    pub accuracy: i32,
    pub weapon_range: u8,
    pub weapon_damage: i32,
    pub weapon_ap_cost: u8,
    pub incapacitated: bool,
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
            armour: def.armour,
            accuracy: def.accuracy,
            weapon_range: def.weapon_range,
            weapon_damage: def.weapon_damage,
            weapon_ap_cost: def.weapon_ap_cost,
            incapacitated: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Command {
    Move {
        unit_id: String,
        to: TilePos,
    },
    Attack {
        attacker_id: String,
        target_id: String,
    },
    Interact {
        unit_id: String,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommandCost {
    pub action_points: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuleError {
    WrongPhase,
    UnknownUnit,
    WrongTeam,
    Incapacitated,
    Occupied,
    NoPath,
    OutOfRange,
    InsufficientActionPoints,
    InvalidTarget,
    ObjectiveUnavailable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BattleEvent {
    UnitMoved {
        unit_id: String,
        path: Vec<TilePos>,
        cost: u8,
    },
    AttackRolled {
        attacker_id: String,
        target_id: String,
        roll: u8,
        hit_chance: u8,
    },
    DamageApplied {
        target_id: String,
        amount: i32,
        remaining: i32,
    },
    UnitIncapacitated {
        unit_id: String,
    },
    ObjectiveSecured {
        unit_id: String,
    },
    PhaseStarted {
        phase: TacticalPhase,
        round: u32,
    },
    BattleEnded {
        outcome: ObjectiveState,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TacticalState {
    pub fog: FlatGrid<FogState>,
    pub blocked: HashSet<TilePos>,
    pub terrain_costs: Vec<(TilePos, u8)>,
    pub cover_edges: Vec<CoverEdgeDef>,
    pub units: Vec<UnitState>,
    pub selected_unit: Option<String>,
    pub selected_tile: TilePos,
    pub objective_tile: TilePos,
    pub objective_state: ObjectiveState,
    pub phase: TacticalPhase,
    pub round: u32,
    pub round_limit: u32,
    pub materials: i32,
    pub rng: SeededRng,
    pub event_log: Vec<BattleEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveData {
    pub version: String,
    pub tactical: TacticalState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MissionOutcome {
    pub result: ObjectiveState,
    pub colonists_deployed: usize,
    pub colonists_incapacitated: Vec<String>,
    pub hostiles_neutralised: usize,
    pub materials_awarded: i32,
}

#[derive(Debug, Clone)]
pub struct GameSession {
    pub tactical: TacticalState,
}

impl GameSession {
    pub fn new(config: &GameConfig, mission: &MissionDef, roster: &[UnitDef]) -> Self {
        let fog = FlatGrid::new(config.world_width, config.world_height, FogState::Visible);
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
                blocked: mission
                    .blocked_tiles
                    .iter()
                    .map(|position| tile(*position))
                    .collect(),
                terrain_costs: mission
                    .terrain_costs
                    .iter()
                    .map(|entry| (tile(entry.position), entry.cost))
                    .collect(),
                cover_edges: mission.cover_edges.clone(),
                units,
                selected_unit,
                selected_tile,
                objective_tile: tile(mission.objective_tile),
                objective_state: ObjectiveState::Active,
                phase: TacticalPhase::Player,
                round: 1,
                round_limit: mission.round_limit,
                materials: 20,
                rng: SeededRng::new(config.battle_seed),
                event_log: vec![BattleEvent::PhaseStarted {
                    phase: TacticalPhase::Player,
                    round: 1,
                }],
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
        self.unit(self.tactical.selected_unit.as_deref()?)
    }

    pub fn unit(&self, id: &str) -> Option<&UnitState> {
        self.tactical.units.iter().find(|unit| unit.id == id)
    }

    pub fn select_tile(&mut self, tile: TilePos) {
        if !self.tactical.fog.is_valid(tile) {
            return;
        }
        self.tactical.selected_tile = tile;
        if let Some(unit) =
            self.tactical.units.iter().find(|unit| {
                unit.position == tile && unit.team == Team::Colony && !unit.incapacitated
            })
        {
            self.tactical.selected_unit = Some(unit.id.clone());
        }
    }

    pub fn move_selection(&mut self, dx: i32, dy: i32) {
        self.select_tile(TilePos::new(
            self.tactical.selected_tile.x + dx,
            self.tactical.selected_tile.y + dy,
        ));
    }

    pub fn validate(&self, command: &Command) -> Result<CommandCost, RuleError> {
        if self.tactical.objective_state == ObjectiveState::Victory
            || self.tactical.objective_state == ObjectiveState::Failed
        {
            return Err(RuleError::WrongPhase);
        }
        match command {
            Command::Move { unit_id, to } => self.validate_move(unit_id, *to),
            Command::Attack {
                attacker_id,
                target_id,
            } => self.validate_attack(attacker_id, target_id),
            Command::Interact { unit_id } => self.validate_interact(unit_id),
        }
    }

    pub fn execute(&mut self, command: Command) -> Result<Vec<BattleEvent>, RuleError> {
        self.validate(&command)?;
        let events = match command {
            Command::Move { unit_id, to } => self.execute_move(&unit_id, to),
            Command::Attack {
                attacker_id,
                target_id,
            } => self.execute_attack(&attacker_id, &target_id),
            Command::Interact { unit_id } => self.execute_interact(&unit_id),
        };
        self.tactical.event_log.extend(events.iter().cloned());
        Ok(events)
    }

    pub fn can_move_selected_to(&self, to: TilePos) -> bool {
        self.tactical.selected_unit.as_ref().is_some_and(|unit_id| {
            self.validate(&Command::Move {
                unit_id: unit_id.clone(),
                to,
            })
            .is_ok()
        })
    }

    pub fn move_selected_to(&mut self, to: TilePos) -> bool {
        let Some(unit_id) = self.tactical.selected_unit.clone() else {
            return false;
        };
        self.execute(Command::Move { unit_id, to }).is_ok()
    }

    pub fn can_attack_selected(&self, target_id: &str) -> bool {
        self.tactical
            .selected_unit
            .as_ref()
            .is_some_and(|attacker_id| {
                self.validate(&Command::Attack {
                    attacker_id: attacker_id.clone(),
                    target_id: target_id.to_owned(),
                })
                .is_ok()
            })
    }

    pub fn attack_selected(&mut self, target_id: &str) -> Result<Vec<BattleEvent>, RuleError> {
        let attacker_id = self
            .tactical
            .selected_unit
            .clone()
            .ok_or(RuleError::UnknownUnit)?;
        self.execute(Command::Attack {
            attacker_id,
            target_id: target_id.to_owned(),
        })
    }

    pub fn interact_selected(&mut self) -> Result<Vec<BattleEvent>, RuleError> {
        let unit_id = self
            .tactical
            .selected_unit
            .clone()
            .ok_or(RuleError::UnknownUnit)?;
        self.execute(Command::Interact { unit_id })
    }

    pub fn can_interact_selected(&self) -> bool {
        self.tactical.selected_unit.as_ref().is_some_and(|unit_id| {
            self.validate(&Command::Interact {
                unit_id: unit_id.clone(),
            })
            .is_ok()
        })
    }

    pub fn end_player_phase(&mut self, config: &GameConfig) {
        if self.tactical.phase != TacticalPhase::Player || self.battle_is_over() {
            return;
        }
        self.tactical.phase = TacticalPhase::Enemy;
        self.push_event(BattleEvent::PhaseStarted {
            phase: TacticalPhase::Enemy,
            round: self.tactical.round,
        });
        self.refresh_team(Team::Hostile, config.max_action_points);
        self.resolve_enemy_phase();
        self.tactical.round += 1;
        if self.tactical.round > self.tactical.round_limit && !self.battle_is_over() {
            self.finish_battle(ObjectiveState::Failed);
            return;
        }
        self.tactical.phase = TacticalPhase::Player;
        self.refresh_team(Team::Colony, config.max_action_points);
        self.push_event(BattleEvent::PhaseStarted {
            phase: TacticalPhase::Player,
            round: self.tactical.round,
        });
    }

    pub fn battle_is_over(&self) -> bool {
        matches!(
            self.tactical.objective_state,
            ObjectiveState::Victory | ObjectiveState::Failed
        )
    }

    pub fn mission_outcome(&self, mission: &MissionDef) -> Option<MissionOutcome> {
        self.battle_is_over().then(|| MissionOutcome {
            result: self.tactical.objective_state,
            colonists_deployed: self
                .tactical
                .units
                .iter()
                .filter(|unit| unit.team == Team::Colony)
                .count(),
            colonists_incapacitated: self
                .tactical
                .units
                .iter()
                .filter(|unit| unit.team == Team::Colony && unit.incapacitated)
                .map(|unit| unit.name.clone())
                .collect(),
            hostiles_neutralised: self
                .tactical
                .units
                .iter()
                .filter(|unit| unit.team == Team::Hostile && unit.incapacitated)
                .count(),
            materials_awarded: if self.tactical.objective_state == ObjectiveState::Victory {
                mission.materials_reward
            } else {
                0
            },
        })
    }

    fn validate_move(&self, unit_id: &str, to: TilePos) -> Result<CommandCost, RuleError> {
        let unit = self.active_unit_for_phase(unit_id)?;
        if self
            .tactical
            .units
            .iter()
            .any(|other| other.position == to && !other.incapacitated)
        {
            return Err(RuleError::Occupied);
        }
        let path = self
            .path_for(unit.position, to, Some(unit_id))
            .ok_or(RuleError::NoPath)?;
        let cost = path_cost(&path, &self.tactical.terrain_costs);
        if cost == 0 || cost > unit.move_range || cost > unit.action_points {
            return Err(if cost > unit.action_points {
                RuleError::InsufficientActionPoints
            } else {
                RuleError::OutOfRange
            });
        }
        Ok(CommandCost {
            action_points: cost,
        })
    }

    fn validate_attack(
        &self,
        attacker_id: &str,
        target_id: &str,
    ) -> Result<CommandCost, RuleError> {
        let attacker = self.active_unit_for_phase(attacker_id)?;
        let target = self.unit(target_id).ok_or(RuleError::UnknownUnit)?;
        if target.incapacitated || attacker.team == target.team {
            return Err(RuleError::InvalidTarget);
        }
        if manhattan(attacker.position, target.position) > attacker.weapon_range as i32 {
            return Err(RuleError::OutOfRange);
        }
        if attacker.action_points < attacker.weapon_ap_cost {
            return Err(RuleError::InsufficientActionPoints);
        }
        Ok(CommandCost {
            action_points: attacker.weapon_ap_cost,
        })
    }

    fn validate_interact(&self, unit_id: &str) -> Result<CommandCost, RuleError> {
        let unit = self.active_unit_for_phase(unit_id)?;
        if self.tactical.objective_state != ObjectiveState::Active
            || manhattan(unit.position, self.tactical.objective_tile) > 1
        {
            return Err(RuleError::ObjectiveUnavailable);
        }
        if unit.action_points < 1 {
            return Err(RuleError::InsufficientActionPoints);
        }
        Ok(CommandCost { action_points: 1 })
    }

    fn active_unit_for_phase(&self, id: &str) -> Result<&UnitState, RuleError> {
        let unit = self.unit(id).ok_or(RuleError::UnknownUnit)?;
        let active_team = match self.tactical.phase {
            TacticalPhase::Player => Team::Colony,
            TacticalPhase::Enemy => Team::Hostile,
        };
        if unit.team != active_team {
            return Err(RuleError::WrongTeam);
        }
        if unit.incapacitated {
            return Err(RuleError::Incapacitated);
        }
        Ok(unit)
    }

    fn execute_move(&mut self, unit_id: &str, to: TilePos) -> Vec<BattleEvent> {
        let from = self.unit(unit_id).unwrap().position;
        let path = self.path_for(from, to, Some(unit_id)).unwrap();
        let cost = path_cost(&path, &self.tactical.terrain_costs);
        let unit = self
            .tactical
            .units
            .iter_mut()
            .find(|unit| unit.id == unit_id)
            .unwrap();
        unit.position = to;
        unit.action_points -= cost;
        if self.tactical.selected_unit.as_deref() == Some(unit_id) {
            self.tactical.selected_tile = to;
        }
        vec![BattleEvent::UnitMoved {
            unit_id: unit_id.to_owned(),
            path,
            cost,
        }]
    }

    fn execute_attack(&mut self, attacker_id: &str, target_id: &str) -> Vec<BattleEvent> {
        let attacker = self.unit(attacker_id).unwrap().clone();
        let target = self.unit(target_id).unwrap().clone();
        let hit_chance = self.hit_chance(&attacker, &target);
        let roll = self.tactical.rng.range_i32(1, 101) as u8;
        self.tactical
            .units
            .iter_mut()
            .find(|unit| unit.id == attacker_id)
            .unwrap()
            .action_points -= attacker.weapon_ap_cost;
        let mut events = vec![BattleEvent::AttackRolled {
            attacker_id: attacker_id.to_owned(),
            target_id: target_id.to_owned(),
            roll,
            hit_chance,
        }];
        if roll <= hit_chance {
            let critical = roll <= 10;
            let damage = (attacker.weapon_damage + i32::from(critical) * 2 - target.armour).max(1);
            let target = self
                .tactical
                .units
                .iter_mut()
                .find(|unit| unit.id == target_id)
                .unwrap();
            target.health = (target.health - damage).max(0);
            events.push(BattleEvent::DamageApplied {
                target_id: target_id.to_owned(),
                amount: damage,
                remaining: target.health,
            });
            if target.health == 0 {
                target.incapacitated = true;
                events.push(BattleEvent::UnitIncapacitated {
                    unit_id: target_id.to_owned(),
                });
            }
        }
        self.check_outcome(&mut events);
        events
    }

    fn execute_interact(&mut self, unit_id: &str) -> Vec<BattleEvent> {
        self.tactical
            .units
            .iter_mut()
            .find(|unit| unit.id == unit_id)
            .unwrap()
            .action_points -= 1;
        self.tactical.objective_state = ObjectiveState::Secured;
        let mut events = vec![BattleEvent::ObjectiveSecured {
            unit_id: unit_id.to_owned(),
        }];
        self.check_outcome(&mut events);
        events
    }

    fn hit_chance(&self, attacker: &UnitState, target: &UnitState) -> u8 {
        let distance = manhattan(attacker.position, target.position);
        let range_penalty = (distance - i32::from(attacker.weapon_range) / 2).max(0) * 5;
        let cover = self.cover_against(target.position, attacker.position);
        (attacker.accuracy - range_penalty - cover).clamp(5, 95) as u8
    }

    fn cover_against(&self, target: TilePos, attacker: TilePos) -> i32 {
        let direction = if (attacker.x - target.x).abs() >= (attacker.y - target.y).abs() {
            if attacker.x < target.x {
                EdgeDirection::West
            } else {
                EdgeDirection::East
            }
        } else if attacker.y < target.y {
            EdgeDirection::North
        } else {
            EdgeDirection::South
        };
        self.tactical
            .cover_edges
            .iter()
            .find(|edge| tile(edge.position) == target && edge.direction == direction)
            .map_or(0, |edge| i32::from(edge.strength))
    }

    fn path_for(&self, start: TilePos, goal: TilePos, mover: Option<&str>) -> Option<Vec<TilePos>> {
        let occupied = self
            .tactical
            .units
            .iter()
            .filter(|unit| !unit.incapacitated && Some(unit.id.as_str()) != mover)
            .map(|unit| unit.position)
            .collect::<HashSet<_>>();
        let path = find_path_with(
            Pos::new(start.x, start.y),
            Pos::new(goal.x, goal.y),
            self.tactical.fog.width,
            self.tactical.fog.height,
            |pos| {
                let pos = TilePos::new(pos.x, pos.y);
                !self.tactical.blocked.contains(&pos) && !occupied.contains(&pos)
            },
            |pos| {
                f32::from(terrain_cost(
                    TilePos::new(pos.x, pos.y),
                    &self.tactical.terrain_costs,
                ))
            },
            Heuristic::Manhattan,
            false,
        )?;
        Some(
            path.waypoints
                .into_iter()
                .map(|pos| TilePos::new(pos.x, pos.y))
                .collect(),
        )
    }

    fn resolve_enemy_phase(&mut self) {
        let mut enemies = self
            .tactical
            .units
            .iter()
            .filter(|unit| unit.team == Team::Hostile && !unit.incapacitated)
            .map(|unit| unit.id.clone())
            .collect::<Vec<_>>();
        enemies.sort();
        for enemy_id in enemies {
            if self.battle_is_over() {
                break;
            }
            if let Some(target_id) = self.best_attack_target(&enemy_id) {
                let _ = self.execute(Command::Attack {
                    attacker_id: enemy_id.clone(),
                    target_id,
                });
                continue;
            }
            if let Some(destination) = self.best_enemy_move(&enemy_id) {
                let _ = self.execute(Command::Move {
                    unit_id: enemy_id.clone(),
                    to: destination,
                });
            }
            if let Some(target_id) = self.best_attack_target(&enemy_id) {
                let _ = self.execute(Command::Attack {
                    attacker_id: enemy_id.clone(),
                    target_id,
                });
            }
        }
    }

    fn best_attack_target(&self, attacker_id: &str) -> Option<String> {
        let mut targets = self
            .tactical
            .units
            .iter()
            .filter(|unit| {
                unit.team == Team::Colony
                    && !unit.incapacitated
                    && self
                        .validate(&Command::Attack {
                            attacker_id: attacker_id.to_owned(),
                            target_id: unit.id.clone(),
                        })
                        .is_ok()
            })
            .collect::<Vec<_>>();
        targets.sort_by_key(|unit| (unit.health, unit.id.clone()));
        targets.first().map(|unit| unit.id.clone())
    }

    fn best_enemy_move(&self, enemy_id: &str) -> Option<TilePos> {
        let enemy = self.unit(enemy_id)?;
        let target = self
            .tactical
            .units
            .iter()
            .filter(|unit| unit.team == Team::Colony && !unit.incapacitated)
            .min_by_key(|unit| (manhattan(enemy.position, unit.position), unit.id.clone()))?;
        let mut candidates = enemy
            .position
            .neighbors_4way()
            .into_iter()
            .filter(|to| {
                self.validate(&Command::Move {
                    unit_id: enemy_id.to_owned(),
                    to: *to,
                })
                .is_ok()
            })
            .collect::<Vec<_>>();
        candidates.sort_by_key(|to| (manhattan(*to, target.position), to.y, to.x));
        candidates.first().copied()
    }

    fn check_outcome(&mut self, events: &mut Vec<BattleEvent>) {
        let colonists_alive = self
            .tactical
            .units
            .iter()
            .any(|unit| unit.team == Team::Colony && !unit.incapacitated);
        let hostiles_alive = self
            .tactical
            .units
            .iter()
            .any(|unit| unit.team == Team::Hostile && !unit.incapacitated);
        let outcome = if !colonists_alive {
            Some(ObjectiveState::Failed)
        } else if !hostiles_alive && self.tactical.objective_state == ObjectiveState::Secured {
            Some(ObjectiveState::Victory)
        } else {
            None
        };
        if let Some(outcome) = outcome {
            self.tactical.objective_state = outcome;
            events.push(BattleEvent::BattleEnded { outcome });
        }
    }

    fn finish_battle(&mut self, outcome: ObjectiveState) {
        self.tactical.objective_state = outcome;
        self.push_event(BattleEvent::BattleEnded { outcome });
    }

    fn refresh_team(&mut self, team: Team, action_points: u8) {
        for unit in &mut self.tactical.units {
            if unit.team == team && !unit.incapacitated {
                unit.action_points = action_points;
            }
        }
    }

    fn push_event(&mut self, event: BattleEvent) {
        self.tactical.event_log.push(event);
    }
}

fn tile(position: [i32; 2]) -> TilePos {
    TilePos::new(position[0], position[1])
}

fn terrain_cost(position: TilePos, costs: &[(TilePos, u8)]) -> u8 {
    costs
        .iter()
        .find(|(tile, _)| *tile == position)
        .map_or(1, |(_, cost)| *cost)
}

fn path_cost(path: &[TilePos], costs: &[(TilePos, u8)]) -> u8 {
    path.iter()
        .skip(1)
        .map(|tile| terrain_cost(*tile, costs))
        .sum()
}

fn manhattan(a: TilePos, b: TilePos) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn session() -> (GameConfig, GameSession) {
        let data = crate::data::GameData::load().unwrap();
        let session = GameSession::new(&data.config, &data.mission, &data.roster);
        (data.config, session)
    }

    #[test]
    fn weighted_path_spends_terrain_cost() {
        let (_, mut session) = session();
        let unit_id = session.tactical.selected_unit.clone().unwrap();
        session
            .tactical
            .units
            .iter_mut()
            .find(|unit| unit.id == unit_id)
            .unwrap()
            .position = TilePos::new(2, 2);
        let events = session
            .execute(Command::Move {
                unit_id,
                to: TilePos::new(3, 2),
            })
            .unwrap();
        assert!(matches!(events[0], BattleEvent::UnitMoved { cost: 2, .. }));
    }

    #[test]
    fn attacks_are_deterministic_and_emit_ordered_events() {
        let data = crate::data::GameData::load().unwrap();
        let mut a = GameSession::new(&data.config, &data.mission, &data.roster);
        let mut b = a.clone();
        for session in [&mut a, &mut b] {
            session
                .tactical
                .units
                .iter_mut()
                .find(|unit| unit.id == "kira_voss")
                .unwrap()
                .position = TilePos::new(8, 2);
        }
        let command = Command::Attack {
            attacker_id: "kira_voss".into(),
            target_id: "brood_stalker_a".into(),
        };
        assert_eq!(
            a.execute(command.clone()).unwrap(),
            b.execute(command).unwrap()
        );
    }

    #[test]
    fn edge_cover_reduces_accuracy_from_its_facing_direction() {
        let (_, session) = session();
        let attacker = UnitState {
            position: TilePos::new(3, 4),
            ..session.unit("kira_voss").unwrap().clone()
        };
        let target = UnitState {
            position: TilePos::new(4, 4),
            ..session.unit("brood_stalker_a").unwrap().clone()
        };
        assert_eq!(
            session.cover_against(target.position, attacker.position),
            25
        );
        assert!(session.hit_chance(&attacker, &target) < attacker.accuracy as u8);
    }

    #[test]
    fn objective_requires_adjacency_and_one_action_point() {
        let (_, mut session) = session();
        let id = session.tactical.selected_unit.clone().unwrap();
        assert_eq!(
            session.validate(&Command::Interact {
                unit_id: id.clone()
            }),
            Err(RuleError::ObjectiveUnavailable)
        );
        session
            .tactical
            .units
            .iter_mut()
            .find(|unit| unit.id == id)
            .unwrap()
            .position = TilePos::new(9, 4);
        assert!(session.execute(Command::Interact { unit_id: id }).is_ok());
        assert_eq!(session.tactical.objective_state, ObjectiveState::Secured);
    }

    #[test]
    fn ending_phase_runs_deterministic_enemy_ai_and_refreshes_colonists() {
        let (config, mut session) = session();
        let before = session.tactical.event_log.len();
        session.end_player_phase(&config);
        assert_eq!(session.tactical.round, 2);
        assert!(session.tactical.event_log.len() > before);
        assert!(session
            .tactical
            .units
            .iter()
            .filter(|unit| unit.team == Team::Colony && !unit.incapacitated)
            .all(|unit| unit.action_points == config.max_action_points));
    }

    #[test]
    fn phase_zero_save_payload_migrates_to_battle_schema() {
        let data = crate::data::GameData::load().unwrap();
        let current = GameSession::new(&data.config, &data.mission, &data.roster).to_save("0.2.0");
        let mut legacy = serde_json::to_value(current).unwrap();
        let tactical = legacy["tactical"].as_object_mut().unwrap();
        for key in [
            "terrain_costs",
            "cover_edges",
            "objective_tile",
            "objective_state",
            "round_limit",
            "rng",
            "event_log",
        ] {
            tactical.remove(key);
        }
        for unit in tactical["units"].as_array_mut().unwrap() {
            let unit = unit.as_object_mut().unwrap();
            for key in [
                "armour",
                "accuracy",
                "weapon_range",
                "weapon_damage",
                "weapon_ap_cost",
                "incapacitated",
            ] {
                unit.remove(key);
            }
        }
        let migrated =
            crate::persistence::migrate_save_value(Some("0.1.0".to_owned()), legacy, &data.config)
                .unwrap();
        assert_eq!(migrated.version, "0.2.0");
        assert!(!migrated.tactical.units.is_empty());
    }

    #[test]
    fn victorious_mission_outcome_carries_debrief_consequences() {
        let data = crate::data::GameData::load().unwrap();
        let mut session = GameSession::new(&data.config, &data.mission, &data.roster);
        session.tactical.objective_state = ObjectiveState::Victory;
        session
            .tactical
            .units
            .iter_mut()
            .find(|unit| unit.id == "ilya_reed")
            .unwrap()
            .incapacitated = true;
        let outcome = session.mission_outcome(&data.mission).unwrap();
        assert_eq!(outcome.colonists_deployed, 4);
        assert_eq!(outcome.colonists_incapacitated, vec!["Ilya Reed"]);
        assert_eq!(outcome.materials_awarded, data.mission.materials_reward);
    }
}
