//! Brief, skippable presentation beats for an atomically resolved hostile phase.

use crate::state::BattleEvent;
use crate::ui::UiAction;
use crate::ui_widgets::button;
use macroquad::prelude::*;
use macroquad_toolkit::prelude::{dark, draw_surface, SurfaceStyle, TextStyle};

const BEAT_SECONDS: f32 = 0.48;
const MAX_BEATS: usize = 8;

pub(crate) fn skip_button_bounds() -> Rect {
    Rect::new(650.0, 560.0, 168.0, 36.0)
}

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
            .filter(|event| is_replay_event(event))
            .collect::<Vec<_>>();
        let final_phase = significant
            .iter()
            .rev()
            .find(|event| matches!(event, BattleEvent::PhaseStarted { .. }))
            .map(|event| replay_summary(event));
        let mut beats = grouped_replay_summaries(&significant);
        if beats.len() > MAX_BEATS {
            beats.truncate(MAX_BEATS);
            if let Some(final_phase) = final_phase {
                beats[MAX_BEATS - 1] = final_phase;
            }
        }
        self.beats = beats;
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

    pub fn draw(&self, mouse: Vec2, actions: &mut Vec<UiAction>) {
        let Some(beat) = self.beats.get(self.current) else {
            return;
        };
        draw_rectangle(
            18.0,
            544.0,
            820.0,
            82.0,
            Color::new(0.01, 0.02, 0.025, 0.70),
        );
        let rect = Rect::new(220.0, 552.0, 420.0, 54.0);
        draw_surface(
            rect,
            &SurfaceStyle::new(Color::new(0.12, 0.055, 0.045, 0.96))
                .with_border(2.0, Color::new(1.0, 0.34, 0.16, 1.0)),
        );
        draw_text_ex(
            beat,
            rect.x + 12.0,
            rect.y + 24.0,
            TextStyle::new(14.0, dark::TEXT_BRIGHT).params(),
        );
        draw_text_ex(
            format!(
                "HOSTILE ACTIVITY {}/{}  ·  AUTO ADVANCE",
                self.current + 1,
                self.beats.len()
            ),
            rect.x + 12.0,
            rect.y + 44.0,
            TextStyle::new(10.0, Color::new(0.96, 0.55, 0.35, 1.0)).params(),
        );
        for (index, _) in self.beats.iter().enumerate() {
            let marker = vec2(52.0 + index as f32 * 20.0, 578.0);
            let active = index == self.current;
            let complete = index < self.current;
            draw_poly(
                marker.x,
                marker.y,
                4,
                if active { 8.0 } else { 5.0 },
                45.0,
                if active {
                    Color::new(1.0, 0.36, 0.16, 1.0)
                } else if complete {
                    dark::POSITIVE
                } else {
                    Color::new(0.22, 0.30, 0.31, 1.0)
                },
            );
            if index + 1 < self.beats.len() {
                draw_line(
                    marker.x + 7.0,
                    marker.y,
                    marker.x + 13.0,
                    marker.y,
                    2.0,
                    Color::new(0.35, 0.48, 0.46, 0.7),
                );
            }
        }
        draw_text_ex(
            "PHASE REPLAY",
            42.0,
            558.0,
            TextStyle::new(11.0, Color::new(0.96, 0.55, 0.35, 1.0)).params(),
        );
        if button(skip_button_bounds(), "SKIP REPLAY", true, mouse) {
            actions.push(UiAction::SkipPhaseReplay);
        }
    }
}

fn is_replay_event(event: &BattleEvent) -> bool {
    matches!(
        event,
        BattleEvent::UnitMoved { .. }
            | BattleEvent::AttackRolled { .. }
            | BattleEvent::DamageApplied { .. }
            | BattleEvent::ReactionTriggered { .. }
            | BattleEvent::EnemyAbilityActivated { .. }
            | BattleEvent::ReinforcementsArrived { .. }
            | BattleEvent::UnitIncapacitated { .. }
            | BattleEvent::ObjectiveDamaged { .. }
            | BattleEvent::ObjectiveDestroyed
            | BattleEvent::CoverDamaged { .. }
            | BattleEvent::CoverDestroyed { .. }
            | BattleEvent::PhaseStarted { .. }
    )
}

fn grouped_replay_summaries(events: &[&BattleEvent]) -> Vec<String> {
    let mut summaries = Vec::new();
    let mut index = 0;
    while index < events.len() {
        let event = events[index];
        let mut summary = replay_summary(event);
        if matches!(event, BattleEvent::AttackRolled { .. }) {
            while let Some(consequence) = events.get(index + 1) {
                let Some(suffix) = replay_attack_result_suffix(consequence) else {
                    break;
                };
                summary.push_str(" // ");
                summary.push_str(&suffix);
                index += 1;
            }
        }
        summaries.push(summary);
        index += 1;
    }
    summaries
}

fn replay_summary(event: &BattleEvent) -> String {
    crate::ui_widgets::event_summary(event)
        .replace('_', " ")
        .to_uppercase()
}

fn replay_attack_result_suffix(event: &BattleEvent) -> Option<String> {
    Some(match event {
        BattleEvent::DamageApplied {
            amount, remaining, ..
        } => format!("{} DMG · {} REMAIN", amount, remaining),
        BattleEvent::UnitIncapacitated { .. } => "INCAPACITATED".to_owned(),
        _ => return None,
    })
}

#[cfg(test)]
mod tests;
