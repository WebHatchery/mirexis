//! Campaign progression, choices, and research resolution.

use super::*;

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

    // Serialized strategy compatibility retains this accessor for older save
    // readers even when the current UI does not display it directly.
    pub fn resolve_mission(
        &mut self,
        outcome: &MissionOutcome,
        mission: &MissionInstance,
        data: &GameData,
    ) {
        self.resolve_mission_with_options(outcome, mission, data, 0, 2);
    }

    pub fn resolve_mission_with_options(
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

    pub fn complete_research_with_discount(
        &mut self,
        research_id: &str,
        colony: &mut ColonyState,
        materials_discount: i32,
    ) -> Result<String, String> {
        let research = self
            .research
            .iter_mut()
            .find(|research| research.id == research_id)
            .ok_or_else(|| format!("Unknown research: {}", research_id))?;
        if research.completed {
            return Err("Research is already complete".to_owned());
        }
        let cost = (research.materials_cost - materials_discount).max(5);
        if colony.resources.materials < cost {
            return Err(format!("Research requires {} materials", cost));
        }
        colony.resources.materials -= cost;
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

    pub fn can_resolve_first_event(&self, available_food: i32) -> bool {
        self.available_event()
            .is_some_and(|event| event.food_cost <= available_food)
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
}
