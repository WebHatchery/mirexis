//! Compact visible zoom controls for the touch-first colony camera.

use macroquad::prelude::{is_mouse_button_released, MouseButton, Rect, Vec2};

pub const CONTROL_WIDTH: f32 = 38.0;
pub const CONTROL_HEIGHT: f32 = 28.0;

pub fn zoom_factor(mouse: Vec2, origin: Vec2, allow_activation: bool) -> Option<f32> {
    if !allow_activation || !is_mouse_button_released(MouseButton::Left) {
        return None;
    }
    if Rect::new(origin.x, origin.y, CONTROL_WIDTH, CONTROL_HEIGHT).contains(mouse) {
        Some(1.0 / 1.25)
    } else if Rect::new(origin.x + 42.0, origin.y, CONTROL_WIDTH, CONTROL_HEIGHT).contains(mouse) {
        Some(1.25)
    } else {
        None
    }
}
