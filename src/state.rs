//! Deterministic tactical command simulation and persistence model.

use crate::campaign::CampaignState;
use crate::data::{EdgeDirection, GameConfig, MissionDef, ObjectiveKind, Team, UnitDef};
use crate::tactical::{line_between, manhattan, path_cost, terrain_cost};
pub use crate::tactical::{
    BattleEvent, Command, CommandCost, DestructibleCover, ObjectiveState, ReinforcementWave,
    RuleError, StatusKind, TacticalPhase, TacticalState, UnitState,
};
use macroquad_toolkit::grid::{FlatGrid, FogState, TilePos};
use macroquad_toolkit::pathfinding::{find_path_with, Heuristic, Pos};
use macroquad_toolkit::rng::SeededRng;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveData {
    pub version: String,
    pub campaign: CampaignState,
    pub tactical: Option<TacticalState>,
}

impl SaveData {
    pub fn campaign_only(version: &str, campaign: &CampaignState) -> Self {
        Self {
            version: version.to_owned(),
            campaign: campaign.clone(),
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
        let reinforcement_waves = crate::reinforcements::create_waves(config, mission, &units);

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
                destructible_cover: mission
                    .blocked_tiles
                    .iter()
                    .map(|position| DestructibleCover {
                        position: tile(*position),
                        health: mission.cover_integrity,
                        max_health: mission.cover_integrity,
                    })
                    .collect(),
                units,
                selected_unit,
                selected_tile,
                objective_tile: tile(mission.objective_tile),
                objective_kind: mission.objective_kind,
                objective_state: ObjectiveState::Active,
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
            },
        }
    }

    pub fn from_tactical(tactical: TacticalState) -> Self {
        Self { tactical }
    }

    pub fn to_save(&self, version: &str, campaign: &CampaignState) -> SaveData {
        SaveData {
            version: version.to_owned(),
            campaign: campaign.clone(),
            tactical: Some(self.tactical.clone()),
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
            Command::ActivateMutation { unit_id } => self.validate_mutation(unit_id),
            Command::ActivateClassAction { unit_id, target_id } => {
                crate::class_actions::validate(self, unit_id, target_id.as_deref())
            }
            Command::UseEquipment {
                unit_id,
                equipment_id,
                target_id,
            } => crate::equipment_actions::validate(self, unit_id, equipment_id, target_id),
            Command::AttackCover {
                attacker_id,
                position,
            } => crate::cover_actions::validate(self, attacker_id, *position),
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
            Command::ActivateMutation { unit_id } => self.execute_mutation(&unit_id),
            Command::ActivateClassAction { unit_id, target_id } => {
                crate::class_actions::execute(self, &unit_id, target_id.as_deref())
            }
            Command::UseEquipment {
                unit_id,
                equipment_id,
                target_id,
            } => crate::equipment_actions::execute(self, &unit_id, &equipment_id, &target_id),
            Command::AttackCover {
                attacker_id,
                position,
            } => crate::cover_actions::execute(self, &attacker_id, position),
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

    pub fn activate_selected_mutation(&mut self) -> Result<Vec<BattleEvent>, RuleError> {
        let unit_id = self
            .tactical
            .selected_unit
            .clone()
            .ok_or(RuleError::UnknownUnit)?;
        self.execute(Command::ActivateMutation { unit_id })
    }

    pub fn can_activate_selected_mutation(&self) -> bool {
        self.tactical.selected_unit.as_ref().is_some_and(|unit_id| {
            self.validate(&Command::ActivateMutation {
                unit_id: unit_id.clone(),
            })
            .is_ok()
        })
    }

    pub fn end_player_phase(&mut self, config: &GameConfig) {
        if self.tactical.phase != TacticalPhase::Player || self.battle_is_over() {
            return;
        }
        self.advance_statuses(Team::Colony);
        self.tactical.phase = TacticalPhase::Enemy;
        self.push_event(BattleEvent::PhaseStarted {
            phase: TacticalPhase::Enemy,
            round: self.tactical.round,
        });
        self.refresh_team(Team::Hostile, config.max_action_points);
        crate::reinforcements::deploy(self, config.max_action_points);
        crate::tactical_ai::resolve_enemy_phase(self);
        self.advance_statuses(Team::Hostile);
        if self.battle_is_over() {
            return;
        }
        self.tactical.round += 1;
        if self.tactical.round > self.tactical.round_limit && !self.battle_is_over() {
            self.finish_battle(if self.tactical.objective_kind == ObjectiveKind::Holdout {
                ObjectiveState::Victory
            } else {
                ObjectiveState::Failed
            });
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
                .map(|unit| CharacterConsequence {
                    id: unit.id.clone(),
                    name: unit.name.clone(),
                })
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
        if cost == 0 || cost > unit.effective_move_range() || cost > unit.action_points {
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
        if !self.has_line_of_fire(attacker.position, target.position) {
            return Err(RuleError::NoLineOfFire);
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
        if !matches!(
            self.tactical.objective_kind,
            ObjectiveKind::SecureAndClear | ObjectiveKind::Extraction
        ) || self.tactical.objective_state != ObjectiveState::Active
            || manhattan(unit.position, self.tactical.objective_tile) > 1
        {
            return Err(RuleError::ObjectiveUnavailable);
        }
        if unit.action_points < 1 {
            return Err(RuleError::InsufficientActionPoints);
        }
        Ok(CommandCost { action_points: 1 })
    }

    fn validate_mutation(&self, unit_id: &str) -> Result<CommandCost, RuleError> {
        let unit = self.active_unit_for_phase(unit_id)?;
        if unit.mutation_gift_used
            || mutation_gift_name(&unit.mutation).is_none()
            || (unit.mutation == "Regenerative Tissue" && unit.health == unit.max_health)
        {
            return Err(RuleError::MutationUnavailable);
        }
        if unit.action_points < 1 {
            return Err(RuleError::InsufficientActionPoints);
        }
        Ok(CommandCost { action_points: 1 })
    }

    pub(crate) fn active_unit_for_phase(&self, id: &str) -> Result<&UnitState, RuleError> {
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
            let damage = (attacker.effective_weapon_damage() + i32::from(critical) * 2
                - target.effective_armour())
            .max(1);
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
            if !target.incapacitated {
                let status = match attacker.role.as_str() {
                    "Artillery" | "Battlefield Controller" => Some(StatusKind::Hindered),
                    "Combat Drone" => Some(StatusKind::Disrupted),
                    _ => None,
                };
                if let Some(status) = status {
                    crate::class_actions::apply_status(self, target_id, status, 1, &mut events);
                }
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
        let mut events = vec![match self.tactical.objective_kind {
            ObjectiveKind::Extraction => BattleEvent::ExtractionCompleted {
                unit_id: unit_id.to_owned(),
            },
            _ => BattleEvent::ObjectiveSecured {
                unit_id: unit_id.to_owned(),
            },
        }];
        self.check_outcome(&mut events);
        events
    }

    fn execute_mutation(&mut self, unit_id: &str) -> Vec<BattleEvent> {
        let unit = self
            .tactical
            .units
            .iter_mut()
            .find(|unit| unit.id == unit_id)
            .unwrap();
        unit.action_points -= 1;
        unit.mutation_gift_used = true;
        let gift = mutation_gift_name(&unit.mutation).unwrap().to_owned();
        let mut events = vec![BattleEvent::MutationActivated {
            unit_id: unit_id.to_owned(),
            gift,
        }];
        match unit.mutation.as_str() {
            "Neural Bloom" => unit.temporary_accuracy = 20,
            "Chitinous Growth" => unit.temporary_armour = 3,
            "Regenerative Tissue" => {
                let before = unit.health;
                unit.health = (unit.health + 3).min(unit.max_health);
                events.push(BattleEvent::UnitHealed {
                    unit_id: unit_id.to_owned(),
                    amount: unit.health - before,
                    remaining: unit.health,
                });
            }
            "Elastic Musculature" => {
                unit.temporary_move_range = 3;
                unit.action_points = unit.action_points.saturating_add(3);
            }
            "Symbiotic Organism" => unit.temporary_weapon_damage = 2,
            _ => unreachable!("validated mutation has a tactical gift"),
        }
        events
    }

    fn hit_chance(&self, attacker: &UnitState, target: &UnitState) -> u8 {
        let distance = manhattan(attacker.position, target.position);
        let range_penalty = (distance - i32::from(attacker.weapon_range) / 2).max(0) * 5;
        let cover = self.cover_against(target.position, attacker.position);
        (attacker.effective_accuracy() - range_penalty - cover).clamp(5, 95) as u8
    }

    pub(crate) fn has_line_of_fire(&self, from: TilePos, to: TilePos) -> bool {
        line_between(from, to)
            .into_iter()
            .all(|position| !self.tactical.blocked.contains(&position))
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

    pub(crate) fn check_outcome(&mut self, events: &mut Vec<BattleEvent>) {
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
        } else {
            match self.tactical.objective_kind {
                ObjectiveKind::SecureAndClear
                    if !hostiles_alive
                        && self.tactical.objective_state == ObjectiveState::Secured =>
                {
                    Some(ObjectiveState::Victory)
                }
                ObjectiveKind::EliminateAll if !hostiles_alive => Some(ObjectiveState::Victory),
                ObjectiveKind::Holdout
                    if !hostiles_alive && self.tactical.reinforcement_waves.is_empty() =>
                {
                    Some(ObjectiveState::Victory)
                }
                ObjectiveKind::Extraction
                    if self.tactical.objective_state == ObjectiveState::Secured =>
                {
                    Some(ObjectiveState::Victory)
                }
                _ => None,
            }
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
                unit.mutation_gift_used = false;
                unit.class_action_used = false;
                unit.temporary_armour = 0;
                unit.temporary_accuracy = 0;
                unit.temporary_move_range = 0;
                unit.temporary_weapon_damage = 0;
                if unit.round_regeneration > 0 {
                    unit.health = (unit.health + unit.round_regeneration).min(unit.max_health);
                }
                if unit.has_status(StatusKind::Regenerating) {
                    unit.health = (unit.health + 1).min(unit.max_health);
                }
            }
        }
    }

    fn advance_statuses(&mut self, team: Team) {
        for unit in self
            .tactical
            .units
            .iter_mut()
            .filter(|unit| unit.team == team)
        {
            for status in &mut unit.statuses {
                status.remaining_phases = status.remaining_phases.saturating_sub(1);
            }
            unit.statuses.retain(|status| status.remaining_phases > 0);
        }
    }

    fn push_event(&mut self, event: BattleEvent) {
        self.tactical.event_log.push(event);
    }
}

fn tile(position: [i32; 2]) -> TilePos {
    TilePos::new(position[0], position[1])
}

fn mutation_gift_name(mutation: &str) -> Option<&'static str> {
    match mutation {
        "Neural Bloom" => Some("NEURAL FOCUS · +20 accuracy this round"),
        "Chitinous Growth" => Some("HARDEN CARAPACE · +3 armour this round"),
        "Regenerative Tissue" => Some("ACCELERATE TISSUE · restore 3 vitality"),
        "Elastic Musculature" => Some("COIL MUSCLE · +2 AP and +3 movement this round"),
        "Symbiotic Organism" => Some("FEEDING FRENZY · +2 weapon damage this round"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn session() -> (GameConfig, GameSession) {
        let data = crate::data::GameData::load().unwrap();
        let session = GameSession::new(&data.config, &data.mission, &data.roster);
        (data.config, session)
    }

    fn unit_mut<'a>(session: &'a mut GameSession, id: &str) -> &'a mut UnitState {
        session
            .tactical
            .units
            .iter_mut()
            .find(|unit| unit.id == id)
            .unwrap()
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
    fn solid_obstacles_block_line_of_fire() {
        let (_, mut session) = session();
        unit_mut(&mut session, "kira_voss").position = TilePos::new(8, 2);
        session.tactical.blocked.insert(TilePos::new(9, 2));
        assert_eq!(
            session.validate(&Command::Attack {
                attacker_id: "kira_voss".into(),
                target_id: "brood_stalker_a".into(),
            }),
            Err(RuleError::NoLineOfFire)
        );
    }

    #[test]
    fn each_colonist_mutation_gift_changes_its_tactical_options() {
        let (_, base) = session();

        let mut neural = base.clone();
        neural.tactical.selected_unit = Some("kira_voss".into());
        neural.activate_selected_mutation().unwrap();
        assert_eq!(neural.unit("kira_voss").unwrap().temporary_accuracy, 20);

        let mut chitin = base.clone();
        chitin.tactical.selected_unit = Some("mara_venn".into());
        chitin.activate_selected_mutation().unwrap();
        assert_eq!(chitin.unit("mara_venn").unwrap().temporary_armour, 3);

        let mut regen = base.clone();
        regen.tactical.selected_unit = Some("ilya_reed".into());
        unit_mut(&mut regen, "ilya_reed").health = 4;
        regen.activate_selected_mutation().unwrap();
        assert_eq!(regen.unit("ilya_reed").unwrap().health, 7);

        let mut healthy_regen = base.clone();
        healthy_regen.tactical.selected_unit = Some("ilya_reed".into());
        assert!(!healthy_regen.can_activate_selected_mutation());

        let mut elastic = base;
        elastic.tactical.selected_unit = Some("sol_cairn".into());
        let before = elastic.unit("sol_cairn").unwrap().action_points;
        elastic.activate_selected_mutation().unwrap();
        let sol = elastic.unit("sol_cairn").unwrap();
        assert_eq!(sol.action_points, before + 2);
        assert_eq!(sol.temporary_move_range, 3);
        assert!(!elastic.can_activate_selected_mutation());
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
    fn typed_objectives_resolve_with_distinct_victory_rules() {
        let (_, base) = session();

        let mut eliminate = base.clone();
        eliminate.tactical.objective_kind = ObjectiveKind::EliminateAll;
        for hostile in eliminate
            .tactical
            .units
            .iter_mut()
            .filter(|unit| unit.team == Team::Hostile)
        {
            hostile.incapacitated = true;
        }
        let mut events = Vec::new();
        eliminate.check_outcome(&mut events);
        assert_eq!(eliminate.tactical.objective_state, ObjectiveState::Victory);

        let mut secure = base.clone();
        for hostile in secure
            .tactical
            .units
            .iter_mut()
            .filter(|unit| unit.team == Team::Hostile)
        {
            hostile.incapacitated = true;
        }
        secure.check_outcome(&mut Vec::new());
        assert_eq!(secure.tactical.objective_state, ObjectiveState::Active);

        let mut extraction = base.clone();
        extraction.tactical.objective_kind = ObjectiveKind::Extraction;
        let colonist_id = extraction.tactical.selected_unit.clone().unwrap();
        extraction
            .tactical
            .units
            .iter_mut()
            .find(|unit| unit.id == colonist_id)
            .unwrap()
            .position = TilePos::new(
            extraction.tactical.objective_tile.x - 1,
            extraction.tactical.objective_tile.y,
        );
        let events = extraction
            .execute(Command::Interact {
                unit_id: colonist_id,
            })
            .unwrap();
        assert_eq!(extraction.tactical.objective_state, ObjectiveState::Victory);
        assert!(extraction
            .tactical
            .units
            .iter()
            .any(|unit| unit.team == Team::Hostile && !unit.incapacitated));
        assert!(events
            .iter()
            .any(|event| matches!(event, BattleEvent::ExtractionCompleted { .. })));

        let (config, mut holdout) = session();
        holdout.tactical.objective_kind = ObjectiveKind::Holdout;
        holdout.tactical.round_limit = 1;
        for colonist in holdout
            .tactical
            .units
            .iter_mut()
            .filter(|unit| unit.team == Team::Colony)
        {
            colonist.health = 100;
            colonist.max_health = 100;
        }
        holdout.end_player_phase(&config);
        assert_eq!(holdout.tactical.objective_state, ObjectiveState::Victory);
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
    fn hostile_activations_spend_their_available_attack_economy() {
        let (config, mut session) = session();
        let kira_position = session.unit("kira_voss").unwrap().position;
        let stalker = session
            .tactical
            .units
            .iter_mut()
            .find(|unit| unit.id == "brood_stalker_a")
            .unwrap();
        stalker.position = TilePos::new(kira_position.x + 1, kira_position.y);
        session.end_player_phase(&config);
        let attacks = session
            .tactical
            .event_log
            .iter()
            .filter(|event| {
                matches!(
                    event,
                    BattleEvent::AttackRolled { attacker_id, .. }
                        if attacker_id == "brood_stalker_a"
                )
            })
            .count();
        assert_eq!(attacks, 2);
    }

    #[test]
    fn specialist_enemy_hits_apply_role_statuses() {
        let (_, mut session) = session();
        let kira_position = session.unit("kira_voss").unwrap().position;
        let sporecaster = session
            .tactical
            .units
            .iter_mut()
            .find(|unit| unit.id == "brood_stalker_b")
            .unwrap();
        sporecaster.position = TilePos::new(kira_position.x + 1, kira_position.y);
        sporecaster.accuracy = 100;
        session.tactical.phase = TacticalPhase::Enemy;
        let seed = (0..1000)
            .find(|seed| {
                let mut rng = SeededRng::new(*seed);
                rng.range_i32(1, 101) <= 95
            })
            .unwrap();
        session.tactical.rng = SeededRng::new(seed);
        session
            .execute(Command::Attack {
                attacker_id: "brood_stalker_b".into(),
                target_id: "kira_voss".into(),
            })
            .unwrap();
        assert!(session
            .unit("kira_voss")
            .unwrap()
            .has_status(StatusKind::Hindered));
    }

    #[test]
    fn holdouts_wait_for_serialized_reinforcement_waves() {
        let data = crate::data::GameData::load().unwrap();
        let campaign = CampaignState::new(&data);
        let mut mission = data.mission.clone();
        mission.objective_kind = ObjectiveKind::Holdout;
        mission.hostile_faction = "ascendants".to_owned();
        mission.round_limit = 6;
        let roster = campaign.deployment_roster(&data, &mission);
        let mut session = GameSession::new(&data.config, &mission, &roster);
        assert_eq!(session.tactical.reinforcement_waves.len(), 2);
        for unit in &mut session.tactical.units {
            if unit.team == Team::Hostile {
                unit.incapacitated = true;
                unit.health = 0;
            } else {
                unit.health = 100;
                unit.max_health = 100;
            }
        }
        session.check_outcome(&mut Vec::new());
        assert_eq!(session.tactical.objective_state, ObjectiveState::Active);
        for _ in 0..3 {
            session.end_player_phase(&data.config);
        }
        assert_eq!(session.tactical.reinforcement_waves.len(), 1);
        assert_eq!(
            session
                .tactical
                .units
                .iter()
                .filter(|unit| unit.team == Team::Hostile && !unit.incapacitated)
                .count(),
            2
        );
        assert!(session.tactical.event_log.iter().any(|event| matches!(
            event,
            BattleEvent::ReinforcementsArrived { round: 3, count: 2 }
        )));
    }

    #[test]
    fn phase_zero_save_payload_migrates_to_battle_schema() {
        let data = crate::data::GameData::load().unwrap();
        let campaign = CampaignState::new(&data);
        let current =
            GameSession::new(&data.config, &data.mission, &data.roster).to_save("0.2.0", &campaign);
        let mut legacy = serde_json::to_value(current).unwrap();
        legacy.as_object_mut().unwrap().remove("campaign");
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
                "round_regeneration",
            ] {
                unit.remove(key);
            }
        }
        let migrated =
            crate::persistence::migrate_save_value(Some("0.1.0".to_owned()), legacy, &data)
                .unwrap();
        assert_eq!(migrated.version, "1.4.0");
        assert!(!migrated.tactical.unwrap().units.is_empty());
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
        assert_eq!(outcome.colonists_incapacitated[0].id, "ilya_reed");
        assert_eq!(outcome.materials_awarded, data.mission.materials_reward);
    }
}
