//! Campaign state mutations and progression actions.

use super::*;

impl CampaignState {
    pub fn new(data: &GameData) -> Self {
        let mut roster = data
            .characters
            .iter()
            .filter(|character| character.recruitment_protocol.is_empty())
            .map(CharacterRecord::from_def)
            .collect::<Vec<_>>();
        for (index, character) in roster.iter_mut().enumerate() {
            character.deployment_selected = index < SQUAD_LIMIT;
        }
        Self {
            selected_character_id: roster
                .first()
                .map(|character| character.id.clone())
                .unwrap_or_default(),
            roster,
            colony: ColonyState::new_with_resources(&data.config.starting_resources),
            strategy: StrategyState::new(data),
            operations_completed: 0,
            relationships: Vec::new(),
            lost_objectives: Vec::new(),
            first_hour: crate::first_hour::FirstHourProgress::default(),
            colony_story: crate::colony_story::ColonyStoryState::default(),
            outsider_arc_stage: 0,
            outsider_disagreements: 0,
            outsider_final_choice: String::new(),
            outsider_arc_states: BTreeMap::new(),
            commons_meals_hosted: 0,
            commons_meal_operation: None,
            relay_scans_used: 0,
            relay_scan_operation: None,
            identity_stewardship_completed: 0,
            identity_stewardship_operation: None,
            identity_preparations_completed: 0,
            identity_preparation_operation: None,
            salvage_cache_count: 0,
            salvage_yard_operation: None,
            research_insight: 0,
            salvage_prototypes: 0,
        }
    }

    pub fn ensure_roster_characters(&mut self, data: &GameData) -> usize {
        let mut added = 0;
        for definition in &data.characters {
            if !definition.recruitment_protocol.is_empty() {
                continue;
            }
            if self
                .roster
                .iter()
                .any(|character| character.id == definition.id)
            {
                continue;
            }
            let mut character = CharacterRecord::from_def(definition);
            character.deployment_selected = false;
            self.roster.push(character);
            added += 1;
        }
        added
    }

    pub fn deployment_roster(
        &self,
        data: &GameData,
        mission: &crate::data::MissionDef,
    ) -> Vec<UnitDef> {
        let mut deployment = data
            .roster
            .iter()
            .chain(data.recruitable_roster.iter())
            .filter_map(|base| {
                if base.team != Team::Colony {
                    return None;
                }
                let character = self.roster.iter().find(|record| record.id == base.id)?;
                (character.availability == Availability::Ready && character.deployment_selected)
                    .then(|| {
                        derive_unit_with_evolution_options(
                            base,
                            character,
                            data,
                            self.colony.has_active_upgrade(
                                BuildingKind::GeneLab,
                                STABILISATION_WING_UPGRADE,
                            ),
                            self.colony
                                .has_active_upgrade(BuildingKind::Infirmary, TRAUMA_WARD_UPGRADE),
                        )
                    })
            })
            .take(SQUAD_LIMIT)
            .collect::<Vec<_>>();
        self.first_hour
            .apply_second_operation_bonus(self.operations_completed, &mut deployment);
        crate::relationships::apply_deployment_bonuses(&mut deployment, &self.relationships);
        deployment.extend(
            data.roster
                .iter()
                .filter(|base| {
                    if base.team != Team::Hostile {
                        return false;
                    }
                    if mission.hostile_unit_ids.is_empty() {
                        base.faction.as_deref() == Some(&mission.hostile_faction)
                    } else {
                        mission.hostile_unit_ids.contains(&base.id)
                    }
                })
                .cloned(),
        );
        deployment::spread_hostile_deployment(
            &mut deployment,
            mission,
            data.config.world_width as i32,
            data.config.world_height as i32,
        );
        deployment
    }

    pub fn derived_character_unit(&self, character_id: &str, data: &GameData) -> Option<UnitDef> {
        let character = self
            .roster
            .iter()
            .find(|record| record.id == character_id)?;
        let base = data
            .roster
            .iter()
            .chain(data.recruitable_roster.iter())
            .find(|unit| unit.team == Team::Colony && unit.id == character_id)?;
        Some(derive_unit_with_evolution_options(
            base,
            character,
            data,
            self.colony
                .has_active_upgrade(BuildingKind::GeneLab, STABILISATION_WING_UPGRADE),
            self.colony
                .has_active_upgrade(BuildingKind::Infirmary, TRAUMA_WARD_UPGRADE),
        ))
    }

