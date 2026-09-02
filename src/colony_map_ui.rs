//! Interactive isometric colony map and modular settlement building art.

use crate::campaign::CampaignState;
use crate::colony::{BuildingKind, COLONY_HEIGHT, COLONY_WIDTH, SETTLEMENT_CENTER};
use crate::data::GameData;
use crate::grid_ui::WorldCamera;
use crate::ui::UiAction;
use crate::visual_assets::VisualCatalog;
use macroquad::prelude::*;
use macroquad_toolkit::assets::AssetManager;
use macroquad_toolkit::prelude::{draw_surface, SurfaceStyle};
use macroquad_toolkit::ui::VirtualUi;

pub(crate) const COLONY_HALF_WIDTH: f32 = 26.0;
pub(crate) const COLONY_HALF_HEIGHT: f32 = 13.0;

mod building_art;
mod controls;
mod identity_art;
mod interaction;
mod plot;
mod terrain;
pub(crate) mod view;
#[cfg(test)]
use plot::in_clearance_zone;
use view::ColonyView;

pub(crate) struct ColonyMapContext<'a> {
    pub(crate) campaign: &'a CampaignState,
    pub(crate) data: &'a GameData,
    pub(crate) assets: &'a AssetManager,
    pub(crate) visuals: &'a VisualCatalog,
    pub(crate) ui: &'a VirtualUi,
    pub(crate) camera: &'a mut WorldCamera,
    pub(crate) explorer: &'a mut crate::colony_exploration::ColonyExplorer,
    pub(crate) mouse: Vec2,
    pub(crate) operations_open: bool,
    pub(crate) interaction_enabled: bool,
    pub(crate) actions: &'a mut Vec<UiAction>,
}

