//! Interactive isometric colony map and modular settlement building art.

use crate::campaign::CampaignState;
use crate::colony::{BuildingKind, COLONY_HEIGHT, COLONY_WIDTH, SETTLEMENT_CENTER};
use crate::grid_ui::WorldCamera;
use crate::tactical::{UnitAnimationState, UnitFacing};
use crate::ui::UiAction;
use crate::ui_widgets::button;
use crate::visual_assets::VisualCatalog;
use macroquad::prelude::*;
use macroquad_toolkit::assets::AssetManager;
use macroquad_toolkit::prelude::{dark, draw_surface_with_title, SurfaceStyle, TextStyle};
use macroquad_toolkit::ui::VirtualUi;

pub(crate) const COLONY_HALF_WIDTH: f32 = 26.0;
pub(crate) const COLONY_HALF_HEIGHT: f32 = 13.0;

mod terrain;
mod view;
use view::ColonyView;

pub(crate) fn draw(
    campaign: &CampaignState,
    assets: &AssetManager,
    visuals: &VisualCatalog,
    ui: &VirtualUi,
    camera: &mut WorldCamera,
    mouse: Vec2,
    actions: &mut Vec<UiAction>,
) -> bool {
    let panel = Rect::new(10.0, 74.0, 842.0, 608.0);
    draw_surface_with_title(
        panel,
        Some("MIREXIS SETTLEMENT // EXPANSION CAMERA"),
        &SurfaceStyle::new(Color::new(0.026, 0.047, 0.053, 0.98))
            .with_border(1.0, Color::new(0.20, 0.50, 0.48, 0.9))
            .with_inner_border(6.0, 1.0, Color::new(0.12, 0.28, 0.27, 0.7))
            .with_header(28.0, Color::new(0.06, 0.10, 0.11, 1.0)),
        TextStyle::new(13.0, dark::TEXT),
    );
    let viewport = Rect::new(
        panel.x + 8.0,
        panel.y + 32.0,
        panel.w - 16.0,
        panel.h - 102.0,
    );
    let camera_control_clicked = crate::camera_controls::draw_zoom(
        camera,
        viewport,
        mouse,
        camera_controls_origin(panel),
        !camera.primary_gesture_active(),
    );
    let camera_dragged = camera.update(viewport, mouse);
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
            let x = sum - y;
            if !(0..COLONY_WIDTH).contains(&x) {
                continue;
            }
            if view.visible([x, y]) {
                draw_plot(
                    campaign,
                    assets,
                    visuals,
                    view,
                    [x, y],
                    hovered == Some([x, y]),
                    planning_site,
                );
            }
        }
    }
    draw_inhabitants(campaign, assets, visuals, view);
    draw_ending_manifestation(campaign, assets, visuals, view);
    crate::ui::set_ui_clip(ui, None);
    draw_hover_card(campaign, hovered, camera.pending_colony_plot());
    handle_plot_click(campaign, camera, hovered, suppress_plot_click, actions);
    draw_build_controls(campaign, mouse, actions);
    draw_text(
        format!(
            "DRAG MAP TO PAN // WHEEL OR -/+ ZOOM // {}%",
            (camera.zoom * 100.0) as i32
        ),
        panel.x + 14.0,
        panel.bottom() - 76.0,
        11.0,
        Color::new(0.46, 0.68, 0.66, 1.0),
    );
    camera_dragged
}

