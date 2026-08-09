//! Phase One campaign pressure, research, events, and mission generation.

use crate::colony::ColonyState;
use crate::data::{
    CoverEdgeDef, EdgeDirection, GameData, MissionDef, MissionTemplateDef, ObjectiveKind,
    OperationModifier,
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
    #[serde(default)]
    pub attention_faction: String,
    #[serde(default)]
    pub legacy_name: String,
    #[serde(default)]
    pub legacy_character_id: String,
    #[serde(default)]
    pub legacy_stat: String,
    #[serde(default)]
    pub legacy_amount: i32,
    #[serde(default)]
    pub required_protocol: String,
    #[serde(default)]
    pub requires_contact_trace: bool,
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
    #[serde(default)]
    pub biomass_reward: i32,
    #[serde(default)]
    pub power_reward: i32,
    #[serde(default)]
    pub operation_modifier: OperationModifier,
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
    #[serde(default)]
    pub isolation_victories: u8,
    #[serde(default)]
    pub first_assault_repulsed: bool,
    #[serde(default)]
    pub isolation_complete: bool,
    #[serde(default)]
    pub contact_protocol_id: String,
    #[serde(default)]
    pub contact_trace_completed: bool,
    #[serde(default)]
    pub contact_complete: bool,
    #[serde(default)]
    pub adaptation_operation_completed: bool,
    #[serde(default)]
    pub adaptation_complete: bool,
    #[serde(default)]
    pub escalation_operation_completed: bool,
    #[serde(default)]
    pub escalation_response_id: String,
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
            biomass_reward: data.mission.biomass_reward,
            power_reward: data.mission.power_reward,
            operation_modifier: data.mission.operation_modifier,
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
                .map(character_event_from_def)
                .collect(),
            mission_offers: vec![initial.clone()],
            selected_mission_id: initial.id,
            isolation_victories: 0,
            first_assault_repulsed: false,
            isolation_complete: false,
            contact_protocol_id: String::new(),
            contact_trace_completed: false,
            contact_complete: false,
            adaptation_operation_completed: false,
            adaptation_complete: false,
            escalation_operation_completed: false,
            escalation_response_id: String::new(),
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
        let strategic_bonus = self.strategic_reward_bonus(data);
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
                }
                + strategic_bonus.0,
            biomass_reward: instance.biomass_reward + strategic_bonus.1,
            power_reward: instance.power_reward + strategic_bonus.2,
            operation_modifier: instance.operation_modifier,
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
        if victory {
            self.isolation_victories = self.isolation_victories.saturating_add(1).min(3);
            if mission.map_recipe == "colony_defense" {
                self.first_assault_repulsed = true;
            }
            if data
                .campaign
                .mission_templates
                .iter()
                .find(|template| template.id == mission.template_id)
                .is_some_and(|template| {
                    !template.required_protocol.is_empty()
                        && template.required_protocol == self.contact_protocol_id
                })
            {
                self.contact_trace_completed = true;
            }
            if data
                .campaign
                .mission_templates
                .iter()
                .find(|template| template.id == mission.template_id)
                .is_some_and(|template| template.required_phase == "adaptation")
            {
                self.adaptation_operation_completed = true;
            }
            if data
                .campaign
                .mission_templates
                .iter()
                .find(|template| template.id == mission.template_id)
                .is_some_and(|template| template.required_phase == "escalation")
            {
                self.escalation_operation_completed = true;
            }
        }
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
        let name = research.name.clone();
        self.refresh_isolation_completion(colony);
        Ok(name)
    }

    pub fn research_completed(&self, research_id: &str) -> bool {
        self.research
            .iter()
            .any(|research| research.id == research_id && research.completed)
    }

    pub fn refresh_isolation_completion(&mut self, colony: &mut ColonyState) -> bool {
        if self.isolation_complete
            || self.isolation_victories < 3
            || !self.first_assault_repulsed
            || !self.research.iter().any(|research| research.completed)
        {
            return false;
        }
        self.isolation_complete = true;
        self.phase_id = "contact".to_owned();
        self.phase_name = "PHASE TWO: CONTACT".to_owned();
        self.phase_summary =
            "Mirexis survived isolation. Enemy signals now answer the colony by name.".to_owned();
        colony.resources.alien_components += 2;
        true
    }

    pub fn choose_contact_protocol(
        &mut self,
        protocol_id: &str,
        colony: &mut ColonyState,
        data: &GameData,
    ) -> Result<String, String> {
        if !self.isolation_complete {
            return Err("Contact protocols unlock after Isolation".to_owned());
        }
        if !self.contact_protocol_id.is_empty() {
            return Err("A Contact protocol is already active".to_owned());
        }
        let protocol = data
            .campaign
            .contact_protocols
            .iter()
            .find(|protocol| protocol.id == protocol_id)
            .ok_or_else(|| format!("Unknown Contact protocol: {}", protocol_id))?;
        if colony.resources.alien_components < protocol.alien_components_cost {
            return Err(format!(
                "{} requires {} Alien Components",
                protocol.name, protocol.alien_components_cost
            ));
        }
        colony.resources.alien_components -= protocol.alien_components_cost;
        self.contact_protocol_id = protocol.id.clone();
        self.generate_missions(data);
        Ok(protocol.name.clone())
    }

    pub fn refresh_contact_completion(
        &mut self,
        aftermath_resolved: bool,
        prototype_equipped: bool,
    ) -> bool {
        if self.contact_complete
            || !self.contact_trace_completed
            || !aftermath_resolved
            || !prototype_equipped
        {
            return false;
        }
        self.contact_complete = true;
        self.phase_id = "adaptation".to_owned();
        self.phase_name = "PHASE THREE: ADAPTATION".to_owned();
        self.phase_summary =
            "Contact changed Mirexis. The colony must now decide which changes it can survive."
                .to_owned();
        true
    }

    pub fn refresh_adaptation_completion(
        &mut self,
        enough_evolutions: bool,
        gene_lab_powered: bool,
    ) -> bool {
        if self.adaptation_complete
            || !self.contact_complete
            || !self.adaptation_operation_completed
            || !enough_evolutions
            || !gene_lab_powered
        {
            return false;
        }
        self.adaptation_complete = true;
        self.phase_id = "escalation".to_owned();
        self.phase_name = "PHASE FOUR: ESCALATION".to_owned();
        self.phase_summary =
            "Mirexis has learned to direct its changes. The three powers now move openly against it."
                .to_owned();
        true
    }

    pub fn choose_escalation_response(
        &mut self,
        response_id: &str,
        colony: &mut ColonyState,
        data: &GameData,
    ) -> Result<String, String> {
        if !self.escalation_operation_completed {
            return Err("Escalation responses unlock after Three Knives".to_owned());
        }
        if !self.escalation_response_id.is_empty() {
            return Err("The colony has already answered the convergence".to_owned());
        }
        let response = data
            .campaign
            .escalation_responses
            .iter()
            .find(|response| response.id == response_id)
            .ok_or_else(|| format!("Unknown Escalation response: {}", response_id))?;
        if colony.resources.materials < response.materials_cost
            || colony.resources.biomass < response.biomass_cost
            || colony.resources.power < response.power_cost
        {
            return Err(format!(
                "{} requires {} materials, {} biomass, and {} power",
                response.name, response.materials_cost, response.biomass_cost, response.power_cost
            ));
        }
        colony.resources.materials -= response.materials_cost;
        colony.resources.biomass -= response.biomass_cost;
        colony.resources.power -= response.power_cost;
        for faction in &mut self.factions {
            faction.attention = (faction.attention + response.attention_change_all).clamp(0, 100);
        }
        for threat in &mut self.threats {
            threat.operations_until = threat
                .operations_until
                .saturating_add(response.threat_delay);
        }
        self.escalation_response_id = response.id.clone();
        self.generate_missions(data);
        Ok(response.name.clone())
    }

    fn strategic_reward_bonus(&self, data: &GameData) -> (i32, i32, i32) {
        let contact = data
            .campaign
            .contact_protocols
            .iter()
            .find(|protocol| protocol.id == self.contact_protocol_id)
            .map_or((0, 0, 0), |protocol| {
                (
                    protocol.materials_bonus,
                    protocol.biomass_bonus,
                    protocol.power_bonus,
                )
            });
        let escalation_materials = data
            .campaign
            .escalation_responses
            .iter()
            .find(|response| response.id == self.escalation_response_id)
            .map_or(0, |response| response.materials_bonus);
        (contact.0 + escalation_materials, contact.1, contact.2)
    }

    pub fn resolve_first_event(&mut self, colony: &mut ColonyState) -> Result<String, String> {
        let index = self
            .character_events
            .iter()
            .position(|event| {
                event_is_available(
                    event,
                    &self.contact_protocol_id,
                    self.contact_trace_completed,
                )
            })
            .ok_or_else(|| "No unresolved character events".to_owned())?;
        let event = &self.character_events[index];
        let (title, food_cost, attention_change, attention_faction) = (
            event.title.clone(),
            event.food_cost,
            event.attention_change,
            if event.attention_faction.is_empty() {
                "directorate".to_owned()
            } else {
                event.attention_faction.clone()
            },
        );
        if colony.resources.food < food_cost {
            return Err(format!("This choice requires {} food", food_cost));
        }
        colony.resources.food -= food_cost;
        if let Some(faction) = self
            .factions
            .iter_mut()
            .find(|faction| faction.id == attention_faction)
        {
            faction.attention = (faction.attention + attention_change).clamp(0, 100);
        }
        self.character_events[index].resolved = true;
        Ok(title)
    }

    pub fn available_event(&self) -> Option<&CharacterEventState> {
        self.character_events.iter().find(|event| {
            event_is_available(
                event,
                &self.contact_protocol_id,
                self.contact_trace_completed,
            )
        })
    }

    pub fn ensure_character_events(&mut self, data: &GameData) {
        for definition in &data.campaign.events {
            if !self
                .character_events
                .iter()
                .any(|event| event.id == definition.id)
            {
                self.character_events
                    .push(character_event_from_def(definition));
            }
        }
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
                biomass_reward: 0,
                power_reward: 0,
                operation_modifier: OperationModifier::DirectorateFireControl,
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
        let mut templates = data
            .campaign
            .mission_templates
            .iter()
            .filter(|template| {
                (template.required_protocol.is_empty()
                    || template.required_protocol == self.contact_protocol_id)
                    && (template.required_phase.is_empty()
                        || template.required_phase == self.phase_id)
                    && (template.required_response.is_empty()
                        || template.required_response == self.escalation_response_id)
            })
            .collect::<Vec<_>>();
        templates.sort_by_key(|template| {
            (
                template.required_phase != self.phase_id,
                template.required_response != self.escalation_response_id,
                template.required_protocol != self.contact_protocol_id,
                template.faction.as_str() != highest_faction.unwrap_or(""),
                template.id.clone(),
            )
        });
        if templates.len() > 1 {
            let rotate = self.rng.below(templates.len());
            templates.rotate_left(rotate);
            templates.sort_by_key(|template| {
                (
                    template.required_phase != self.phase_id,
                    template.required_response != self.escalation_response_id,
                    template.required_protocol != self.contact_protocol_id,
                    template.faction.as_str() != highest_faction.unwrap_or(""),
                )
            });
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
        let attention = self
            .factions
            .iter()
            .find(|faction| faction.id == template.faction)
            .map_or(0, |faction| faction.attention);
        let operation_modifier = if template.required_phase == "escalation" {
            OperationModifier::EscalationCrossfire
        } else if attention < 20 {
            OperationModifier::None
        } else {
            match template.faction.as_str() {
                "directorate" => OperationModifier::DirectorateFireControl,
                "brood" => OperationModifier::BroodFrenzy,
                "ascendants" => OperationModifier::AscendantInterference,
                _ => OperationModifier::None,
            }
        };
        MissionInstance {
            id: format!("{}_{}", template.id, seed & 0xffff),
            template_id: template.id.clone(),
            name: template.name.clone(),
            briefing: if !template.required_response.is_empty() {
                format!(
                    "The {} response has opened a new {} route through the convergence war.",
                    template.required_response.replace('_', " "),
                    template.faction
                )
            } else if template.required_phase == "escalation" {
                "All three powers have converged on the same targeting beacon; holding it means fighting inside their crossfire."
                    .to_owned()
            } else if !template.required_phase.is_empty() {
                format!(
                    "Gene Lab telemetry has located a scavenging route through this {} site.",
                    template.faction
                )
            } else if template.required_protocol.is_empty() {
                format!(
                    "Isolation intelligence identifies a {} operation beyond the floodlights.",
                    template.faction
                )
            } else {
                format!(
                    "The {} Contact protocol has exposed a signal route into this {} operation.",
                    template.required_protocol.replace('_', " "),
                    template.faction
                )
            },
            objective: template.objective.clone(),
            objective_kind: template.objective_kind,
            faction_id: template.faction.clone(),
            map_recipe: template.map_recipe.clone(),
            seed,
            round_limit: template.round_limit,
            materials_reward: template.materials_reward,
            biomass_reward: template.biomass_reward,
            power_reward: template.power_reward,
            operation_modifier,
        }
    }

    pub fn regenerate_missions(&mut self, data: &GameData) {
        self.generate_missions(data);
    }
}

