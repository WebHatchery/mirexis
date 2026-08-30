//! Irreversible campaign choices and their immediate strategic transactions.

use crate::colony::ColonyState;
use crate::data::GameData;
use crate::strategy::StrategyState;

impl StrategyState {
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
        self.regenerate_missions(data);
        Ok(response.name.clone())
    }

    pub fn choose_mirexis_path(
        &mut self,
        path_id: &str,
        colony: &mut ColonyState,
        data: &GameData,
    ) -> Result<String, String> {
        if !self.escalation_complete {
            return Err("Mirexis paths unlock after Escalation".to_owned());
        }
        if !self.mirexis_path_id.is_empty() {
            return Err("The colony has already chosen what Mirexis will become".to_owned());
        }
        let path = data
            .campaign
            .mirexis_paths
            .iter()
            .find(|path| path.id == path_id)
            .ok_or_else(|| format!("Unknown Mirexis path: {}", path_id))?;
        if colony.resources.materials < path.materials_cost
            || colony.resources.biomass < path.biomass_cost
            || colony.resources.power < path.power_cost
        {
            return Err(format!(
                "{} requires {} materials, {} biomass, and {} power",
                path.name, path.materials_cost, path.biomass_cost, path.power_cost
            ));
        }
        colony.ensure_identity_building(&path.id)?;
        colony.resources.materials -= path.materials_cost;
        colony.resources.biomass -= path.biomass_cost;
        colony.resources.power -= path.power_cost;
        self.mirexis_path_id = path.id.clone();
        self.regenerate_missions(data);
        Ok(path.name.clone())
    }

    pub fn refresh_mirexis_completion(&mut self, data: &GameData) -> bool {
        if self.campaign_complete || !self.escalation_complete || !self.mirexis_operation_completed
        {
            return false;
        }
        let Some(path) = data
            .campaign
            .mirexis_paths
            .iter()
            .find(|path| path.id == self.mirexis_path_id)
        else {
            return false;
        };
        self.campaign_complete = true;
        self.phase_name = path.ending_title.clone();
        self.phase_summary = format!("{} {}", path.revelation, path.legacy);
        true
    }
}
