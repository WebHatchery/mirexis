//! Visible touch controls for colony interaction and construction mode.

use crate::campaign::CampaignState;
use crate::colony::BuildingKind;
use crate::colony_exploration::ColonyExplorer;
use crate::data::GameData;
use crate::ui::UiAction;
use crate::ui_widgets::button;
use macroquad::prelude::{Rect, Vec2};

fn build_button_label(
    kind: BuildingKind,
    selected: bool,
    materials: i32,
    already_built: bool,
    project_queued: bool,
) -> String {
    let status = if already_built {
        "ALREADY BUILT".to_owned()
    } else if project_queued {
        "PROJECT QUEUED".to_owned()
    } else if materials < kind.material_cost() {
        format!("NEED {} MAT", kind.material_cost())
    } else {
        format!("{} MAT", kind.material_cost())
    };
    format!(
        "{}{} // {status}",
        if selected { "> " } else { "" },
        kind.name().to_uppercase()
    )
}

pub(super) fn draw_build_controls(
    campaign: &CampaignState,
    mouse: Vec2,
    actions: &mut Vec<UiAction>,
) {
    let mut kinds = vec![BuildingKind::Barricade, BuildingKind::PowerPlant];
    kinds.push(BuildingKind::Commons);
    kinds.push(BuildingKind::RelayMast);
    kinds.push(BuildingKind::Watchtower);
    kinds.push(BuildingKind::ResearchAnnex);
    kinds.push(BuildingKind::SalvageYard);
    if campaign.strategy.contact_complete
        && !campaign
            .colony
            .buildings
            .iter()
            .any(|building| building.kind == BuildingKind::GeneLab)
        && !campaign
            .colony
            .construction_queue
            .iter()
            .any(|project| project.kind == BuildingKind::GeneLab)
    {
        kinds.push(BuildingKind::GeneLab);
    }
    if campaign.waystation_unlocked()
        && !campaign
            .colony
            .buildings
            .iter()
            .any(|building| building.kind == BuildingKind::Waystation)
        && !campaign
            .colony
            .construction_queue
            .iter()
            .any(|project| project.kind == BuildingKind::Waystation)
    {
        kinds.push(BuildingKind::Waystation);
    }
    for (index, kind) in kinds.into_iter().enumerate() {
        let selected = campaign.colony.planned_construction == kind;
        let unique = matches!(
            kind,
            BuildingKind::GeneLab
                | BuildingKind::ResearchAnnex
                | BuildingKind::SalvageYard
                | BuildingKind::Waystation
                | BuildingKind::Commons
                | BuildingKind::RelayMast
                | BuildingKind::Watchtower
        );
        let already_built = unique
            && campaign
                .colony
                .buildings
                .iter()
                .any(|building| building.kind == kind);
        let project_queued = unique
            && campaign
                .colony
                .construction_queue
                .iter()
                .any(|project| project.kind == kind);
        let column = index % 5;
        let row = index / 5;
        if button(
            Rect::new(
                20.0 + column as f32 * 162.0,
                606.0 + row as f32 * 48.0,
                158.0,
                42.0,
            ),
            &build_button_label(
                kind,
                selected,
                campaign.colony.resources.materials,
                already_built,
                project_queued,
            ),
            campaign.construction_available(kind),
            mouse,
        ) {
            actions.push(UiAction::SelectConstruction(kind));
        }
    }
}

#[cfg(test)]
mod tests;

pub(super) fn draw_exploration_controls(
    campaign: &CampaignState,
    data: &GameData,
    explorer: &mut ColonyExplorer,
    mouse: Vec2,
    actions: &mut Vec<UiAction>,
) {
    explorer.set_touch_direction(Vec2::ZERO);
    if explorer.build_mode() {
        if button(
            Rect::new(668.0, 654.0, 158.0, 42.0),
            "RETURN TO EXPLORE",
            true,
            mouse,
        ) {
            explorer.set_build_mode(false);
        }
        return;
    }
    if button(
        Rect::new(210.0, 654.0, 112.0, 42.0),
        "TALK",
        explorer.can_talk(campaign, data),
        mouse,
    ) {
        explorer.interact(campaign, data);
    }
    if campaign.first_hour.stage == crate::first_hour::FirstHourStage::Complete
        && button(
            Rect::new(332.0, 654.0, 150.0, 42.0),
            "BUILD MODE",
            true,
            mouse,
        )
    {
        explorer.set_build_mode(true);
        actions.clear();
    }
}
