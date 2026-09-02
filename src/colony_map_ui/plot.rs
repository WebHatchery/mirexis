//! Colony plot ground and depth-sorted vertical content.

use super::{building_art, identity_art, terrain, ColonyView};
use crate::campaign::CampaignState;
use crate::colony::BuildingKind;
use crate::visual_assets::VisualCatalog;
use macroquad::prelude::*;
use macroquad_toolkit::assets::AssetManager;

pub(super) fn draw_ground(
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
}

pub(super) fn draw_contents(
    campaign: &CampaignState,
    assets: &AssetManager,
    visuals: &VisualCatalog,
    view: ColonyView,
    position: [i32; 2],
    planning_site: Option<[i32; 2]>,
) {
    let center = view.plot_center(position);
    let building = campaign.colony.building_at(position);
    let project = campaign.colony.project_at(position);
    let occupied = building.is_some() || project.is_some();
    let planning_clearance =
        planning_site.is_some_and(|anchor| in_clearance_zone(anchor, position));
    crate::world_art::draw_colony_dressing(
        assets,
        visuals,
        view,
        position,
        occupied,
        planning_clearance,
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
        match building.kind {
            BuildingKind::Waystation => {
                building_art::draw_waystation(center, view.zoom, powered, building.damaged)
            }
            BuildingKind::Commons => {
                building_art::draw_commons(center, view.zoom, powered, building.damaged)
            }
            BuildingKind::RelayMast => {
                building_art::draw_relay_mast(center, view.zoom, powered, building.damaged)
            }
            BuildingKind::Watchtower => {
                building_art::draw_watchtower(center, view.zoom, powered, building.damaged)
            }
            BuildingKind::RedoubtArsenal => {
                building_art::draw_redoubt_arsenal(center, view.zoom, powered, building.damaged)
            }
            BuildingKind::ChoirGarden => {
                building_art::draw_choir_garden(center, view.zoom, powered, building.damaged)
            }
            BuildingKind::ThresholdSpire => {
                building_art::draw_threshold_spire(center, view.zoom, powered, building.damaged)
            }
            BuildingKind::ResearchAnnex => {
                building_art::draw_research_annex(center, view.zoom, powered, building.damaged)
            }
            BuildingKind::SalvageYard => {
                building_art::draw_salvage_yard(center, view.zoom, powered, building.damaged)
            }
            _ => {}
        }
        identity_art::draw_ambient(building.kind, center, view.zoom, powered, building.damaged);
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

pub(super) fn in_clearance_zone(anchor: [i32; 2], position: [i32; 2]) -> bool {
    (anchor[0] - position[0]).abs() <= 1 && (anchor[1] - position[1]).abs() <= 1
}

pub(super) fn building_index(kind: BuildingKind) -> usize {
    match kind {
        BuildingKind::CommandCentre => 0,
        BuildingKind::Barracks => 1,
        BuildingKind::Infirmary => 2,
        BuildingKind::Workshop => 3,
        BuildingKind::Barricade => 4,
        BuildingKind::Hydroponics => 5,
        BuildingKind::PowerPlant => 6,
        BuildingKind::GeneLab | BuildingKind::ResearchAnnex | BuildingKind::Waystation => 7,
        BuildingKind::SalvageYard => 3,
        BuildingKind::Commons | BuildingKind::RedoubtArsenal => 1,
        BuildingKind::RelayMast | BuildingKind::Watchtower => 6,
        BuildingKind::ChoirGarden => 5,
        BuildingKind::ThresholdSpire => 7,
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
