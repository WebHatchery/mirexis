//! Hostile intent inspector and on-grid forecast line.

use crate::data::Team;
use crate::enemy_intent::{self, IntentAction};
use crate::grid_ui::GridView;
use crate::state::GameSession;
use crate::tactical::manhattan;
use macroquad::prelude::*;
use macroquad_toolkit::grid::TilePos;
use macroquad_toolkit::prelude::{dark, TextStyle};

pub(crate) fn inspected_hostile(session: &GameSession) -> Option<&crate::state::UnitState> {
    session.tactical.units.iter().find(|unit| {
        unit.team == Team::Hostile
            && !unit.incapacitated
            && unit.position == session.tactical.selected_tile
    })
}

pub(crate) fn draw_inspector(session: &GameSession, max_ap: u8, panel: Rect) -> bool {
    let Some(unit) = inspected_hostile(session) else {
        return false;
    };
    let Some(intent) = enemy_intent::preview(session, &unit.id, max_ap) else {
        return false;
    };
    let x = panel.x + 18.0;
    draw_text_ex(
        "HOSTILE INTENT // INSPECTED",
        x,
        panel.y + 178.0,
        TextStyle::new(15.0, Color::new(0.96, 0.45, 0.32, 1.0)).params(),
    );
    draw_text_ex(
        &unit.name,
        x,
        panel.y + 210.0,
        TextStyle::new(25.0, dark::TEXT_BRIGHT).params(),
    );
    draw_text_ex(
        format!(
            "{} // {}",
            unit.role,
            faction_label(unit.faction.as_deref())
        ),
        x,
        panel.y + 234.0,
        TextStyle::new(15.0, dark::TEXT_DIM).params(),
    );
    draw_text_ex(
        format!(
            "{} VITALS // {} DMG · R{} · {} AP · A{} · M{}",
            unit.health,
            unit.effective_weapon_damage(),
            unit.weapon_range,
            unit.weapon_ap_cost,
            unit.effective_armour(),
            unit.effective_move_range()
        ),
        x,
        panel.y + 258.0,
        TextStyle::new(13.0, dark::ACCENT).params(),
    );
    if let Some(ability) = intent.ability {
        draw_text_ex(
            format!("ABILITY // {}", ability),
            x,
            panel.y + 286.0,
            TextStyle::new(14.0, Color::new(0.95, 0.74, 0.24, 1.0)).params(),
        );
    }
    draw_text_ex(
        action_label(session, &intent.action),
        x,
        panel.y + 310.0,
        TextStyle::new(14.0, dark::TEXT_BRIGHT).params(),
    );
    true
}

pub(crate) fn draw_forecast(session: &GameSession, max_ap: u8, view: GridView) {
    let Some(unit) = inspected_hostile(session) else {
        return;
    };
    draw_weapon_range(session, &unit.id, view);
    let Some(intent) = enemy_intent::preview(session, &unit.id, max_ap) else {
        return;
    };
    let target = match intent.action {
        IntentAction::AttackUnit(ref target_id) => {
            session.unit(target_id).map(|unit| unit.position)
        }
        IntentAction::AttackObjective => Some(session.tactical.objective_tile),
        IntentAction::Advance { destination, .. } => Some(destination),
        IntentAction::Hold => None,
    };
    let Some(target) = target else {
        return;
    };
    let from = view.tile_rect(unit.position);
    let to = view.tile_rect(target);
    draw_line(
        from.x + from.w * 0.5,
        from.y + from.h * 0.5,
        to.x + to.w * 0.5,
        to.y + to.h * 0.5,
        4.0,
        Color::new(0.98, 0.48, 0.25, 0.8),
    );
    draw_circle_lines(
        to.x + to.w * 0.5,
        to.y + to.h * 0.5,
        to.w * 0.36,
        3.0,
        Color::new(0.98, 0.48, 0.25, 0.9),
    );
}

fn draw_weapon_range(session: &GameSession, hostile_id: &str, view: GridView) {
    for tile in threatened_tiles(session, hostile_id) {
        let rect = view.tile_rect(tile);
        draw_rectangle(
            rect.x + 3.0,
            rect.y + 3.0,
            rect.w - 6.0,
            rect.h - 6.0,
            Color::new(0.82, 0.20, 0.16, 0.10),
        );
        draw_rectangle_lines(
            rect.x + 4.0,
            rect.y + 4.0,
            rect.w - 8.0,
            rect.h - 8.0,
            1.0,
            Color::new(0.96, 0.40, 0.28, 0.34),
        );
    }
}

pub(crate) fn threatened_tiles(session: &GameSession, hostile_id: &str) -> Vec<TilePos> {
    let Some(hostile) = session.unit(hostile_id) else {
        return Vec::new();
    };
    session
        .tactical
        .fog
        .iter_with_pos()
        .map(|(position, _)| position)
        .filter(|position| {
            *position != hostile.position
                && manhattan(hostile.position, *position) <= i32::from(hostile.weapon_range)
                && session.has_line_of_fire(hostile.position, *position)
        })
        .collect()
}

fn action_label(session: &GameSession, action: &IntentAction) -> String {
    match action {
        IntentAction::AttackUnit(target_id) => {
            let name = session
                .unit(target_id)
                .map_or(target_id.as_str(), |unit| &unit.name);
            format!("FIRST ACTION // ATTACK {}", name.to_uppercase())
        }
        IntentAction::AttackObjective => "FIRST ACTION // ATTACK FIELD ASSET".to_owned(),
        IntentAction::Advance { target, .. } => format!("FIRST ACTION // ADVANCE ON {}", target),
        IntentAction::Hold => "FIRST ACTION // HOLD POSITION".to_owned(),
    }
}

fn faction_label(faction: Option<&str>) -> &'static str {
    match faction {
        Some("directorate") => "DIRECTORATE",
        Some("brood") => "BROOD",
        Some("ascendants") => "ASCENDANT",
        _ => "UNKNOWN POWER",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::{GameData, Team};

    #[test]
    fn weapon_range_respects_distance_and_blocked_line_of_fire() {
        let data = GameData::load().unwrap();
        let mut session = GameSession::new(&data.config, &data.mission, &data.roster);
        let hostile = session
            .tactical
            .units
            .iter_mut()
            .find(|unit| unit.team == Team::Hostile)
            .unwrap();
        hostile.position = TilePos::new(6, 3);
        hostile.weapon_range = 3;
        let hostile_id = hostile.id.clone();
        session.tactical.blocked.clear();
        session.tactical.blocked.insert(TilePos::new(6, 2));

        let threatened = threatened_tiles(&session, &hostile_id);

        assert!(threatened.contains(&TilePos::new(9, 3)));
        assert!(!threatened.contains(&TilePos::new(6, 1)));
        assert!(!threatened.contains(&TilePos::new(10, 3)));
    }
}
