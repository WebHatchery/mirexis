//! Short-lived, save-free battlefield callouts derived from new simulation events.

use crate::grid_ui::GridView;
use crate::state::{BattleEvent, GameSession};
use crate::visual_assets::VisualCatalog;
use macroquad::prelude::*;
use macroquad_toolkit::assets::AssetManager;
use macroquad_toolkit::grid::TilePos;
use macroquad_toolkit::prelude::TextStyle;

const CALLOUT_BASE_OFFSET: f32 = 58.0;
const CALLOUT_STACK_STEP: f32 = 18.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FeedbackTone {
    Damage,
    Healing,
    Hit,
    Miss,
    Status,
}

#[derive(Debug, Clone)]
enum FeedbackAnchor {
    Unit {
        unit_id: String,
        fallback_unit_id: Option<String>,
    },
    Tile(TilePos),
    Objective,
}

#[derive(Debug, Clone)]
struct CombatCallout {
    anchor: FeedbackAnchor,
    label: String,
    tone: FeedbackTone,
    remaining: f32,
}

#[derive(Debug, Clone)]
struct CombatImpact {
    attacker_id: String,
    target_id: String,
    critical: bool,
    remaining: f32,
}

#[derive(Debug, Default, Clone)]
pub(crate) struct CombatFeedback {
    callouts: Vec<CombatCallout>,
    impacts: Vec<CombatImpact>,
}

impl CombatFeedback {
    pub fn update(&mut self, dt: f32) {
        for callout in &mut self.callouts {
            callout.remaining -= dt;
        }
        self.callouts.retain(|callout| callout.remaining > 0.0);
        for impact in &mut self.impacts {
            impact.remaining -= dt;
        }
        self.impacts.retain(|impact| impact.remaining > 0.0);
    }

    pub fn clear(&mut self) {
        self.callouts.clear();
        self.impacts.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.callouts.is_empty() && self.impacts.is_empty()
    }

    pub fn sync(&mut self, events: Option<&[BattleEvent]>, observed_count: &mut usize) -> bool {
        let Some(events) = events else {
            self.clear();
            *observed_count = 0;
            return false;
        };
        if events.len() < *observed_count {
            self.clear();
            *observed_count = 0;
        }
        let new_events = &events[*observed_count..];
        let impact = new_events.iter().any(|event| {
            matches!(
                event,
                BattleEvent::DamageApplied { .. }
                    | BattleEvent::UnitIncapacitated { .. }
                    | BattleEvent::ObjectiveDamaged { .. }
                    | BattleEvent::ObjectiveDestroyed
                    | BattleEvent::CoverDamaged { .. }
                    | BattleEvent::CoverDestroyed { .. }
            )
        });
        self.record(new_events);
        *observed_count = events.len();
        impact
    }

    pub fn record(&mut self, events: &[BattleEvent]) {
        self.record_for(events, 1.5);
    }

