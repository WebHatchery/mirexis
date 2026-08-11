//! Compact visible controls for touch-first tactical and colony cameras.

use crate::grid_ui::WorldCamera;
use crate::ui_widgets::button;
use macroquad::prelude::{vec2, Rect, Vec2};

pub(crate) const CONTROL_WIDTH: f32 = 40.0;
pub(crate) const CONTROL_HEIGHT: f32 = 28.0;
pub(crate) const STRIP_WIDTH: f32 = 256.0;

pub(crate) fn draw(
    camera: &mut WorldCamera,
    viewport: Rect,
    mouse: Vec2,
    origin: Vec2,
    allow_activation: bool,
) -> bool {
    let mut activated = false;
    let controls = [
        ("<", vec2(-1.0, 0.0)),
        ("^", vec2(0.0, -1.0)),
        ("v", vec2(0.0, 1.0)),
        (">", vec2(1.0, 0.0)),
    ];
    for (index, (label, direction)) in controls.into_iter().enumerate() {
        if button(
            Rect::new(
                origin.x + index as f32 * 42.0,
                origin.y,
                CONTROL_WIDTH,
                CONTROL_HEIGHT,
            ),
            label,
            true,
            mouse,
        ) && allow_activation
        {
            camera.nudge(direction);
            activated = true;
        }
    }
    activated
        | draw_zoom(
            camera,
            viewport,
            mouse,
            vec2(origin.x + 174.0, origin.y),
            allow_activation,
        )
}

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
