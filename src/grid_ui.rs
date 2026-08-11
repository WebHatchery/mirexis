//! Three-quarter tactical projection shared by rendering and inverse hit testing.

use macroquad::prelude::{is_mouse_button_down, mouse_wheel, vec2, MouseButton, Rect, Vec2};
use macroquad_toolkit::grid::TilePos;

pub(crate) const TACTICAL_HALF_WIDTH: f32 = 24.0;
pub(crate) const TACTICAL_HALF_HEIGHT: f32 = 12.0;
pub(crate) const TERRAIN_ART_SCALE: f32 = 1.70;
pub(crate) const TERRAIN_ART_PIVOT: [f32; 2] = [0.50, 0.46];
pub(crate) const STRUCTURE_ART_SCALE: f32 = 1.78;
pub(crate) const STRUCTURE_ART_PIVOT: [f32; 2] = [0.50, 0.64];
pub(crate) const CANOPY_ART_SCALE: f32 = 2.10;
pub(crate) const CANOPY_ART_PIVOT: [f32; 2] = [0.50, 0.70];

#[derive(Debug, Clone, Copy)]
pub(crate) struct WorldCamera {
    pub(crate) center: Vec2,
    pub(crate) zoom: f32,
    drag_anchor: Option<Vec2>,
    tracked_tile: Option<TilePos>,
}

impl WorldCamera {
    pub(crate) fn tactical_start(tile: TilePos) -> Self {
        Self {
            center: projected_tile(tile, TACTICAL_HALF_WIDTH, TACTICAL_HALF_HEIGHT),
            zoom: 1.0,
            drag_anchor: None,
            tracked_tile: Some(tile),
        }
    }

    pub(crate) fn tactical_view(center_tile: TilePos, tracked_tile: TilePos, zoom: f32) -> Self {
        Self {
            center: projected_tile(center_tile, TACTICAL_HALF_WIDTH, TACTICAL_HALF_HEIGHT),
            zoom: zoom.clamp(0.65, 1.85),
            drag_anchor: None,
            tracked_tile: Some(tracked_tile),
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
            tracked_tile: None,
        }
    }

    pub(crate) fn update(&mut self, viewport: Rect, mouse: Vec2) {
        let inside = viewport.contains(mouse);
        let dragging = inside
            && (is_mouse_button_down(MouseButton::Middle)
                || is_mouse_button_down(MouseButton::Right));
        if dragging {
            if let Some(previous) = self.drag_anchor {
                self.pan_screen(mouse - previous);
            }
            self.drag_anchor = Some(mouse);
        } else {
            self.drag_anchor = None;
        }

        let wheel = mouse_wheel().1;
        if inside && wheel.abs() > f32::EPSILON {
            self.zoom_at(viewport, mouse, 1.13_f32.powf(wheel));
        }
    }

    fn pan_screen(&mut self, delta: Vec2) {
        self.center -= delta / self.zoom;
    }

    fn zoom_at(&mut self, viewport: Rect, cursor: Vec2, factor: f32) {
        let old_zoom = self.zoom;
        self.zoom = (self.zoom * factor).clamp(0.65, 1.85);
        let from_center = cursor - viewport.center();
        self.center += from_center / old_zoom - from_center / self.zoom;
    }

    pub(crate) fn reveal_changed_tactical_selection(&mut self, tile: TilePos, viewport: Rect) {
        if self.tracked_tile == Some(tile) {
            return;
        }
        self.tracked_tile = Some(tile);
        let projected = projected_tile(tile, TACTICAL_HALF_WIDTH, TACTICAL_HALF_HEIGHT);
        let screen = viewport.center() + (projected - self.center) * self.zoom;
        let safe = Rect::new(
            viewport.x + 96.0,
            viewport.y + 76.0,
            (viewport.w - 192.0).max(1.0),
            (viewport.h - 152.0).max(1.0),
        );
        if !safe.contains(screen) {
            self.center = projected;
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

    pub(crate) fn ground_anchor(self, tile: TilePos) -> Vec2 {
        self.tile_center(tile)
    }

    pub(crate) fn art_bounds(self, tile: TilePos, scale: f32, pivot: [f32; 2]) -> Rect {
        let anchor = self.ground_anchor(tile);
        let size = self.half_width * 2.0 * scale;
        Rect::new(
            anchor.x - size * pivot[0],
            anchor.y - size * pivot[1],
            size,
            size,
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

    pub(crate) fn elevation(self, tile: TilePos) -> i8 {
        if inside_region(tile, 13, 12, 3, 2) || inside_region(tile, 29, 17, 3, 3) {
            2
        } else if inside_region(tile, 13, 12, 7, 5) || inside_region(tile, 29, 17, 6, 5) {
            1
        } else if inside_region(tile, 20, 29, 7, 5) || inside_region(tile, 7, 31, 4, 4) {
            -1
        } else {
            0
        }
    }

    pub(crate) fn elevation_step(self) -> f32 {
        self.elevation_step
    }

    pub(crate) fn cliff_drop(self, tile: TilePos, neighbor: TilePos) -> u8 {
        if neighbor.x < 0
            || neighbor.y < 0
            || neighbor.x >= self.width as i32
            || neighbor.y >= self.height as i32
        {
            return 0;
        }
        self.elevation(tile)
            .saturating_sub(self.elevation(neighbor))
            .max(0) as u8
    }

    pub(crate) fn is_visible(self, tile: TilePos, viewport: Rect, margin: f32) -> bool {
        let rect = self.tile_rect(tile);
        rect.right() >= viewport.x - margin
            && rect.x <= viewport.right() + margin
            && rect.bottom() >= viewport.y - margin
            && rect.y <= viewport.bottom() + margin
    }

    pub(crate) fn terrain_is_visible(self, tile: TilePos, viewport: Rect, margin: f32) -> bool {
        let bounds = self.art_bounds(tile, STRUCTURE_ART_SCALE, STRUCTURE_ART_PIVOT);
        bounds.right() >= viewport.x - margin
            && bounds.x <= viewport.right() + margin
            && bounds.bottom() >= viewport.y - margin
            && bounds.y <= viewport.bottom() + margin
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

fn inside_region(tile: TilePos, cx: i32, cy: i32, rx: i32, ry: i32) -> bool {
    let dx = tile.x - cx;
    let dy = tile.y - cy;
    dx * dx * ry * ry + dy * dy * rx * rx <= rx * rx * ry * ry
}

#[cfg(test)]
mod tests;
