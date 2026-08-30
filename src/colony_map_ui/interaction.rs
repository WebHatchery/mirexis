//! Hover copy and touch-safe plot actions for the interactive colony map.

use crate::campaign::CampaignState;
use crate::colony::BuildingKind;
use crate::grid_ui::WorldCamera;
use crate::ui::UiAction;
use macroquad::prelude::*;

pub(super) fn draw_hover_card(
    campaign: &CampaignState,
    hovered: Option<[i32; 2]>,
    pending: Option<[i32; 2]>,
) {
    let Some(position) = hovered else { return };
    let building = campaign.colony.building_at(position);
    let project = campaign.colony.project_at(position);
    let site_unavailable = building.is_none()
        && project.is_none()
        && campaign
            .colony
            .validate_construction_site(position)
            .is_err();
    let text = if let Some(building) = building {
        if building.damaged {
            format!(
                "{} // DAMAGED // REPAIR {} MAT",
                building.kind.name().to_uppercase(),
                building.kind.repair_cost()
            )
        } else if !campaign.colony.building_is_powered(&building.id) {
            format!(
                "{} // OFFLINE: INSUFFICIENT POWER",
                building.kind.name().to_uppercase()
            )
        } else if building.kind == BuildingKind::GeneLab {
            "GENE LAB // TAP TO OPEN EVOLUTION CHAMBER".to_owned()
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
            if !campaign
                .roster
                .iter()
                .any(|character| character.id == "veya_orn")
            {
                "WAYSTATION // TAP TO REVIEW DIRECTORATE EXILE".to_owned()
            } else {
                "WAYSTATION // CONTACT ROUTE STABLE // ONLINE".to_owned()
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
            Ok(()) => format!(
                "OPEN PLOT // BUILD {} // {} MAT",
                campaign.colony.planned_construction.name().to_uppercase(),
                campaign.colony.planned_construction.material_cost()
            ),
            Err(error) => format!("CLEARANCE REQUIRED // {}", error.to_uppercase()),
        }
    };
    let text = if site_unavailable {
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

pub(super) fn handle_plot_click(
    campaign: &CampaignState,
    camera: &mut WorldCamera,
    hovered: Option<[i32; 2]>,
    suppress_click: bool,
    actions: &mut Vec<UiAction>,
) {
    if suppress_click || !is_mouse_button_released(MouseButton::Left) {
        return;
    }
    let Some(position) = hovered else { return };
    if campaign.colony.building_at(position).is_none()
        && campaign.colony.project_at(position).is_none()
        && campaign
            .colony
            .validate_construction_site(position)
            .is_err()
    {
        camera.clear_pending_colony_plot();
        return;
    }
    if !camera.confirm_colony_plot(position) {
        return;
    }
    if let Some(building) = campaign.colony.building_at(position) {
        if building.damaged {
            actions.push(UiAction::RepairBuilding(building.id.clone()));
        } else if building.kind == BuildingKind::GeneLab
            && campaign.colony.building_is_powered(&building.id)
        {
            actions.push(UiAction::OpenGeneLab);
        } else if building.kind == BuildingKind::Waystation
            && campaign.colony.building_is_powered(&building.id)
            && !campaign
                .roster
                .iter()
                .any(|character| character.id == "veya_orn")
        {
            actions.push(UiAction::RecruitOutsider);
        } else if building.kind == BuildingKind::Commons
            && campaign.colony.building_is_powered(&building.id)
            && campaign.commons_meal_available()
        {
            actions.push(UiAction::HostCommonsMeal);
        } else if building.kind == BuildingKind::RelayMast
            && campaign.colony.building_is_powered(&building.id)
            && campaign.relay_scan_available()
        {
            actions.push(UiAction::RunRelayScan);
        }
    } else if campaign.colony.project_at(position).is_none() {
        actions.push(UiAction::ConstructBuilding(
            campaign.colony.planned_construction,
            position,
        ));
    }
}
