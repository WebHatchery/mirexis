//! Three-quarter tactical projection shared by rendering and inverse hit testing.

use macroquad::prelude::{is_mouse_button_down, mouse_wheel, vec2, MouseButton, Rect, Vec2};
use macroquad_toolkit::grid::TilePos;

pub(crate) const TACTICAL_HALF_WIDTH: f32 = 24.0;
pub(crate) const TACTICAL_HALF_HEIGHT: f32 = 12.0;

#[derive(Debug, Clone, Copy)]
pub(crate) struct WorldCamera {
    pub(crate) center: Vec2,
    pub(crate) zoom: f32,
    drag_anchor: Option<Vec2>,
}

impl WorldCamera {
    pub(crate) fn tactical_start(tile: TilePos) -> Self {
        Self {
            center: projected_tile(tile, TACTICAL_HALF_WIDTH, TACTICAL_HALF_HEIGHT),
            zoom: 1.0,
            drag_anchor: None,
        }
    }

    pub(crate) fn colony_start(position: [i32; 2]) -> Self {
        Self {
            center: projected_tile(
                TilePos::new(position[0], position[1]),
                crate::colony_map_ui::COLONY_HALF_WIDTH,
                crate::colony_map_ui::COLONY_HALF_HEIGHT,
            ),
            zoom: 1.0,
            drag_anchor: None,
        }
    }

    pub(crate) fn update(&mut self, viewport: Rect, mouse: Vec2) {
        let inside = viewport.contains(mouse);
        let dragging = inside
            && (is_mouse_button_down(MouseButton::Middle)
                || is_mouse_button_down(MouseButton::Right));
        if dragging {
            if let Some(previous) = self.drag_anchor {
                self.center -= (mouse - previous) / self.zoom;
            }
            self.drag_anchor = Some(mouse);
        } else {
            self.drag_anchor = None;
        }

        let wheel = mouse_wheel().1;
        if inside && wheel.abs() > f32::EPSILON {
            let old_zoom = self.zoom;
            self.zoom = (self.zoom * 1.13_f32.powf(wheel)).clamp(0.65, 1.85);
            let from_center = mouse - viewport.center();
            self.center += from_center / old_zoom - from_center / self.zoom;
        }
    }

    pub(crate) fn clamp_isometric(
        &mut self,
        width: usize,
        height: usize,
        half_width: f32,
        half_height: f32,
        viewport: Rect,
    ) {
        let min = vec2(-(height.saturating_sub(1) as f32) * half_width, 0.0);
        let max = vec2(
            width.saturating_sub(1) as f32 * half_width,
            (width.saturating_add(height).saturating_sub(2) as f32) * half_height,
        );
        let visible_half = viewport.size() * 0.5 / self.zoom;
        self.center.x = clamp_axis(self.center.x, min.x, max.x, visible_half.x);
        self.center.y = clamp_axis(self.center.y, min.y, max.y, visible_half.y);
    }
}

fn clamp_axis(value: f32, min: f32, max: f32, visible_half: f32) -> f32 {
    let lower = min + visible_half;
    let upper = max - visible_half;
    if lower <= upper {
        value.clamp(lower, upper)
    } else {
        (min + max) * 0.5
    }
}

