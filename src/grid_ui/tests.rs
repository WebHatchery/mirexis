use super::*;
use crate::colony::{COLONY_HEIGHT, COLONY_WIDTH, SETTLEMENT_CENTER};
use crate::colony_map_ui::{COLONY_HALF_HEIGHT, COLONY_HALF_WIDTH};

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
        TilePos::new(12, 19),
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
fn inverse_hit_testing_survives_panned_and_extreme_zoom_cameras() {
    let viewport = Rect::new(18.0, 106.0, 884.0, 568.0);
    for camera in [
        WorldCamera::tactical_view(TilePos::new(4, 19), TilePos::new(4, 19), 0.65),
        WorldCamera::tactical_view(TilePos::new(31, 18), TilePos::new(32, 18), 1.85),
        WorldCamera::tactical_view(TilePos::new(20, 29), TilePos::new(20, 29), 1.25),
    ] {
        let view = GridView::with_camera(40, 40, viewport, &camera);
        for tile in [
            TilePos::new(4, 19),
            TilePos::new(12, 19),
            TilePos::new(20, 29),
            TilePos::new(31, 18),
        ] {
            for offset in [
                vec2(0.0, 0.0),
                vec2(view.half_width * 0.25, 0.0),
                vec2(0.0, -view.half_height * 0.25),
            ] {
                assert_eq!(
                    view.tile_at(view.tile_center(tile) + offset),
                    Some(tile),
                    "camera {camera:?} missed {tile:?} at {offset:?}"
                );
            }
        }
    }
}

#[test]
fn inverse_hits_follow_visible_diamond_order_at_every_large_world_center() {
    let viewport = Rect::new(18.0, 106.0, 884.0, 568.0);
    for camera in [
        WorldCamera::tactical_view(TilePos::new(4, 19), TilePos::new(4, 19), 0.65),
        WorldCamera::tactical_view(TilePos::new(31, 18), TilePos::new(31, 18), 1.85),
    ] {
        let view = GridView::with_camera(40, 40, viewport, &camera);
        let mut draw_order = (0..40)
            .flat_map(|y| (0..40).map(move |x| TilePos::new(x, y)))
            .collect::<Vec<_>>();
        draw_order.sort_by_key(|tile| tile.x + tile.y);
        for y in 0..40 {
            for x in 0..40 {
                let tile = TilePos::new(x, y);
                for offset in [
                    Vec2::ZERO,
                    vec2(view.half_width * 0.4, 0.0),
                    vec2(-view.half_width * 0.4, 0.0),
                    vec2(0.0, view.half_height * 0.4),
                    vec2(0.0, -view.half_height * 0.4),
                ] {
                    let point = view.tile_center(tile) + offset;
                    let expected = draw_order.iter().copied().rev().find(|candidate| {
                        let center = view.tile_center(*candidate);
                        (point.x - center.x).abs() / view.half_width
                            + (point.y - center.y).abs() / view.half_height
                            <= 1.0
                    });
                    assert_eq!(
                        view.tile_at(point),
                        expected,
                        "camera {camera:?} disagreed at {tile:?} offset {offset:?}"
                    );
                }
            }
        }
    }
}

#[test]
fn battlefield_exposes_raised_flat_and_lowered_height_bands() {
    let view = GridView::new(40, 40, Rect::new(0.0, 0.0, 780.0, 450.0));
    assert_eq!(view.elevation(TilePos::new(3, 20)), 0);
    assert_eq!(view.elevation(TilePos::new(17, 19)), 1);
    assert_eq!(view.elevation(TilePos::new(12, 19)), 2);
    assert_eq!(view.elevation(TilePos::new(20, 29)), -1);
}

#[test]
fn raised_foreground_tile_owns_the_visibly_occluded_transition_area() {
    let view = GridView::new(40, 40, Rect::new(0.0, 0.0, 780.0, 450.0));
    let rim = TilePos::new(12, 24);
    let basin_rim = TilePos::new(20, 23);
    assert_eq!(view.cliff_drop(rim, TilePos::new(12, 25)), 1);
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
        TilePos::new(12, 19),
        TilePos::new(17, 19),
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
    let tile = TilePos::new(12, 19);
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
        assert_eq!(bounds.h, bounds.w * TERRAIN_ART_ASPECT);
    }
}

