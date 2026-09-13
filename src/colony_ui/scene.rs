//! Colony scene composition: map, overlays, and returned UI intents.

use super::{ColonyDrawContext, ColonyDrawResult};
use crate::ui::{LOGICAL_HEIGHT, LOGICAL_WIDTH};
use macroquad::prelude::*;

pub fn draw_colony(context: ColonyDrawContext<'_>) -> ColonyDrawResult {
    let ColonyDrawContext {
        campaign,
        data,
        assets,
        visuals,
        ui,
        mut camera,
        mut explorer,
        operations_open,
        mut facility_upgrade_open,
        mut salvage_open,
        settings_open,
        field_notes_open,
        memorial_open,
        memorial_page,
        selected_field_note,
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
    let interaction_enabled = colony_map_input_enabled(
        campaign.first_hour.help_open,
        settings_open,
        field_notes_open,
        memorial_open,
        facility_upgrade_open,
        salvage_open,
    );
    let suppress_actions = crate::colony_map_ui::draw(crate::colony_map_ui::ColonyMapContext {
        campaign,
        data,
        assets,
        visuals,
        ui,
        camera: &mut camera,
        explorer: &mut explorer,
        mouse,
        operations_open,
        interaction_enabled,
        actions: &mut actions,
    });
    crate::colony_header_ui::draw(
        campaign,
        mouse,
        operations_open,
        interaction_enabled,
        &mut actions,
    );
    if operations_open {
        super::draw_operations(
            campaign,
            data,
            assets,
            visuals,
            mouse,
            &mut facility_upgrade_open,
            &mut salvage_open,
            &mut actions,
        );
    }
    if !facility_upgrade_open && !salvage_open && !field_notes_open && !memorial_open {
        crate::first_hour_colony_ui::draw_focus(
            &campaign.first_hour,
            operations_open,
            explorer.talking_to(),
        );
    }
    crate::ui::suppress_map_release_actions(&mut actions, suppress_actions);
    if field_notes_open {
        crate::field_notes_ui::draw_modal(
            &campaign.colony_story,
            selected_field_note,
            mouse,
            &mut actions,
        );
    } else if memorial_open {
        crate::memorial_ui::draw_modal(campaign, memorial_page, mouse, &mut actions);
    }
    ColonyDrawResult {
        actions,
        camera,
        explorer,
        operations_open,
        facility_upgrade_open,
        salvage_open,
    }
}

pub fn colony_map_input_enabled(
    first_hour_help_open: bool,
    settings_open: bool,
    field_notes_open: bool,
    memorial_open: bool,
    facility_upgrade_open: bool,
    salvage_open: bool,
) -> bool {
    !first_hour_help_open
        && !settings_open
        && !field_notes_open
        && !memorial_open
        && !facility_upgrade_open
        && !salvage_open
}
