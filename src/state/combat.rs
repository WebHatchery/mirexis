//! Tactical command execution and battle resolution.

use super::*;

impl GameSession {
    pub fn execute_move(&mut self, unit_id: &str, to: TilePos) -> Vec<BattleEvent> {
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

    pub fn execute_attack(&mut self, attacker_id: &str, target_id: &str) -> Vec<BattleEvent> {
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

    pub fn execute_interact(&mut self, unit_id: &str) -> Vec<BattleEvent> {
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

    pub fn execute_mutation(&mut self, unit_id: &str) -> Vec<BattleEvent> {
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

    pub fn hit_chance(&self, attacker: &UnitState, target: &UnitState) -> u8 {
        let distance = manhattan(attacker.position, target.position);
        let range_penalty = (distance - i32::from(attacker.weapon_range) / 2).max(0) * 5;
        let cover = self.cover_against(target.position, attacker.position);
        (attacker.effective_accuracy() - range_penalty - cover).clamp(5, 95) as u8
    }

    pub fn has_line_of_fire(&self, from: TilePos, to: TilePos) -> bool {
        line_between(from, to)
            .into_iter()
            .all(|position| !self.tactical.blocked.contains(&position))
    }

    pub fn cover_penalty(&self, attacker: &UnitState, target: &UnitState) -> i32 {
        self.cover_against(target.position, attacker.position)
    }

    pub fn cover_against(&self, target: TilePos, attacker: TilePos) -> i32 {
        crate::cover_rules::penalty(&self.tactical.cover_edges, target, attacker)
            + self.tactical.obscuring_penalty(target)
    }

    pub fn check_outcome(&mut self, events: &mut Vec<BattleEvent>) {
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

    pub fn finish_battle(&mut self, outcome: ObjectiveState) {
        self.tactical.objective_state = outcome;
        self.push_event(BattleEvent::BattleEnded { outcome });
    }

    pub fn advance_statuses(&mut self, team: Team) {
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

    pub fn push_event(&mut self, event: BattleEvent) {
        self.tactical.event_log.push(event);
    }
}
