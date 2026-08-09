//! Mirexis executable entry point.

use macroquad::prelude::*;
use macroquad_toolkit::capture;

mod action_preview;
mod action_preview_ui;
mod battle_log_ui;
mod briefing_intel_ui;
mod briefing_loadout_ui;
mod campaign;
mod class_action_ui;
mod class_actions;
mod class_training;
mod colony;
mod colony_ui;
mod combat_feedback;
mod cover_actions;
mod cover_ui;
mod danger_rating;
mod data;
mod defense_objective;
mod enemy_abilities;
mod enemy_intent;
mod enemy_intent_ui;
mod equipment_actions;
mod equipment_catalog;
mod equipment_ui;
mod game;
mod gene_lab_ui;
mod grid_ui;
mod hazard_ui;
mod hazards;
mod help_ui;
mod map_variants;
mod objective_ui;
mod operation_modifiers;
mod overwatch;
mod persistence;
mod phase_refresh;
mod phase_replay;
mod reinforcement_ui;
mod reinforcements;
mod relationships;
mod roster_ui;
mod state;
mod strategy;
mod strategy_choices;
mod strategy_events;
mod strategy_rewards;
mod tactical;
mod tactical_ai;
mod tactical_unit_ui;
mod trauma;
mod ui;
mod ui_action;
mod ui_debrief;
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
