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
fn clearance_plots_remain_unowned_but_cannot_be_planned() {
    let data = crate::data::GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    let anchor = [3, 3];
    campaign
        .colony
        .place_construction(BuildingKind::Barricade, anchor)
        .unwrap();

    for neighbour in [
        [2, 2],
        [3, 2],
        [4, 2],
        [2, 3],
        [4, 3],
        [2, 4],
        [3, 4],
        [4, 4],
    ] {
        assert!(campaign.colony.project_at(neighbour).is_none());
        assert!(campaign.colony.building_at(neighbour).is_none());
        assert!(campaign
            .colony
            .validate_construction_site(neighbour)
            .is_err());
    }
    for overlapping_anchor in [[5, 3], [5, 5], [1, 1]] {
        assert!(campaign
            .colony
            .validate_construction_site(overlapping_anchor)
            .is_err());
    }
    assert!(campaign.colony.validate_construction_site([6, 3]).is_ok());
}

#[test]
fn prospective_building_preview_marks_its_complete_three_by_three_zone() {
    let anchor = [7, 9];
    let marked = (5..=11)
        .flat_map(|y| (5..=11).map(move |x| [x, y]))
        .filter(|position| in_clearance_zone(anchor, *position))
        .collect::<Vec<_>>();

    assert_eq!(marked.len(), 9);
    assert!(marked.contains(&anchor));
    assert!(marked.contains(&[6, 8]));
    assert!(marked.contains(&[8, 10]));
    assert!(!marked.contains(&[9, 9]));
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
fn first_hour_route_targets_the_current_guided_colonist() {
    let data = crate::data::GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    assert!(first_hour_destination(&campaign).is_none());

    campaign.first_hour.advance_arrival();
    assert_eq!(
        first_hour_destination(&campaign),
        crate::colony_exploration::npc_position(&campaign, "mara_venn")
    );

    campaign.first_hour.acknowledge_colonist("mara_venn");
    assert!(first_hour_destination(&campaign).is_none());

    campaign.first_hour.stage = crate::first_hour::FirstHourStage::FirstReturn;
    assert!(first_hour_destination(&campaign).is_none());
    campaign.first_hour.returned_to_colony();
    assert_eq!(
        first_hour_destination(&campaign),
        crate::colony_exploration::npc_position(&campaign, "ilya_reed")
    );

    campaign.first_hour.guidance_enabled = false;
    assert!(first_hour_destination(&campaign).is_none());
}

#[test]
fn visible_camera_controls_stay_below_the_interactive_colony_viewport() {
    let panel = panel_bounds(false);
    let viewport = viewport_bounds(panel);
    let origin = camera_controls_origin(panel);
    assert!(origin.y >= viewport.bottom());
    assert!(origin.y + 28.0 <= panel.bottom());
}

#[test]
fn collapsed_operations_gives_the_settlement_the_full_screen_width() {
    let collapsed = panel_bounds(false);
    let open = panel_bounds(true);
    assert_eq!(collapsed.right(), 1270.0);
    assert_eq!(open.right(), 852.0);
    assert!(viewport_bounds(collapsed).w > viewport_bounds(open).w + 400.0);
}

#[test]
fn colony_map_only_arms_plots_with_a_real_action() {
    let data = crate::data::GameData::load().unwrap();
    let campaign = CampaignState::new(&data);

    assert_eq!(
        interaction::plot_action(&campaign, &data, SETTLEMENT_CENTER),
        None
    );

    let position = (0..COLONY_HEIGHT)
        .flat_map(|y| (0..COLONY_WIDTH).map(move |x| [x, y]))
        .find(|position| campaign.can_construct_building(BuildingKind::Barricade, *position))
        .expect("new colony has an open construction plot");
    assert!(matches!(
        interaction::plot_action(&campaign, &data, position),
        Some(UiAction::ConstructBuilding(BuildingKind::Barricade, actual)) if actual == position
    ));
}

#[test]
fn colony_map_keeps_completed_projects_and_offline_facilities_non_actionable() {
    let data = crate::data::GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    let position = (0..COLONY_HEIGHT)
        .flat_map(|y| (0..COLONY_WIDTH).map(move |x| [x, y]))
        .find(|position| campaign.can_construct_building(BuildingKind::Barricade, *position))
        .expect("new colony has an open construction plot");
    campaign
        .colony
        .place_construction(BuildingKind::Barricade, position)
        .unwrap();
    assert_eq!(interaction::plot_action(&campaign, &data, position), None);

    campaign.strategy.contact_complete = true;
    campaign.colony.resources.power = 10;
    campaign.colony.ensure_gene_lab();
    let gene_lab = campaign
        .colony
        .buildings
        .iter()
        .find(|building| building.kind == BuildingKind::GeneLab)
        .expect("contact unlocks the Gene Lab facility");
    assert_eq!(
        interaction::plot_action(&campaign, &data, gene_lab.position),
        Some(UiAction::OpenGeneLab)
    );
    campaign.colony.resources.power = 0;
    assert_eq!(
        interaction::plot_action(&campaign, &data, gene_lab.position),
        None
    );
}
