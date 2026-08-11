use super::*;

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
