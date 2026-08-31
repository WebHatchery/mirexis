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
        return match character.id.as_str() {
            "ilya_reed" if !character.injuries.is_empty() => treatment_button_label(campaign),
            "nadi_vale" | "sedge" => gene_lab_button_label(campaign),
            _ => super::npc_action_label(character, guest).to_owned(),
        };
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

fn treatment_button_label(campaign: &CampaignState) -> String {
    let has_infirmary = campaign.colony.has_facility(BuildingKind::Infirmary);
    treatment_button_label_for_state(
        has_infirmary,
        campaign.colony.resources.biomass,
        campaign.treatment_cost(),
    )
}

fn treatment_button_label_for_state(
    has_infirmary: bool,
    biomass: i32,
    treatment_cost: i32,
) -> String {
    if !has_infirmary {
        "TREAT // NEED INFIRMARY".to_owned()
    } else if biomass < treatment_cost {
        format!("TREAT // NEED {treatment_cost} BIO")
    } else {
        format!("TREAT // {treatment_cost} BIO")
    }
}

#[derive(Clone, Copy)]
enum GeneLabState {
    Damaged,
    Unpowered,
    Powered,
}

fn gene_lab_button_label(campaign: &CampaignState) -> String {
    let building = campaign
        .colony
        .buildings
        .iter()
        .find(|building| building.kind == BuildingKind::GeneLab);
    let state = building.map(|building| {
        if building.damaged {
            GeneLabState::Damaged
        } else if campaign.colony.building_is_powered(&building.id) {
            GeneLabState::Powered
        } else {
            GeneLabState::Unpowered
        }
    });
    let project_queued = campaign
        .colony
        .construction_queue
        .iter()
        .any(|project| project.kind == BuildingKind::GeneLab);
    gene_lab_button_label_for_state(campaign.strategy.contact_complete, project_queued, state)
        .to_owned()
}

fn gene_lab_button_label_for_state(
    unlocked: bool,
    project_queued: bool,
    state: Option<GeneLabState>,
) -> &'static str {
    if !unlocked {
        "GENE LAB // NEED ADAPTATION"
    } else if project_queued {
        "GENE LAB // PROJECT QUEUED"
    } else {
        match state {
            Some(GeneLabState::Damaged) => "GENE LAB // REPAIR FIRST",
            Some(GeneLabState::Unpowered) => "GENE LAB // NEED POWER",
            Some(GeneLabState::Powered) => "ENTER GENE LAB",
            None => "GENE LAB // BUILD FIRST",
        }
    }
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