    pub fn record_for(&mut self, events: &[BattleEvent], lifetime: f32) {
        let mut pending_attack: Option<(&str, &str, bool)> = None;
        for event in events {
            if let BattleEvent::AttackRolled {
                attacker_id,
                target_id,
                roll,
                hit_chance,
            } = event
            {
                let hit = roll <= hit_chance;
                pending_attack = Some((attacker_id, target_id, hit && *roll <= 10));
                if hit {
                    self.callouts.push(CombatCallout {
                        anchor: FeedbackAnchor::Unit {
                            unit_id: target_id.clone(),
                            fallback_unit_id: Some(attacker_id.clone()),
                        },
                        label: "HIT".to_owned(),
                        tone: FeedbackTone::Hit,
                        remaining: lifetime,
                    });
                } else {
                    self.callouts.push(CombatCallout {
                        anchor: FeedbackAnchor::Unit {
                            unit_id: target_id.clone(),
                            fallback_unit_id: Some(attacker_id.clone()),
                        },
                        label: "MISS".to_owned(),
                        tone: FeedbackTone::Miss,
                        remaining: lifetime,
                    });
                }
                continue;
            }
            if let BattleEvent::DamageApplied { target_id, .. } = event {
                if let Some((attacker_id, expected_target, critical)) =
                    pending_attack.filter(|(_, expected_target, _)| *expected_target == target_id)
                {
                    self.impacts.push(CombatImpact {
                        attacker_id: attacker_id.to_owned(),
                        target_id: expected_target.to_owned(),
                        critical,
                        remaining: lifetime,
                    });
                }
            }
            let mapped = match event {
                BattleEvent::DamageApplied {
                    target_id, amount, ..
                } => Some((
                    FeedbackAnchor::Unit {
                        unit_id: target_id.clone(),
                        fallback_unit_id: None,
                    },
                    format!("-{}", amount),
                    FeedbackTone::Damage,
                )),
                BattleEvent::UnitHealed {
                    unit_id, amount, ..
                } => Some((
                    FeedbackAnchor::Unit {
                        unit_id: unit_id.clone(),
                        fallback_unit_id: None,
                    },
                    format!("+{}", amount),
                    FeedbackTone::Healing,
                )),
                BattleEvent::UnitIncapacitated { unit_id } => Some((
                    FeedbackAnchor::Unit {
                        unit_id: unit_id.clone(),
                        fallback_unit_id: None,
                    },
                    "INCAPACITATED".to_owned(),
                    FeedbackTone::Status,
                )),
                BattleEvent::ObjectiveSecured { unit_id } => Some((
                    FeedbackAnchor::Unit {
                        unit_id: unit_id.clone(),
                        fallback_unit_id: None,
                    },
                    "OBJECTIVE SECURED".to_owned(),
                    FeedbackTone::Status,
                )),
                BattleEvent::ExtractionCompleted { unit_id } => Some((
                    FeedbackAnchor::Unit {
                        unit_id: unit_id.clone(),
                        fallback_unit_id: None,
                    },
                    "EXTRACTED".to_owned(),
                    FeedbackTone::Status,
                )),
                BattleEvent::ObjectiveDamaged { amount, .. } => Some((
                    FeedbackAnchor::Objective,
                    format!("OBJECTIVE -{}", amount),
                    FeedbackTone::Damage,
                )),
                BattleEvent::ObjectiveDestroyed => Some((
                    FeedbackAnchor::Objective,
                    "OBJECTIVE DESTROYED".to_owned(),
                    FeedbackTone::Status,
                )),
                BattleEvent::CoverDamaged {
                    position, amount, ..
                } => Some((
                    FeedbackAnchor::Tile(*position),
                    format!("COVER -{}", amount),
                    FeedbackTone::Damage,
                )),
                BattleEvent::CoverDestroyed { position } => Some((
                    FeedbackAnchor::Tile(*position),
                    "COVER DESTROYED".to_owned(),
                    FeedbackTone::Status,
                )),
                BattleEvent::StatusApplied { unit_id, status } => Some((
                    FeedbackAnchor::Unit {
                        unit_id: unit_id.clone(),
                        fallback_unit_id: None,
                    },
                    format!("{:?}", status).to_uppercase(),
                    FeedbackTone::Status,
                )),
                _ => None,
            };
            let Some((unit_id, label, tone)) = mapped else {
                continue;
            };
            self.callouts.push(CombatCallout {
                anchor: unit_id,
                label,
                tone,
                remaining: lifetime,
            });
        }
        if self.callouts.len() > 12 {
            self.callouts.drain(..self.callouts.len() - 12);
        }
    }

    pub fn draw(
        &self,
        session: &GameSession,
        view: GridView,
        assets: &AssetManager,
        visuals: &VisualCatalog,
    ) {
        if self.is_empty() {
            return;
        }
        for impact in &self.impacts {
            draw_impact(session, view, impact, assets, visuals);
        }
        for (index, callout) in self.callouts.iter().enumerate() {
            let Some(position) = feedback_anchor_position(session, &callout.anchor) else {
                continue;
            };
            let same_unit_before = self.callouts[..index]
                .iter()
                .filter(|prior| same_feedback_anchor(&prior.anchor, &callout.anchor))
                .count();
            let rect = view.tile_rect(position);
            let color = match callout.tone {
                FeedbackTone::Damage => Color::new(1.0, 0.32, 0.22, 1.0),
                FeedbackTone::Healing => Color::new(0.32, 1.0, 0.58, 1.0),
                FeedbackTone::Hit => Color::new(0.50, 1.0, 0.64, 1.0),
                FeedbackTone::Miss => Color::new(0.38, 0.76, 1.0, 1.0),
                FeedbackTone::Status => Color::new(0.98, 0.78, 0.24, 1.0),
            };
            let size = match callout.tone {
                FeedbackTone::Status => 12.0,
                FeedbackTone::Hit => 16.0,
                FeedbackTone::Miss => 18.0,
                _ => 20.0,
            };
            let dimensions = measure_text(&callout.label, None, size as u16, 1.0);
            let y = callout_y(rect, same_unit_before);
            if callout.tone == FeedbackTone::Healing && same_unit_before == 0 {
                visuals.draw_atlas_cell(
                    assets,
                    &visuals.effects,
                    3,
                    Rect::new(rect.center().x - 30.0, rect.center().y - 36.0, 60.0, 60.0),
                    Color::new(1.0, 1.0, 1.0, callout.remaining.min(1.0)),
                );
            }
            draw_text_ex(
                &callout.label,
                rect.x + (rect.w - dimensions.width) * 0.5,
                y,
                TextStyle::new(size, color).params(),
            );
        }
    }
}

fn callout_y(rect: Rect, stack_index: usize) -> f32 {
    (rect.y - CALLOUT_BASE_OFFSET - stack_index as f32 * CALLOUT_STACK_STEP).max(24.0)
}