    pub fn selected_squad_count(&self) -> usize {
        self.roster
            .iter()
            .filter(|character| {
                character.availability == Availability::Ready && character.deployment_selected
            })
            .count()
            .min(SQUAD_LIMIT)
    }

    pub fn deployment_food_cost(&self, data: &GameData) -> i32 {
        let base = self
            .roster
            .iter()
            .filter(|character| {
                character.availability == Availability::Ready && character.deployment_selected
            })
            .take(SQUAD_LIMIT)
            .map(|character| {
                1 + derived_mutation_traits_with_options(
                    character,
                    data,
                    self.colony
                        .has_active_upgrade(BuildingKind::GeneLab, STABILISATION_WING_UPGRADE),
                )
                .get("food_upkeep")
                .copied()
                .unwrap_or(0)
                .max(0)
            })
            .sum::<i32>();
        if base == 0 {
            return 0;
        }
        let discount = data
            .campaign
            .mirexis_paths
            .iter()
            .find(|path| path.id == self.strategy.mirexis_path_id)
            .map_or(0, |path| path.deployment_food_discount);
        (base - discount).max(1)
    }

    pub fn prepare_deployment(&mut self, data: &GameData) -> Result<i32, String> {
        let cost = self.deployment_food_cost(data);
        if cost == 0 {
            return Err("At least one ready colonist must deploy".to_owned());
        }
        if self.colony.resources.food < cost {
            return Err(format!("Deployment requires {} food", cost));
        }
        self.colony.resources.food -= cost;
        Ok(cost)
    }

    pub fn selected_character(&self) -> Option<&CharacterRecord> {
        self.roster
            .iter()
            .find(|character| character.id == self.selected_character_id)
            .or_else(|| self.roster.first())
    }

    pub fn select_character(&mut self, character_id: &str) -> Result<(), String> {
        if !self
            .roster
            .iter()
            .any(|character| character.id == character_id)
        {
            return Err(format!("Unknown colonist: {}", character_id));
        }
        self.selected_character_id = character_id.to_owned();
        Ok(())
    }

    pub fn toggle_deployment(&mut self, character_id: &str) -> Result<bool, String> {
        let selected_count = self.selected_squad_count();
        let character = self
            .roster
            .iter_mut()
            .find(|character| character.id == character_id)
            .ok_or_else(|| format!("Unknown colonist: {}", character_id))?;
        if character.availability != Availability::Ready {
            return Err(format!("{} is still recovering", character.name));
        }
        if character.deployment_selected {
            if selected_count <= 1 {
                return Err("At least one colonist must deploy".to_owned());
            }
            character.deployment_selected = false;
        } else {
            if selected_count >= SQUAD_LIMIT {
                return Err(format!("Squad limit is {} colonists", SQUAD_LIMIT));
            }
            character.deployment_selected = true;
        }
        Ok(character.deployment_selected)
    }

    pub fn can_toggle_deployment(&self, character_id: &str) -> bool {
        let Some(character) = self
            .roster
            .iter()
            .find(|character| character.id == character_id)
        else {
            return false;
        };
        if character.availability != Availability::Ready {
            return false;
        }
        if character.deployment_selected {
            self.selected_squad_count() > 1
        } else {
            self.selected_squad_count() < SQUAD_LIMIT
        }
    }

