//! Three-quarter tactical projection shared by rendering and inverse hit testing.

use macroquad::prelude::{vec2, Rect, Vec2};
use macroquad_toolkit::grid::TilePos;

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
    pub(crate) fn new(width: usize, height: usize, rect: Rect) -> Self {
        let diagonal = (width + height).max(1) as f32;
        let half_width_from_width = (rect.w - 18.0).max(80.0) / diagonal;
        let half_height_from_height = (rect.h - 52.0).max(60.0) / (diagonal + 3.0);
        let half_width = half_width_from_width
            .min(half_height_from_height * 2.0)
            .floor();
        let half_height = (half_width * 0.5).floor();
        let elevation_step = (half_height * 0.82).max(8.0);
        Self {
            origin: vec2(
                rect.x + rect.w * 0.5,
                rect.y + 10.0 + elevation_step * 2.0 + half_height,
            ),
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
}
