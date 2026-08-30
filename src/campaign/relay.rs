//! The Relay Mast's risky operation-intelligence action.

use super::CampaignState;
use crate::colony::BuildingKind;
use crate::data::GameData;

pub(crate) const RELAY_SCAN_POWER_COST: i32 = 2;
pub(crate) const RELAY_SIGNAL_ATTENTION: i32 = 4;

impl CampaignState {
    pub fn relay_scan_available(&self) -> bool {
        !self.strategy.campaign_complete
            && self.colony.has_facility(BuildingKind::RelayMast)
            && self.relay_scan_operation != Some(self.operations_completed)
            && self.colony.resources.power >= RELAY_SCAN_POWER_COST
    }

    pub fn run_relay_scan(&mut self, data: &GameData) -> Result<String, String> {
        if self.strategy.campaign_complete {
            return Err("The Relay Mast has no unresolved route after the campaign".to_owned());
        }
        if !self.colony.has_facility(BuildingKind::RelayMast) {
            return Err("An operational Relay Mast is required".to_owned());
        }
        if self.relay_scan_operation == Some(self.operations_completed) {
            return Err("The Relay Mast has already scanned this operation".to_owned());
        }
        if self.colony.resources.power < RELAY_SCAN_POWER_COST {
            return Err(format!(
                "A relay scan requires {} stored power",
                RELAY_SCAN_POWER_COST
            ));
        }
        let (faction_id, faction_name) = self
            .strategy
            .factions
            .iter()
            .max_by_key(|faction| (faction.attention, faction.id.clone()))
            .map(|faction| (faction.id.clone(), faction.name.clone()))
            .ok_or_else(|| "No faction signal is available to scan".to_owned())?;
        self.colony.resources.power -= RELAY_SCAN_POWER_COST;
        if let Some(faction) = self
            .strategy
            .factions
            .iter_mut()
            .find(|faction| faction.id == faction_id)
        {
            faction.attention = (faction.attention + RELAY_SIGNAL_ATTENTION).clamp(0, 100);
        }
        self.refresh_mission_offers(data);
        self.relay_scans_used = self.relay_scans_used.saturating_add(1);
        self.relay_scan_operation = Some(self.operations_completed);
        Ok(format!(
            "RELAY SCAN // {} signal exposed // mission routes refreshed",
            faction_name.to_uppercase()
        ))
    }
}
