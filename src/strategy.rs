//! Phase One campaign pressure, research, events, and mission generation.

use crate::colony::ColonyState;
use crate::data::{
    CoverEdgeDef, EdgeDirection, GameData, MissionDef, MissionTemplateDef, ObjectiveKind,
};
use crate::state::{MissionOutcome, ObjectiveState};
use macroquad_toolkit::rng::SeededRng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FactionPressure {
    pub id: String,
    pub name: String,
    pub attention: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TelegraphedThreat {
    pub id: String,
    pub faction_id: String,
    pub name: String,
    pub operations_until: u8,
    pub strength: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResearchOpportunity {
    pub id: String,
    pub name: String,
    pub description: String,
    pub materials_cost: i32,
    pub power_reward: i32,
    pub completed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CharacterEventState {
    pub id: String,
    pub title: String,
    pub description: String,
    pub participants: Vec<String>,
    pub food_cost: i32,
    pub attention_change: i32,
    pub resolved: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MissionInstance {
    pub id: String,
    pub template_id: String,
    pub name: String,
    pub briefing: String,
    pub objective: String,
    #[serde(default)]
    pub objective_kind: ObjectiveKind,
    pub faction_id: String,
    pub map_recipe: String,
    pub seed: u64,
    pub round_limit: u32,
    pub materials_reward: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyState {
    pub phase_id: String,
    pub phase_name: String,
    pub phase_summary: String,
    pub factions: Vec<FactionPressure>,
    pub threats: Vec<TelegraphedThreat>,
    pub research: Vec<ResearchOpportunity>,
    pub character_events: Vec<CharacterEventState>,
    pub mission_offers: Vec<MissionInstance>,
    pub selected_mission_id: String,
    rng: SeededRng,
}

impl StrategyState {
    pub fn new(data: &GameData) -> Self {
        let initial = MissionInstance {
            id: data.mission.id.clone(),
            template_id: "authored_rescue".to_owned(),
            name: data.mission.name.clone(),
            briefing: data.mission.briefing.clone(),
            objective: data.mission.objective.clone(),
            objective_kind: data.mission.objective_kind,
            faction_id: "brood".to_owned(),
            map_recipe: "outer_mire".to_owned(),
            seed: data.config.battle_seed,
            round_limit: data.mission.round_limit,
            materials_reward: data.mission.materials_reward,
        };
        Self {
            phase_id: data.campaign.phase_id.clone(),
            phase_name: data.campaign.phase_name.clone(),
            phase_summary: data.campaign.summary.clone(),
            factions: data
                .campaign
                .factions
                .iter()
                .map(|faction| FactionPressure {
                    id: faction.id.clone(),
                    name: faction.name.clone(),
                    attention: faction.initial_attention,
                })
                .collect(),
            threats: vec![TelegraphedThreat {
                id: "directorate_first_assault".to_owned(),
                faction_id: "directorate".to_owned(),
                name: "DIRECTORATE ASSAULT EXPECTED".to_owned(),
                operations_until: 3,
                strength: 3,
            }],
            research: data
                .campaign
                .research
                .iter()
                .map(|research| ResearchOpportunity {
                    id: research.id.clone(),
                    name: research.name.clone(),
                    description: research.description.clone(),
                    materials_cost: research.materials_cost,
                    power_reward: research.power_reward,
                    completed: false,
                })
                .collect(),
            character_events: data
                .campaign
                .events
                .iter()
                .map(|event| CharacterEventState {
                    id: event.id.clone(),
                    title: event.title.clone(),
                    description: event.description.clone(),
                    participants: event.participants.clone(),
                    food_cost: event.food_cost,
                    attention_change: event.attention_change,
                    resolved: false,
                })
                .collect(),
            mission_offers: vec![initial.clone()],
            selected_mission_id: initial.id,
            rng: SeededRng::new(data.config.battle_seed ^ 0x1501_A710),
        }
    }

    pub fn selected_mission(&self) -> Option<&MissionInstance> {
        self.mission_offers
            .iter()
            .find(|mission| mission.id == self.selected_mission_id)
            .or_else(|| self.mission_offers.first())
    }

    pub fn select_mission(&mut self, mission_id: &str) -> Result<(), String> {
        if !self
            .mission_offers
            .iter()
            .any(|mission| mission.id == mission_id)
        {
            return Err(format!("Unknown mission offer: {}", mission_id));
        }
        self.selected_mission_id = mission_id.to_owned();
        Ok(())
    }

    pub fn materialize_selected(&self, data: &GameData, colony: &ColonyState) -> MissionDef {
        let base = &data.mission;
        let Some(instance) = self.selected_mission() else {
            return base.clone();
        };
        let defense = colony.defense_map();
        let colony_defense = instance.map_recipe == "colony_defense";
        let recipe = data
            .campaign
            .map_recipes
            .iter()
            .find(|recipe| recipe.id == instance.map_recipe);
        let layout =
            recipe.map(|recipe| crate::map_variants::materialize(recipe, data, instance.seed));
        MissionDef {
            id: instance.id.clone(),
            name: instance.name.clone(),
            briefing: instance.briefing.clone(),
            objective: instance.objective.clone(),
            objective_kind: instance.objective_kind,
            hostile_faction: instance.faction_id.clone(),
            round_limit: instance.round_limit,
            materials_reward: instance.materials_reward
                + if self.research_completed("salvage_doctrine") {
                    8
                } else {
                    0
                },
            cover_integrity: if colony_defense && self.research_completed("field_fortifications") {
                10
            } else {
                base.cover_integrity
            },
            seed: instance.seed,
            blocked_tiles: if colony_defense {
                defense
                    .blocked_tiles
                    .iter()
                    .map(|tile| [tile.x, tile.y])
                    .collect()
            } else if let Some(layout) = &layout {
                layout.blocked_tiles.clone()
            } else {
                base.blocked_tiles.clone()
            },
            objective_tile: if colony_defense {
                defense
                    .critical_objectives
                    .first()
                    .map_or(base.objective_tile, |tile| [tile.x, tile.y])
            } else if let Some(layout) = &layout {
                layout.objective_tile
            } else {
                base.objective_tile
            },
            terrain_costs: layout.as_ref().map_or_else(
                || base.terrain_costs.clone(),
                |layout| layout.terrain_costs.clone(),
            ),
            cover_edges: if colony_defense {
                defense
                    .cover_tiles
                    .iter()
                    .map(|tile| CoverEdgeDef {
                        position: [tile.x, tile.y],
                        direction: EdgeDirection::West,
                        strength: 25,
                    })
                    .collect()
            } else if let Some(layout) = &layout {
                layout.cover_edges.clone()
            } else {
                base.cover_edges.clone()
            },
        }
    }

    pub fn resolve_mission(
        &mut self,
        outcome: &MissionOutcome,
        mission: &MissionInstance,
        data: &GameData,
    ) {
        let victory = outcome.result == ObjectiveState::Victory;
        if let Some(faction) = self
            .factions
            .iter_mut()
            .find(|faction| faction.id == mission.faction_id)
        {
            faction.attention = (faction.attention + if victory { 8 } else { 3 }).clamp(0, 100);
        }
        for threat in &mut self.threats {
            if mission.map_recipe == "colony_defense" && threat.faction_id == mission.faction_id {
                threat.operations_until = if victory { 4 } else { 2 };
                threat.strength = if victory {
                    threat.strength.saturating_sub(1).max(1)
                } else {
                    (threat.strength + 1).min(5)
                };
            } else {
                threat.operations_until = threat.operations_until.saturating_sub(1);
                if victory && threat.faction_id == mission.faction_id {
                    threat.strength = threat.strength.saturating_sub(1).max(1);
                }
            }
        }
        self.generate_missions(data);
    }

    pub fn complete_research(
        &mut self,
        research_id: &str,
        colony: &mut ColonyState,
    ) -> Result<String, String> {
        let research = self
            .research
            .iter_mut()
            .find(|research| research.id == research_id)
            .ok_or_else(|| format!("Unknown research: {}", research_id))?;
        if research.completed {
            return Err("Research is already complete".to_owned());
        }
        if colony.resources.materials < research.materials_cost {
            return Err(format!(
                "Research requires {} materials",
                research.materials_cost
            ));
        }
        colony.resources.materials -= research.materials_cost;
        colony.resources.power += research.power_reward;
        research.completed = true;
        Ok(research.name.clone())
    }

    pub fn research_completed(&self, research_id: &str) -> bool {
        self.research
            .iter()
            .any(|research| research.id == research_id && research.completed)
    }

    pub fn resolve_first_event(&mut self, colony: &mut ColonyState) -> Result<String, String> {
        let event = self
            .character_events
            .iter_mut()
            .find(|event| !event.resolved)
            .ok_or_else(|| "No unresolved character events".to_owned())?;
        if colony.resources.food < event.food_cost {
            return Err(format!("This choice requires {} food", event.food_cost));
        }
        colony.resources.food -= event.food_cost;
        if let Some(directorate) = self
            .factions
            .iter_mut()
            .find(|faction| faction.id == "directorate")
        {
            directorate.attention = (directorate.attention + event.attention_change).clamp(0, 100);
        }
        event.resolved = true;
        Ok(event.title.clone())
    }

    pub fn active_threat(&self) -> Option<&TelegraphedThreat> {
        self.threats.first()
    }

    fn generate_missions(&mut self, data: &GameData) {
        if let Some(threat) = self
            .threats
            .iter()
            .find(|threat| threat.operations_until == 0)
        {
            let defense = MissionInstance {
                id: format!("colony_defense_{}", self.rng.next_u64() & 0xffff),
                template_id: "colony_defense".to_owned(),
                name: "ISOLATION: HOLD THE FLOODLIGHTS".to_owned(),
                briefing: format!(
                    "{} has reached Mirexis. The settlement layout is now the battlefield.",
                    threat.name
                ),
                objective: "Hold the command centre and break the assault before the colony falls."
                    .to_owned(),
                objective_kind: ObjectiveKind::EliminateAll,
                faction_id: threat.faction_id.clone(),
                map_recipe: "colony_defense".to_owned(),
                seed: self.rng.next_u64(),
                round_limit: 8,
                materials_reward: 18,
            };
            self.selected_mission_id = defense.id.clone();
            self.mission_offers = vec![defense];
            return;
        }
        let highest_faction = self
            .factions
            .iter()
            .max_by_key(|faction| (faction.attention, faction.id.clone()))
            .map(|faction| faction.id.as_str());
        let mut templates = data.campaign.mission_templates.iter().collect::<Vec<_>>();
        templates.sort_by_key(|template| {
            (
                template.faction.as_str() != highest_faction.unwrap_or(""),
                template.id.clone(),
            )
        });
        if templates.len() > 1 {
            let rotate = self.rng.below(templates.len());
            templates.rotate_left(rotate);
            templates
                .sort_by_key(|template| template.faction.as_str() != highest_faction.unwrap_or(""));
        }
        self.mission_offers = templates
            .into_iter()
            .take(2)
            .map(|template| self.instantiate(template))
            .collect();
        if let Some(first) = self.mission_offers.first() {
            self.selected_mission_id = first.id.clone();
        }
    }

    fn instantiate(&mut self, template: &MissionTemplateDef) -> MissionInstance {
        let seed = self.rng.next_u64();
        MissionInstance {
            id: format!("{}_{}", template.id, seed & 0xffff),
            template_id: template.id.clone(),
            name: template.name.clone(),
            briefing: format!(
                "Isolation intelligence identifies a {} operation beyond the floodlights.",
                template.faction
            ),
            objective: template.objective.clone(),
            objective_kind: template.objective_kind,
            faction_id: template.faction.clone(),
            map_recipe: template.map_recipe.clone(),
            seed,
            round_limit: template.round_limit,
            materials_reward: template.materials_reward,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn isolation_starts_with_pressure_threats_and_content() {
        let data = GameData::load().unwrap();
        let strategy = StrategyState::new(&data);
        assert_eq!(strategy.phase_id, "isolation");
        assert_eq!(strategy.factions.len(), 3);
        assert_eq!(strategy.active_threat().unwrap().operations_until, 3);
        assert!(!strategy.research.is_empty());
        assert!(!strategy.character_events.is_empty());
    }

    #[test]
    fn mission_resolution_advances_pressure_and_generates_seeded_offers() {
        let data = GameData::load().unwrap();
        let mut a = StrategyState::new(&data);
        let mut b = a.clone();
        let mission = a.selected_mission().unwrap().clone();
        let outcome = MissionOutcome {
            result: ObjectiveState::Victory,
            colonists_deployed: 4,
            colonists_incapacitated: Vec::new(),
            hostiles_neutralised: 3,
            materials_awarded: 30,
        };
        a.resolve_mission(&outcome, &mission, &data);
        b.resolve_mission(&outcome, &mission, &data);
        assert_eq!(a.mission_offers, b.mission_offers);
        assert_eq!(a.active_threat().unwrap().operations_until, 2);
        assert_eq!(a.mission_offers.len(), 2);
    }

    #[test]
    fn expired_threat_generates_a_defense_mission_from_colony_placement() {
        let data = GameData::load().unwrap();
        let mut strategy = StrategyState::new(&data);
        let outcome = MissionOutcome {
            result: ObjectiveState::Victory,
            colonists_deployed: 4,
            colonists_incapacitated: Vec::new(),
            hostiles_neutralised: 3,
            materials_awarded: 30,
        };
        for _ in 0..3 {
            let mission = strategy.selected_mission().unwrap().clone();
            strategy.resolve_mission(&outcome, &mission, &data);
        }
        let defense = strategy.selected_mission().unwrap();
        assert_eq!(defense.map_recipe, "colony_defense");
        let mut colony = ColonyState::new();
        colony
            .place_construction(crate::colony::BuildingKind::Barricade, [1, 1])
            .unwrap();
        colony.advance_operation();
        let materialized = strategy.materialize_selected(&data, &colony);
        assert!(materialized.blocked_tiles.contains(&[3, 2]));
    }

    #[test]
    fn completed_research_changes_future_mission_materialization() {
        let data = GameData::load().unwrap();
        let colony = ColonyState::new();
        let mut strategy = StrategyState::new(&data);
        let base_reward = strategy.selected_mission().unwrap().materials_reward;
        strategy
            .research
            .iter_mut()
            .find(|research| research.id == "salvage_doctrine")
            .unwrap()
            .completed = true;
        assert_eq!(
            strategy
                .materialize_selected(&data, &colony)
                .materials_reward,
            base_reward + 8
        );

        strategy
            .research
            .iter_mut()
            .find(|research| research.id == "field_fortifications")
            .unwrap()
            .completed = true;
        strategy.threats[0].operations_until = 0;
        strategy.generate_missions(&data);
        assert_eq!(
            strategy
                .materialize_selected(&data, &colony)
                .cover_integrity,
            10
        );
    }

    #[test]
    fn each_faction_template_materializes_its_own_battlefield() {
        let data = GameData::load().unwrap();
        let colony = ColonyState::new();
        let mut layouts = std::collections::HashSet::new();
        for template in &data.campaign.mission_templates {
            let mut strategy = StrategyState::new(&data);
            let instance = strategy.instantiate(template);
            strategy.selected_mission_id = instance.id.clone();
            strategy.mission_offers = vec![instance];
            let mission = strategy.materialize_selected(&data, &colony);
            assert_eq!(mission.hostile_faction, template.faction);
            assert!(layouts.insert(mission.blocked_tiles));
        }
        assert_eq!(layouts.len(), 4);
    }

    #[test]
    fn mission_seed_selects_a_repeatable_safe_map_variant() {
        let data = GameData::load().unwrap();
        let colony = ColonyState::new();
        let template = data
            .campaign
            .mission_templates
            .iter()
            .find(|template| template.id == "courier_extraction")
            .unwrap();
        let mut strategy = StrategyState::new(&data);
        let mut instance = strategy.instantiate(template);
        instance.seed = 2;
        strategy.selected_mission_id = instance.id.clone();
        strategy.mission_offers = vec![instance.clone()];
        let authored = strategy.materialize_selected(&data, &colony);
        instance.seed = 3;
        strategy.mission_offers = vec![instance.clone()];
        let variant = strategy.materialize_selected(&data, &colony);
        assert_ne!(authored.blocked_tiles, variant.blocked_tiles);
        assert_eq!(
            variant.blocked_tiles,
            strategy.materialize_selected(&data, &colony).blocked_tiles
        );
        assert!(!variant.blocked_tiles.contains(&variant.objective_tile));
    }
}
