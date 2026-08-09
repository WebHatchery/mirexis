//! Mirexis executable entry point.

use macroquad::prelude::*;
use macroquad_toolkit::capture;

mod data;
mod game;
mod state;
mod ui;

use game::Game;

fn window_conf() -> Conf {
    capture::capture_window_conf(
        "MIREXIS",
        "Mirexis",
        ui::LOGICAL_WIDTH as i32,
        ui::LOGICAL_HEIGHT as i32,
    )
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new().await;

    if let Some(config) = capture::CaptureConfig::from_env("MIREXIS") {
        capture::run_capture(&config, |dt| {
            game.update(dt);
            game.draw();
        })
        .await;
        return;
    }

    loop {
        let dt = get_frame_time().min(0.1);
        game.update(dt);
        game.draw();
        next_frame().await;
    }
}
