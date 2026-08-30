//! Portrait-led title tableau and launch controls.

use crate::data::GameData;
use crate::ui::{draw_ui_text_ex, TitleDrawContext, UiAction, LOGICAL_HEIGHT, LOGICAL_WIDTH};
use crate::ui_widgets::button;
use crate::visual_assets::VisualCatalog;
use macroquad::prelude::*;
use macroquad_toolkit::assets::AssetManager;
use macroquad_toolkit::prelude::{dark, draw_text_block_ex, TextStyle};

pub(crate) fn draw(context: TitleDrawContext<'_>) -> Vec<UiAction> {
    let TitleDrawContext {
        data,
        save_exists,
        assets,
        visuals,
        ui,
        controller_focus_continue,
        hover_preview,
        new_campaign_armed,
    } = context;
    let mut actions = Vec::new();
    let mouse = if hover_preview {
        vec2(198.0, 512.0)
    } else {
        crate::ui::pointer_position(ui)
    };
    draw_background();
    crate::world_art::draw_title_dressing(assets, visuals);
    draw_character_tableau(data, assets, visuals);
    draw_title_copy(data);
    if button(
        Rect::new(76.0, 486.0, 244.0, 52.0),
        if new_campaign_armed {
            "CONFIRM NEW COLONY"
        } else {
            "NEW OPERATION"
        },
        true,
        mouse,
    ) {
        actions.push(UiAction::StartMission);
    }
    if button(
        Rect::new(334.0, 486.0, 244.0, 52.0),
        "CONTINUE",
        save_exists,
        mouse,
    ) {
        actions.push(UiAction::Continue);
    }
    if button(
        Rect::new(334.0, 654.0, 244.0, 36.0),
        "SETTINGS",
        true,
        mouse,
    ) {
        actions.push(UiAction::ToggleSettings);
    }
    if let Some(focus_continue) = controller_focus_continue {
        let focus = if focus_continue {
            Rect::new(330.0, 482.0, 252.0, 60.0)
        } else {
            Rect::new(72.0, 482.0, 252.0, 60.0)
        };
        draw_rectangle_lines(focus.x, focus.y, focus.w, focus.h, 2.0, dark::WARNING);
        draw_ui_text_ex(
            "PAD // LEFT/RIGHT SELECT  ·  A CONFIRM",
            78.0,
            568.0,
            TextStyle::new(12.0, dark::WARNING).params(),
        );
    }
    draw_ui_text_ex(
        "PHASE ONE  /  ISOLATION",
        78.0,
        600.0,
        TextStyle::new(13.0, Color::new(0.42, 0.78, 0.72, 1.0)).params(),
    );
    draw_ui_text_ex(
        "KEEP THE COLONY ALIVE. DECIDE WHAT IT BECOMES.",
        78.0,
        626.0,
        TextStyle::new(15.0, Color::new(0.58, 0.68, 0.67, 1.0)).params(),
    );
    actions
}

fn draw_background() {
    draw_rectangle(
        0.0,
        0.0,
        LOGICAL_WIDTH,
        LOGICAL_HEIGHT,
        Color::new(0.018, 0.030, 0.040, 1.0),
    );
    draw_rectangle(
        0.0,
        0.0,
        660.0,
        LOGICAL_HEIGHT,
        Color::new(0.028, 0.050, 0.058, 0.96),
    );
    draw_triangle(
        vec2(560.0, 0.0),
        vec2(760.0, 0.0),
        vec2(560.0, LOGICAL_HEIGHT),
        Color::new(0.028, 0.050, 0.058, 0.96),
    );
    draw_circle(1015.0, 260.0, 270.0, Color::new(0.07, 0.23, 0.22, 0.16));
    draw_circle(1015.0, 260.0, 210.0, Color::new(0.12, 0.38, 0.32, 0.08));
    for index in 0..22 {
        let x = index as f32 * 76.0 - 220.0;
        draw_line(
            x,
            LOGICAL_HEIGHT,
            x + 430.0,
            0.0,
            2.0,
            Color::new(0.10, 0.28, 0.28, 0.22),
        );
    }
    draw_rectangle(
        0.0,
        672.0,
        LOGICAL_WIDTH,
        48.0,
        Color::new(0.01, 0.02, 0.026, 1.0),
    );
    draw_line(
        0.0,
        672.0,
        LOGICAL_WIDTH,
        672.0,
        2.0,
        Color::new(0.20, 0.64, 0.55, 0.52),
    );
}

