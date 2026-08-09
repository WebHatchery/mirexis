//! Short-lived, save-free battlefield callouts derived from new simulation events.

use crate::grid_ui::GridView;
use crate::state::{BattleEvent, GameSession};
use macroquad::prelude::*;
use macroquad_toolkit::prelude::TextStyle;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FeedbackTone {
    Damage,
    Healing,
    Status,
}

#[derive(Debug, Clone)]
struct CombatCallout {
    unit_id: String,
    label: String,
    tone: FeedbackTone,
    remaining: f32,
}

#[derive(Debug, Default, Clone)]
pub(crate) struct CombatFeedback {
    callouts: Vec<CombatCallout>,
}

impl CombatFeedback {
    pub fn update(&mut self, dt: f32) {
        for callout in &mut self.callouts {
            callout.remaining -= dt;
        }
        self.callouts.retain(|callout| callout.remaining > 0.0);
    }

    pub fn clear(&mut self) {
        self.callouts.clear();
    }

    pub fn sync(&mut self, events: Option<&[BattleEvent]>, observed_count: &mut usize) {
        let Some(events) = events else {
            self.clear();
            *observed_count = 0;
            return;
        };
        if events.len() < *observed_count {
            self.clear();
            *observed_count = 0;
        }
        self.record(&events[*observed_count..]);
        *observed_count = events.len();
    }

    pub fn record(&mut self, events: &[BattleEvent]) {
        self.record_for(events, 1.5);
    }

    pub fn record_for(&mut self, events: &[BattleEvent], lifetime: f32) {
        for event in events {
            let mapped = match event {
                BattleEvent::DamageApplied {
                    target_id, amount, ..
                } => Some((target_id, format!("-{}", amount), FeedbackTone::Damage)),
                BattleEvent::UnitHealed {
                    unit_id, amount, ..
                } => Some((unit_id, format!("+{}", amount), FeedbackTone::Healing)),
                BattleEvent::StatusApplied { unit_id, status } => Some((
                    unit_id,
                    format!("{:?}", status).to_uppercase(),
                    FeedbackTone::Status,
                )),
                _ => None,
            };
            let Some((unit_id, label, tone)) = mapped else {
                continue;
            };
            self.callouts.push(CombatCallout {
                unit_id: unit_id.clone(),
                label,
                tone,
                remaining: lifetime,
            });
        }
        if self.callouts.len() > 12 {
            self.callouts.drain(..self.callouts.len() - 12);
        }
    }

    pub fn draw(&self, session: &GameSession, view: GridView) {
        for (index, callout) in self.callouts.iter().enumerate() {
            let Some(unit) = session.unit(&callout.unit_id) else {
                continue;
            };
            let same_unit_before = self.callouts[..index]
                .iter()
                .filter(|prior| prior.unit_id == callout.unit_id)
                .count();
            let rect = view.tile_rect(unit.position);
            let color = match callout.tone {
                FeedbackTone::Damage => Color::new(1.0, 0.32, 0.22, 1.0),
                FeedbackTone::Healing => Color::new(0.32, 1.0, 0.58, 1.0),
                FeedbackTone::Status => Color::new(0.98, 0.78, 0.24, 1.0),
            };
            let size = if callout.tone == FeedbackTone::Status {
                12.0
            } else {
                20.0
            };
            let dimensions = measure_text(&callout.label, None, size as u16, 1.0);
            let y = rect.y - 4.0 - same_unit_before as f32 * 15.0;
            draw_text_ex(
                &callout.label,
                rect.x + (rect.w - dimensions.width) * 0.5,
                y,
                TextStyle::new(size, color).params(),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::StatusKind;

    #[test]
    fn feedback_tracks_only_damage_healing_and_status_events_for_a_bounded_time() {
        let mut feedback = CombatFeedback::default();
        feedback.record(&[
            BattleEvent::DamageApplied {
                target_id: "kira_voss".to_owned(),
                amount: 3,
                remaining: 6,
            },
            BattleEvent::UnitHealed {
                unit_id: "kira_voss".to_owned(),
                amount: 2,
                remaining: 8,
            },
            BattleEvent::StatusApplied {
                unit_id: "kira_voss".to_owned(),
                status: StatusKind::Guarded,
            },
            BattleEvent::PhaseStarted {
                phase: crate::state::TacticalPhase::Player,
                round: 2,
            },
        ]);

        assert_eq!(feedback.callouts.len(), 3);
        assert_eq!(feedback.callouts[0].label, "-3");
        assert_eq!(feedback.callouts[1].label, "+2");
        assert_eq!(feedback.callouts[2].label, "GUARDED");
        feedback.update(1.6);
        assert!(feedback.callouts.is_empty());
    }
}
