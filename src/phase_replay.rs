//! Brief, skippable presentation beats for an atomically resolved hostile phase.

use crate::state::BattleEvent;
use macroquad::prelude::*;
use macroquad_toolkit::prelude::{dark, draw_surface, SurfaceStyle, TextStyle};

const BEAT_SECONDS: f32 = 0.48;
const MAX_BEATS: usize = 8;

#[derive(Debug, Default, Clone)]
pub(crate) struct PhaseReplay {
    beats: Vec<String>,
    current: usize,
    remaining: f32,
}

impl PhaseReplay {
    pub fn start(&mut self, events: &[BattleEvent]) {
        let significant = events
            .iter()
            .filter(|event| {
                matches!(
                    event,
                    BattleEvent::UnitMoved { .. }
                        | BattleEvent::AttackRolled { .. }
                        | BattleEvent::ReactionTriggered { .. }
                        | BattleEvent::EnemyAbilityActivated { .. }
                        | BattleEvent::ReinforcementsArrived { .. }
                        | BattleEvent::UnitIncapacitated { .. }
                        | BattleEvent::PhaseStarted { .. }
                )
            })
            .collect::<Vec<_>>();
        let mut selected = significant
            .iter()
            .take(MAX_BEATS)
            .copied()
            .collect::<Vec<_>>();
        if significant.len() > MAX_BEATS {
            if let Some(final_phase) = significant
                .iter()
                .rev()
                .find(|event| matches!(event, BattleEvent::PhaseStarted { .. }))
            {
                selected[MAX_BEATS - 1] = final_phase;
            }
        }
        self.beats = selected
            .into_iter()
            .map(|event| {
                crate::ui_widgets::event_summary(event)
                    .replace('_', " ")
                    .to_uppercase()
            })
            .collect();
        self.current = 0;
        self.remaining = BEAT_SECONDS;
    }

    pub fn hold_for_capture(&mut self, events: &[BattleEvent]) {
        self.start(events);
        self.remaining = 100.0;
    }

    pub fn update(&mut self, dt: f32) {
        if !self.is_active() {
            return;
        }
        self.remaining -= dt;
        if self.remaining <= 0.0 {
            self.current += 1;
            self.remaining = BEAT_SECONDS;
            if !self.is_active() {
                self.clear();
            }
        }
    }

    pub fn is_active(&self) -> bool {
        self.current < self.beats.len()
    }

    pub fn clear(&mut self) {
        self.beats.clear();
        self.current = 0;
        self.remaining = 0.0;
    }

    pub fn draw(&self) {
        let Some(beat) = self.beats.get(self.current) else {
            return;
        };
        let rect = Rect::new(220.0, 566.0, 420.0, 42.0);
        draw_surface(
            rect,
            &SurfaceStyle::new(Color::new(0.12, 0.055, 0.045, 0.96))
                .with_border(2.0, Color::new(1.0, 0.34, 0.16, 1.0)),
        );
        draw_text_ex(
            beat,
            rect.x + 12.0,
            rect.y + 18.0,
            TextStyle::new(13.0, dark::TEXT_BRIGHT).params(),
        );
        draw_text_ex(
            format!(
                "HOSTILE ACTIVITY {}/{}  ·  SPACE SKIPS",
                self.current + 1,
                self.beats.len()
            ),
            rect.x + 12.0,
            rect.y + 34.0,
            TextStyle::new(10.0, Color::new(0.96, 0.55, 0.35, 1.0)).params(),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::TacticalPhase;

    #[test]
    fn replay_keeps_important_events_bounded_and_advances_by_time() {
        let mut events = (0..10)
            .map(|index| BattleEvent::UnitMoved {
                unit_id: format!("hostile_{}", index),
                path: Vec::new(),
                cost: 1,
            })
            .collect::<Vec<_>>();
        events.push(BattleEvent::PhaseStarted {
            phase: TacticalPhase::Player,
            round: 2,
        });
        let mut replay = PhaseReplay::default();
        replay.start(&events);

        assert_eq!(replay.beats.len(), MAX_BEATS);
        assert!(replay.beats.last().unwrap().contains("PLAYER PHASE"));
        assert!(replay.is_active());
        replay.update(BEAT_SECONDS + 0.01);
        assert_eq!(replay.current, 1);
        replay.clear();
        assert!(!replay.is_active());
    }
}
