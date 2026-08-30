//! Visible touch controls for colony walking and construction mode.

use crate::campaign::CampaignState;
use crate::colony::BuildingKind;
use crate::colony_exploration::ColonyExplorer;
use crate::data::GameData;
use crate::ui::UiAction;
use crate::ui_widgets::button;
use macroquad::prelude::{is_mouse_button_down, vec2, MouseButton, Rect, Vec2};

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
        let column = index % 4;
        let row = index / 4;
        if button(
            Rect::new(
                20.0 + column as f32 * 170.0,
                606.0 + row as f32 * 48.0,
                164.0,
                42.0,
            ),
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
    data: &GameData,
    explorer: &mut ColonyExplorer,
    mouse: Vec2,
    actions: &mut Vec<UiAction>,
) {
    explorer.set_touch_direction(Vec2::ZERO);
    if explorer.build_mode() {
        if button(
            Rect::new(540.0, 654.0, 196.0, 42.0),
            "RETURN TO EXPLORE",
            true,
            mouse,
        ) {
            explorer.set_build_mode(false);
        }
        return;
    }
    let mut movement = Vec2::ZERO;
    for (index, (label, direction)) in [
        ("NW", vec2(-1.0, 0.0)),
        ("NE", vec2(0.0, -1.0)),
        ("SW", vec2(0.0, 1.0)),
        ("SE", vec2(1.0, 0.0)),
    ]
    .into_iter()
    .enumerate()
    {
        let rect = Rect::new(20.0 + index as f32 * 46.0, 654.0, 42.0, 42.0);
        button(rect, label, true, mouse);
        if rect.contains(mouse) && is_mouse_button_down(MouseButton::Left) {
            movement += direction;
        }
    }
    explorer.set_touch_direction(movement.normalize_or_zero());
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
