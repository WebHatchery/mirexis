//! Read-only player command costs and consequences for tactical inspection.

use crate::data::HazardKind;
use crate::state::{Command, GameSession, RuleError};
use macroquad_toolkit::grid::TilePos;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ActionPreview {
    Move {
        cost: u8,
        hazard: Option<HazardKind>,
        path: Vec<TilePos>,
    },
    Attack {
        target_name: String,
        attacker_position: TilePos,
        target_position: TilePos,
        cost: u8,
        hit_chance: u8,
        cover_penalty: i32,
        damage: i32,
        critical_damage: i32,
    },
    Invalid {
        action: &'static str,
        reason: RuleError,
    },
}

pub(crate) fn for_tile(session: &GameSession, tile: TilePos) -> Option<ActionPreview> {
    let unit_id = session.tactical.selected_unit.as_ref()?;
    let selected = session.unit(unit_id)?;
    if let Some(target) = session
        .tactical
        .units
        .iter()
        .find(|unit| unit.position == tile && !unit.incapacitated)
    {
        if target.team == selected.team {
            return None;
        }
        let command = Command::Attack {
            attacker_id: unit_id.clone(),
            target_id: target.id.clone(),
        };
        let cost = match session.validate(&command) {
            Ok(cost) => cost,
            Err(reason) => {
                return Some(ActionPreview::Invalid {
                    action: "ATTACK",
                    reason,
                });
            }
        };
        let attacker = selected;
        let damage = (attacker.effective_weapon_damage() - target.effective_armour()).max(1);
        let critical_damage =
            (attacker.effective_weapon_damage() + 2 - target.effective_armour()).max(1);
        return Some(ActionPreview::Attack {
            target_name: target.name.clone(),
            attacker_position: attacker.position,
            target_position: target.position,
            cost: cost.action_points,
            hit_chance: session.hit_chance(attacker, target),
            cover_penalty: session.cover_penalty(attacker, target),
            damage,
            critical_damage,
        });
    }
    if tile == selected.position {
        return None;
    }
    let command = Command::Move {
        unit_id: unit_id.clone(),
        to: tile,
    };
    let cost = match session.validate(&command) {
        Ok(cost) => cost,
        Err(reason) => {
            return Some(ActionPreview::Invalid {
                action: "MOVE",
                reason,
            });
        }
    };
    let hazard = session
        .tactical
        .hazards
        .iter()
        .find(|hazard| hazard.position == tile)
        .map(|hazard| hazard.kind);
    let path = session.movement_path(unit_id, tile)?;
    Some(ActionPreview::Move {
        cost: cost.action_points,
        hazard,
        path,
    })
}

#[cfg(test)]
mod tests;
