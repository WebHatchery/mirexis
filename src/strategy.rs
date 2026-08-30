//! Phase One campaign pressure, research, events, and mission generation.

use crate::colony::{BuildingKind, ColonyState, SIGNAL_CARTOGRAPHY_UPGRADE};
use crate::data::{GameData, MissionTemplateDef, ObjectiveKind, OperationModifier};
use crate::state::{MissionOutcome, ObjectiveState};
use crate::strategy_events::{
    from_definition as character_event_from_def, is_available as event_is_available,
};
use macroquad_toolkit::rng::SeededRng;
use serde::{Deserialize, Serialize};

mod materialization;

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
    #[serde(default)]
    pub hostile_unit_ids: Vec<String>,
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
    #[serde(default)]
    pub escalation_branch_completed: bool,
    #[serde(default)]
    pub escalation_complete: bool,
    #[serde(default)]
    pub mirexis_path_id: String,
    #[serde(default)]
    pub mirexis_operation_completed: bool,
    #[serde(default)]
    pub campaign_complete: bool,
    #[serde(default)]
    pub post_campaign_operations_completed: u32,
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
            hostile_unit_ids: Vec::new(),
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
            escalation_branch_completed: false,
            escalation_complete: false,
            mirexis_path_id: String::new(),
            mirexis_operation_completed: false,
            campaign_complete: false,
            post_campaign_operations_completed: 0,
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

    #[allow(dead_code)]
    pub fn resolve_mission(
        &mut self,
        outcome: &MissionOutcome,
        mission: &MissionInstance,
        data: &GameData,
    ) {
        self.resolve_mission_with_options(outcome, mission, data, 0, 2);
    }

    pub(crate) fn resolve_mission_with_options(
        &mut self,
        outcome: &MissionOutcome,
        mission: &MissionInstance,
        data: &GameData,
        attention_relief: i32,
        mission_offer_limit: usize,
    ) {
        let victory = outcome.result == ObjectiveState::Victory;
        let is_post_campaign_operation = data
            .campaign
            .mission_templates
            .iter()
            .find(|template| template.id == mission.template_id)
            .is_some_and(|template| template.post_campaign);
        if victory && self.campaign_complete && is_post_campaign_operation {
            self.post_campaign_operations_completed =
                self.post_campaign_operations_completed.saturating_add(1);
        }
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
            if data
                .campaign
                .mission_templates
                .iter()
                .find(|template| template.id == mission.template_id)
                .is_some_and(|template| {
                    !template.required_response.is_empty()
                        && template.required_response == self.escalation_response_id
                })
            {
                self.escalation_branch_completed = true;
            }
            if data
                .campaign
                .mission_templates
                .iter()
                .find(|template| template.id == mission.template_id)
                .is_some_and(|template| {
                    !template.required_mirexis_path.is_empty()
                        && template.required_mirexis_path == self.mirexis_path_id
                })
            {
                self.mirexis_operation_completed = true;
                self.refresh_mirexis_completion(data);
            }
        }
        if let Some(faction) = self
            .factions
            .iter_mut()
            .find(|faction| faction.id == mission.faction_id)
        {
            faction.attention = (faction.attention
                + (if victory { 8 } else { 3 } - attention_relief).max(0))
            .clamp(0, 100);
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
        self.generate_missions_with_limit(data, mission_offer_limit);
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
        self.generate_missions_with_limit(
            data,
            if colony.has_active_upgrade(BuildingKind::CommandCentre, SIGNAL_CARTOGRAPHY_UPGRADE) {
                3
            } else {
                2
            },
        );
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

    pub fn refresh_escalation_completion(&mut self) -> bool {
        if self.escalation_complete
            || !self.adaptation_complete
            || !self.escalation_operation_completed
            || self.escalation_response_id.is_empty()
            || !self.escalation_branch_completed
        {
            return false;
        }
        self.escalation_complete = true;
        self.phase_id = "mirexis".to_owned();
        self.phase_name = "PHASE FIVE // MIREXIS".to_owned();
        self.phase_summary =
            "The convergence has broken open. Mirexis must decide what the colony will become."
                .to_owned();
        true
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
        self.generate_missions_with_limit(data, 2);
    }

    fn generate_missions_with_limit(&mut self, data: &GameData, offer_limit: usize) {
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
                hostile_unit_ids: Vec::new(),
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
                    && (template.required_mirexis_path.is_empty()
                        || template.required_mirexis_path == self.mirexis_path_id)
                    && (if self.campaign_complete {
                        template.required_mirexis_path.is_empty() || template.post_campaign
                    } else {
                        !template.post_campaign
                    })
            })
            .collect::<Vec<_>>();
        templates.sort_by_key(|template| {
            (
                template.post_campaign != self.campaign_complete,
                template.required_phase != self.phase_id,
                template.required_response != self.escalation_response_id,
                template.required_mirexis_path != self.mirexis_path_id,
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
                    template.post_campaign != self.campaign_complete,
                    template.required_phase != self.phase_id,
                    template.required_response != self.escalation_response_id,
                    template.required_mirexis_path != self.mirexis_path_id,
                    template.required_protocol != self.contact_protocol_id,
                    template.faction.as_str() != highest_faction.unwrap_or(""),
                )
            });
        }
        self.mission_offers = templates
            .into_iter()
            .take(offer_limit.max(1))
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
        let operation_modifier = match template.required_mirexis_path.as_str() {
            "human_redoubt" => OperationModifier::MirexisRedoubt,
            "living_commonwealth" => OperationModifier::MirexisCommonwealth,
            "open_threshold" => OperationModifier::MirexisThreshold,
            _ if template.required_phase == "escalation" => OperationModifier::EscalationCrossfire,
            _ if attention < 20 => OperationModifier::None,
            _ => match template.faction.as_str() {
                "directorate" => OperationModifier::DirectorateFireControl,
                "brood" => OperationModifier::BroodFrenzy,
                "ascendants" => OperationModifier::AscendantInterference,
                _ => OperationModifier::None,
            },
        };
        MissionInstance {
            id: format!("{}_{}", template.id, seed & 0xffff),
            template_id: template.id.clone(),
            name: template.name.clone(),
            briefing: if template.post_campaign {
                format!(
                    "The {} path remains open after the campaign; this {} operation is work the refuge still has to do.",
                    template.required_mirexis_path.replace('_', " "),
                    template.faction
                )
            } else if !template.required_mirexis_path.is_empty() {
                format!(
                    "The {} path has exposed a final {} battlefield beyond the convergence.",
                    template.required_mirexis_path.replace('_', " "),
                    template.faction
                )
            } else if !template.required_response.is_empty() {
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
            hostile_unit_ids: template.hostile_unit_ids.clone(),
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

    pub(crate) fn regenerate_missions_with_offer_limit(
        &mut self,
        data: &GameData,
        offer_limit: usize,
    ) {
        self.generate_missions_with_limit(data, offer_limit);
    }
}

#[cfg(test)]
mod tests;
