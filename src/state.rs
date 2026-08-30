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
#[cfg(test)]
use macroquad_toolkit::rng::SeededRng;
use serde::{Deserialize, Serialize};

mod creation;
mod pathfinding;

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

impl GameSession {
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

    pub fn deployed_colonist_ids(&self) -> Vec<String> {
        self.tactical
            .units
            .iter()
            .filter(|unit| unit.team == Team::Colony)
            .map(|unit| unit.id.clone())
            .collect()
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
            Command::ActivateClassAction {
                unit_id,
                target_id,
                target_tile,
            } => crate::class_actions::validate(self, unit_id, target_id.as_deref(), *target_tile),
            Command::ActivateSkill {
                unit_id,
                skill_id,
                target_id,
                target_tile,
            } => {
                crate::skills::validate(self, unit_id, skill_id, target_id.as_deref(), *target_tile)
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
            Command::SetOverwatch { unit_id } => crate::overwatch::validate(self, unit_id),
            Command::AttackObjective { attacker_id } => {
                crate::defense_objective::validate(self, attacker_id)
            }
            Command::ActivateEnemyAbility { unit_id, target_id } => {
                crate::enemy_abilities::validate(self, unit_id, target_id.as_deref())
            }
        }
    }

    pub fn execute(&mut self, command: Command) -> Result<Vec<BattleEvent>, RuleError> {
        self.validate(&command)?;
        let events = match command {
            Command::Move { unit_id, to } => {
                let mut events = self.execute_move(&unit_id, to);
                events.extend(crate::hazards::resolve_after_move(self, &unit_id));
                events.extend(crate::overwatch::resolve_after_hostile_move(self, &unit_id));
                events
            }
            Command::Attack {
                attacker_id,
                target_id,
            } => self.execute_attack(&attacker_id, &target_id),
            Command::Interact { unit_id } => self.execute_interact(&unit_id),
            Command::ActivateMutation { unit_id } => self.execute_mutation(&unit_id),
            Command::ActivateClassAction {
                unit_id,
                target_id,
                target_tile,
            } => crate::class_actions::execute(self, &unit_id, target_id.as_deref(), target_tile),
            Command::ActivateSkill {
                unit_id,
                skill_id,
                target_id,
                target_tile,
            } => {
                crate::skills::execute(self, &unit_id, &skill_id, target_id.as_deref(), target_tile)
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
            Command::SetOverwatch { unit_id } => crate::overwatch::execute(self, &unit_id),
            Command::AttackObjective { attacker_id } => {
                crate::defense_objective::execute(self, &attacker_id)
            }
            Command::ActivateEnemyAbility { unit_id, target_id } => {
                crate::enemy_abilities::execute(self, &unit_id, target_id.as_deref())
            }
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
        self.advance_obscuring_fields();
        if self.battle_is_over() {
            return;
        }
        self.tactical.round += 1;
        if self.tactical.round > self.tactical.round_limit && !self.battle_is_over() {
            let timed_success = self.tactical.objective_kind == ObjectiveKind::Holdout
                || (self.tactical.objective_kind == ObjectiveKind::SignalTrace
                    && self.tactical.objective_state == ObjectiveState::Secured)
                || crate::defense_objective::survives_deadline(self);
            self.finish_battle(if timed_success {
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
            biomass_awarded: if self.tactical.objective_state == ObjectiveState::Victory {
                mission.biomass_reward
            } else {
                0
            },
            power_awarded: if self.tactical.objective_state == ObjectiveState::Victory {
                mission.power_reward
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
            ObjectiveKind::SecureAndClear | ObjectiveKind::Extraction | ObjectiveKind::SignalTrace
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

    pub(crate) fn execute_move(&mut self, unit_id: &str, to: TilePos) -> Vec<BattleEvent> {
        let from = self.unit(unit_id).unwrap().position;
        let path = self.path_for(from, to, Some(unit_id)).unwrap();
        let cost = path_cost(&path, &self.tactical.terrain_costs);
        let unit = self
            .tactical
            .units
            .iter_mut()
            .find(|unit| unit.id == unit_id)
            .unwrap();
        unit.facing = UnitFacing::toward(from, to);
        unit.position = to;
        unit.action_points -= cost;
        unit.presentation_state = UnitAnimationState::Move;
        unit.presentation_seconds = 0.38;
        if self.tactical.selected_unit.as_deref() == Some(unit_id) {
            self.tactical.selected_tile = to;
        }
        vec![BattleEvent::UnitMoved {
            unit_id: unit_id.to_owned(),
            path,
            cost,
        }]
    }

    pub(crate) fn execute_attack(
        &mut self,
        attacker_id: &str,
        target_id: &str,
    ) -> Vec<BattleEvent> {
        let attacker = self.unit(attacker_id).unwrap().clone();
        let target = self.unit(target_id).unwrap().clone();
        let marked = attacker.team == Team::Colony && target.has_status(StatusKind::Marked);
        let hit_chance = (i32::from(self.hit_chance(&attacker, &target)) + i32::from(marked) * 20)
            .clamp(5, 95) as u8;
        let roll = self.tactical.rng.range_i32(1, 101) as u8;
        let attacking_unit = self
            .tactical
            .units
            .iter_mut()
            .find(|unit| unit.id == attacker_id)
            .unwrap();
        attacking_unit.action_points -= attacker.weapon_ap_cost;
        let ignores_armour = attacking_unit.next_attack_ignores_armour;
        attacking_unit.next_attack_ignores_armour = false;
        attacking_unit.facing = UnitFacing::toward(attacker.position, target.position);
        attacking_unit.presentation_state = UnitAnimationState::AttackRelease;
        attacking_unit.presentation_seconds = 0.52;
        let mut events = vec![BattleEvent::AttackRolled {
            attacker_id: attacker_id.to_owned(),
            target_id: target_id.to_owned(),
            roll,
            hit_chance,
        }];
        if roll <= hit_chance {
            let critical = roll <= 10;
            let armour = if ignores_armour {
                target.effective_armour() / 2
            } else {
                target.effective_armour()
            };
            let damage =
                (attacker.effective_weapon_damage() + i32::from(critical) * 2 - armour).max(1);
            let target = self
                .tactical
                .units
                .iter_mut()
                .find(|unit| unit.id == target_id)
                .unwrap();
            target.facing = UnitFacing::toward(target.position, attacker.position);
            target.presentation_state = UnitAnimationState::Hit;
            target.presentation_seconds = 0.42;
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
        if marked {
            if let Some(target) = self
                .tactical
                .units
                .iter_mut()
                .find(|unit| unit.id == target_id)
            {
                target
                    .statuses
                    .retain(|status| status.kind != StatusKind::Marked);
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

    pub(crate) fn hit_chance(&self, attacker: &UnitState, target: &UnitState) -> u8 {
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

    pub(crate) fn cover_penalty(&self, attacker: &UnitState, target: &UnitState) -> i32 {
        self.cover_against(target.position, attacker.position)
    }

    fn cover_against(&self, target: TilePos, attacker: TilePos) -> i32 {
        crate::cover_rules::penalty(&self.tactical.cover_edges, target, attacker)
            + self.tactical.obscuring_penalty(target)
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
        let outcome = if crate::defense_objective::is_destroyed(self) || !colonists_alive {
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
                ObjectiveKind::SignalTrace
                    if self.tactical.objective_state == ObjectiveState::Secured
                        && !hostiles_alive
                        && self.tactical.reinforcement_waves.is_empty() =>
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
mod tests;
