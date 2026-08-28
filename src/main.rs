//! Mirexis executable entry point.

use macroquad::prelude::*;
use macroquad_toolkit::capture;

mod action_preview;
mod action_preview_ui;
mod battle_log_ui;
mod briefing_deployment_ui;
mod briefing_intel_ui;
mod briefing_loadout_ui;
mod camera_controls;
mod campaign;
mod class_action_ui;
mod class_actions;
mod class_training;
mod colony;
mod colony_decision_ui;
mod colony_exploration;
mod colony_header_ui;
mod colony_map_ui;
mod colony_ui;
mod combat_feedback;
mod cover_actions;
mod cover_rules;
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
mod formation;
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
mod phase_readiness;
mod phase_refresh;
mod phase_replay;
mod portrait_ui;
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
mod tactical_map_ui;
mod tactical_unit_ui;
mod title_scene_ui;
mod trauma;
mod ui;
mod ui_action;
mod ui_debrief;
mod ui_widgets;
mod visual_assets;

use game::Game;

const UI_FONT_SIZES: &[u16] = &[
    9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 24, 25, 30, 34, 82,
];

fn warm_ui_font_atlases() {
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