fn draw_title_copy(data: &GameData) {
    draw_ui_text_ex(
        "WEBHATCHERY / TACTICAL FRONTIER",
        78.0,
        76.0,
        TextStyle::new(12.0, Color::new(0.36, 0.68, 0.64, 1.0)).params(),
    );
    draw_ui_text_ex(
        "MIREXIS",
        70.0,
        184.0,
        TextStyle::new(82.0, Color::new(0.80, 1.0, 0.92, 1.0)).params(),
    );
    draw_rectangle(78.0, 204.0, 92.0, 4.0, Color::new(0.34, 0.92, 0.72, 1.0));
    draw_rectangle(178.0, 204.0, 36.0, 4.0, Color::new(0.96, 0.34, 0.29, 1.0));
    draw_rectangle(222.0, 204.0, 36.0, 4.0, Color::new(0.74, 0.44, 1.0, 1.0));
    draw_ui_text_ex(
        "ONE COLONY. THREE POWERS. NO SAFE GROUND.",
        78.0,
        244.0,
        TextStyle::new(18.0, Color::new(0.45, 0.88, 0.78, 1.0)).params(),
    );
    draw_text_block_ex(
        &data.mission.briefing,
        78.0,
        282.0,
        480.0,
        120.0,
        TextStyle::new(18.0, dark::TEXT_DIM)
            .with_line_gap(6.0)
            .with_macroquad_font(),
        12.0,
    );
    draw_ui_text_ex(
        "DIRECTORATE  //  BROOD  //  ASCENDANTS",
        78.0,
        438.0,
        TextStyle::new(12.0, Color::new(0.48, 0.62, 0.62, 1.0)).params(),
    );
}

fn draw_character_tableau(data: &GameData, assets: &AssetManager, visuals: &VisualCatalog) {
    let cards = [
        (
            Rect::new(690.0, 174.0, 190.0, 356.0),
            1_usize,
            Color::new(0.90, 0.38, 0.28, 1.0),
        ),
        (
            Rect::new(1060.0, 174.0, 170.0, 356.0),
            2_usize,
            Color::new(0.64, 0.40, 0.90, 1.0),
        ),
        (
            Rect::new(842.0, 96.0, 250.0, 474.0),
            0_usize,
            Color::new(0.28, 0.88, 0.72, 1.0),
        ),
    ];
    for (rect, index, accent) in cards {
        let unit = data.roster.get(index).or_else(|| data.roster.first());
        let (id, name, role) = unit.map_or(("missing", "COLONIST", "SCOUT"), |unit| {
            (unit.id.as_str(), unit.name.as_str(), unit.role.as_str())
        });
        crate::portrait_ui::draw_character_portrait(assets, visuals, rect, id, name, accent);
        draw_rectangle(
            rect.x,
            rect.bottom() - 46.0,
            rect.w,
            46.0,
            Color::new(0.015, 0.035, 0.040, 0.94),
        );
        draw_ui_text_ex(
            &name.to_uppercase(),
            rect.x + 12.0,
            rect.bottom() - 22.0,
            TextStyle::new(
                if rect.w > 200.0 { 18.0 } else { 14.0 },
                Color::new(0.82, 0.94, 0.90, 1.0),
            )
            .params(),
        );
        draw_ui_text_ex(
            &role.to_uppercase(),
            rect.x + 12.0,
            rect.bottom() - 7.0,
            TextStyle::new(10.0, accent).params(),
        );
    }
    draw_ui_text_ex(
        "THE FIRST SURVEY TEAM",
        848.0,
        80.0,
        TextStyle::new(12.0, Color::new(0.52, 0.74, 0.70, 1.0)).params(),
    );
}
