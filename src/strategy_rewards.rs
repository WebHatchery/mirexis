//! Persistent strategic bonuses folded into generated mission recovery.

use crate::data::GameData;

pub(crate) fn bonus(
    contact_protocol_id: &str,
    escalation_response_id: &str,
    mirexis_path_id: &str,
    data: &GameData,
) -> (i32, i32, i32) {
    let contact = data
        .campaign
        .contact_protocols
        .iter()
        .find(|protocol| protocol.id == contact_protocol_id)
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
        .find(|response| response.id == escalation_response_id)
        .map_or(0, |response| response.materials_bonus);
    let mirexis_power = data
        .campaign
        .mirexis_paths
        .iter()
        .find(|path| path.id == mirexis_path_id)
        .map_or(0, |path| path.power_bonus);
    (
        contact.0 + escalation_materials,
        contact.1,
        contact.2 + mirexis_power,
    )
}
