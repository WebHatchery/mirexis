//! Mission offer generation and materialization.

use super::*;

impl StrategyState {
    pub fn generate_missions(&mut self, data: &GameData) {
        self.generate_missions_with_limit(data, 2);
    }

    pub fn generate_missions_with_limit(&mut self, data: &GameData, offer_limit: usize) {
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

    pub fn instantiate(&mut self, template: &MissionTemplateDef) -> MissionInstance {
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

    pub fn regenerate_missions_with_offer_limit(&mut self, data: &GameData, offer_limit: usize) {
        self.generate_missions_with_limit(data, offer_limit);
    }
}
