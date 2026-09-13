//! Integration tests for the game orchestration modules.

#[path = "support/test_prelude.rs"]
pub mod test_prelude;

#[path = "game/audio_flow.rs"]
mod audio_flow;
#[path = "game/capture_scenes.rs"]
mod capture_scenes;
#[path = "game/colony_flow.rs"]
mod colony_flow;
#[path = "game/first_hour_flow.rs"]
mod first_hour_flow;
#[path = "game/input.rs"]
mod input;
#[path = "game/persistence_io.rs"]
mod persistence_io;
#[path = "game/session_flow.rs"]
mod session_flow;