#[test]
fn terrain_culling_keeps_every_intersecting_art_family_and_cliff_face() {
    let viewport = Rect::new(18.0, 106.0, 884.0, 568.0);
    let overlaps = |a: Rect, b: Rect| {
        a.right() >= b.x && a.x <= b.right() && a.bottom() >= b.y && a.y <= b.bottom()
    };
    for camera in [
        WorldCamera::tactical_view(TilePos::new(4, 19), TilePos::new(4, 19), 0.65),
        WorldCamera::tactical_view(TilePos::new(20, 20), TilePos::new(20, 20), 1.0),
        WorldCamera::tactical_view(TilePos::new(31, 18), TilePos::new(31, 18), 1.85),
    ] {
        let view = GridView::with_camera(40, 40, viewport, &camera);
        for y in 0..40 {
            for x in 0..40 {
                let tile = TilePos::new(x, y);
                let mut cliff_bounds = view.tile_rect(tile);
                let drop = view
                    .cliff_drop(tile, TilePos::new(x + 1, y))
                    .max(view.cliff_drop(tile, TilePos::new(x, y + 1)));
                cliff_bounds.h += f32::from(drop) * view.elevation_step();
                let intersects = [
                    view.tile_rect(tile),
                    view.art_bounds(tile, TERRAIN_ART_SCALE, TERRAIN_ART_PIVOT),
                    view.art_bounds(tile, STRUCTURE_ART_SCALE, STRUCTURE_ART_PIVOT),
                    view.art_bounds(tile, CANOPY_ART_SCALE, CANOPY_ART_PIVOT),
                    cliff_bounds,
                ]
                .into_iter()
                .any(|bounds| overlaps(bounds, viewport));
                assert!(
                    !intersects || view.terrain_is_visible(tile, viewport, 30.0),
                    "camera {camera:?} culled intersecting art at {tile:?}"
                );
            }
        }
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
fn primary_drag_pans_after_a_touch_safe_threshold_and_suppresses_release() {
    let mut camera = WorldCamera::tactical_start(TilePos::new(8, 5));
    camera.zoom = 1.5;
    let before = camera.center;
    assert!(!camera.update_primary_drag(true, false, vec2(100.0, 100.0)));
    assert!(camera.primary_gesture_active());
    assert!(!camera.update_primary_drag(true, false, vec2(104.0, 102.0)));
    assert_eq!(camera.center, before);
    assert!(!camera.update_primary_drag(true, false, vec2(112.0, 106.0)));
    assert_eq!(camera.center, before - vec2(12.0, 6.0) / 1.5);
    assert!(camera.update_primary_drag(false, true, vec2(112.0, 106.0)));
    assert!(!camera.primary_gesture_active());
}

#[test]
fn primary_release_recovers_a_threshold_crossing_between_render_frames() {
    let mut camera = WorldCamera::tactical_start(TilePos::new(8, 5));
    camera.zoom = 1.5;
    let before = camera.center;
    assert!(!camera.update_primary_drag(true, false, vec2(100.0, 100.0)));
    assert!(camera.update_primary_drag(false, true, vec2(112.0, 106.0)));
    assert_eq!(camera.center, before - vec2(12.0, 6.0) / 1.5);
    assert!(!camera.primary_gesture_active());
}

#[test]
fn primary_drag_suppression_survives_a_browser_frame_without_button_state() {
    let mut camera = WorldCamera::tactical_start(TilePos::new(8, 5));
    assert!(!camera.update_primary_drag(true, false, vec2(100.0, 100.0)));
    assert!(!camera.update_primary_drag(true, false, vec2(112.0, 106.0)));

    assert!(!camera.update_primary_drag(false, false, vec2(112.0, 106.0)));
    assert!(camera.update_primary_drag(false, true, vec2(112.0, 106.0)));
}

#[test]
fn camera_control_guard_blocks_an_untracked_release_but_not_a_new_map_press() {
    let mut camera = WorldCamera::tactical_start(TilePos::new(8, 5));
    camera.guard_next_primary_release();
    assert!(camera.update_primary_drag(false, true, vec2(200.0, 200.0)));

    camera.guard_next_primary_release();
    camera.begin_primary_press(true, true);
    assert!(!camera.update_primary_drag(true, false, vec2(200.0, 200.0)));
    assert!(!camera.update_primary_drag(false, true, vec2(200.0, 200.0)));
}

#[test]
fn colony_plot_actions_require_two_taps_on_the_same_plot() {
    let mut camera = WorldCamera::colony_start(SETTLEMENT_CENTER);
    assert!(!camera.confirm_colony_plot([12, 11]));
    assert_eq!(camera.pending_colony_plot(), Some([12, 11]));
    assert!(!camera.confirm_colony_plot([13, 11]));
    assert!(camera.confirm_colony_plot([13, 11]));
    assert_eq!(camera.pending_colony_plot(), None);
}

#[test]
fn primary_tap_does_not_pan_or_suppress_selection() {
    let mut camera = WorldCamera::tactical_start(TilePos::new(8, 5));
    let before = camera.center;
    assert!(!camera.update_primary_drag(true, false, vec2(100.0, 100.0)));
    assert!(!camera.update_primary_drag(false, true, vec2(103.0, 102.0)));
    assert_eq!(camera.center, before);
}

#[test]
fn primary_tracking_requires_an_inside_press_and_survives_boundary_excursions() {
    assert!(primary_tracking(false, true, true, true));
    assert!(primary_tracking(true, false, false, true));
    assert!(primary_tracking(true, true, false, true));
    assert!(!primary_tracking(false, true, false, true));
    assert!(!primary_tracking(false, false, true, true));
    assert!(!primary_tracking(true, false, false, false));
}

#[test]
fn visible_camera_controls_nudge_at_screen_scale_and_zoom_from_center() {
    let viewport = Rect::new(18.0, 106.0, 884.0, 568.0);
    let mut camera = WorldCamera::tactical_start(TilePos::new(8, 5));
    camera.zoom = 1.5;
    let before = camera.center;
    camera.nudge(vec2(1.0, -1.0));
    assert_eq!(camera.center, before + vec2(64.0, -64.0));
    camera.zoom_center(viewport, 1.25);
    assert_eq!(camera.zoom, 1.85);
    assert_eq!(camera.center, before + vec2(64.0, -64.0));
}

#[test]
fn camera_clamp_keeps_the_large_world_in_view_at_every_zoom_limit() {
    let viewport = Rect::new(18.0, 106.0, 884.0, 568.0);
    let world_min = vec2(-40.0 * TACTICAL_HALF_WIDTH, -TACTICAL_HALF_HEIGHT);
    let world_max = vec2(40.0 * TACTICAL_HALF_WIDTH, 79.0 * TACTICAL_HALF_HEIGHT);
    for zoom in [0.65, 1.0, 1.85] {
        let visible_half = viewport.size() * 0.5 / zoom;
        for extreme in [vec2(-10_000.0, -10_000.0), vec2(10_000.0, 10_000.0)] {
            let mut camera = WorldCamera::tactical_start(TilePos::new(8, 5));
            camera.zoom = zoom;
            camera.center = extreme;
            camera.clamp_isometric(40, 40, TACTICAL_HALF_WIDTH, TACTICAL_HALF_HEIGHT, viewport);
            assert_eq!(
                camera.center.x,
                clamp_axis(extreme.x, world_min.x, world_max.x, visible_half.x)
            );
            assert_eq!(
                camera.center.y,
                clamp_axis(extreme.y, world_min.y, world_max.y, visible_half.y)
            );
        }
    }
}

#[test]
fn colony_camera_clamp_handles_world_smaller_and_larger_than_the_view() {
    let viewport = Rect::new(18.0, 106.0, 884.0, 506.0);
    let world_min = vec2(-20.0 * COLONY_HALF_WIDTH, -COLONY_HALF_HEIGHT);
    let world_max = vec2(20.0 * COLONY_HALF_WIDTH, 39.0 * COLONY_HALF_HEIGHT);
    for zoom in [0.65, 1.0, 1.85] {
        let visible_half = viewport.size() * 0.5 / zoom;
        for extreme in [vec2(-10_000.0, -10_000.0), vec2(10_000.0, 10_000.0)] {
            let mut camera = WorldCamera::colony_start(SETTLEMENT_CENTER);
            camera.zoom = zoom;
            camera.center = extreme;
            camera.clamp_isometric(
                COLONY_WIDTH as usize,
                COLONY_HEIGHT as usize,
                COLONY_HALF_WIDTH,
                COLONY_HALF_HEIGHT,
                viewport,
            );
            assert_eq!(
                camera.center.x,
                clamp_axis(extreme.x, world_min.x, world_max.x, visible_half.x)
            );
            assert_eq!(
                camera.center.y,
                clamp_axis(extreme.y, world_min.y, world_max.y, visible_half.y)
            );
        }
    }
}

#[test]
fn maximum_pan_keeps_boundary_tile_diamonds_inside_the_viewport() {
    let viewport = Rect::new(18.0, 106.0, 884.0, 568.0);
    for (extreme, boundary, edge) in [
        (vec2(-10_000.0, 468.0), TilePos::new(0, 39), 0),
        (vec2(10_000.0, 468.0), TilePos::new(39, 0), 1),
        (vec2(0.0, -10_000.0), TilePos::new(0, 0), 2),
        (vec2(0.0, 10_000.0), TilePos::new(39, 39), 3),
    ] {
        let mut camera = WorldCamera::tactical_start(TilePos::new(4, 19));
        camera.center = extreme;
        camera.clamp_isometric(40, 40, TACTICAL_HALF_WIDTH, TACTICAL_HALF_HEIGHT, viewport);
        let view = GridView::with_camera(40, 40, viewport, &camera);
        let bounds = view.tile_rect(boundary);
        match edge {
            0 => assert!(bounds.x >= viewport.x - 0.001),
            1 => assert!(bounds.right() <= viewport.right() + 0.001),
            2 => assert!(bounds.y >= viewport.y - 0.001),
            _ => assert!(bounds.bottom() <= viewport.bottom() + 0.001),
        }
    }
}

#[test]
fn visible_zoom_controls_stop_at_declared_camera_limits() {
    let viewport = Rect::new(18.0, 106.0, 884.0, 568.0);
    let mut camera = WorldCamera::tactical_start(TilePos::new(8, 5));
    for _ in 0..20 {
        camera.zoom_center(viewport, 1.25);
    }
    assert_eq!(camera.zoom, 1.85);
    for _ in 0..30 {
        camera.zoom_center(viewport, 1.0 / 1.25);
    }
    assert_eq!(camera.zoom, 0.65);
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
