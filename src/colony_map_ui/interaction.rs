//! Hover copy and touch-safe plot actions for the interactive colony map.

use crate::campaign::CampaignState;
use crate::colony::BuildingKind;
use crate::data::GameData;
use crate::grid_ui::WorldCamera;
use crate::ui::UiAction;
use macroquad::prelude::*;

pub(super) fn draw_hover_card(
    campaign: &CampaignState,
    data: &GameData,
    hovered: Option<[i32; 2]>,
    pending: Option<[i32; 2]>,
) {
    let Some(position) = hovered else { return };
    let building = campaign.colony.building_at(position);
    let project = campaign.colony.project_at(position);
    let action_available = plot_action(campaign, data, position).is_some();
    let text = if let Some(building) = building {
        if building.damaged {
            let cost = campaign
                .colony
                .repair_cost_for(&building.id)
                .unwrap_or_else(|| building.kind.repair_cost());
            if campaign.colony.resources.materials >= cost {
                format!(
                    "{} // DAMAGED // REPAIR {} MAT",
                    building.kind.name().to_uppercase(),
                    cost
                )
            } else {
                format!(
                    "{} // DAMAGED // NEED {} MAT // {} MORE REQUIRED",
                    building.kind.name().to_uppercase(),
                    cost,
                    cost - campaign.colony.resources.materials
                )
            }
        } else if building.kind.is_identity() {
            campaign
                .identity_preparation_copy()
                .or_else(|| campaign.identity_stewardship_copy())
                .unwrap_or_else(|| identity_building_copy(campaign, building))
        } else if !campaign.colony.building_is_powered(&building.id) {
            format!(
                "{} // OFFLINE: INSUFFICIENT POWER",
                building.kind.name().to_uppercase()
            )
        } else if building.kind == BuildingKind::GeneLab {
            "GENE LAB // TAP TO OPEN EVOLUTION CHAMBER".to_owned()
        } else if building.kind == BuildingKind::ResearchAnnex {
            let doctrines = campaign
                .strategy
                .research
                .iter()
                .filter(|research| research.completed)
                .count();
            format!(
                "RESEARCH ANNEX // ONLINE // {} DOCTRINES // -5 RESEARCH MAT",
                doctrines
            )
        } else if building.kind == BuildingKind::SalvageYard {
            if campaign.salvage_available() {
                format!(
                    "SALVAGE YARD // {} OBJECT{} // TAP TO SORT",
                    campaign.salvage_cache_count,
                    if campaign.salvage_cache_count == 1 {
                        ""
                    } else {
                        "S"
                    }
                )
            } else if campaign.salvage_yard_operation == Some(campaign.operations_completed) {
                "SALVAGE YARD // SORTED THIS OPERATION".to_owned()
            } else {
                "SALVAGE YARD // ONLINE // WAITING FOR RECOVERY".to_owned()
            }
        } else if building.kind == BuildingKind::Commons {
            let food_cost = campaign.commons_meal_food_cost();
            if campaign.commons_meal_available() {
                format!("COMMONS // TAP TO HOST SHARED MEAL // {food_cost} FOOD")
            } else if campaign.commons_meal_operation == Some(campaign.operations_completed) {
                "COMMONS // SHARED MEAL ALREADY HOSTED THIS OPERATION".to_owned()
            } else {
                format!("COMMONS // NEEDS {food_cost} FOOD + 2 READY SQUAD MEMBERS")
            }
        } else if building.kind == BuildingKind::RelayMast {
            if campaign.relay_scan_available() {
                format!(
                    "RELAY MAST // TAP TO SCAN // {} POWER // +{} ATTENTION",
                    crate::campaign::RELAY_SCAN_POWER_COST,
                    crate::campaign::RELAY_SIGNAL_ATTENTION
                )
            } else if campaign.relay_scan_operation == Some(campaign.operations_completed) {
                "RELAY MAST // SCANNED THIS OPERATION // ROUTES REFRESHED".to_owned()
            } else {
                format!(
                    "RELAY MAST // NEEDS {} STORED POWER",
                    crate::campaign::RELAY_SCAN_POWER_COST
                )
            }
        } else if building.kind == BuildingKind::Watchtower {
            "WATCHTOWER // ONLINE // STRONG WESTERN COVER".to_owned()
        } else if building.kind == BuildingKind::Waystation {
            if let Some(outsider) = campaign.available_outsider(data) {
                let resource = if outsider.recruitment_resource.is_empty() {
                    "MATERIALS"
                } else {
                    outsider.recruitment_resource.as_str()
                };
                if campaign.can_recruit_outsider(data) {
                    format!(
                        "WAYSTATION // TAP TO RECRUIT {} // {} {}",
                        outsider.name.to_uppercase(),
                        outsider.recruitment_cost,
                        resource.to_uppercase()
                    )
                } else {
                    format!(
                        "WAYSTATION // {} // NEED {} {}",
                        outsider.origin.to_uppercase(),
                        outsider.recruitment_cost,
                        resource.to_uppercase()
                    )
                }
            } else if campaign
                .roster
                .iter()
                .any(|character| !character.origin.is_empty())
            {
                "WAYSTATION // CONTACT ROUTE STABLE // ONLINE".to_owned()
            } else {
                "WAYSTATION // ONLINE // AWAITING CONTACT ROUTE".to_owned()
            }
        } else {
            format!(
                "{} // ONLINE // LEVEL {}",
                building.kind.name().to_uppercase(),
                building.level
            )
        }
    } else if let Some(project) = project {
        format!(
            "{} // UNDER CONSTRUCTION // {} OPERATION",
            project.kind.name().to_uppercase(),
            project.operations_remaining
        )
    } else {
        match campaign.colony.validate_construction_site(position) {
            Ok(()) if campaign.construction_available(campaign.colony.planned_construction) => {
                format!(
                    "OPEN PLOT // BUILD {} // {} MAT",
                    campaign.colony.planned_construction.name().to_uppercase(),
                    campaign.colony.planned_construction.material_cost()
                )
            }
            Ok(()) => format!(
                "OPEN PLOT // BUILD {} UNAVAILABLE // {} MAT",
                campaign.colony.planned_construction.name().to_uppercase(),
                campaign.colony.planned_construction.material_cost()
            ),
            Err(error) => format!("CLEARANCE REQUIRED // {}", error.to_uppercase()),
        }
    };
    let text = if !action_available {
        text
    } else if pending == Some(position) {
        format!("TAP AGAIN TO CONFIRM // {text}")
    } else {
        format!("TAP TO ARM // {text}")
    };
    draw_rectangle(
        70.0,
        566.0,
        708.0,
        28.0,
        Color::new(0.025, 0.075, 0.078, 0.94),
    );
    draw_text(&text, 84.0, 585.0, 14.0, Color::new(0.70, 0.92, 0.84, 1.0));
}

