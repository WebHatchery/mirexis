use super::*;

#[test]
fn colony_ground_has_basin_plain_shelf_and_settlement_levels() {
    let camera = WorldCamera::colony_start(SETTLEMENT_CENTER);
    let view = ColonyView::new(Rect::new(18.0, 106.0, 826.0, 506.0), &camera);
    let levels =
        [[2, 16], [0, 0], [5, 10], SETTLEMENT_CENTER].map(|position| view.elevation(position));
    assert_eq!(levels, [-1, 0, 1, 2]);
}

#[test]
fn transformed_colony_centers_round_trip_through_hover_testing() {
    let viewport = Rect::new(18.0, 106.0, 884.0, 506.0);
    for (focus, zoom) in [([3, 3], 1.0), ([14, 12], 0.72), ([9, 16], 1.7)] {
        let mut camera = WorldCamera::colony_start(focus);
        camera.zoom = zoom;
        let view = ColonyView::new(viewport, &camera);
        for position in [[0, 0], [3, 2], [10, 10], [19, 19]] {
            let center = view.plot_center(position);
            if viewport.contains(center) {
                assert_eq!(hovered_plot(view, center), Some(position));
            }
        }
    }
}

#[test]
fn every_visible_colony_plot_round_trips_at_each_zoom_and_clamp_extreme() {
    let viewport = Rect::new(18.0, 106.0, 884.0, 506.0);
    for zoom in [0.65, 1.0, 1.85] {
        for center in [
            vec2(-10_000.0, 0.0),
            vec2(10_000.0, 0.0),
            vec2(0.0, -10_000.0),
            vec2(0.0, 10_000.0),
            vec2(0.0, 0.0),
        ] {
            let mut camera = WorldCamera::colony_start(SETTLEMENT_CENTER);
            camera.zoom = zoom;
            camera.center = center;
            camera.clamp_isometric(
                COLONY_WIDTH as usize,
                COLONY_HEIGHT as usize,
                COLONY_HALF_WIDTH,
                COLONY_HALF_HEIGHT,
                viewport,
            );
            let view = ColonyView::new(viewport, &camera);
            for y in 0..COLONY_HEIGHT {
                for x in 0..COLONY_WIDTH {
                    let position = [x, y];
                    let center = view.plot_center(position);
                    if viewport.contains(center) {
                        assert_eq!(
                            hovered_plot(view, center),
                            Some(position),
                            "plot {position:?} failed at zoom {zoom} and camera {:?}",
                            camera.center
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn colony_clamp_exposes_each_outer_diamond_vertex_without_clipping() {
    let viewport = Rect::new(18.0, 106.0, 884.0, 506.0);
    let cases = [
        (
            vec2(-10_000.0, 0.0),
            [0, COLONY_HEIGHT - 1],
            vec2(-1.0, 0.0),
        ),
        (vec2(10_000.0, 0.0), [COLONY_WIDTH - 1, 0], vec2(1.0, 0.0)),
        (vec2(0.0, -10_000.0), [0, 0], vec2(0.0, -1.0)),
        (
            vec2(0.0, 10_000.0),
            [COLONY_WIDTH - 1, COLONY_HEIGHT - 1],
            vec2(0.0, 1.0),
        ),
    ];
    for zoom in [0.65, 1.0, 1.85] {
        for (requested_center, position, vertex_direction) in cases {
            let mut camera = WorldCamera::colony_start(SETTLEMENT_CENTER);
            camera.zoom = zoom;
            camera.center = requested_center;
            camera.clamp_isometric(
                COLONY_WIDTH as usize,
                COLONY_HEIGHT as usize,
                COLONY_HALF_WIDTH,
                COLONY_HALF_HEIGHT,
                viewport,
            );
            let view = ColonyView::new(viewport, &camera);
            let vertex = view.plot_center(position)
                + vec2(
                    vertex_direction.x * view.half_width,
                    vertex_direction.y * view.half_height,
                );
            assert!(
                vertex.x >= viewport.x - 0.01
                    && vertex.x <= viewport.right() + 0.01
                    && vertex.y >= viewport.y - 0.01
                    && vertex.y <= viewport.bottom() + 0.01,
                "outer vertex for {position:?} clipped at zoom {zoom}: {vertex:?}"
            );
            assert_eq!(
                hovered_plot(view, view.plot_center(position)),
                Some(position)
            );
        }
    }
}

#[test]
fn colony_clamp_exposes_complete_frontier_plot_art() {
    let viewport = Rect::new(18.0, 106.0, 884.0, 506.0);
    let cases = [
        (vec2(-10_000.0, 0.0), [0, COLONY_HEIGHT - 1], 0),
        (vec2(10_000.0, 0.0), [COLONY_WIDTH - 1, 0], 1),
        (vec2(0.0, -10_000.0), [0, 0], 2),
        (
            vec2(0.0, 10_000.0),
            [COLONY_WIDTH - 1, COLONY_HEIGHT - 1],
            3,
        ),
    ];
    for zoom in [0.65, 1.0, 1.85] {
        for (requested_center, position, edge) in cases {
            let mut camera = WorldCamera::colony_start(SETTLEMENT_CENTER);
            camera.zoom = zoom;
            camera.center = requested_center;
            camera.clamp_isometric_with_insets(
                COLONY_WIDTH as usize,
                COLONY_HEIGHT as usize,
                COLONY_HALF_WIDTH,
                COLONY_HALF_HEIGHT,
                viewport,
                ColonyView::camera_insets(zoom),
            );
            let bounds = ColonyView::new(viewport, &camera).plot_render_bounds(position);
            match edge {
                0 => assert!(bounds.x >= viewport.x - 0.01),
                1 => assert!(bounds.right() <= viewport.right() + 0.01),
                2 => assert!(bounds.y >= viewport.y - 0.01),
                _ => assert!(bounds.bottom() <= viewport.bottom() + 0.01),
            }
        }
    }
}

#[test]
fn maximum_zoom_culling_retains_scaled_building_edges() {
    let viewport = Rect::new(18.0, 106.0, 884.0, 506.0);
    let position = SETTLEMENT_CENTER;
    for desired_center in [
        vec2(viewport.center().x, viewport.bottom() + 90.0),
        vec2(viewport.right() + 60.0, viewport.center().y),
    ] {
        let mut camera = WorldCamera::colony_start(position);
        camera.zoom = 1.85;
        camera.center -= (desired_center - viewport.center()) / camera.zoom;
        let view = ColonyView::new(viewport, &camera);
        let bounds = view.plot_render_bounds(position);
        assert!(bounds.right() >= viewport.x);
        assert!(bounds.x <= viewport.right());
        assert!(bounds.bottom() >= viewport.y);
        assert!(bounds.y <= viewport.bottom());
        assert!(view.visible(position));
    }
}

#[test]
fn colony_buildings_retain_fixed_default_world_scale() {
    let viewport = Rect::new(18.0, 106.0, 884.0, 506.0);
    let small_world_camera = WorldCamera::colony_start([3, 3]);
    let large_world_camera = WorldCamera::colony_start([10, 10]);
    let small = ColonyView::new(viewport, &small_world_camera);
    let large = ColonyView::new(viewport, &large_world_camera);
    assert_eq!(small.half_width, COLONY_HALF_WIDTH);
    assert_eq!(large.half_width, COLONY_HALF_WIDTH);
    assert_eq!(small.half_height, COLONY_HALF_HEIGHT);
    assert_eq!(large.half_height, COLONY_HALF_HEIGHT);
}

#[test]
fn initial_camera_frames_the_centered_settlement_without_fitting_the_colony() {
    let viewport = Rect::new(18.0, 106.0, 884.0, 506.0);
    let camera = WorldCamera::colony_start(SETTLEMENT_CENTER);
    let view = ColonyView::new(viewport, &camera);
    assert!(viewport.contains(view.plot_center(SETTLEMENT_CENTER)));
    assert!(!viewport.contains(view.plot_center([0, COLONY_HEIGHT - 1])));
    assert!(!viewport.contains(view.plot_center([COLONY_WIDTH - 1, 0])));
}

#[test]
fn visible_camera_controls_stay_below_the_interactive_colony_viewport() {
    let panel = Rect::new(10.0, 74.0, 900.0, 608.0);
    let viewport = Rect::new(
        panel.x + 8.0,
        panel.y + 32.0,
        panel.w - 16.0,
        panel.h - 102.0,
    );
    let origin = camera_controls_origin(panel);
    assert!(origin.y >= viewport.bottom());
    assert!(origin.y + 28.0 <= panel.bottom());
}