fn character_event_from_def(event: &crate::data::CharacterEventDef) -> CharacterEventState {
    CharacterEventState {
        id: event.id.clone(),
        title: event.title.clone(),
        description: event.description.clone(),
        participants: event.participants.clone(),
        food_cost: event.food_cost,
        attention_change: event.attention_change,
        attention_faction: event.attention_faction.clone(),
        legacy_name: event.legacy_name.clone(),
        legacy_character_id: event.legacy_character_id.clone(),
        legacy_stat: event.legacy_stat.clone(),
        legacy_amount: event.legacy_amount,
        required_protocol: event.required_protocol.clone(),
        requires_contact_trace: event.requires_contact_trace,
        resolved: false,
    }
}

fn event_is_available(
    event: &CharacterEventState,
    protocol_id: &str,
    trace_complete: bool,
) -> bool {
    !event.resolved
        && (event.required_protocol.is_empty() || event.required_protocol == protocol_id)
        && (!event.requires_contact_trace || trace_complete)
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
            biomass_awarded: 0,
            power_awarded: 0,
        };
        a.resolve_mission(&outcome, &mission, &data);
        b.resolve_mission(&outcome, &mission, &data);
        assert_eq!(a.mission_offers, b.mission_offers);
        assert_eq!(a.active_threat().unwrap().operations_until, 2);
        assert_eq!(a.mission_offers.len(), 2);
    }

    #[test]
    fn isolation_completion_requires_victories_research_and_a_repulsed_assault() {
        let data = GameData::load().unwrap();
        let mut strategy = StrategyState::new(&data);
        strategy.isolation_victories = 2;
        let mut defense = strategy.selected_mission().unwrap().clone();
        defense.map_recipe = "colony_defense".to_owned();
        let outcome = MissionOutcome {
            result: ObjectiveState::Victory,
            colonists_deployed: 3,
            colonists_incapacitated: Vec::new(),
            hostiles_neutralised: 3,
            materials_awarded: 0,
            biomass_awarded: 0,
            power_awarded: 0,
        };
        strategy.resolve_mission(&outcome, &defense, &data);
        assert_eq!(strategy.isolation_victories, 3);
        assert!(strategy.first_assault_repulsed);

        let mut colony = ColonyState::new();
        assert!(!strategy.refresh_isolation_completion(&mut colony));
        strategy.research[0].completed = true;
        assert!(strategy.refresh_isolation_completion(&mut colony));
        assert_eq!(strategy.phase_id, "contact");
        assert_eq!(colony.resources.alien_components, 2);
        assert!(!strategy.refresh_isolation_completion(&mut colony));
        assert_eq!(colony.resources.alien_components, 2);
    }

    #[test]
    fn contact_protocols_spend_the_reward_and_change_future_recovery() {
        let data = GameData::load().unwrap();
        for (protocol_id, expected_bonus) in [
            ("directorate_requisition", (10, 0, 0)),
            ("brood_cultivation", (0, 4, 0)),
            ("ascendant_capacitor", (0, 0, 2)),
        ] {
            let mut strategy = StrategyState::new(&data);
            let mut colony = ColonyState::new();
            assert!(strategy
                .choose_contact_protocol(protocol_id, &mut colony, &data)
                .is_err());
            strategy.isolation_victories = 3;
            strategy.first_assault_repulsed = true;
            strategy.research[0].completed = true;
            assert!(strategy.refresh_isolation_completion(&mut colony));
            strategy
                .choose_contact_protocol(protocol_id, &mut colony, &data)
                .unwrap();
            assert_eq!(colony.resources.alien_components, 0);
            let base = strategy.selected_mission().unwrap().clone();
            assert_eq!(
                base.template_id,
                format!("{}_contact_trace", protocol_id.split('_').next().unwrap())
            );
            assert!(strategy.mission_offers.iter().all(|offer| {
                data.campaign
                    .mission_templates
                    .iter()
                    .find(|template| template.id == offer.template_id)
                    .is_none_or(|template| {
                        template.required_protocol.is_empty()
                            || template.required_protocol == protocol_id
                    })
            }));
            assert!(strategy
                .choose_contact_protocol(protocol_id, &mut colony, &data)
                .is_err());
            let mission = strategy.materialize_selected(&data, &colony);
            assert_eq!(
                (
                    mission.materials_reward - base.materials_reward,
                    mission.biomass_reward - base.biomass_reward,
                    mission.power_reward - base.power_reward,
                ),
                expected_bonus
            );
        }
    }

    #[test]
    fn victorious_matching_contact_trace_unlocks_its_aftermath() {
        let data = GameData::load().unwrap();
        let mut strategy = StrategyState::new(&data);
        let mut colony = ColonyState::new();
        strategy.isolation_victories = 3;
        strategy.first_assault_repulsed = true;
        strategy.research[0].completed = true;
        strategy.refresh_isolation_completion(&mut colony);
        strategy
            .choose_contact_protocol("ascendant_capacitor", &mut colony, &data)
            .unwrap();
        let mission = strategy.selected_mission().unwrap().clone();
        let outcome = MissionOutcome {
            result: ObjectiveState::Victory,
            colonists_deployed: 3,
            colonists_incapacitated: Vec::new(),
            hostiles_neutralised: 3,
            materials_awarded: 0,
            biomass_awarded: 0,
            power_awarded: 0,
        };
        assert!(!strategy.contact_trace_completed);
        strategy.resolve_mission(&outcome, &mission, &data);
        assert!(strategy.contact_trace_completed);
    }

    #[test]
    fn contact_completion_requires_trace_aftermath_and_prototype() {
        let data = GameData::load().unwrap();
        let mut strategy = StrategyState::new(&data);
        let mut colony = ColonyState::new();
        strategy.isolation_victories = 3;
        strategy.first_assault_repulsed = true;
        strategy.research[0].completed = true;
        strategy.refresh_isolation_completion(&mut colony);
        strategy.contact_trace_completed = true;
        assert!(!strategy.refresh_contact_completion(false, false));
        assert!(!strategy.refresh_contact_completion(true, false));
        assert!(strategy.refresh_contact_completion(true, true));
        assert_eq!(strategy.phase_id, "adaptation");
        assert!(!strategy.refresh_contact_completion(true, true));
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
            biomass_awarded: 0,
            power_awarded: 0,
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
    fn high_faction_attention_adds_its_pressure_modifier() {
        let data = GameData::load().unwrap();
        for (faction_id, expected) in [
            ("directorate", OperationModifier::DirectorateFireControl),
            ("brood", OperationModifier::BroodFrenzy),
            ("ascendants", OperationModifier::AscendantInterference),
        ] {
            let mut strategy = StrategyState::new(&data);
            strategy
                .factions
                .iter_mut()
                .find(|faction| faction.id == faction_id)
                .unwrap()
                .attention = 20;
            let template = data
                .campaign
                .mission_templates
                .iter()
                .find(|template| template.faction == faction_id)
                .unwrap();
            assert_eq!(strategy.instantiate(template).operation_modifier, expected);
        }
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
        assert_eq!(layouts.len(), data.campaign.mission_templates.len());
    }

    #[test]
    fn new_faction_operations_have_distinct_objectives_and_recovery() {
        let data = GameData::load().unwrap();
        let sporefield = data
            .campaign
            .mission_templates
            .iter()
            .find(|template| template.id == "sporefield_extraction")
            .unwrap();
        assert_eq!(sporefield.objective_kind, ObjectiveKind::Extraction);
        assert_eq!(sporefield.biomass_reward, 8);
        let vault = data
            .campaign
            .mission_templates
            .iter()
            .find(|template| template.id == "vault_purge")
            .unwrap();
        assert_eq!(vault.objective_kind, ObjectiveKind::EliminateAll);
        assert_eq!(vault.power_reward, 3);
        assert_ne!(sporefield.map_recipe, vault.map_recipe);
    }

    #[test]
    fn adaptation_operation_is_phase_gated_and_prioritized() {
        let data = GameData::load().unwrap();
        let mut strategy = StrategyState::new(&data);
        strategy.regenerate_missions(&data);
        assert!(strategy
            .mission_offers
            .iter()
            .all(|mission| mission.template_id != "adaptation_glass_nerve"));
        strategy.phase_id = "adaptation".to_owned();
        strategy.regenerate_missions(&data);
        assert_eq!(
            strategy.mission_offers[0].template_id,
            "adaptation_glass_nerve"
        );
    }

    #[test]
    fn escalation_operation_is_phase_gated_and_forces_crossfire() {
        let data = GameData::load().unwrap();
        let mut strategy = StrategyState::new(&data);
        strategy.phase_id = "escalation".to_owned();
        strategy.regenerate_missions(&data);
        assert_eq!(
            strategy.mission_offers[0].template_id,
            "escalation_three_knives"
        );
        assert_eq!(
            strategy.mission_offers[0].operation_modifier,
            OperationModifier::EscalationCrossfire
        );
        let mission = strategy.mission_offers[0].clone();
        let outcome = MissionOutcome {
            result: ObjectiveState::Victory,
            colonists_deployed: 3,
            colonists_incapacitated: Vec::new(),
            hostiles_neutralised: 3,
            materials_awarded: mission.materials_reward,
            biomass_awarded: mission.biomass_reward,
            power_awarded: mission.power_reward,
        };
        strategy.resolve_mission(&outcome, &mission, &data);
        assert!(strategy.escalation_operation_completed);
    }

    #[test]
    fn escalation_responses_trade_distinct_resources_for_distinct_strategy_effects() {
        let data = GameData::load().unwrap();

        let mut bastion = StrategyState::new(&data);
        let mut colony = ColonyState::new();
        assert!(bastion
            .choose_escalation_response("bastion_beacon", &mut colony, &data)
            .is_err());
        bastion.escalation_operation_completed = true;
        bastion.phase_id = "escalation".to_owned();
        let materials_before = colony.resources.materials;
        let threat_before = bastion.threats[0].operations_until;
        bastion
            .choose_escalation_response("bastion_beacon", &mut colony, &data)
            .unwrap();
        assert_eq!(colony.resources.materials, materials_before - 30);
        assert_eq!(bastion.threats[0].operations_until, threat_before + 2);
        assert_eq!(
            bastion.mission_offers[0].template_id,
            "escalation_bastion_breakwater"
        );

        let mut decoy = StrategyState::new(&data);
        let mut colony = ColonyState::new();
        decoy.escalation_operation_completed = true;
        decoy.phase_id = "escalation".to_owned();
        colony.resources.biomass = 20;
        let attention_before = decoy.factions[0].attention;
        decoy
            .choose_escalation_response("living_decoy", &mut colony, &data)
            .unwrap();
        assert_eq!(colony.resources.biomass, 12);
        assert_eq!(decoy.factions[0].attention, attention_before - 8);
        assert_eq!(
            decoy.mission_offers[0].template_id,
            "escalation_living_false_heart"
        );

        let mut lattice = StrategyState::new(&data);
        let mut colony = ColonyState::new();
        lattice.escalation_operation_completed = true;
        lattice.phase_id = "escalation".to_owned();
        lattice.regenerate_missions(&data);
        let power_before = colony.resources.power;
        lattice
            .choose_escalation_response("weaponized_lattice", &mut colony, &data)
            .unwrap();
        assert_eq!(colony.resources.power, power_before - 4);
        assert_eq!(
            lattice.mission_offers[0].template_id,
            "escalation_lattice_live_wire"
        );
        let base_reward = lattice.selected_mission().unwrap().materials_reward;
        assert_eq!(
            lattice
                .materialize_selected(&data, &colony)
                .materials_reward,
            base_reward + 8
        );
        assert!(lattice
            .choose_escalation_response("living_decoy", &mut colony, &data)
            .is_err());
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