fn identity_building_copy(
    campaign: &CampaignState,
    building: &crate::colony::BuildingState,
) -> String {
    let power = campaign.colony.building_is_powered(&building.id);
    match (building.kind, power) {
        (BuildingKind::RedoubtArsenal, true) => {
            "REDOUBT ARSENAL // ONLINE // RESILIENT COVER // PRIORITY OBJECTIVE".to_owned()
        }
        (BuildingKind::RedoubtArsenal, false) => {
            "REDOUBT ARSENAL // OFFLINE // PHYSICAL COVER // PRIORITY OBJECTIVE".to_owned()
        }
        (BuildingKind::ChoirGarden, true) => {
            "CHOIR GARDEN // ONLINE // +1 BIOMASS / OPERATION // LIVING COVER".to_owned()
        }
        (BuildingKind::ChoirGarden, false) => {
            "CHOIR GARDEN // OFFLINE // BIOMASS PAUSED // LIVING COVER OFFLINE".to_owned()
        }
        (BuildingKind::ThresholdSpire, true) => {
            "THRESHOLD SPIRE // ONLINE // SHIELD COVER // PRIORITY OBJECTIVE".to_owned()
        }
        (BuildingKind::ThresholdSpire, false) => {
            "THRESHOLD SPIRE // OFFLINE // NEEDS 2 POWER // SHIELD OFFLINE".to_owned()
        }
        _ => format!(
            "{} // IDENTITY PROJECT",
            building.kind.name().to_uppercase()
        ),
    }
}

pub(super) fn plot_action(
    campaign: &CampaignState,
    data: &GameData,
    position: [i32; 2],
) -> Option<UiAction> {
    if let Some(building) = campaign.colony.building_at(position) {
        return building_action(campaign, data, building);
    }
    if campaign.colony.project_at(position).is_none()
        && campaign.can_construct_building(campaign.colony.planned_construction, position)
    {
        Some(UiAction::ConstructBuilding(
            campaign.colony.planned_construction,
            position,
        ))
    } else {
        None
    }
}

fn building_action(
    campaign: &CampaignState,
    data: &GameData,
    building: &crate::colony::BuildingState,
) -> Option<UiAction> {
    if building.damaged {
        return campaign
            .colony
            .can_repair_building(&building.id)
            .then(|| UiAction::RepairBuilding(building.id.clone()));
    }
    if !campaign.colony.building_is_powered(&building.id) {
        return None;
    }
    match building.kind {
        BuildingKind::GeneLab => Some(UiAction::OpenGeneLab),
        BuildingKind::Waystation if campaign.can_recruit_outsider(data) => {
            Some(UiAction::RecruitOutsider)
        }
        BuildingKind::Commons if campaign.commons_meal_available() => {
            Some(UiAction::HostCommonsMeal)
        }
        BuildingKind::RelayMast if campaign.relay_scan_available() => Some(UiAction::RunRelayScan),
        BuildingKind::SalvageYard if campaign.salvage_available() => Some(UiAction::OpenSalvage),
        kind if kind.is_identity()
            && campaign
                .identity_preparation_definition()
                .is_some_and(|definition| definition.building == kind)
            && campaign.identity_preparation_available() =>
        {
            Some(UiAction::PrepareIdentityBuilding)
        }
        kind if kind.is_identity()
            && campaign
                .identity_stewardship_definition()
                .is_some_and(|definition| definition.building == kind)
            && campaign.identity_stewardship_available() =>
        {
            Some(UiAction::RunIdentityStewardship)
        }
        _ => None,
    }
}

pub(super) fn handle_plot_click(
    campaign: &CampaignState,
    data: &GameData,
    camera: &mut WorldCamera,
    hovered: Option<[i32; 2]>,
    suppress_click: bool,
    actions: &mut Vec<UiAction>,
) {
    if suppress_click || !is_mouse_button_released(MouseButton::Left) {
        return;
    }
    let Some(position) = hovered else { return };
    let Some(action) = plot_action(campaign, data, position) else {
        camera.clear_pending_colony_plot();
        return;
    };
    if !camera.confirm_colony_plot(position) {
        return;
    }
    actions.push(action);
}
