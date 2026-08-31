//! Expandable recent battle-event history over the live tactical field.

use crate::state::{BattleEvent, GameSession};
use crate::ui::UiAction;
use crate::ui_action::BattleLogFilter;
use crate::ui_widgets::{button, button_with_state, event_summary};
use macroquad::prelude::*;
use macroquad_toolkit::prelude::{dark, draw_surface, SurfaceStyle, TextStyle};

const VISIBLE_EVENTS: usize = 9;
const FILTERS: [(BattleLogFilter, &str); 4] = [
    (BattleLogFilter::All, "ALL"),
    (BattleLogFilter::Combat, "COMBAT"),
    (BattleLogFilter::Ground, "GROUND"),
    (BattleLogFilter::System, "SYSTEM"),
];

pub(crate) fn draw(
    session: &GameSession,
    filter: BattleLogFilter,
    mouse: Vec2,
    actions: &mut Vec<UiAction>,
) {
    draw_rectangle(0.0, 0.0, 1280.0, 720.0, Color::new(0.01, 0.02, 0.025, 0.62));
    let panel = Rect::new(690.0, 88.0, 550.0, 544.0);
    draw_surface(
        panel,
        &SurfaceStyle::new(Color::new(0.045, 0.065, 0.07, 0.99))
            .with_border(2.0, Color::new(0.95, 0.55, 0.25, 0.9)),
    );
    draw_text_ex(
        "BATTLE HISTORY // RECENT EVENTS",
        panel.x + 24.0,
        panel.y + 42.0,
        TextStyle::new(24.0, dark::TEXT_BRIGHT).params(),
    );
    draw_text_ex(
        "Oldest at top · newest at bottom · review pauses commands",
        panel.x + 24.0,
        panel.y + 67.0,
        TextStyle::new(13.0, dark::TEXT_DIM).params(),
    );
    let filter_counts = FILTERS.map(|(option, _)| filtered_event_count(session, option));
    for (index, (option, label)) in FILTERS.iter().enumerate() {
        let rect = Rect::new(
            panel.x + 24.0 + index as f32 * 122.0,
            panel.y + 78.0,
            116.0,
            28.0,
        );
        let label = format!("{} {}", label, filter_counts[index]);
        if button_with_state(rect, &label, true, filter == *option, mouse) {
            actions.push(UiAction::SetBattleLogFilter(*option));
        }
    }
    let filtered_events = session
        .tactical
        .event_log
        .iter()
        .enumerate()
        .filter(|(_, event)| filter_matches(filter, event))
        .collect::<Vec<_>>();
    let start = filtered_events.len().saturating_sub(VISIBLE_EVENTS);
    for (row, &(event_index, event)) in filtered_events[start..].iter().enumerate() {
        let y = panel.y + 112.0 + row as f32 * 39.0;
        let row_rect = Rect::new(panel.x + 18.0, y, panel.w - 36.0, 34.0);
        draw_rectangle(
            row_rect.x,
            row_rect.y,
            row_rect.w,
            row_rect.h,
            if row % 2 == 0 {
                Color::new(0.06, 0.095, 0.10, 0.92)
            } else {
                Color::new(0.045, 0.075, 0.08, 0.92)
            },
        );
        draw_event_icon(event, vec2(row_rect.x + 18.0, row_rect.y + 17.0));
        draw_text_ex(
            format!("{:02}", event_index + 1),
            row_rect.x + 34.0,
            row_rect.y + 14.0,
            TextStyle::new(10.0, Color::new(0.46, 0.66, 0.62, 1.0)).params(),
        );
        draw_text_ex(
            event_summary(event).replace('_', " "),
            row_rect.x + 65.0,
            row_rect.y + 23.0,
            TextStyle::new(13.0, event_color(event)).params(),
        );
        draw_text_ex(
            event_kind(event),
            row_rect.right() - 92.0,
            row_rect.y + 12.0,
            TextStyle::new(10.0, Color::new(0.45, 0.62, 0.60, 1.0)).params(),
        );
    }
    if filtered_events.is_empty() {
        draw_text_ex(
            if session.tactical.event_log.is_empty() {
                "No tactical events recorded yet."
            } else {
                "No events match this filter."
            },
            panel.x + 24.0,
            panel.y + 236.0,
            TextStyle::new(15.0, dark::TEXT_DIM).params(),
        );
    }
    if button(
        Rect::new(
            panel.x + panel.w - 210.0,
            panel.bottom() - 60.0,
            180.0,
            40.0,
        ),
        "RETURN TO BATTLE",
        true,
        mouse,
    ) {
        actions.push(UiAction::ToggleBattleLog);
    }
}

fn filter_matches(filter: BattleLogFilter, event: &BattleEvent) -> bool {
    filter == BattleLogFilter::All || event_filter(event) == filter
}

fn filtered_event_count(session: &GameSession, filter: BattleLogFilter) -> usize {
    session
        .tactical
        .event_log
        .iter()
        .filter(|event| filter_matches(filter, event))
        .count()
}

