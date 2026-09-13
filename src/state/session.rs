//! Session construction, selection, and command validation.

use super::*;

impl GameSession {
    pub fn from_tactical(tactical: TacticalState) -> Self {
        Self { tactical }
    }

    pub fn to_save(&self, version: &str, campaign: &CampaignState) -> SaveData {
        SaveData {
            version: version.to_owned(),
            campaign: campaign.clone(),
            active_mission: None,
            tactical: Some(self.tactical.clone()),
        }
    }

    pub fn to_save_with_mission(
        &self,
        version: &str,
        campaign: &CampaignState,
        mission: &MissionDef,
    ) -> SaveData {
        let mut save = self.to_save(version, campaign);
        save.active_mission = Some(mission.clone());
        save
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

    pub fn validate_move(&self, unit_id: &str, to: TilePos) -> Result<CommandCost, RuleError> {
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

    pub fn validate_attack(
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

    pub fn validate_interact(&self, unit_id: &str) -> Result<CommandCost, RuleError> {
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

    pub fn validate_mutation(&self, unit_id: &str) -> Result<CommandCost, RuleError> {
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

    pub fn active_unit_for_phase(&self, id: &str) -> Result<&UnitState, RuleError> {
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
}