pub(crate) fn draw(context: ColonyMapContext<'_>) -> bool {
    let ColonyMapContext {
        campaign,
        data,
        assets,
        visuals,
        ui,
        camera,
        explorer,
        mouse,
        operations_open,
        interaction_enabled,
        actions,
    } = context;
    let panel = panel_bounds(operations_open);
    draw_surface(
        panel,
        &SurfaceStyle::new(Color::new(0.026, 0.047, 0.053, 0.98))
            .with_border(1.0, Color::new(0.20, 0.50, 0.48, 0.9))
            .with_inner_border(6.0, 1.0, Color::new(0.12, 0.28, 0.27, 0.7)),
    );
    let viewport = viewport_bounds(panel);
    let camera_control_clicked = interaction_enabled
        && crate::camera_controls::draw_zoom(
            camera,
            viewport,
            mouse,
            zoom_controls_origin(panel),
            !camera.primary_gesture_active(),
        );
    let camera_dragged = if interaction_enabled {
        camera.update(viewport, mouse)
    } else {
        camera.clear_pointer_interaction();
        false
    };
    if camera_control_clicked {
        camera.guard_next_primary_release();
    }
    if camera_control_clicked || camera_dragged {
        camera.clear_pending_colony_plot();
    }
    let suppress_plot_click = camera_control_clicked || camera_dragged;
    camera.clamp_isometric_with_insets(
        COLONY_WIDTH as usize,
        COLONY_HEIGHT as usize,
        COLONY_HALF_WIDTH,
        COLONY_HALF_HEIGHT,
        viewport,
        ColonyView::camera_insets(camera.zoom),
    );
    let view = ColonyView::new(viewport, camera);
    draw_wetland_backdrop(viewport);
    crate::ui::set_ui_clip(ui, Some(viewport));
    draw_campaign_evolution(campaign, view);
    draw_service_paths(campaign, view);
    if interaction_enabled {
        explorer.update_approach(campaign, data);
    }
    let interaction_mouse = if interaction_enabled {
        mouse
    } else {
        vec2(-1_000_000.0, -1_000_000.0)
    };
    let hovered = hovered_plot(view, mouse);
    let planning_site = hovered.filter(|position| {
        campaign.colony.building_at(*position).is_none()
            && campaign.colony.project_at(*position).is_none()
            && campaign
                .colony
                .validate_construction_site(*position)
                .is_ok()
    });
    for sum in 0..(COLONY_WIDTH + COLONY_HEIGHT - 1) {
        for y in 0..COLONY_HEIGHT {
            let position = [sum - y, y];
            if (0..COLONY_WIDTH).contains(&position[0]) && view.visible(position) {
                plot::draw_ground(
                    campaign,
                    assets,
                    visuals,
                    view,
                    position,
                    hovered == Some(position),
                    planning_site,
                );
            }
        }
    }
    let mut clicked_npc = None;
    for sum in 0..(COLONY_WIDTH + COLONY_HEIGHT - 1) {
        for y in 0..COLONY_HEIGHT {
            let x = sum - y;
            if !(0..COLONY_WIDTH).contains(&x) {
                continue;
            }
            if view.visible([x, y]) {
                plot::draw_contents(campaign, assets, visuals, view, [x, y], planning_site);
            }
        }
        clicked_npc = clicked_npc.or_else(|| {
            explorer.draw_depth(
                sum,
                crate::colony_exploration::ColonyDepthContext {
                    campaign,
                    data,
                    assets,
                    visuals,
                    view,
                    mouse: interaction_mouse,
                },
            )
        });
    }
    draw_first_hour_route(campaign, explorer, view);
    crate::first_hour_consequences_ui::draw(campaign, view);
    draw_ending_manifestation(campaign, assets, visuals, view);
    crate::ui::set_ui_clip(ui, None);
    if interaction_enabled {
        if let Some(npc_id) = clicked_npc.as_deref() {
            if let Some(position) = crate::colony_exploration::npc_position(campaign, npc_id) {
                explorer.request_approach(npc_id, position, &campaign.colony);
            }
            camera.guard_next_primary_release();
        }
    }
    if interaction_enabled && explorer.build_mode() {
        interaction::draw_hover_card(campaign, data, hovered, camera.pending_colony_plot());
        interaction::handle_plot_click(
            campaign,
            data,
            camera,
            hovered,
            suppress_plot_click,
            actions,
        );
        controls::draw_build_controls(campaign, mouse, actions);
    } else if interaction_enabled
        && !explorer.build_mode()
        && !suppress_plot_click
        && clicked_npc.is_none()
        && !explorer.is_talking()
        && is_mouse_button_released(MouseButton::Left)
    {
        if let Some(position) = hovered {
            explorer.request_walk(view.world_position(mouse, position), &campaign.colony);
        }
    }
    if interaction_enabled {
        controls::draw_exploration_controls(campaign, data, explorer, mouse, actions);
    } else {
        explorer.set_keyboard_direction(Vec2::ZERO);
        explorer.set_touch_direction(Vec2::ZERO);
    }
    if interaction_enabled {
        explorer.draw_dialogue(campaign, data, mouse, actions);
    }
    let instruction_y = if explorer.build_mode() {
        panel.y + 42.0
    } else {
        panel.bottom() - 76.0
    };
    draw_text(
        if explorer.build_mode() {
            format!(
                "BUILD MODE // TAP A PLOT TWICE TO CONFIRM // {}%",
                (camera.zoom * 100.0) as i32
            )
        } else {
            format!(
                "TAP ANY GROUND POINT TO WALK // DRAG/WHEEL TO SCAN // {}%",
                (camera.zoom * 100.0) as i32
            )
        },
        panel.x + 14.0,
        instruction_y,
        11.0,
        Color::new(0.46, 0.68, 0.66, 1.0),
    );
    camera_dragged
}

fn zoom_controls_origin(panel: Rect) -> Vec2 {
    vec2(panel.right() - 104.0, panel.bottom() - 56.0)
}

fn panel_bounds(operations_open: bool) -> Rect {
    Rect::new(
        10.0,
        74.0,
        if operations_open { 842.0 } else { 1260.0 },
        636.0,
    )
}

fn viewport_bounds(panel: Rect) -> Rect {
    Rect::new(panel.x + 8.0, panel.y + 8.0, panel.w - 16.0, panel.h - 76.0)
}

