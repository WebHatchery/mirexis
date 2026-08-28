//! Colony identity header and its touch-visible title recovery control.

use crate::campaign::CampaignState;
use crate::ui::{UiAction, LOGICAL_WIDTH};
use crate::ui_widgets::button;
use macroquad::prelude::*;
use macroquad_toolkit::prelude::{dark, draw_surface, SurfaceStyle, TextStyle};

#[cfg(test)]
mod tests;

fn title_bounds() -> Rect {
    Rect::new(1160.0, 22.0, 88.0, 28.0)
}

fn operations_bounds() -> Rect {
    Rect::new(1028.0, 22.0, 124.0, 28.0)
}

fn settings_bounds() -> Rect {
    Rect::new(924.0, 22.0, 96.0, 28.0)
}

pub(crate) fn draw(
    campaign: &CampaignState,
    mouse: Vec2,
    operations_open: &mut bool,
    actions: &mut Vec<UiAction>,
) {
    draw_surface(
        Rect::new(10.0, 10.0, LOGICAL_WIDTH - 20.0, 52.0),
        &SurfaceStyle::new(Color::new(0.045, 0.075, 0.085, 0.98))
            .with_border(1.0, Color::new(0.22, 0.62, 0.56, 0.8))
            .with_left_accent(5.0, Color::new(0.34, 0.86, 0.68, 1.0)),
    );
    text(
        "MIREXIS COLONY",
        32.0,
        44.0,
        TextStyle::new(24.0, dark::TEXT_BRIGHT).params(),
    );
    text(
        &campaign.strategy.phase_name,
        224.0,
        40.0,
        TextStyle::new(15.0, dark::ACCENT).params(),
    );
    let resources = &campaign.colony.resources;
    text(
        &format!(
            "MAT {}  //  POWER {}/{}  //  FOOD {}  //  BIOMASS {}  //  XENO {}",
            resources.materials,
            campaign.colony.power_supply(),
            campaign.colony.power_demand(),
            resources.food,
            resources.biomass,
            resources.alien_components
        ),
        448.0,
        40.0,
        TextStyle::new(12.0, dark::TEXT_DIM).params(),
    );
    if button(settings_bounds(), "SETTINGS", true, mouse) {
        actions.push(UiAction::ToggleSettings);
    }
    if button(
        operations_bounds(),
        if *operations_open {
            "CLOSE"
        } else {
            "OPERATIONS"
        },
        true,
        mouse,
    ) {
        *operations_open = !*operations_open;
    }
    if button(title_bounds(), "TITLE", true, mouse) {
        actions.push(UiAction::ReturnToTitle);
    }
}

fn text<'a>(value: &str, x: f32, y: f32, mut params: TextParams<'a>) -> TextDimensions {
    params.font = None;
    draw_text_ex(value, x, y, params)
}
