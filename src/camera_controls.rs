//! Compact visible zoom controls for the touch-first colony camera.

use crate::grid_ui::WorldCamera;
use crate::ui_widgets::button;
use macroquad::prelude::{Rect, Vec2};

pub(crate) const CONTROL_WIDTH: f32 = 38.0;
pub(crate) const CONTROL_HEIGHT: f32 = 28.0;

pub(crate) fn draw_zoom(
    camera: &mut WorldCamera,
    viewport: Rect,
    mouse: Vec2,
    origin: Vec2,
    allow_activation: bool,
) -> bool {
    let mut activated = false;
    if button(
        Rect::new(origin.x, origin.y, CONTROL_WIDTH, CONTROL_HEIGHT),
        "-",
        true,
        mouse,
    ) && allow_activation
    {
        camera.zoom_center(viewport, 1.0 / 1.25);
        activated = true;
    }
    if button(
        Rect::new(origin.x + 42.0, origin.y, CONTROL_WIDTH, CONTROL_HEIGHT),
        "+",
        true,
        mouse,
    ) && allow_activation
    {
        camera.zoom_center(viewport, 1.25);
        activated = true;
    }
    activated
}
