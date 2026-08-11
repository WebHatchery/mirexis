//! Keyboard-to-intent translation for each application state.

use super::{AppState, Game};
use crate::data::Team;
use crate::ui::{self, UiAction};
use macroquad::prelude::{is_key_pressed, KeyCode};
use macroquad_toolkit::prelude::InputState;

impl Game {
    pub(super) fn capture_input(&mut self) -> bool {
        let input = InputState::capture();
        let pad = self.gamepad.capture();
        if pad.connected && !self.gamepad_connected {
            self.notifications
                .info("Controller online // A confirm · B cancel · Y help");
        } else if !pad.connected && self.gamepad_connected {
            self.notifications.warning("Controller disconnected");
        }
        self.gamepad_connected = pad.connected;
        match self.state {
            AppState::Title => {
                if input.left_pressed {
                    self.title_focus_active = false;
                }
                if pad.left || pad.up || pad.previous {
                    self.title_focus_continue = false;
                    self.title_focus_active = true;
                } else if (pad.right || pad.down || pad.next) && self.save_exists {
                    self.title_focus_continue = true;
                    self.title_focus_active = true;
                }
                if input.space_pressed || is_key_pressed(KeyCode::Enter) {
                    self.events.push(UiAction::StartMission);
                }
                if pad.confirm {
                    self.title_focus_active = true;
                    self.events
                        .push(if self.title_focus_continue && self.save_exists {
                            UiAction::Continue
                        } else {
                            UiAction::StartMission
                        });
                }
            }
            AppState::Colony => {
                if input.escape_pressed || pad.cancel {
                    self.events.push(UiAction::ReturnToTitle);
                }
                if pad.confirm || pad.menu {
                    self.events.push(UiAction::OpenMissionBriefing);
                }
                if pad.secondary {
                    self.events.push(UiAction::OpenRoster);
                }
                if pad.up || pad.left || pad.previous {
                    self.cycle_selected_mission(-1);
                } else if pad.down || pad.right || pad.next {
                    self.cycle_selected_mission(1);
                }
            }
            AppState::Roster | AppState::GeneLab => {
                if input.escape_pressed || pad.cancel {
                    self.events.push(UiAction::ReturnToColony);
                }
                if pad.up || pad.left || pad.previous {
                    self.cycle_selected_character(-1);
                } else if pad.down || pad.right || pad.next {
                    self.cycle_selected_character(1);
                }
            }
            AppState::MissionBriefing => {
                if input.escape_pressed || pad.cancel {
                    self.events.push(UiAction::ReturnToColony);
                }
                if input.space_pressed || is_key_pressed(KeyCode::Enter) || pad.menu {
                    self.events.push(UiAction::DeployMission);
                }
                if pad.left || pad.right || pad.secondary {
                    self.events.push(UiAction::CycleFormation);
                }
                if pad.up || pad.previous {
                    self.cycle_selected_character(-1);
                } else if pad.down || pad.next {
                    self.cycle_selected_character(1);
                }
                if pad.confirm {
                    let selected = self.campaign.selected_character_id.clone();
                    self.events.push(UiAction::ToggleDeployment(selected));
                }
            }
            AppState::Tactical => {
                if self.phase_replay.is_active() {
                    if input.space_pressed || input.escape_pressed || pad.confirm || pad.cancel {
                        self.phase_replay.clear();
                    }
                    return true;
                }
                if input.escape_pressed || pad.cancel {
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
                if pad.tertiary {
                    self.events.push(UiAction::ToggleTacticalHelp);
                }
                if is_key_pressed(KeyCode::B) || pad.secondary {
                    self.events.push(UiAction::ToggleBattleLog);
                }
                if !self.show_tactical_help && !self.show_battle_log {
                    if is_key_pressed(KeyCode::Tab) || pad.next || pad.previous {
                        self.events.push(UiAction::SelectNextReady);
                    }
                    if is_key_pressed(KeyCode::S) {
                        self.events.push(UiAction::Save);
                    }
                    if is_key_pressed(KeyCode::L) {
                        self.events.push(UiAction::Load);
                    }
                    if is_key_pressed(KeyCode::Enter) {
                        self.events.push(UiAction::EndPhase);
                    }
                    if pad.menu {
                        self.events.push(UiAction::EndPhase);
                    }
                    if self.targeting.is_none() {
                        if let Some((dx, dy)) = ui::tile_move_from_keys() {
                            self.session.move_selection(dx, dy);
                        }
                    }
                    let dx = (pad.right as i32) - (pad.left as i32);
                    let dy = (pad.down as i32) - (pad.up as i32);
                    if dx != 0 || dy != 0 {
                        self.session.move_selection(dx, dy);
                    }
                    if pad.confirm {
                        self.controller_confirm_tactical();
                    }
                }
            }
            AppState::Debrief => {
                if input.space_pressed || is_key_pressed(KeyCode::Enter) || pad.confirm {
                    self.events.push(UiAction::ReturnToColony);
                }
            }
        }
        false
    }

    fn cycle_selected_character(&mut self, delta: i32) {
        if self.campaign.roster.is_empty() {
            return;
        }
        let current = self
            .campaign
            .roster
            .iter()
            .position(|character| character.id == self.campaign.selected_character_id)
            .unwrap_or(0);
        let next = (current as i32 + delta).rem_euclid(self.campaign.roster.len() as i32) as usize;
        self.events.push(UiAction::SelectColonist(
            self.campaign.roster[next].id.clone(),
        ));
    }

    fn cycle_selected_mission(&mut self, delta: i32) {
        let offers = &self.campaign.strategy.mission_offers;
        if offers.is_empty() {
            return;
        }
        let current = offers
            .iter()
            .position(|mission| mission.id == self.campaign.strategy.selected_mission_id)
            .unwrap_or(0);
        let next = (current as i32 + delta).rem_euclid(offers.len() as i32) as usize;
        self.events
            .push(UiAction::SelectMission(offers[next].id.clone()));
    }

    fn controller_confirm_tactical(&mut self) {
        let tile = self.session.tactical.selected_tile;
        if let Some(targeting) = &self.targeting {
            let target = self
                .session
                .tactical
                .units
                .iter()
                .find(|unit| unit.position == tile);
            let action = target.and_then(|target| match targeting {
                super::TacticalTargeting::Equipment {
                    unit_id,
                    equipment_id,
                } if self
                    .session
                    .can_use_equipment(unit_id, equipment_id, &target.id) =>
                {
                    Some(UiAction::UseEquipmentOn(target.id.clone()))
                }
                super::TacticalTargeting::ClassAction { unit_id }
                    if self.session.can_target_class_action(unit_id, &target.id) =>
                {
                    Some(UiAction::UseClassActionOn(target.id.clone()))
                }
                _ => None,
            });
            self.events
                .push(action.unwrap_or(UiAction::CancelTargeting));
            return;
        }

        let hostile = self
            .session
            .tactical
            .units
            .iter()
            .find(|unit| unit.position == tile && unit.team == Team::Hostile)
            .map(|unit| unit.id.clone());
        if let Some(hostile_id) = hostile.filter(|id| self.session.can_attack_selected(id)) {
            self.events.push(UiAction::AttackSelected(hostile_id));
        } else if self.session.can_attack_selected_cover(tile) {
            self.events.push(UiAction::AttackCover(tile));
        } else if self.session.can_move_selected_to(tile) {
            self.events.push(UiAction::MoveSelected(tile));
        } else {
            self.events.push(UiAction::SelectTile(tile));
        }
    }
}
