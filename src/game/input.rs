//! Keyboard-to-intent translation for each application state.

use super::{AppState, Game};
use crate::ui::{self, UiAction};
use macroquad::prelude::{is_key_pressed, KeyCode};
use macroquad_toolkit::prelude::InputState;

impl Game {
    pub(super) fn capture_input(&mut self) -> bool {
        let input = InputState::capture();
        match self.state {
            AppState::Title => {
                if input.space_pressed || is_key_pressed(KeyCode::Enter) {
                    self.events.push(UiAction::StartMission);
                }
            }
            AppState::Colony => {
                if input.escape_pressed {
                    self.events.push(UiAction::ReturnToTitle);
                }
            }
            AppState::Roster | AppState::GeneLab => {
                if input.escape_pressed {
                    self.events.push(UiAction::ReturnToColony);
                }
            }
            AppState::MissionBriefing => {
                if input.escape_pressed {
                    self.events.push(UiAction::ReturnToColony);
                }
                if input.space_pressed || is_key_pressed(KeyCode::Enter) {
                    self.events.push(UiAction::DeployMission);
                }
            }
            AppState::Tactical => {
                if self.phase_replay.is_active() {
                    if input.space_pressed || input.escape_pressed {
                        self.phase_replay.clear();
                    }
                    return true;
                }
                if input.escape_pressed {
                    self.events.push(if self.show_tactical_help {
                        UiAction::ToggleTacticalHelp
                    } else if self.show_battle_log {
                        UiAction::ToggleBattleLog
                    } else if self.targeting.is_some() {
                        UiAction::CancelTargeting
                    } else {
                        UiAction::ReturnToTitle
                    });
                }
                if is_key_pressed(KeyCode::H) {
                    self.events.push(UiAction::ToggleTacticalHelp);
                }
                if is_key_pressed(KeyCode::B) {
                    self.events.push(UiAction::ToggleBattleLog);
                }
                if !self.show_tactical_help && !self.show_battle_log {
                    if is_key_pressed(KeyCode::S) {
                        self.events.push(UiAction::Save);
                    }
                    if is_key_pressed(KeyCode::L) {
                        self.events.push(UiAction::Load);
                    }
                    if is_key_pressed(KeyCode::Enter) {
                        self.events.push(UiAction::EndPhase);
                    }
                    if self.targeting.is_none() {
                        if let Some((dx, dy)) = ui::tile_move_from_keys() {
                            self.session.move_selection(dx, dy);
                        }
                    }
                }
            }
            AppState::Debrief => {
                if input.space_pressed || is_key_pressed(KeyCode::Enter) {
                    self.events.push(UiAction::ReturnToColony);
                }
            }
        }
        false
    }
}