fn event_filter(event: &BattleEvent) -> BattleLogFilter {
    use BattleEvent::*;
    match event {
        AttackRolled { .. }
        | DamageApplied { .. }
        | UnitIncapacitated { .. }
        | UnitHealed { .. }
        | StatusApplied { .. }
        | MutationActivated { .. }
        | ClassActionActivated { .. }
        | SkillActivated { .. }
        | EquipmentUsed { .. }
        | OverwatchSet { .. }
        | ReactionTriggered { .. }
        | EnemyAbilityActivated { .. } => BattleLogFilter::Combat,
        UnitMoved { .. }
        | ObjectiveSecured { .. }
        | ObjectiveDamaged { .. }
        | ObjectiveDestroyed
        | ExtractionCompleted { .. }
        | CoverDamaged { .. }
        | CoverDestroyed { .. }
        | HazardTriggered { .. }
        | HazardConverted { .. }
        | ReinforcementsArrived { .. } => BattleLogFilter::Ground,
        _ => BattleLogFilter::System,
    }
}

fn event_kind(event: &BattleEvent) -> &'static str {
    use BattleEvent::*;
    match event {
        UnitMoved { .. } => "MOVEMENT",
        AttackRolled { .. } | ReactionTriggered { .. } => "ATTACK",
        DamageApplied { .. } | UnitIncapacitated { .. } => "IMPACT",
        UnitHealed { .. } => "RECOVERY",
        StatusApplied { .. } => "STATUS",
        MutationActivated { .. }
        | ClassActionActivated { .. }
        | SkillActivated { .. }
        | EquipmentUsed { .. }
        | OverwatchSet { .. }
        | EnemyAbilityActivated { .. } => "ABILITY",
        ObjectiveSecured { .. }
        | ObjectiveDamaged { .. }
        | ObjectiveDestroyed
        | ExtractionCompleted { .. } => "OBJECTIVE",
        CoverDamaged { .. } | CoverDestroyed { .. } => "COVER",
        PhaseStarted { .. } => "PHASE",
        BattleEnded { .. } => "OUTCOME",
        HazardTriggered { .. } | HazardConverted { .. } => "HAZARD",
        ReinforcementsArrived { .. } => "WAVE",
    }
}

fn draw_event_icon(event: &crate::state::BattleEvent, center: Vec2) {
    let color = event_color(event);
    match event_kind(event) {
        "ATTACK" | "IMPACT" => {
            draw_line(
                center.x - 6.0,
                center.y + 6.0,
                center.x + 6.0,
                center.y - 6.0,
                3.0,
                color,
            );
            draw_circle(center.x + 6.0, center.y - 6.0, 3.0, color);
        }
        "MOVEMENT" => {
            draw_line(
                center.x - 7.0,
                center.y + 5.0,
                center.x + 6.0,
                center.y - 3.0,
                2.0,
                color,
            );
            draw_triangle(
                vec2(center.x + 8.0, center.y - 4.0),
                vec2(center.x + 2.0, center.y - 6.0),
                vec2(center.x + 5.0, center.y),
                color,
            );
        }
        "HAZARD" => draw_poly(center.x, center.y, 6, 8.0, 0.0, color),
        "RECOVERY" => {
            draw_line(
                center.x - 7.0,
                center.y,
                center.x + 7.0,
                center.y,
                2.0,
                color,
            );
            draw_line(
                center.x,
                center.y - 7.0,
                center.x,
                center.y + 7.0,
                2.0,
                color,
            );
        }
        "COVER" => draw_rectangle_lines(center.x - 7.0, center.y - 7.0, 14.0, 14.0, 2.0, color),
        "OBJECTIVE" => {
            draw_circle_lines(center.x, center.y, 7.0, 2.0, color);
            draw_circle(center.x, center.y, 2.0, color);
        }
        _ => draw_poly(center.x, center.y, 4, 7.0, 45.0, color),
    }
}

fn event_color(event: &BattleEvent) -> Color {
    match event {
        BattleEvent::DamageApplied { .. }
        | BattleEvent::UnitIncapacitated { .. }
        | BattleEvent::HazardTriggered { .. }
        | BattleEvent::ObjectiveDamaged { .. }
        | BattleEvent::ObjectiveDestroyed
        | BattleEvent::CoverDamaged { .. }
        | BattleEvent::CoverDestroyed { .. } => Color::new(0.98, 0.55, 0.38, 1.0),
        BattleEvent::UnitHealed { .. } => Color::new(0.38, 0.98, 0.62, 1.0),
        BattleEvent::HazardConverted { .. } => Color::new(0.42, 0.94, 0.72, 1.0),
        BattleEvent::ObjectiveSecured { .. } | BattleEvent::ExtractionCompleted { .. } => {
            Color::new(0.48, 0.94, 0.72, 1.0)
        }
        BattleEvent::StatusApplied { .. }
        | BattleEvent::MutationActivated { .. }
        | BattleEvent::ClassActionActivated { .. }
        | BattleEvent::SkillActivated { .. }
        | BattleEvent::EquipmentUsed { .. }
        | BattleEvent::OverwatchSet { .. }
        | BattleEvent::EnemyAbilityActivated { .. } => Color::new(0.94, 0.76, 0.34, 1.0),
        BattleEvent::PhaseStarted { .. } | BattleEvent::BattleEnded { .. } => {
            Color::new(0.46, 0.86, 0.72, 1.0)
        }
        _ => dark::TEXT_DIM,
    }
}

#[cfg(test)]
mod tests;