fn draw_ending_manifestation(
    campaign: &CampaignState,
    assets: &AssetManager,
    visuals: &VisualCatalog,
    view: ColonyView,
) {
    if !campaign.strategy.campaign_complete {
        return;
    }
    match campaign.strategy.mirexis_path_id.as_str() {
        "human_redoubt" => {
            for position in [[1, 1], [5, 1], [1, 4], [5, 4]] {
                let center = view.plot_center(position);
                visuals.draw_atlas_cell(
                    assets,
                    &visuals.colony,
                    plot::building_index(BuildingKind::Barricade),
                    Rect::new(center.x - 44.0, center.y - 64.0, 88.0, 88.0),
                    Color::new(0.78, 0.92, 1.0, 0.96),
                );
            }
            draw_text(
                "HUMAN REDOUBT // PERIMETER SEALED",
                252.0,
                530.0,
                16.0,
                Color::new(0.42, 0.78, 1.0, 1.0),
            );
        }
        "living_commonwealth" => {
            for position in [[1, 1], [5, 1], [1, 4], [5, 4]] {
                let center = view.plot_center(position);
                visuals.draw_atlas_cell(
                    assets,
                    &visuals.colony,
                    plot::building_index(BuildingKind::Hydroponics),
                    Rect::new(center.x - 46.0, center.y - 68.0, 92.0, 92.0),
                    Color::new(0.65, 1.0, 0.52, 0.94),
                );
                draw_line(
                    392.0,
                    294.0,
                    center.x,
                    center.y,
                    4.0,
                    Color::new(0.52, 0.94, 0.32, 0.48),
                );
            }
            draw_text(
                "LIVING COMMONWEALTH // ROOT NETWORK AWAKE",
                220.0,
                530.0,
                16.0,
                Color::new(0.62, 0.96, 0.36, 1.0),
            );
        }
        "open_threshold" => {
            let center = vec2(690.0, 244.0);
            for radius in [74.0, 58.0, 40.0] {
                draw_poly_lines(
                    center.x,
                    center.y,
                    4,
                    radius,
                    45.0,
                    4.0,
                    Color::new(0.72, 0.48, 1.0, 0.84),
                );
            }
            visuals.draw_atlas_cell(
                assets,
                &visuals.colony,
                plot::building_index(BuildingKind::GeneLab),
                Rect::new(center.x - 58.0, center.y - 76.0, 116.0, 116.0),
                Color::new(0.76, 0.58, 1.0, 0.88),
            );
            draw_text(
                "OPEN THRESHOLD // NETWORK APERTURE STABLE",
                236.0,
                530.0,
                16.0,
                Color::new(0.76, 0.58, 1.0, 1.0),
            );
        }
        _ => {}
    }
}

fn draw_campaign_evolution(campaign: &CampaignState, view: ColonyView) {
    let (accent, label) = if campaign.strategy.campaign_complete {
        match campaign.strategy.mirexis_path_id.as_str() {
            "human_redoubt" => (Color::new(0.35, 0.72, 0.92, 0.54), "REDOUBT BULWARK"),
            "living_commonwealth" => (Color::new(0.58, 0.95, 0.30, 0.54), "COMMONWEALTH ROOT"),
            "open_threshold" => (Color::new(0.72, 0.48, 1.0, 0.54), "THRESHOLD APERTURE"),
            _ => (Color::new(0.34, 0.86, 0.68, 0.48), "MIREXIS AWAKENED"),
        }
    } else if campaign.strategy.escalation_complete {
        (
            Color::new(0.96, 0.50, 0.24, 0.42),
            "THREE-POWER CONVERGENCE",
        )
    } else if campaign.strategy.adaptation_complete {
        (Color::new(0.64, 0.88, 0.34, 0.40), "ADAPTIVE BIOSPHERE")
    } else if campaign.strategy.contact_complete {
        (Color::new(0.38, 0.68, 0.96, 0.38), "CONTACT LATTICE")
    } else {
        return;
    };
    let anchor = view.plot_center([11, 3]);
    for ring in 0..4 {
        draw_poly_lines(
            anchor.x,
            anchor.y,
            match campaign.strategy.mirexis_path_id.as_str() {
                "living_commonwealth" => 7,
                "open_threshold" => 4,
                _ => 6,
            },
            34.0 + ring as f32 * 14.0,
            45.0,
            2.0,
            Color::new(accent.r, accent.g, accent.b, accent.a / (ring as f32 + 1.0)),
        );
    }
    draw_line(
        anchor.x,
        anchor.y - 64.0,
        anchor.x,
        anchor.y + 64.0,
        2.0,
        accent,
    );
    draw_text(
        label,
        594.0,
        318.0,
        11.0,
        Color::new(accent.r, accent.g, accent.b, 0.9),
    );
}

