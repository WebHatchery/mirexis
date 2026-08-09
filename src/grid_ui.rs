//! Tactical grid viewport geometry shared by map presentation and input hit-testing.

use macroquad::prelude::{vec2, Rect, Vec2};
use macroquad_toolkit::grid::TilePos;

#[derive(Debug, Clone, Copy)]
pub(crate) struct GridView {
    origin: Vec2,
    tile_size: f32,
    width: usize,
    height: usize,
}

impl GridView {
    pub(crate) fn new(width: usize, height: usize, rect: Rect) -> Self {
        let tile_size = (rect.w / width as f32).min(rect.h / height as f32).floor();
        Self {
            origin: vec2(
                rect.x + (rect.w - width as f32 * tile_size) * 0.5,
                rect.y + (rect.h - height as f32 * tile_size) * 0.5,
            ),
            tile_size,
            width,
            height,
        }
    }

    pub(crate) fn tile_rect(self, tile: TilePos) -> Rect {
        Rect::new(
            self.origin.x + tile.x as f32 * self.tile_size,
            self.origin.y + tile.y as f32 * self.tile_size,
            self.tile_size,
            self.tile_size,
        )
    }

    pub(crate) fn tile_at(self, point: Vec2) -> Option<TilePos> {
        let tile = TilePos::new(
            ((point.x - self.origin.x) / self.tile_size).floor() as i32,
            ((point.y - self.origin.y) / self.tile_size).floor() as i32,
        );
        (tile.x >= 0 && tile.y >= 0 && tile.x < self.width as i32 && tile.y < self.height as i32)
            .then_some(tile)
    }
}
