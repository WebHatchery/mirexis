//! Operations-panel affordance for first-injury treatment.

use crate::campaign::CampaignState;
use crate::colony::BuildingKind;

fn treatment_button_label_for_state(
    has_infirmary: bool,
    has_injury: bool,
    biomass: i32,
    treatment_cost: i32,
) -> String {
    if !has_infirmary {
        "TREAT // NEED INFIRMARY".to_owned()
    } else if !has_injury {
        "TREAT // NO INJURY".to_owned()
    } else if biomass < treatment_cost {
        format!("TREAT // NEED {} BIO", treatment_cost)
    } else {
        format!("TREAT // {} BIO", treatment_cost)
    }
}

pub(super) fn treatment_button_label(campaign: &CampaignState) -> String {
    let treatment_cost = campaign.treatment_cost();
    treatment_button_label_for_state(
        campaign.colony.has_facility(BuildingKind::Infirmary),
        campaign
            .roster
            .iter()
            .any(|character| !character.injuries.is_empty()),
        campaign.colony.resources.biomass,
        treatment_cost,
    )
}

#[cfg(test)]
mod tests;
