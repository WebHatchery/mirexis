//! Contextual dialogue action labels and availability.

use crate::campaign::{CampaignState, CharacterRecord};
use crate::colony::BuildingKind;
use crate::data::GameData;
use crate::ui::UiAction;

#[cfg(test)]
mod tests;

pub(super) fn npc_action_button_label(
    campaign: &CampaignState,
    data: &GameData,
    character: &CharacterRecord,
    guest: bool,
) -> String {
    if !guest {
        return super::npc_action_label(character, guest).to_owned();
    }
    let Some(outsider) = campaign.available_outsider(data) else {
        return super::npc_action_label(character, guest).to_owned();
    };
    let resource = if outsider.recruitment_resource.is_empty() {
        "materials"
    } else {
        outsider.recruitment_resource.as_str()
    };
    recruitment_button_label(
        outsider.recruitment_cost,
        resource,
        campaign.recruitment_resource_amount(resource),
    )
}

fn recruitment_button_label(cost: i32, resource: &str, available: i32) -> String {
    let resource = resource.to_uppercase();
    if available < cost {
        format!("RECRUIT // NEED {cost} {resource}")
    } else {
        format!("RECRUIT // {cost} {resource}")
    }
}

pub(super) fn npc_action_enabled(
    campaign: &CampaignState,
    data: &GameData,
    action: &UiAction,
    guest: bool,
) -> bool {
    if guest {
        return campaign.can_recruit_outsider(data);
    }
    match action {
        UiAction::TreatInjury => campaign.can_treat_first_injury(),
        UiAction::OpenGeneLab => campaign.colony.has_facility(BuildingKind::GeneLab),
        _ => true,
    }
}
