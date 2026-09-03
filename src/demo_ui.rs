//! Three-operation demo finale.

use crate::campaign::CampaignState;
use crate::ui::{draw_ui_text_ex, UiAction, LOGICAL_HEIGHT, LOGICAL_WIDTH};
use crate::ui_widgets::button;
use crate::visual_assets::VisualCatalog;
use macroquad::prelude::*;
use macroquad_toolkit::assets::AssetManager;
use macroquad_toolkit::prelude::{dark, draw_surface, SurfaceStyle, TextStyle};
use macroquad_toolkit::ui::VirtualUi;

pub(crate) fn draw(
    campaign: &CampaignState,
    assets: &AssetManager,
    visuals: &VisualCatalog,
    ui: &VirtualUi,
) -> Vec<UiAction> {
    let mut actions = Vec::new();
    draw_rectangle(
        0.0,
        0.0,
        LOGICAL_WIDTH,
        LOGICAL_HEIGHT,
        Color::new(0.018, 0.030, 0.040, 1.0),
    );
    crate::world_art::draw_title_dressing(assets, visuals);
    draw_rectangle(
        0.0,
        0.0,
        LOGICAL_WIDTH,
        LOGICAL_HEIGHT,
        Color::new(0.01, 0.025, 0.03, 0.76),
    );

    let panel = Rect::new(250.0, 92.0, 780.0, 536.0);
    draw_surface(
        panel,
        &SurfaceStyle::new(Color::new(0.035, 0.065, 0.070, 0.98)).with_border(2.0, dark::POSITIVE),
    );
    draw_ui_text_ex(
        "DEMO OPERATIONS COMPLETE",
        346.0,
        168.0,
        TextStyle::new(34.0, Color::new(0.80, 1.0, 0.92, 1.0)).params(),
    );
    draw_ui_text_ex(
        "THE COLONY SURVIVES ITS FIRST CONTACT WITH MIREXIS.",
        352.0,
        218.0,
        TextStyle::new(16.0, dark::ACCENT).params(),
    );
    draw_ui_text_ex(
        &format!(
            "{} operations  //  {} colonists  //  {} materials recovered",
            campaign.operations_completed,
            campaign.roster.len(),
            campaign.colony.resources.materials
        ),
        352.0,
        270.0,
        TextStyle::new(15.0, dark::TEXT).params(),
    );
    draw_ui_text_ex(
        "The full campaign continues with colony assaults, faction Contact,",
        352.0,
        332.0,
        TextStyle::new(17.0, dark::TEXT_DIM).params(),
    );
    draw_ui_text_ex(
        "new recruits, deeper mutations, and the truth beneath the mire.",
        352.0,
        360.0,
        TextStyle::new(17.0, dark::TEXT_DIM).params(),
    );
    draw_ui_text_ex(
        "THANK YOU FOR PLAYING  //  FOLLOW MIREXIS ON ITCH.IO",
        352.0,
        430.0,
        TextStyle::new(15.0, dark::WARNING).params(),
    );

    if button(
        Rect::new(478.0, 512.0, 324.0, 58.0),
        "RETURN TO TITLE",
        true,
        crate::ui::pointer_position(ui),
    ) {
        actions.push(UiAction::ReturnToTitle);
    }
    draw_ui_text_ex(
        "TAP RETURN TO TITLE  //  ENTER / SPACE / PAD A",
        456.0,
        596.0,
        TextStyle::new(12.0, dark::TEXT_DIM).params(),
    );
    actions
}
