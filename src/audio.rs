//! Procedural response palette and cross-platform preferences.

use crate::state::{BattleEvent, ObjectiveState};
use macroquad_toolkit::audio::SoundManager;
use macroquad_toolkit::persistence::{load_from_slot, save_to_slot};
use macroquad_toolkit::synth::{render_wav, SynthConfig, Voice, Wave};
use serde::{Deserialize, Serialize};

const SETTINGS_SLOT: &str = "audio_preferences";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum SoundCue {
    Focus,
    Invalid,
    Move,
    Hit,
    Miss,
    Damage,
    Recovery,
    Ability,
    Objective,
    Reinforcement,
    Phase,
    Victory,
    Defeat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct AudioSettings {
    pub volume_percent: u8,
    pub muted: bool,
    #[serde(default)]
    pub reduced_motion: bool,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            volume_percent: 75,
            muted: false,
            reduced_motion: false,
        }
    }
}

impl AudioSettings {
    fn normalized(mut self) -> Self {
        self.volume_percent = self.volume_percent.min(100);
        self
    }
}

pub(crate) struct AudioSystem {
    sounds: SoundManager<SoundCue>,
    pub settings: AudioSettings,
    pub load_failures: Vec<String>,
}

impl AudioSystem {
    pub(crate) async fn new(game_name: &str) -> Self {
        let settings = load_from_slot::<AudioSettings>(game_name, SETTINGS_SLOT)
            .unwrap_or_default()
            .normalized();
        let mut system = Self {
            sounds: SoundManager::new(),
            settings,
            load_failures: Vec::new(),
        };
        for (cue, voices, seed) in palette() {
            let bytes = render_wav(&voices, &SynthConfig::default(), seed);
            if let Err(error) = system.sounds.load_sound_bytes(cue, &bytes).await {
                system.load_failures.push(format!("{cue:?}: {error}"));
            }
        }
        system
    }

    pub(crate) fn play(&mut self, cue: SoundCue) {
        self.apply_volume();
        if !self.settings.muted {
            self.sounds.play_sfx(cue, 1.0);
        }
    }

    pub(crate) fn report_failures(
        &self,
        notifications: &mut macroquad_toolkit::notifications::NotificationManager,
    ) {
        if !self.load_failures.is_empty() {
            notifications.danger(format!(
                "AUDIO UNAVAILABLE: {}",
                self.load_failures.join("; ")
            ));
        }
    }

    pub(crate) fn play_events(&mut self, events: &[BattleEvent]) {
        let cue = prioritized_event_cue(events);
        if let Some(cue) = cue {
            self.play(cue);
        }
    }

    pub(crate) fn adjust_volume(&mut self, delta: i8, game_name: &str) {
        self.settings.volume_percent =
            (self.settings.volume_percent as i16 + delta as i16).clamp(0, 100) as u8;
        if self.settings.volume_percent > 0 {
            self.settings.muted = false;
        }
        self.persist(game_name);
        self.apply_volume();
    }

    pub(crate) fn toggle_mute(&mut self, game_name: &str) {
        self.settings.muted = !self.settings.muted;
        self.persist(game_name);
        self.apply_volume();
    }

    pub(crate) fn toggle_reduced_motion(&mut self, game_name: &str) {
        self.settings.reduced_motion = !self.settings.reduced_motion;
        self.persist(game_name);
    }

    fn persist(&mut self, game_name: &str) {
        if let Err(error) = save_to_slot(game_name, SETTINGS_SLOT, &self.settings) {
            self.load_failures
                .push(format!("Could not persist audio settings: {error}"));
        }
    }

    fn apply_volume(&mut self) {
        self.sounds.sfx_volume = if self.settings.muted {
            0.0
        } else {
            self.settings.volume_percent as f32 / 100.0
        };
    }
}