fn projected_tile(tile: TilePos, half_width: f32, half_height: f32) -> Vec2 {
    vec2(
        (tile.x - tile.y) as f32 * half_width,
        (tile.x + tile.y) as f32 * half_height,
    )
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct GridView {
    origin: Vec2,
    half_width: f32,
    half_height: f32,
    elevation_step: f32,
    width: usize,
    height: usize,
}

impl GridView {
    #[cfg(test)]
    pub(crate) fn new(width: usize, height: usize, rect: Rect) -> Self {
        Self::with_camera(
            width,
            height,
            rect,
            &WorldCamera::tactical_start(TilePos::new(
                width.saturating_sub(1) as i32 / 2,
                height.saturating_sub(1) as i32 / 2,
            )),
        )
    }

    pub(crate) fn with_camera(
        width: usize,
        height: usize,
        rect: Rect,
        camera: &WorldCamera,
    ) -> Self {
        let half_width = TACTICAL_HALF_WIDTH * camera.zoom;
        let half_height = TACTICAL_HALF_HEIGHT * camera.zoom;
        let elevation_step = (half_height * 0.82).max(8.0);
        Self {
            origin: rect.center() - camera.center * camera.zoom,
            half_width,
            half_height,
            elevation_step,
            width,
            height,
        }
    }

    pub(crate) fn tile_center(self, tile: TilePos) -> Vec2 {
        let elevation = self.elevation(tile) as f32;
        vec2(
            self.origin.x + (tile.x - tile.y) as f32 * self.half_width,
            self.origin.y + (tile.x + tile.y) as f32 * self.half_height
                - elevation * self.elevation_step,
        )
    }

    pub(crate) fn tile_rect(self, tile: TilePos) -> Rect {
        let center = self.tile_center(tile);
        Rect::new(
            center.x - self.half_width,
            center.y - self.half_height,
            self.half_width * 2.0,
            self.half_height * 2.0,
        )
    }

    pub(crate) fn diamond(self, tile: TilePos) -> [Vec2; 4] {
        let center = self.tile_center(tile);
        [
            vec2(center.x, center.y - self.half_height),
            vec2(center.x + self.half_width, center.y),
            vec2(center.x, center.y + self.half_height),
            vec2(center.x - self.half_width, center.y),
        ]
    }

    pub(crate) fn base_diamond(self, tile: TilePos) -> [Vec2; 4] {
        let mut points = self.diamond(tile);
        let offset = self.elevation(tile) as f32 * self.elevation_step;
        for point in &mut points {
            point.y += offset;
        }
        points
    }

    pub(crate) fn elevation(self, tile: TilePos) -> u8 {
        if tile.x >= 8 && tile.y <= 3 {
            2
        } else if tile.x >= 5 && tile.y <= 5 {
            1
        } else {
            0
        }
    }

    pub(crate) fn elevation_step(self) -> f32 {
        self.elevation_step
    }

    pub(crate) fn is_visible(self, tile: TilePos, viewport: Rect, margin: f32) -> bool {
        let rect = self.tile_rect(tile);
        rect.right() >= viewport.x - margin
            && rect.x <= viewport.right() + margin
            && rect.bottom() >= viewport.y - margin
            && rect.y <= viewport.bottom() + margin
    }

    pub(crate) fn tile_at(self, point: Vec2) -> Option<TilePos> {
        let mut best = None;
        let mut best_depth = f32::NEG_INFINITY;
        for y in 0..self.height as i32 {
            for x in 0..self.width as i32 {
                let tile = TilePos::new(x, y);
                let center = self.tile_center(tile);
                let normalized = (point.x - center.x).abs() / self.half_width
                    + (point.y - center.y).abs() / self.half_height;
                if normalized <= 1.0 {
                    let depth = (x + y) as f32 + self.elevation(tile) as f32 * 0.1;
                    if depth >= best_depth {
                        best = Some(tile);
                        best_depth = depth;
                    }
                }
            }
        }
        best
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inverse_hit_testing_recovers_every_projected_tile_center() {
        let view = GridView::new(12, 8, Rect::new(0.0, 0.0, 780.0, 450.0));
        for y in 0..8 {
            for x in 0..12 {
                let tile = TilePos::new(x, y);
                assert_eq!(view.tile_at(view.tile_center(tile)), Some(tile));
            }
        }
    }

    #[test]
    fn inverse_hit_testing_covers_projected_interiors_in_each_height_band() {
        let view = GridView::new(12, 8, Rect::new(0.0, 0.0, 780.0, 450.0));
        let offsets = [
            vec2(0.0, 0.0),
            vec2(view.half_width * 0.32, 0.0),
            vec2(-view.half_width * 0.32, 0.0),
            vec2(0.0, view.half_height * 0.32),
            vec2(0.0, -view.half_height * 0.32),
        ];
        for tile in [TilePos::new(1, 7), TilePos::new(6, 4), TilePos::new(9, 2)] {
            for offset in offsets {
                assert_eq!(
                    view.tile_at(view.tile_center(tile) + offset),
                    Some(tile),
                    "projected interior missed tile {tile:?} at {offset:?}"
                );
            }
        }
    }

    #[test]
    fn battlefield_exposes_three_visual_height_bands() {
        let view = GridView::new(12, 8, Rect::new(0.0, 0.0, 780.0, 450.0));
        assert_eq!(view.elevation(TilePos::new(1, 7)), 0);
        assert_eq!(view.elevation(TilePos::new(6, 4)), 1);
        assert_eq!(view.elevation(TilePos::new(9, 2)), 2);
    }

    #[test]
    fn raised_foreground_tile_owns_the_visibly_occluded_transition_area() {
        let view = GridView::new(12, 8, Rect::new(0.0, 0.0, 780.0, 450.0));
        let lower = TilePos::new(4, 0);
        let visibly_covered_point = view.tile_center(lower) + vec2(view.half_width * 0.32, 0.0);
        assert_eq!(
            view.tile_at(visibly_covered_point),
            Some(TilePos::new(5, 0))
        );
    }

    #[test]
    fn tactical_tile_scale_is_independent_of_world_and_viewport_size() {
        let small = GridView::new(12, 8, Rect::new(0.0, 0.0, 780.0, 450.0));
        let large = GridView::new(50, 50, Rect::new(0.0, 0.0, 1180.0, 620.0));
        assert_eq!(small.half_width, TACTICAL_HALF_WIDTH);
        assert_eq!(large.half_width, TACTICAL_HALF_WIDTH);
        assert_eq!(small.half_height, TACTICAL_HALF_HEIGHT);
        assert_eq!(large.half_height, TACTICAL_HALF_HEIGHT);
    }

    #[test]
    fn forty_tile_world_is_cropped_at_default_zoom() {
        let viewport = Rect::new(0.0, 0.0, 884.0, 568.0);
        let mut camera = WorldCamera::tactical_start(TilePos::new(8, 5));
        camera.clamp_isometric(40, 40, TACTICAL_HALF_WIDTH, TACTICAL_HALF_HEIGHT, viewport);
        let view = GridView::with_camera(40, 40, viewport, &camera);
        let visible = (0..40)
            .flat_map(|y| (0..40).map(move |x| TilePos::new(x, y)))
            .filter(|tile| view.is_visible(*tile, viewport, 0.0))
            .count();
        assert!(
            visible < 40 * 40 / 2,
            "default view exposed {visible} tiles"
        );
    }
}
