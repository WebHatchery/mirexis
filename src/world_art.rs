//! Runtime use of the promoted concept sprites in the tactical world.

use crate::grid_ui::GridView;
use crate::visual_assets::VisualCatalog;
use macroquad::prelude::*;
use macroquad_toolkit::assets::AssetManager;
use macroquad_toolkit::grid::TilePos;

pub(crate) fn draw_title_dressing(assets: &AssetManager, visuals: &VisualCatalog) {
    draw_concept_cell(
        assets,
        visuals,
        "passage_wreckage",
        0,
        Rect::new(696.0, 218.0, 310.0, 310.0),
        Color::new(0.78, 0.92, 0.90, 0.18),
    );
    draw_concept_cell(
        assets,
        visuals,
        "flora",
        7,
        Rect::new(1010.0, 272.0, 220.0, 220.0),
        Color::new(0.70, 0.88, 0.84, 0.14),
    );
    draw_concept_cell(
        assets,
        visuals,
        "emplacements",
        10,
        Rect::new(760.0, 372.0, 190.0, 190.0),
        Color::new(0.82, 0.72, 1.0, 0.12),
    );
}

pub(crate) struct TacticalDressingContext<'a> {
    pub(crate) assets: &'a AssetManager,
    pub(crate) visuals: &'a VisualCatalog,
    pub(crate) view: GridView,
    pub(crate) position: TilePos,
}

pub(crate) fn draw_tactical_dressing(
    context: TacticalDressingContext<'_>,
    blocked: bool,
    occupied: bool,
    cluttered: bool,
    hostile_faction: &str,
) {
    let signature = tile_signature(context.position);
    if blocked {
        if signature.is_multiple_of(3) {
            let cell = faction_emplacement_cell(hostile_faction, signature);
            draw_tile_cell(
                &context,
                "emplacements",
                cell,
                1.04,
                [0.50, 0.84],
                Color::new(1.0, 1.0, 1.0, 0.86),
            );
        } else {
            draw_tile_cell(
                &context,
                "passage_wreckage",
                (signature % 12) as usize,
                1.00,
                [0.50, 0.84],
                Color::new(0.90, 0.94, 0.92, 0.82),
            );
        }
        return;
    }
    if occupied || cluttered {
        return;
    }

    match signature % 23 {
        0 | 1 => draw_tile_cell(
            &context,
            "flora",
            flora_cell(signature),
            0.78,
            [0.50, 0.84],
            Color::new(0.78, 0.94, 0.88, 0.52),
        ),
        2 => draw_tile_cell(
            &context,
            "brood_fauna",
            (signature % 8) as usize,
            0.84,
            [0.50, 0.86],
            Color::new(1.0, 0.90, 0.90, 0.58),
        ),
        3 | 4 => draw_tile_cell(
            &context,
            "terrain_dressing",
            terrain_dressing_cell(signature),
            0.86,
            [0.50, 0.84],
            Color::new(0.86, 0.94, 0.92, 0.42),
        ),
        _ => {}
    }
}

pub(crate) fn draw_colony_dressing(
    assets: &AssetManager,
    visuals: &VisualCatalog,
    view: crate::colony_map_ui::view::ColonyView,
    position: [i32; 2],
    occupied: bool,
    planning_clearance: bool,
) {
    if occupied || planning_clearance {
        return;
    }
    let signature = tile_signature_xy(position[0], position[1]);
    let center = view.plot_center(position);
    let tile_width = view.half_width * 2.0;
    match signature % 19 {
        0 => draw_concept_cell(
            assets,
            visuals,
            "colony_props",
            (signature % 5) as usize,
            square_bounds(center, tile_width * 1.02, [0.50, 0.84]),
            Color::new(0.88, 0.96, 0.92, 0.48),
        ),
        1 | 2 => draw_concept_cell(
            assets,
            visuals,
            "flora",
            match signature % 3 {
                0 => 1,
                1 => 8,
                _ => 11,
            },
            square_bounds(center, tile_width * 0.84, [0.50, 0.86]),
            Color::new(0.66, 0.94, 0.76, 0.42),
        ),
        3 => draw_concept_cell(
            assets,
            visuals,
            "colony_machinery",
            (signature % 8) as usize,
            square_bounds(center, tile_width * 0.86, [0.50, 0.84]),
            Color::new(0.86, 0.94, 0.92, 0.48),
        ),
        _ => {}
    }
}

fn draw_tile_cell(
    context: &TacticalDressingContext<'_>,
    atlas_id: &str,
    index: usize,
    scale: f32,
    pivot: [f32; 2],
    tint: Color,
) {
    let tile = context.view.tile_rect(context.position);
    let size = tile.w * scale;
    let center = context.view.ground_anchor(context.position);
    draw_concept_cell(
        context.assets,
        context.visuals,
        atlas_id,
        index,
        square_bounds(center, size, pivot),
        tint,
    );
}

fn draw_concept_cell(
    assets: &AssetManager,
    visuals: &VisualCatalog,
    atlas_id: &str,
    index: usize,
    destination: Rect,
    tint: Color,
) {
    visuals.draw_atlas_cell(
        assets,
        visuals.concept_atlas(atlas_id),
        index,
        destination,
        tint,
    );
}

fn square_bounds(center: Vec2, size: f32, pivot: [f32; 2]) -> Rect {
    Rect::new(
        center.x - size * pivot[0],
        center.y - size * pivot[1],
        size,
        size,
    )
}

fn faction_emplacement_cell(faction: &str, signature: u32) -> usize {
    let base = if faction.contains("directorate") {
        6
    } else if faction.contains("ascendant") {
        9
    } else if faction.contains("brood") {
        3
    } else {
        0
    };
    base + (signature % 3) as usize
}

fn flora_cell(signature: u32) -> usize {
    [0, 1, 2, 7, 8, 9, 11][(signature % 7) as usize]
}

fn terrain_dressing_cell(signature: u32) -> usize {
    [0, 1, 4, 5, 8, 9][(signature % 6) as usize]
}

fn tile_signature(position: TilePos) -> u32 {
    tile_signature_xy(position.x, position.y)
}

fn tile_signature_xy(x: i32, y: i32) -> u32 {
    (x as u32).wrapping_mul(73_856_093) ^ (y as u32).wrapping_mul(19_349_663)
}