fn event_cue(event: &BattleEvent) -> Option<SoundCue> {
    match event {
        BattleEvent::BattleEnded {
            outcome: ObjectiveState::Victory,
        } => Some(SoundCue::Victory),
        BattleEvent::BattleEnded { .. } => Some(SoundCue::Defeat),
        BattleEvent::ObjectiveSecured { .. }
        | BattleEvent::ExtractionCompleted { .. }
        | BattleEvent::ObjectiveDestroyed => Some(SoundCue::Objective),
        BattleEvent::MutationActivated { .. }
        | BattleEvent::ClassActionActivated { .. }
        | BattleEvent::SkillActivated { .. }
        | BattleEvent::EquipmentUsed { .. }
        | BattleEvent::EnemyAbilityActivated { .. }
        | BattleEvent::HazardConverted { .. } => Some(SoundCue::Ability),
        BattleEvent::DamageApplied { .. }
        | BattleEvent::ObjectiveDamaged { .. }
        | BattleEvent::HazardTriggered { .. } => Some(SoundCue::Damage),
        BattleEvent::UnitHealed { .. } => Some(SoundCue::Recovery),
        BattleEvent::AttackRolled {
            roll, hit_chance, ..
        } => Some(if roll <= hit_chance {
            SoundCue::Hit
        } else {
            SoundCue::Miss
        }),
        BattleEvent::UnitMoved { .. } => Some(SoundCue::Move),
        BattleEvent::ReinforcementsArrived { .. } => Some(SoundCue::Reinforcement),
        BattleEvent::PhaseStarted { .. } => Some(SoundCue::Phase),
        _ => None,
    }
}

fn prioritized_event_cue(events: &[BattleEvent]) -> Option<SoundCue> {
    events
        .iter()
        .filter_map(event_cue)
        .max_by_key(|cue| cue_priority(*cue))
}

fn cue_priority(cue: SoundCue) -> u8 {
    match cue {
        SoundCue::Victory | SoundCue::Defeat => 100,
        SoundCue::Objective => 90,
        SoundCue::Reinforcement => 85,
        SoundCue::Damage => 70,
        SoundCue::Hit | SoundCue::Miss | SoundCue::Ability => 60,
        SoundCue::Recovery => 50,
        SoundCue::Phase => 40,
        SoundCue::Move => 10,
        SoundCue::Focus | SoundCue::Invalid => 0,
    }
}

fn palette() -> Vec<(SoundCue, Vec<Voice>, u64)> {
    use SoundCue::*;
    vec![
        (Focus, vec![tone(0.06, 880.0, 1160.0, 0.30)], 1),
        (Invalid, vec![tone(0.13, 190.0, 95.0, 0.44)], 2),
        (
            Move,
            vec![noise(0.08, 0.18), tone(0.10, 180.0, 260.0, 0.18)],
            3,
        ),
        (
            Hit,
            vec![noise(0.10, 0.40), tone(0.12, 130.0, 70.0, 0.38)],
            4,
        ),
        (
            Miss,
            vec![noise(0.12, 0.20), tone(0.14, 780.0, 310.0, 0.22)],
            5,
        ),
        (
            Damage,
            vec![noise(0.18, 0.44), tone(0.20, 120.0, 55.0, 0.40)],
            6,
        ),
        (
            Recovery,
            vec![
                tone(0.18, 420.0, 680.0, 0.24),
                tone(0.14, 680.0, 980.0, 0.18),
            ],
            13,
        ),
        (
            Ability,
            vec![
                tone(0.24, 320.0, 920.0, 0.32),
                tone(0.18, 640.0, 1280.0, 0.18),
            ],
            7,
        ),
        (
            Objective,
            vec![
                tone(0.20, 520.0, 780.0, 0.30),
                tone(0.22, 780.0, 1040.0, 0.26),
            ],
            8,
        ),
        (
            Reinforcement,
            vec![
                tone(0.18, 180.0, 360.0, 0.28),
                tone(0.24, 360.0, 720.0, 0.24),
            ],
            14,
        ),
        (
            Phase,
            vec![
                tone(0.18, 240.0, 420.0, 0.22),
                tone(0.12, 420.0, 300.0, 0.16),
            ],
            15,
        ),
        (
            Victory,
            vec![
                tone(0.46, 330.0, 660.0, 0.28),
                tone(0.44, 495.0, 990.0, 0.24),
            ],
            9,
        ),
        (
            Defeat,
            vec![tone(0.55, 260.0, 95.0, 0.34), noise(0.30, 0.12)],
            10,
        ),
    ]
}

fn tone(duration: f32, from: f32, to: f32, gain: f32) -> Voice {
    Voice::tone(0.0, duration, from, gain)
        .glide(to)
        .wave(Wave::Triangle)
}

fn noise(duration: f32, gain: f32) -> Voice {
    Voice::tone(0.0, duration, 440.0, gain).wave(Wave::Noise)
}

#[cfg(test)]
mod tests;
