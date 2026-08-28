//! Visible touch controls for colony walking and construction mode.

use crate::campaign::CampaignState;
use crate::colony::BuildingKind;
use crate::colony_exploration::ColonyExplorer;
use crate::ui::UiAction;
use crate::ui_widgets::button;
use macroquad::prelude::{Rect, Vec2};

pub(super) fn draw_build_controls(
    campaign: &CampaignState,
    mouse: Vec2,
    actions: &mut Vec<UiAction>,
) {
    let mut kinds = vec![BuildingKind::Barricade, BuildingKind::PowerPlant];
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
    for (index, kind) in kinds.into_iter().enumerate() {
        let selected = campaign.colony.planned_construction == kind;
        if button(
            Rect::new(20.0 + index as f32 * 170.0, 620.0, 164.0, 42.0),
            &format!(
                "{}{} // {} MAT",
                if selected { "> " } else { "" },
                kind.name().to_uppercase(),
                kind.material_cost()
            ),
            true,
            mouse,
        ) {
            actions.push(UiAction::SelectConstruction(kind));
        }
    }
}

pub(super) fn draw_exploration_controls(
    campaign: &CampaignState,
    explorer: &mut ColonyExplorer,
    mouse: Vec2,
    actions: &mut Vec<UiAction>,
) {
    if explorer.build_mode() {
        if button(
            Rect::new(540.0, 620.0, 196.0, 42.0),
            "RETURN TO EXPLORE",
            true,
            mouse,
        ) {
            explorer.set_build_mode(false);
        }
        return;
    }
    for (index, (label, delta)) in [
        ("NW", [-1, 0]),
        ("NE", [0, -1]),
        ("SW", [0, 1]),
        ("SE", [1, 0]),
    ]
    .into_iter()
    .enumerate()
    {
        if button(
            Rect::new(20.0 + index as f32 * 46.0, 620.0, 42.0, 42.0),
            label,
            true,
            mouse,
        ) {
            explorer.request_step(delta, &campaign.colony);
        }
    }
    if button(
        Rect::new(210.0, 620.0, 112.0, 42.0),
        "TALK",
        explorer.can_talk(campaign),
        mouse,
    ) {
        explorer.interact(campaign);
    }
    if button(
        Rect::new(332.0, 620.0, 150.0, 42.0),
        "BUILD MODE",
        true,
        mouse,
    ) {
        explorer.set_build_mode(true);
        actions.clear();
    }
}
