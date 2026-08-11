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
    let view = GridView::new(40, 40, Rect::new(0.0, 0.0, 780.0, 450.0));
    let offsets = [
        vec2(0.0, 0.0),
        vec2(view.half_width * 0.32, 0.0),
        vec2(-view.half_width * 0.32, 0.0),
        vec2(0.0, view.half_height * 0.32),
        vec2(0.0, -view.half_height * 0.32),
    ];
    for tile in [
        TilePos::new(3, 20),
        TilePos::new(13, 12),
        TilePos::new(20, 29),
        TilePos::new(29, 17),
    ] {
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
    let view = GridView::new(40, 40, Rect::new(0.0, 0.0, 780.0, 450.0));
    assert_eq!(view.elevation(TilePos::new(3, 20)), 0);
    assert_eq!(view.elevation(TilePos::new(17, 12)), 1);
    assert_eq!(view.elevation(TilePos::new(13, 12)), 2);
    assert_eq!(view.elevation(TilePos::new(20, 29)), -1);
}

#[test]
fn raised_foreground_tile_owns_the_visibly_occluded_transition_area() {
    let view = GridView::new(40, 40, Rect::new(0.0, 0.0, 780.0, 450.0));
    let rim = TilePos::new(13, 17);
    let basin_rim = TilePos::new(20, 23);
    assert_eq!(view.cliff_drop(rim, TilePos::new(13, 18)), 1);
    assert_eq!(view.cliff_drop(basin_rim, TilePos::new(20, 24)), 1);
    assert_eq!(view.tile_at(view.tile_center(rim)), Some(rim));
    assert_eq!(
        view.tile_at(view.tile_center(TilePos::new(20, 24))),
        Some(TilePos::new(20, 24))
    );
}

#[test]
fn elevation_regions_are_large_distributed_and_projected_vertically() {
    let view = GridView::new(40, 40, Rect::new(0.0, 0.0, 780.0, 450.0));
    let mut raised = 0;
    let mut high = 0;
    let mut lowered = 0;
    for y in 0..40 {
        for x in 0..40 {
            match view.elevation(TilePos::new(x, y)) {
                1 => raised += 1,
                2 => high += 1,
                -1 => lowered += 1,
                _ => {}
            }
        }
    }
    assert!(raised >= 100, "only {raised} raised tiles");
    assert!(high >= 30, "only {high} high tiles");
    assert!(lowered >= 100, "only {lowered} lowered tiles");

    for tile in [
        TilePos::new(13, 12),
        TilePos::new(17, 12),
        TilePos::new(20, 29),
    ] {
        let plane_y = view.origin.y + (tile.x + tile.y) as f32 * view.half_height;
        assert_eq!(
            view.tile_center(tile).y,
            plane_y - f32::from(view.elevation(tile)) * view.elevation_step()
        );
    }
}

#[test]
fn terrain_art_bounds_keep_the_declared_pivot_on_the_tile_anchor() {
    let viewport = Rect::new(18.0, 106.0, 884.0, 568.0);
    let tile = TilePos::new(13, 12);
    for camera in [
        WorldCamera::tactical_view(tile, tile, 0.65),
        WorldCamera::tactical_view(TilePos::new(20, 20), tile, 1.0),
        WorldCamera::tactical_view(TilePos::new(8, 25), tile, 1.85),
    ] {
        let view = GridView::with_camera(40, 40, viewport, &camera);
        let anchor = view.ground_anchor(tile);
        let bounds = view.art_bounds(tile, TERRAIN_ART_SCALE, TERRAIN_ART_PIVOT);
        assert!((bounds.x + bounds.w * TERRAIN_ART_PIVOT[0] - anchor.x).abs() < 0.001);
        assert!((bounds.y + bounds.h * TERRAIN_ART_PIVOT[1] - anchor.y).abs() < 0.001);
        assert_eq!(bounds.w, view.tile_rect(tile).w * TERRAIN_ART_SCALE);
        assert_eq!(bounds.w, bounds.h);
    }
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

#[test]
fn cursor_anchored_zoom_preserves_the_world_point_under_the_pointer() {
    let viewport = Rect::new(18.0, 106.0, 884.0, 568.0);
    let cursor = vec2(670.0, 280.0);
    let mut camera = WorldCamera::tactical_start(TilePos::new(8, 5));
    let before = camera.center + (cursor - viewport.center()) / camera.zoom;
    camera.zoom_at(viewport, cursor, 1.4);
    let after = camera.center + (cursor - viewport.center()) / camera.zoom;
    assert!((before - after).length() < 0.001);
}

#[test]
fn panning_uses_screen_distance_at_every_zoom() {
    let mut camera = WorldCamera::tactical_start(TilePos::new(8, 5));
    camera.zoom = 1.5;
    let before = camera.center;
    camera.pan_screen(vec2(150.0, -75.0));
    assert_eq!(camera.center, before + vec2(-100.0, 50.0));
}

#[test]
fn changed_offscreen_selection_recenters_without_fighting_free_pan() {
    let viewport = Rect::new(18.0, 106.0, 884.0, 568.0);
    let start = TilePos::new(2, 2);
    let mut camera = WorldCamera::tactical_start(start);
    camera.center += vec2(400.0, 300.0);
    let freely_panned = camera.center;
    camera.reveal_changed_tactical_selection(start, viewport);
    assert_eq!(camera.center, freely_panned);
    let distant = TilePos::new(30, 30);
    camera.reveal_changed_tactical_selection(distant, viewport);
    assert_eq!(
        camera.center,
        projected_tile(distant, TACTICAL_HALF_WIDTH, TACTICAL_HALF_HEIGHT)
    );
}