fn camera_controls_origin(panel: Rect) -> Vec2 {
    vec2(panel.right() - 104.0, panel.bottom() - 65.0)
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
                    building_index(BuildingKind::Barricade),
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
                    building_index(BuildingKind::Hydroponics),
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
                building_index(BuildingKind::GeneLab),
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

fn draw_plot(
    campaign: &CampaignState,
    assets: &AssetManager,
    visuals: &VisualCatalog,
    view: ColonyView,
    position: [i32; 2],
    hovered: bool,
    planning_site: Option<[i32; 2]>,
) {
    let center = view.plot_center(position);
    let building = campaign.colony.building_at(position);
    let project = campaign.colony.project_at(position);
    let occupied = building.is_some() || project.is_some();
    let planning_clearance =
        planning_site.is_some_and(|anchor| in_clearance_zone(anchor, position));
    let top = if hovered {
        Color::new(0.18, 0.39, 0.34, 1.0)
    } else if planning_clearance {
        Color::new(0.12, 0.31, 0.27, 1.0)
    } else if occupied {
        Color::new(0.12, 0.28, 0.25, 1.0)
    } else {
        Color::new(0.085, 0.19, 0.18, 1.0)
    };
    terrain::draw_ground(
        assets,
        visuals,
        view,
        position,
        top,
        hovered || planning_clearance,
        occupied,
    );
    draw_diamond_outline(
        view,
        center,
        if hovered {
            Color::new(0.42, 0.82, 0.69, 0.92)
        } else if planning_clearance {
            Color::new(0.30, 0.67, 0.57, 0.82)
        } else {
            Color::new(0.18, 0.43, 0.37, 0.72)
        },
    );
    draw_line(
        center.x - view.half_width,
        center.y,
        center.x,
        center.y + view.half_height,
        1.0,
        Color::new(0.20, 0.42, 0.37, 0.6),
    );
    draw_line(
        center.x,
        center.y + view.half_height,
        center.x + view.half_width,
        center.y,
        1.0,
        Color::new(0.05, 0.13, 0.13, 0.9),
    );
    if let Some(building) = building.filter(|building| building.position == position) {
        let powered = !building.damaged && campaign.colony.building_is_powered(&building.id);
        let index = building_index(building.kind) + usize::from(building.damaged) * 8;
        let tint = if powered || building.damaged {
            WHITE
        } else {
            Color::new(0.42, 0.48, 0.52, 1.0)
        };
        visuals.draw_atlas_cell(
            assets,
            &visuals.colony,
            index,
            Rect::new(
                center.x - 35.0 * view.zoom,
                center.y - 52.0 * view.zoom,
                70.0 * view.zoom,
                70.0 * view.zoom,
            ),
            tint,
        );
        draw_building_state(center, building.damaged, powered);
    } else if let Some(project) = project.filter(|project| project.position == position) {
        visuals.draw_atlas_cell(
            assets,
            &visuals.colony,
            building_index(project.kind),
            Rect::new(
                center.x - 33.0 * view.zoom,
                center.y - 49.0 * view.zoom,
                66.0 * view.zoom,
                66.0 * view.zoom,
            ),
            Color::new(0.35, 0.78, 0.72, 0.22),
        );
        draw_project(center, project.kind);
    } else if planning_site == Some(position) {
        draw_blueprint(center, campaign.colony.planned_construction);
    }
}

fn in_clearance_zone(anchor: [i32; 2], position: [i32; 2]) -> bool {
    (anchor[0] - position[0]).abs() <= 1 && (anchor[1] - position[1]).abs() <= 1
}

fn building_index(kind: BuildingKind) -> usize {
    match kind {
        BuildingKind::CommandCentre => 0,
        BuildingKind::Barracks => 1,
        BuildingKind::Infirmary => 2,
        BuildingKind::Workshop => 3,
        BuildingKind::Barricade => 4,
        BuildingKind::Hydroponics => 5,
        BuildingKind::PowerPlant => 6,
        BuildingKind::GeneLab => 7,
    }
}

fn draw_building_state(center: Vec2, damaged: bool, powered: bool) {
    if damaged {
        draw_poly(
            center.x + 24.0,
            center.y - 54.0,
            3,
            7.0,
            0.0,
            Color::new(1.0, 0.24, 0.18, 0.92),
        );
        draw_line(
            center.x + 24.0,
            center.y - 46.0,
            center.x + 24.0,
            center.y - 32.0,
            2.0,
            Color::new(1.0, 0.34, 0.20, 0.72),
        );
    } else if powered {
        draw_circle(
            center.x + 29.0,
            center.y - 43.0,
            3.0,
            Color::new(0.42, 1.0, 0.82, 0.95),
        );
        draw_circle(
            center.x + 29.0,
            center.y - 43.0,
            8.0,
            Color::new(0.18, 0.82, 0.68, 0.14),
        );
    } else {
        draw_poly(
            center.x + 28.0,
            center.y - 42.0,
            4,
            6.0,
            45.0,
            Color::new(0.96, 0.66, 0.18, 0.92),
        );
        draw_line(
            center.x + 24.0,
            center.y - 46.0,
            center.x + 32.0,
            center.y - 38.0,
            2.0,
            Color::new(0.08, 0.10, 0.12, 1.0),
        );
    }
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

fn draw_inhabitants(
    campaign: &CampaignState,
    assets: &AssetManager,
    visuals: &VisualCatalog,
    view: ColonyView,
) {
    let stations = [[8, 10], [7, 9], [10, 8], [13, 9], [11, 12]];
    for (index, character) in campaign.roster.iter().take(5).enumerate() {
        let center = view.plot_center(stations[index]);
        let side = if index % 2 == 0 { -1.0 } else { 1.0 };
        let person = center + vec2(side * 8.0, 9.0);
        draw_ellipse(
            person.x,
            person.y + 4.0,
            6.0,
            2.5,
            0.0,
            Color::new(0.0, 0.0, 0.0, 0.38),
        );
        visuals.draw_unit_pose(
            assets,
            &character.id,
            &character.name,
            if side < 0.0 {
                UnitFacing::SouthEast
            } else {
                UnitFacing::SouthWest
            },
            if index % 3 == 0 {
                UnitAnimationState::Move
            } else {
                UnitAnimationState::Idle
            },
            Rect::new(person.x - 17.0, person.y - 16.0, 34.0, 24.0),
            Color::new(0.92, 0.98, 0.95, 0.96),
        );
    }
}

fn draw_diamond_outline(view: ColonyView, center: Vec2, color: Color) {
    let points = [
        vec2(center.x, center.y - view.half_height),
        vec2(center.x + view.half_width, center.y),
        vec2(center.x, center.y + view.half_height),
        vec2(center.x - view.half_width, center.y),
    ];
    for index in 0..points.len() {
        let next = (index + 1) % points.len();
        draw_line(
            points[index].x,
            points[index].y,
            points[next].x,
            points[next].y,
            1.0,
            color,
        );
    }
}

fn draw_project(center: Vec2, kind: BuildingKind) {
    draw_rectangle_lines(
        center.x - 25.0,
        center.y - 30.0,
        50.0,
        27.0,
        2.0,
        Color::new(0.96, 0.72, 0.23, 0.9),
    );
    draw_line(
        center.x - 25.0,
        center.y - 30.0,
        center.x + 25.0,
        center.y - 3.0,
        1.0,
        Color::new(0.96, 0.72, 0.23, 0.7),
    );
    draw_text(
        kind.name(),
        center.x - 30.0,
        center.y - 35.0,
        10.0,
        Color::new(1.0, 0.82, 0.40, 1.0),
    );
}

fn draw_blueprint(center: Vec2, kind: BuildingKind) {
    draw_rectangle(
        center.x - 20.0,
        center.y - 23.0,
        40.0,
        20.0,
        Color::new(0.20, 0.84, 0.70, 0.11),
    );
    draw_rectangle_lines(
        center.x - 20.0,
        center.y - 23.0,
        40.0,
        20.0,
        1.0,
        Color::new(0.42, 0.92, 0.78, 0.65),
    );
    draw_text(
        kind.name(),
        center.x - 28.0,
        center.y - 28.0,
        9.0,
        Color::new(0.52, 0.96, 0.84, 1.0),
    );
}

fn draw_hover_card(campaign: &CampaignState, hovered: Option<[i32; 2]>, pending: Option<[i32; 2]>) {
    let Some(position) = hovered else { return };
    let building = campaign.colony.building_at(position);
    let project = campaign.colony.project_at(position);
    let site_unavailable = building.is_none()
        && project.is_none()
        && campaign
            .colony
            .validate_construction_site(position)
            .is_err();
    let text = if let Some(building) = building {
        if building.damaged {
            format!(
                "{} // DAMAGED // REPAIR {} MAT",
                building.kind.name().to_uppercase(),
                building.kind.repair_cost()
            )
        } else if !campaign.colony.building_is_powered(&building.id) {
            format!(
                "{} // OFFLINE: INSUFFICIENT POWER",
                building.kind.name().to_uppercase()
            )
        } else if building.kind == BuildingKind::GeneLab {
            "GENE LAB // TAP TO OPEN EVOLUTION CHAMBER".to_owned()
        } else {
            format!(
                "{} // ONLINE // LEVEL {}",
                building.kind.name().to_uppercase(),
                building.level
            )
        }
    } else if let Some(project) = project {
        format!(
            "{} // UNDER CONSTRUCTION // {} OPERATION",
            project.kind.name().to_uppercase(),
            project.operations_remaining
        )
    } else {
        match campaign.colony.validate_construction_site(position) {
            Ok(()) => format!(
                "OPEN PLOT // BUILD {} // {} MAT",
                campaign.colony.planned_construction.name().to_uppercase(),
                campaign.colony.planned_construction.material_cost()
            ),
            Err(error) => format!("CLEARANCE REQUIRED // {}", error.to_uppercase()),
        }
    };
    let text = if site_unavailable {
        text
    } else if pending == Some(position) {
        format!("TAP AGAIN TO CONFIRM // {text}")
    } else {
        format!("TAP TO ARM // {text}")
    };
    draw_rectangle(
        70.0,
        566.0,
        708.0,
        28.0,
        Color::new(0.025, 0.075, 0.078, 0.94),
    );
    draw_text(&text, 84.0, 585.0, 14.0, Color::new(0.70, 0.92, 0.84, 1.0));
}

fn handle_plot_click(
    campaign: &CampaignState,
    camera: &mut WorldCamera,
    hovered: Option<[i32; 2]>,
    suppress_click: bool,
    actions: &mut Vec<UiAction>,
) {
    if suppress_click || !is_mouse_button_released(MouseButton::Left) {
        return;
    }
    let Some(position) = hovered else { return };
    if campaign.colony.building_at(position).is_none()
        && campaign.colony.project_at(position).is_none()
        && campaign
            .colony
            .validate_construction_site(position)
            .is_err()
    {
        camera.clear_pending_colony_plot();
        return;
    }
    if !camera.confirm_colony_plot(position) {
        return;
    }
    if let Some(building) = campaign.colony.building_at(position) {
        if building.damaged {
            actions.push(UiAction::RepairBuilding(building.id.clone()));
        } else if building.kind == BuildingKind::GeneLab
            && campaign.colony.building_is_powered(&building.id)
        {
            actions.push(UiAction::OpenGeneLab);
        }
    } else if campaign.colony.project_at(position).is_none() {
        actions.push(UiAction::ConstructBuilding(
            campaign.colony.planned_construction,
            position,
        ));
    }
}

fn draw_build_controls(campaign: &CampaignState, mouse: Vec2, actions: &mut Vec<UiAction>) {
    let mut kinds = vec![BuildingKind::Barricade, BuildingKind::PowerPlant];
    if campaign.strategy.contact_complete
        && !campaign
            .colony
            .buildings
            .iter()
            .any(|b| b.kind == BuildingKind::GeneLab)
        && !campaign
            .colony
            .construction_queue
            .iter()
            .any(|p| p.kind == BuildingKind::GeneLab)
    {
        kinds.push(BuildingKind::GeneLab);
    }
    for (index, kind) in kinds.into_iter().enumerate() {
        let selected = campaign.colony.planned_construction == kind;
        if button(
            Rect::new(70.0 + index as f32 * 222.0, 620.0, 210.0, 42.0),
            &format!(
                "{}{} // {} MAT",
                if selected { "> " } else { "" },
                kind.name().to_uppercase(),
                kind.material_cost()
            ),
            true,
            mouse,
        ) {
            actions.push(UiAction::SelectConstruction(kind));
        }
    }
}

#[cfg(test)]
mod tests;
