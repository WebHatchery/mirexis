//! Physical colony acknowledgements for the first two resolved operations.

use crate::campaign::CampaignState;
use crate::colony_map_ui::view::ColonyView;
use macroquad::prelude::*;

pub(crate) fn draw(campaign: &CampaignState, view: ColonyView) {
    if campaign.operations_completed >= 1 {
        draw_refuge_signal(campaign, view);
    }
    if campaign.operations_completed >= 2 {
        draw_outer_barricade(campaign, view);
    }
}

fn draw_refuge_signal(campaign: &CampaignState, view: ColonyView) {
    let center = view.plot_center([12, 8]);
    let won = campaign.first_hour.first_outcome_won == Some(true);
    let accent = if won {
        Color::new(0.30, 0.94, 0.72, 0.92)
    } else {
        Color::new(0.96, 0.48, 0.24, 0.92)
    };
    draw_line(
        center.x,
        center.y - 10.0,
        center.x,
        center.y - 78.0,
        3.0,
        accent,
    );
    draw_circle(center.x, center.y - 78.0, 7.0, accent);
    draw_circle_lines(center.x, center.y - 78.0, 16.0, 2.0, accent);
    label(
        center + vec2(-62.0, -94.0),
        if won {
            "REFUGE LINK RESTORED"
        } else {
            "REFUGE MEMORIAL"
        },
        accent,
    );
}

fn draw_outer_barricade(campaign: &CampaignState, view: ColonyView) {
    let center = view.plot_center([8, 12]);
    let breached = outer_route_is_breached(campaign.first_hour.second_outcome_won);
    let accent = outer_route_accent(campaign.first_hour.second_outcome_won);
    for offset in [-26.0, 0.0, 26.0] {
        draw_rectangle(center.x + offset - 10.0, center.y - 8.0, 20.0, 10.0, accent);
        draw_circle(center.x + offset, center.y - 13.0, 4.0, darken(accent));
    }
    if breached {
        draw_line(
            center.x - 42.0,
            center.y + 4.0,
            center.x - 13.0,
            center.y + 4.0,
            4.0,
            accent,
        );
        draw_line(
            center.x + 13.0,
            center.y + 4.0,
            center.x + 42.0,
            center.y + 4.0,
            4.0,
            accent,
        );
    } else {
        draw_line(
            center.x - 42.0,
            center.y + 4.0,
            center.x + 42.0,
            center.y + 4.0,
            4.0,
            accent,
        );
    }
    label(
        center + vec2(-58.0, 26.0),
        outer_route_label(campaign.first_hour.second_outcome_won),
        accent,
    );
}

fn outer_route_is_breached(second_outcome_won: Option<bool>) -> bool {
    second_outcome_won == Some(false)
}

fn outer_route_accent(second_outcome_won: Option<bool>) -> Color {
    if outer_route_is_breached(second_outcome_won) {
        Color::new(0.96, 0.48, 0.24, 0.92)
    } else {
        Color::new(0.92, 0.72, 0.28, 0.96)
    }
}

fn outer_route_label(second_outcome_won: Option<bool>) -> &'static str {
    match second_outcome_won {
        Some(true) => "OUTER ROUTES HARDENED",
        Some(false) => "OUTER ROUTE BREACHED",
        None => "OUTER ROUTE UNRESOLVED",
    }
}

fn label(origin: Vec2, value: &str, accent: Color) {
    let width = measure_text(value, None, 11, 1.0).width + 12.0;
    draw_rectangle(
        origin.x,
        origin.y,
        width,
        18.0,
        Color::new(0.02, 0.05, 0.055, 0.92),
    );
    draw_rectangle_lines(origin.x, origin.y, width, 18.0, 1.0, accent);
    draw_text(value, origin.x + 6.0, origin.y + 13.0, 11.0, accent);
}

fn darken(color: Color) -> Color {
    Color::new(color.r * 0.28, color.g * 0.28, color.b * 0.28, color.a)
}

#[cfg(test)]
mod tests;
