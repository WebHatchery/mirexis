use super::ColonyDrawContext;
use crate::ui::{UiAction, LOGICAL_HEIGHT, LOGICAL_WIDTH};
use macroquad::prelude::*;

pub(crate) fn draw_colony(context: ColonyDrawContext<'_>) -> Vec<UiAction> {
    let ColonyDrawContext {
        campaign,
        data,
        assets,
        visuals,
        ui,
        camera,
        explorer,
        operations_open,
        facility_upgrade_open,
        salvage_open,
    } = context;
    let mut actions = Vec::new();
    let mouse = crate::ui::pointer_position(ui);
    draw_rectangle(
        0.0,
        0.0,
        LOGICAL_WIDTH,
        LOGICAL_HEIGHT,
        Color::new(0.025, 0.04, 0.055, 1.0),
    );
    let suppress_actions = crate::colony_map_ui::draw(crate::colony_map_ui::ColonyMapContext {
        campaign,
        data,
        assets,
        visuals,
        ui,
        camera,
        explorer,
        mouse,
        operations_open: *operations_open,
        actions: &mut actions,
    });
    crate::colony_header_ui::draw(campaign, mouse, operations_open, &mut actions);
    if *operations_open {
        super::draw_operations(
            campaign,
            data,
            assets,
            visuals,
            mouse,
            facility_upgrade_open,
            salvage_open,
            &mut actions,
        );
    }
    crate::ui::suppress_map_release_actions(&mut actions, suppress_actions);
    actions
}
