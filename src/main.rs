//! Mirexis executable entry point.

use macroquad::prelude::*;
use macroquad_toolkit::capture;

use mirexis::game::Game;
use mirexis::ui;

pub const UI_FONT_SIZES: &[u16] = &[
    9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 24, 25, 30, 34, 82,
];

pub fn warm_ui_font_atlases() {
    // Macroquad grows a font atlas by replacing its GPU texture. If that first
    // growth happens after text has already been queued in the same frame, the
    // WebGL batch can retain the deleted texture id. Cache the UI's finite size
    // set before the first frame so atlas replacement happens before any draw.
    let mut characters: Vec<char> = (' '..='~').collect();
    characters.extend(['·', '—']);

    if let Some(font) = macroquad_toolkit::ui::default_ui_font() {
        for size in UI_FONT_SIZES {
            font.populate_font_cache(&characters, *size);
        }
    }
    let macroquad_font = get_default_font();
    for size in UI_FONT_SIZES {
        macroquad_font.populate_font_cache(&characters, *size);
    }
}

pub fn window_conf() -> Conf {
    capture::capture_window_conf(
        "MIREXIS",
        "Mirexis",
        ui::LOGICAL_WIDTH as i32,
        ui::LOGICAL_HEIGHT as i32,
    )
}

#[macroquad::main(window_conf)]
pub async fn main() {
    warm_ui_font_atlases();
    let mut game = Game::new().await;

    if let Some(configs) = capture::CaptureConfig::all_from_env("MIREXIS") {
        for config in configs {
            game.begin_capture_scene(&config.scene);
            capture::run_capture_once(&config, |dt| {
                game.update(dt);
                game.draw();
            })
            .await;
        }
        return;
    }

    loop {
        let dt = get_frame_time().min(0.1);
        game.update(dt);
        game.draw();
        next_frame().await;
    }
}