fn hovered_plot(view: ColonyView, mouse: Vec2) -> Option<[i32; 2]> {
    if !view.viewport.contains(mouse) {
        return None;
    }
    let mut hovered = None;
    for y in 0..COLONY_HEIGHT {
        for x in 0..COLONY_WIDTH {
            let c = view.plot_center([x, y]);
            let dx = (mouse.x - c.x).abs() / view.half_width;
            let dy = (mouse.y - c.y).abs() / view.half_height;
            if dx + dy <= 1.0 {
                hovered = Some([x, y]);
            }
        }
    }
    hovered
}

fn draw_wetland_backdrop(panel: Rect) {
    let field = Rect::new(
        panel.x + 14.0,
        panel.y + 48.0,
        panel.w - 28.0,
        panel.h - 98.0,
    );
    draw_rectangle(
        field.x,
        field.y,
        field.w,
        field.h,
        Color::new(0.018, 0.040, 0.045, 1.0),
    );
    for index in 0..18 {
        let x = field.x + ((index * 83) % 760) as f32;
        let y = field.y + ((index * 47) % 430) as f32;
        draw_line(
            x,
            y,
            x + 13.0,
            y - 18.0,
            2.0,
            Color::new(0.12, 0.28, 0.24, 0.45),
        );
        draw_circle(x + 7.0, y + 3.0, 2.0, Color::new(0.17, 0.42, 0.35, 0.55));
    }
    draw_circle(690.0, 226.0, 70.0, Color::new(0.04, 0.14, 0.15, 0.35));
    draw_circle_lines(690.0, 226.0, 70.0, 2.0, Color::new(0.13, 0.34, 0.34, 0.4));
}

fn draw_service_paths(campaign: &CampaignState, view: ColonyView) {
    let hub = view.plot_center(SETTLEMENT_CENTER);
    for building in &campaign.colony.buildings {
        if building.position == SETTLEMENT_CENTER {
            continue;
        }
        let target = view.plot_center(building.position);
        draw_line(
            hub.x,
            hub.y + 9.0,
            target.x,
            target.y + 9.0,
            10.0,
            Color::new(0.13, 0.19, 0.18, 0.90),
        );
        draw_line(
            hub.x,
            hub.y + 8.0,
            target.x,
            target.y + 8.0,
            2.0,
            Color::new(0.34, 0.42, 0.36, 0.52),
        );
    }
}

fn first_hour_destination(campaign: &CampaignState) -> Option<Vec2> {
    campaign
        .first_hour
        .colony_guidance_target()
        .and_then(|id| crate::colony_exploration::npc_position(campaign, id))
}

fn draw_first_hour_route(
    campaign: &CampaignState,
    explorer: &crate::colony_exploration::ColonyExplorer,
    view: ColonyView,
) {
    let Some(destination) = first_hour_destination(campaign) else {
        return;
    };
    let start = view.world_center(explorer.position());
    let end = view.world_center(destination);
    let direction = (end - start).normalize_or_zero();
    if direction.length_squared() < 0.01 {
        return;
    }
    let route_start = start + direction * 20.0;
    let route_end = end - direction * 22.0;
    let route_length = route_start.distance(route_end);
    let dash = 11.0;
    let gap = 8.0;
    let mut distance = 0.0;
    while distance < route_length {
        let from = route_start + direction * distance;
        let to = route_start + direction * (distance + dash).min(route_length);
        draw_line(
            from.x,
            from.y,
            to.x,
            to.y,
            3.0,
            Color::new(0.95, 0.78, 0.30, 0.72),
        );
        distance += dash + gap;
    }
    let normal = vec2(-direction.y, direction.x);
    let arrow_base = end - direction * 14.0;
    draw_line(
        end.x,
        end.y,
        arrow_base.x + normal.x * 8.0,
        arrow_base.y + normal.y * 8.0,
        3.0,
        Color::new(0.95, 0.78, 0.30, 0.92),
    );
    draw_line(
        end.x,
        end.y,
        arrow_base.x - normal.x * 8.0,
        arrow_base.y - normal.y * 8.0,
        3.0,
        Color::new(0.95, 0.78, 0.30, 0.92),
    );
}

#[cfg(test)]
mod tests;
