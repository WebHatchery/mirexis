//! Audio scene, response, and persistent settings integration.

use super::{AppState, Game};
use crate::audio::{AudioScene, SoundCue};
use crate::ui::UiAction;
use macroquad_toolkit::ui::VirtualUi;

impl Game {
    pub(super) fn apply_audio_action(&mut self, action: &UiAction) -> bool {
        let game_name = self.data.config.game_name.clone();
        match action {
            UiAction::ToggleSettings => {
                self.show_settings = !self.show_settings;
                if self.show_settings {
                    self.clear_colony_explorer_motion();
                    self.show_field_notes = false;
                    self.show_memorial = false;
                }
            }
            UiAction::AudioVolumeDown => {
                self.audio.adjust_volume(-25, &game_name);
                self.audio.play(SoundCue::Focus);
            }
            UiAction::AudioVolumeUp => {
                self.audio.adjust_volume(25, &game_name);
                self.audio.play(SoundCue::Focus);
            }
            UiAction::ToggleMute => {
                let was_muted = self.audio.settings.muted;
                self.audio.toggle_mute(&game_name);
                if was_muted {
                    self.audio.play(SoundCue::Focus);
                }
            }
            UiAction::ToggleReducedMotion => self.audio.toggle_reduced_motion(&game_name),
            _ => {
                if !tactical_command(action) {
                    self.audio.play(SoundCue::Focus);
                }
                return false;
            }
        }
        true
    }

    pub(super) fn update_motion(&mut self, dt: f32) {
        if self.audio.settings.reduced_motion {
            self.session.tactical.update_presentation(1.0);
            self.combat_feedback.update(1.0);
            self.phase_replay.clear();
        } else {
            self.session.tactical.update_presentation(dt);
            self.combat_feedback.update(dt);
            self.phase_replay.update(dt);
        }
    }

    pub(super) fn finish_action_audio(&mut self, action: &UiAction, before_events: usize) {
        if tactical_command(action) && self.session.tactical.event_log.len() == before_events {
            self.audio.play(SoundCue::Invalid);
            self.campaign.first_hour.metrics.invalid_command();
        }
    }

    pub(super) fn draw_settings(&self, ui: &VirtualUi, actions: &mut Vec<UiAction>) {
        crate::settings_ui::draw_modal(
            self.audio.settings,
            self.show_settings,
            crate::ui::pointer_position(ui),
            actions,
        );
    }

    pub(super) fn sync_audio(&mut self) {
        let scene = match self.state {
            AppState::Colony | AppState::Roster | AppState::GeneLab => AudioScene::City,
            AppState::MissionBriefing | AppState::Tactical | AppState::Debrief => {
                AudioScene::Tactical
            }
            AppState::Title => AudioScene::Silent,
        };
        self.audio.set_scene(scene);
        if self.observed_event_count < self.session.tactical.event_log.len() {
            self.audio
                .play_events(&self.session.tactical.event_log[self.observed_event_count..]);
        }
    }
}

fn tactical_command(action: &UiAction) -> bool {
    matches!(
        action,
        UiAction::MoveSelected(_)
            | UiAction::AttackSelected(_)
            | UiAction::AttackCover(_)
            | UiAction::InteractObjective
            | UiAction::ActivateMutation
            | UiAction::ActivateClassAction
            | UiAction::UseClassActionOn(_)
            | UiAction::UseClassActionOnTile(_)
            | UiAction::ActivateSkill(_)
            | UiAction::UseSkillOn(_)
            | UiAction::UseSkillOnTile(_)
            | UiAction::UseEquipmentOn(_)
            | UiAction::SetOverwatch
            | UiAction::EndPhase
    )
}
