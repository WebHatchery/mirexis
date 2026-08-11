//! Colony projection, complete plot bounds, and camera-edge art insets.

use super::{COLONY_HALF_HEIGHT, COLONY_HALF_WIDTH};
use crate::grid_ui::{CameraInsets, WorldCamera};
use macroquad::prelude::{vec2, Rect, Vec2};

#[derive(Clone, Copy)]
pub(super) struct ColonyView {
    pub(super) viewport: Rect,
    origin: Vec2,
    pub(super) half_width: f32,
    pub(super) half_height: f32,
    pub(super) zoom: f32,
}

impl ColonyView {
    pub(super) fn new(viewport: Rect, camera: &WorldCamera) -> Self {
        Self {
            viewport,
            origin: viewport.center() - camera.center * camera.zoom,
            half_width: COLONY_HALF_WIDTH * camera.zoom,
            half_height: COLONY_HALF_HEIGHT * camera.zoom,
            zoom: camera.zoom,
        }
    }

    pub(super) fn plot_center(self, position: [i32; 2]) -> Vec2 {
        vec2(
            self.origin.x + (position[0] - position[1]) as f32 * self.half_width,
            self.origin.y + (position[0] + position[1]) as f32 * self.half_height,
        )
    }

    pub(super) fn plot_render_bounds(self, position: [i32; 2]) -> Rect {
        let center = self.plot_center(position);
        let horizontal = self.half_width.max(35.0 * self.zoom).max(36.0);
        let above = self.half_height.max(52.0 * self.zoom).max(61.0);
        let below = (self.half_height + 7.0).max(18.0 * self.zoom);
        Rect::new(
            center.x - horizontal,
            center.y - above,
            horizontal * 2.0,
            above + below,
        )
    }

    pub(super) fn visible(self, position: [i32; 2]) -> bool {
        let bounds = self.plot_render_bounds(position);
        bounds.right() >= self.viewport.x
            && bounds.x <= self.viewport.right()
            && bounds.bottom() >= self.viewport.y
            && bounds.y <= self.viewport.bottom()
    }

    pub(super) fn camera_insets(zoom: f32) -> CameraInsets {
        let half_width = COLONY_HALF_WIDTH * zoom;
        let half_height = COLONY_HALF_HEIGHT * zoom;
        let horizontal = half_width.max(35.0 * zoom).max(36.0);
        let above = half_height.max(52.0 * zoom).max(61.0);
        let below = (half_height + 7.0).max(18.0 * zoom);
        CameraInsets {
            left: horizontal - half_width,
            top: above - half_height,
            right: horizontal - half_width,
            bottom: below - half_height,
        }
    }
}