fn feedback_anchor_position(session: &GameSession, anchor: &FeedbackAnchor) -> Option<TilePos> {
    match anchor {
        FeedbackAnchor::Unit {
            unit_id,
            fallback_unit_id,
        } => session
            .unit(unit_id)
            .or_else(|| {
                fallback_unit_id
                    .as_deref()
                    .and_then(|unit_id| session.unit(unit_id))
            })
            .map(|unit| unit.position),
        FeedbackAnchor::Tile(position) => Some(*position),
        FeedbackAnchor::Objective => Some(session.tactical.objective_tile),
    }
}

fn same_feedback_anchor(left: &FeedbackAnchor, right: &FeedbackAnchor) -> bool {
    match (left, right) {
        (
            FeedbackAnchor::Unit { unit_id: left, .. },
            FeedbackAnchor::Unit { unit_id: right, .. },
        ) => left == right,
        (FeedbackAnchor::Tile(left), FeedbackAnchor::Tile(right)) => left == right,
        (FeedbackAnchor::Objective, FeedbackAnchor::Objective) => true,
        _ => false,
    }
}

fn draw_impact(
    session: &GameSession,
    view: GridView,
    impact: &CombatImpact,
    assets: &AssetManager,
    visuals: &VisualCatalog,
) {
    let (Some(attacker), Some(target)) = (
        session.unit(&impact.attacker_id),
        session.unit(&impact.target_id),
    ) else {
        return;
    };
    let from = view.tile_center(attacker.position);
    let to = view.tile_center(target.position);
    let direction = (to - from).normalize_or_zero();
    let progress = (1.5 - impact.remaining).clamp(0.0, 1.0);
    let faction = attacker.faction.as_deref().unwrap_or("colony");
    visuals.draw_atlas_cell(
        assets,
        &visuals.effects,
        visuals.faction_effect_cell(faction),
        Rect::new(from.x - 25.0, from.y - 25.0, 50.0, 50.0),
        Color::new(1.0, 1.0, 1.0, impact.remaining.min(1.0)),
    );
    match faction {
        "directorate" => {
            for offset in [-3.0_f32, 0.0, 3.0] {
                let normal = vec2(-direction.y, direction.x) * offset;
                draw_line(
                    from.x + normal.x,
                    from.y + normal.y,
                    to.x + normal.x,
                    to.y + normal.y,
                    1.5,
                    Color::new(1.0, 0.28, 0.18, 0.82),
                );
            }
        }
        "brood" => {
            for step in 1..7 {
                let p = from.lerp(to, step as f32 / 7.0);
                draw_circle(
                    p.x,
                    p.y,
                    1.5 + (step % 2) as f32 * 1.5,
                    Color::new(0.92, 0.18, 0.30, 0.78),
                );
            }
        }
        "ascendants" => {
            for step in 1..6 {
                let p = from.lerp(to, step as f32 / 6.0);
                draw_poly(
                    p.x,
                    p.y,
                    4,
                    3.0 + step as f32 * 0.3,
                    45.0,
                    Color::new(0.72, 0.40, 1.0, 0.74),
                );
            }
        }
        _ => draw_line(
            from.x,
            from.y,
            to.x,
            to.y,
            3.0,
            Color::new(0.40, 1.0, 0.78, 0.78),
        ),
    }
    let recoil = direction * (8.0 + progress * 4.0);
    draw_line(
        to.x,
        to.y,
        to.x + recoil.x,
        to.y + recoil.y,
        5.0,
        Color::new(1.0, 0.82, 0.36, 0.82),
    );
    for sides in [4_u8, 6, 8] {
        draw_poly_lines(
            to.x,
            to.y,
            sides,
            7.0 + sides as f32,
            progress * 60.0,
            2.0,
            Color::new(1.0, 0.48, 0.22, 0.82),
        );
    }
    let faction_effect = match faction {
        "directorate" => Some(4),
        "brood" => Some(5),
        "ascendants" => Some(6),
        _ => None,
    };
    if let Some(index) = faction_effect {
        visuals.draw_atlas_cell(
            assets,
            &visuals.effects,
            index,
            Rect::new(to.x - 32.0, to.y - 32.0, 64.0, 64.0),
            Color::new(1.0, 1.0, 1.0, impact.remaining.min(1.0)),
        );
    }
    visuals.draw_atlas_cell(
        assets,
        &visuals.effects,
        if impact.critical { 2 } else { 1 },
        Rect::new(to.x - 38.0, to.y - 38.0, 76.0, 76.0),
        Color::new(1.0, 1.0, 1.0, impact.remaining.min(1.0)),
    );
    if impact.critical {
        draw_text_ex(
            "CRITICAL",
            to.x - 34.0,
            to.y - 38.0,
            TextStyle::new(18.0, Color::new(1.0, 0.88, 0.42, 1.0)).params(),
        );
    }
}

#[cfg(test)]
mod tests;
