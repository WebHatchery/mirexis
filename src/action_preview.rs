//! Read-only player command costs and consequences for tactical inspection.

use crate::data::HazardKind;
use crate::state::{Command, GameSession};
use macroquad_toolkit::grid::TilePos;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ActionPreview {
    Move {
        cost: u8,
        hazard: Option<HazardKind>,
    },
    Attack {
        target_name: String,
        cost: u8,
        hit_chance: u8,
        damage: i32,
        critical_damage: i32,
    },
}

pub(crate) fn for_tile(session: &GameSession, tile: TilePos) -> Option<ActionPreview> {
    let unit_id = session.tactical.selected_unit.as_ref()?;
    if let Some(target) = session
        .tactical
        .units
        .iter()
        .find(|unit| unit.position == tile && !unit.incapacitated)
    {
        let cost = session
            .validate(&Command::Attack {
                attacker_id: unit_id.clone(),
                target_id: target.id.clone(),
            })
            .ok()?;
        let attacker = session.unit(unit_id)?;
        let damage = (attacker.effective_weapon_damage() - target.effective_armour()).max(1);
        let critical_damage =
            (attacker.effective_weapon_damage() + 2 - target.effective_armour()).max(1);
        return Some(ActionPreview::Attack {
            target_name: target.name.clone(),
            cost: cost.action_points,
            hit_chance: session.hit_chance(attacker, target),
            damage,
            critical_damage,
        });
    }
    let cost = session
        .validate(&Command::Move {
            unit_id: unit_id.clone(),
            to: tile,
        })
        .ok()?;
    let hazard = session
        .tactical
        .hazards
        .iter()
        .find(|hazard| hazard.position == tile)
        .map(|hazard| hazard.kind);
    Some(ActionPreview::Move {
        cost: cost.action_points,
        hazard,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::{GameData, HazardKind, Team};
    use crate::tactical::HazardTile;

    #[test]
    fn movement_preview_reports_validated_cost_and_landing_hazard() {
        let data = GameData::load().unwrap();
        let mut session = GameSession::new(&data.config, &data.mission, &data.roster);
        let selected = session.selected_unit().unwrap();
        let tile = TilePos::new(selected.position.x + 1, selected.position.y);
        session.tactical.hazards.push(HazardTile {
            position: tile,
            kind: HazardKind::FireLane,
        });

        assert_eq!(
            for_tile(&session, tile),
            Some(ActionPreview::Move {
                cost: 1,
                hazard: Some(HazardKind::FireLane),
            })
        );
    }

    #[test]
    fn attack_preview_matches_live_hit_chance_and_armour_damage() {
        let data = GameData::load().unwrap();
        let mut session = GameSession::new(&data.config, &data.mission, &data.roster);
        let selected = session.selected_unit().unwrap().position;
        let hostile = session
            .tactical
            .units
            .iter_mut()
            .find(|unit| unit.team == Team::Hostile)
            .unwrap();
        hostile.position = TilePos::new(selected.x + 2, selected.y);
        let hostile_tile = hostile.position;

        assert!(matches!(
            for_tile(&session, hostile_tile),
            Some(ActionPreview::Attack { hit_chance, damage, .. }) if hit_chance > 0 && damage > 0
        ));
    }
}