    pub fn apply_mission_outcome(
        &mut self,
        outcome: &MissionOutcome,
        mission: &MissionInstance,
        data: &GameData,
    ) -> Vec<String> {
        let deployed_ids = self
            .roster
            .iter()
            .filter(|character| {
                character.availability == Availability::Ready && character.deployment_selected
            })
            .take(SQUAD_LIMIT)
            .map(|character| character.id.clone())
            .collect::<Vec<_>>();
        self.operations_completed += 1;
        self.colony.advance_operation();
        if outcome.result == ObjectiveState::Failed {
            self.lost_objectives.push(LostObjectiveRecord {
                id: format!("failed_operation_{}", self.operations_completed),
                mission_name: mission.name.clone(),
                objective: mission.objective.clone(),
                operation: self.operations_completed,
            });
        }
        self.colony.resources.materials += outcome.materials_awarded;
        self.colony.resources.biomass += outcome.biomass_awarded;
        self.colony.resources.power += outcome.power_awarded;
        if outcome.result == ObjectiveState::Victory {
            self.salvage_cache_count = self.salvage_cache_count.saturating_add(1);
        }
        if mission.map_recipe == "colony_defense" && outcome.result == ObjectiveState::Failed {
            self.colony.damage_for_failed_defense(mission.seed);
        }
        let hot_core_active = self.colony.buildings.iter().any(|building| {
            building.kind == BuildingKind::PowerPlant
                && !building.damaged
                && self.colony.has_upgrade(&building.id, HOT_CORE_UPGRADE)
        });
        if hot_core_active {
            if let Some(faction) = self
                .strategy
                .factions
                .iter_mut()
                .max_by_key(|faction| (faction.attention, faction.id.clone()))
            {
                faction.attention = (faction.attention + 1).min(100);
            }
        }
        let counterintelligence_active = self.colony.has_active_upgrade(
            BuildingKind::CommandCentre,
            COUNTERINTELLIGENCE_CELL_UPGRADE,
        );
        let mission_offer_limit = self.mission_offer_limit();
        self.strategy.resolve_mission_with_options(
            outcome,
            mission,
            data,
            if counterintelligence_active { 2 } else { 0 },
            mission_offer_limit,
        );
        self.strategy.refresh_isolation_completion(&mut self.colony);
        if outcome.result == ObjectiveState::Victory {
            self.strengthen_shared_victory(&deployed_ids);
        }
        let xp = operation_experience(outcome.result);
        for character_id in &deployed_ids {
            if let Some(character) = self
                .roster
                .iter_mut()
                .find(|character| character.id == *character_id)
            {
                character.experience += xp;
                character.level = 1 + (character.experience / 100).min(9) as u8;
            }
        }
        let learned_techniques =
            crate::skill_training::learn_after_operation(self, &deployed_ids, data);
        self.apply_injury_consequences(outcome, data);
        self.refresh_contact_completion(data);
        self.refresh_adaptation_completion(data);
        self.refresh_escalation_completion(data);
        learned_techniques
    }

    pub fn resolve_first_character_event(&mut self, data: &GameData) -> Result<String, String> {
        let recipient_id = self
            .strategy
            .available_event()
            .and_then(|event| {
                (!event.legacy_character_id.is_empty())
                    .then_some(event.legacy_character_id.as_str())
            })
            .or_else(|| {
                self.strategy.available_event().and_then(|event| {
                    data.campaign
                        .events
                        .iter()
                        .find(|definition| definition.id == event.id)
                        .map(|definition| definition.legacy_character_id.as_str())
                })
            })
            .ok_or_else(|| "No event legacy recipient".to_owned())?
            .to_owned();
        self.resolve_first_character_event_for(&recipient_id, data)
    }

    pub fn resolve_first_character_event_for(
        &mut self,
        recipient_id: &str,
        data: &GameData,
    ) -> Result<String, String> {
        let event = self
            .strategy
            .available_event()
            .cloned()
            .ok_or_else(|| "No unresolved character events".to_owned())?;
        if !event
            .participants
            .iter()
            .any(|participant| participant == recipient_id)
        {
            return Err("Choose one of the event participants".to_owned());
        }
        let data_legacy = data
            .campaign
            .events
            .iter()
            .find(|definition| definition.id == event.id)
            .map(|definition| {
                (
                    definition.legacy_name.clone(),
                    definition.legacy_stat.clone(),
                    definition.legacy_amount,
                )
            });
        let (legacy_name, legacy_stat, legacy_amount) = if event.legacy_amount != 0 {
            (
                event.legacy_name.clone(),
                event.legacy_stat.clone(),
                event.legacy_amount,
            )
        } else {
            data_legacy.unwrap_or_default()
        };
        if legacy_amount != 0
            && !self
                .roster
                .iter()
                .any(|character| character.id == recipient_id)
        {
            return Err(format!("Unknown legacy recipient: {}", recipient_id));
        }
        let title = self.strategy.resolve_first_event(&mut self.colony)?;
        self.strengthen_event_participants(&event.participants);
        if legacy_amount != 0 {
            let character = self
                .roster
                .iter_mut()
                .find(|character| character.id == recipient_id)
                .expect("legacy recipient was validated before event mutation");
            if !character
                .event_legacies
                .iter()
                .any(|legacy| legacy.id == event.id)
            {
                character.event_legacies.push(CharacterLegacy {
                    id: event.id,
                    name: legacy_name,
                    stat: legacy_stat,
                    amount: legacy_amount,
                });
            }
        }
        self.refresh_contact_completion(data);
        Ok(title)
    }

