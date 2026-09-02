//! Three-quarter tactical projection shared by rendering and inverse hit testing.

use macroquad::prelude::{
    is_mouse_button_down, is_mouse_button_pressed, is_mouse_button_released, mouse_wheel, vec2,
    MouseButton, Rect, Vec2,
};
use macroquad_toolkit::grid::TilePos;

pub(crate) const TACTICAL_HALF_WIDTH: f32 = 24.0;
pub(crate) const TACTICAL_HALF_HEIGHT: f32 = 12.0;
pub(crate) const TERRAIN_ART_SCALE: f32 = 1.06;
pub(crate) const TERRAIN_ART_PIVOT: [f32; 2] = [0.50, 0.58];
pub(crate) const STRUCTURE_ART_SCALE: f32 = 1.10;
pub(crate) const STRUCTURE_ART_PIVOT: [f32; 2] = [0.50, 0.68];
pub(crate) const CANOPY_ART_SCALE: f32 = 1.15;
pub(crate) const CANOPY_ART_PIVOT: [f32; 2] = [0.50, 0.65];
// Terrain and concept atlases are authored as square cells. Preserve that
// source aspect so their isometric diamonds are not stretched vertically.
pub(crate) const TERRAIN_ART_ASPECT: f32 = 1.0;

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct CameraInsets {
    pub(crate) left: f32,
    pub(crate) top: f32,
    pub(crate) right: f32,
    pub(crate) bottom: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct WorldCamera {
    pub(crate) center: Vec2,
    pub(crate) zoom: f32,
    drag_anchor: Option<Vec2>,
    primary_drag_start: Option<Vec2>,
    primary_drag_anchor: Option<Vec2>,
    primary_dragged: bool,
    primary_release_pending: bool,
    pending_colony_plot: Option<[i32; 2]>,
    tracked_tile: Option<TilePos>,
}

impl WorldCamera {
    pub(crate) fn tactical_start(tile: TilePos) -> Self {
        Self {
            center: projected_tile(tile, TACTICAL_HALF_WIDTH, TACTICAL_HALF_HEIGHT),
            zoom: 1.0,
            drag_anchor: None,
            primary_drag_start: None,
            primary_drag_anchor: None,
            primary_dragged: false,
            primary_release_pending: false,
            pending_colony_plot: None,
            tracked_tile: Some(tile),
        }
    }

    pub(crate) fn tactical_view(center_tile: TilePos, tracked_tile: TilePos, zoom: f32) -> Self {
        Self {
            center: projected_tile(center_tile, TACTICAL_HALF_WIDTH, TACTICAL_HALF_HEIGHT),
            zoom: zoom.clamp(0.65, 1.85),
            drag_anchor: None,
            primary_drag_start: None,
            primary_drag_anchor: None,
            primary_dragged: false,
            primary_release_pending: false,
            pending_colony_plot: None,
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
            primary_drag_start: None,
            primary_drag_anchor: None,
            primary_dragged: false,
            primary_release_pending: false,
            pending_colony_plot: None,
            tracked_tile: None,
        }
    }

    pub(crate) fn update(&mut self, viewport: Rect, mouse: Vec2) -> bool {
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

        let primary_down = is_mouse_button_down(MouseButton::Left);
        let primary_pressed = is_mouse_button_pressed(MouseButton::Left);
        self.begin_primary_press(inside, primary_pressed);
        let track_primary = primary_tracking(
            self.primary_drag_start.is_some(),
            inside,
            primary_pressed,
            primary_down,
        );
        let suppress_primary_click = self.update_primary_drag(
            track_primary && !dragging,
            is_mouse_button_released(MouseButton::Left),
            mouse,
        );

        let wheel = mouse_wheel().1;
        if inside && wheel.abs() > f32::EPSILON {
            self.zoom_at(viewport, mouse, 1.13_f32.powf(wheel));
        }
        suppress_primary_click
    }

    pub(crate) fn primary_gesture_active(&self) -> bool {
        self.primary_drag_start.is_some()
    }

    pub(crate) fn guard_next_primary_release(&mut self) {
        self.primary_release_pending = true;
    }

    pub(crate) fn pending_colony_plot(&self) -> Option<[i32; 2]> {
        self.pending_colony_plot
    }

    pub(crate) fn confirm_colony_plot(&mut self, position: [i32; 2]) -> bool {
        if self.pending_colony_plot == Some(position) {
            self.pending_colony_plot = None;
            true
        } else {
            self.pending_colony_plot = Some(position);
            false
        }
    }

    pub(crate) fn clear_pending_colony_plot(&mut self) {
        self.pending_colony_plot = None;
    }

    pub(crate) fn clear_pointer_interaction(&mut self) {
        self.drag_anchor = None;
        self.primary_drag_start = None;
        self.primary_drag_anchor = None;
        self.primary_dragged = false;
        self.primary_release_pending = false;
        self.pending_colony_plot = None;
    }

    fn begin_primary_press(&mut self, inside: bool, pressed: bool) {
        if inside && pressed {
            self.primary_release_pending = false;
        }
    }

    fn update_primary_drag(&mut self, down: bool, released: bool, mouse: Vec2) -> bool {
        if down {
            let start = *self.primary_drag_start.get_or_insert(mouse);
            let previous = *self.primary_drag_anchor.get_or_insert(mouse);
            if self.primary_dragged {
                self.pan_screen(mouse - previous);
            } else if (mouse - start).length() >= 7.0 {
                self.pan_screen(mouse - start);
                self.primary_dragged = true;
                self.primary_release_pending = true;
            }
            self.primary_drag_anchor = Some(mouse);
            return false;
        }

        let release_delta = self.primary_drag_start.map(|start| mouse - start);
        let crossed_threshold_on_release = released
            && !self.primary_dragged
            && release_delta.is_some_and(|delta| delta.length() >= 7.0);
        if crossed_threshold_on_release {
            self.pan_screen(release_delta.expect("threshold requires a drag origin"));
            self.primary_release_pending = true;
        }
        let suppress = released && self.primary_release_pending;
        self.primary_drag_start = None;
        self.primary_drag_anchor = None;
        self.primary_dragged = false;
        if released {
            self.primary_release_pending = false;
        }
        suppress
    }

    fn pan_screen(&mut self, delta: Vec2) {
        self.center -= delta / self.zoom;
    }

    pub(crate) fn zoom_center(&mut self, viewport: Rect, factor: f32) {
        self.zoom_at(viewport, viewport.center(), factor);
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

    #[cfg(test)]
    pub(crate) fn clamp_isometric(
        &mut self,
        width: usize,
        height: usize,
        half_width: f32,
        half_height: f32,
        viewport: Rect,
    ) {
        self.clamp_isometric_with_insets(
            width,
            height,
            half_width,
            half_height,
            viewport,
            CameraInsets::default(),
        );
    }

    pub(crate) fn clamp_isometric_with_insets(
        &mut self,
        width: usize,
        height: usize,
        half_width: f32,
        half_height: f32,
        viewport: Rect,
        insets: CameraInsets,
    ) {
        let mut min = vec2(
            -(height.saturating_sub(1) as f32) * half_width - half_width,
            -half_height,
        );
        let mut max = vec2(
            width.saturating_sub(1) as f32 * half_width + half_width,
            (width.saturating_add(height).saturating_sub(2) as f32) * half_height + half_height,
        );
        min.x -= insets.left / self.zoom;
        min.y -= insets.top / self.zoom;
        max.x += insets.right / self.zoom;
        max.y += insets.bottom / self.zoom;
        let visible_half = viewport.size() * 0.5 / self.zoom;
        self.center.x = clamp_axis(self.center.x, min.x, max.x, visible_half.x);
        self.center.y = clamp_axis(self.center.y, min.y, max.y, visible_half.y);
    }
}

fn primary_tracking(active: bool, inside: bool, pressed: bool, down: bool) -> bool {
    down && (active || (inside && pressed))
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
        let width = self.half_width * 2.0 * scale;
        let height = width * TERRAIN_ART_ASPECT;
        Rect::new(
            anchor.x - width * pivot[0],
            anchor.y - height * pivot[1],
            width,
            height,
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
        if inside_region(tile, 12, 19, 3, 2) || inside_region(tile, 29, 17, 3, 3) {
            2
        } else if inside_region(tile, 12, 19, 6, 5) || inside_region(tile, 29, 17, 6, 5) {
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
