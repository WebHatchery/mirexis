//! Mirexis executable entry point.

use macroquad::prelude::*;
use macroquad_toolkit::capture;

mod campaign;
mod class_actions;
mod colony;
mod colony_ui;
mod data;
mod equipment_actions;
mod equipment_ui;
mod game;
mod grid_ui;
mod persistence;
mod reinforcements;
mod roster_ui;
mod state;
mod strategy;
mod tactical;
mod tactical_ai;
mod ui;
mod ui_widgets;

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
        game.begin_capture_scene(&config.scene);
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