    pub fn craft_equipment(
        &mut self,
        character_id: &str,
        equipment_id: &str,
        data: &GameData,
    ) -> Result<u32, String> {
        if !self.colony.has_facility(BuildingKind::Workshop) {
            return Err("An operational workshop is required".to_owned());
        }
        let equipment = data
            .equipment
            .iter()
            .find(|equipment| equipment.id == equipment_id)
            .ok_or_else(|| format!("Unknown equipment: {}", equipment_id))?;
        if !self.equipment_is_unlocked(equipment) {
            return Err(format!(
                "{} requires a completed Contact trace",
                equipment.name
            ));
        }
        let base_cost = equipment_cost(&equipment.slot);
        let prototype_available =
            self.salvage_prototypes > 0 && equipment.required_protocol.is_empty();
        let cost = if prototype_available {
            0
        } else if self
            .colony
            .has_active_upgrade(BuildingKind::Workshop, PRECISION_BENCH_UPGRADE)
            && matches!(equipment.slot.as_str(), "primary" | "armour")
        {
            base_cost.saturating_sub(5)
        } else {
            base_cost
        };
        let character = self
            .roster
            .iter_mut()
            .find(|character| character.id == character_id)
            .ok_or_else(|| format!("Unknown character: {}", character_id))?;
        if character.equipment_ids.contains(&equipment.id) {
            return Err(format!(
                "{} already carries {}",
                character.name, equipment.name
            ));
        }
        if !prototype_available && self.colony.resources.materials < cost as i32 {
            return Err(format!("Crafting requires {} materials", cost));
        }
        character.equipment_ids.retain(|id| {
            data.equipment
                .iter()
                .find(|item| &item.id == id)
                .is_none_or(|item| item.slot != equipment.slot)
        });
        if prototype_available {
            self.salvage_prototypes = self.salvage_prototypes.saturating_sub(1);
        } else {
            self.colony.resources.materials -= cost as i32;
        }
        character.equipment_ids.push(equipment.id.clone());
        self.refresh_contact_completion(data);
        Ok(cost)
    }

    pub fn refresh_contact_completion(&mut self, data: &GameData) -> bool {
        let progress = self.contact_completion_progress(data);
        self.strategy
            .refresh_contact_completion(progress.aftermath_resolved, progress.prototype_equipped)
    }

    pub fn contact_completion_progress(&self, data: &GameData) -> ContactCompletionProgress {
        let protocol_id = self.strategy.contact_protocol_id.as_str();
        let aftermath_resolved = data
            .campaign
            .events
            .iter()
            .find(|event| event.required_protocol == protocol_id)
            .is_some_and(|definition| {
                self.strategy
                    .character_events
                    .iter()
                    .any(|event| event.id == definition.id && event.resolved)
            });
        let prototype_equipped = data
            .equipment
            .iter()
            .filter(|equipment| equipment.required_protocol == protocol_id)
            .any(|equipment| {
                self.roster
                    .iter()
                    .any(|character| character.equipment_ids.contains(&equipment.id))
            });
        ContactCompletionProgress {
            trace_completed: self.strategy.contact_trace_completed,
            aftermath_resolved,
            prototype_equipped,
        }
    }

    pub fn equipment_is_unlocked(&self, equipment: &EquipmentDef) -> bool {
        equipment.required_protocol.is_empty()
            || (equipment.required_protocol == self.strategy.contact_protocol_id
                && (!equipment.requires_contact_trace || self.strategy.contact_trace_completed))
    }

    pub fn refresh_escalation_completion(&mut self, data: &GameData) -> bool {
        let changed = self.strategy.refresh_escalation_completion();
        if changed {
            self.refresh_mission_offers(data);
        }
        changed
    }
}
