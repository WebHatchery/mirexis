//! Faction-shaped battlefield hazards resolved when movement ends.

use crate::data::HazardKind;
use crate::state::{BattleEvent, GameSession, StatusKind};

pub(crate) fn resolve_after_move(session: &mut GameSession, unit_id: &str) -> Vec<BattleEvent> {
    let Some(unit) = session.unit(unit_id) else {
        return Vec::new();
    };
    let Some(hazard) = session
        .tactical
        .hazards
        .iter()
        .find(|hazard| hazard.position == unit.position)
        .copied()
    else {
        return Vec::new();
    };
    if unit.faction.as_deref() == Some(immune_faction(hazard.kind)) {
        return Vec::new();
    }

    let mut events = Vec::new();
    match hazard.kind {
        HazardKind::FireLane => apply_damage(session, unit_id, 2, &mut events),
        HazardKind::SporeBloom => {
            apply_damage(session, unit_id, 1, &mut events);
            if session
                .unit(unit_id)
                .is_some_and(|unit| !unit.incapacitated)
            {
                crate::class_actions::apply_status(
                    session,
                    unit_id,
                    StatusKind::Hindered,
                    2,
                    &mut events,
                );
            }
        }
        HazardKind::StaticRift => crate::class_actions::apply_status(
            session,
            unit_id,
            StatusKind::Disrupted,
            2,
            &mut events,
        ),
    }
    events.push(BattleEvent::HazardTriggered {
        unit_id: unit_id.to_owned(),
        kind: hazard.kind,
    });
    session.check_outcome(&mut events);
    events
}

fn immune_faction(kind: HazardKind) -> &'static str {
    match kind {
        HazardKind::FireLane => "directorate",
        HazardKind::SporeBloom => "brood",
        HazardKind::StaticRift => "ascendants",
    }
}

fn apply_damage(
    session: &mut GameSession,
    unit_id: &str,
    damage: i32,
    events: &mut Vec<BattleEvent>,
) {
    let unit = session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == unit_id)
        .unwrap();
    unit.health = (unit.health - damage).max(0);
    events.push(BattleEvent::DamageApplied {
        target_id: unit_id.to_owned(),
        amount: damage,
        remaining: unit.health,
    });
    if unit.health == 0 {
        unit.incapacitated = true;
        events.push(BattleEvent::UnitIncapacitated {
            unit_id: unit_id.to_owned(),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::{GameData, Team};
    use crate::tactical::{Command, HazardTile, TacticalPhase};
    use macroquad_toolkit::grid::TilePos;

    fn session_with_hazard(kind: HazardKind) -> (GameSession, String, TilePos) {
        let data = GameData::load().unwrap();
        let mut session = GameSession::new(&data.config, &data.mission, &data.roster);
        let unit = session
            .tactical
            .units
            .iter_mut()
            .find(|unit| unit.team == Team::Colony)
            .unwrap();
        unit.position = TilePos::new(1, 1);
        unit.action_points = 6;
        let id = unit.id.clone();
        let destination = TilePos::new(2, 1);
        session.tactical.hazards = vec![HazardTile {
            position: destination,
            kind,
        }];
        (session, id, destination)
    }

    #[test]
    fn fire_lane_damages_a_colonist_who_ends_movement_inside_it() {
        let (mut session, unit_id, destination) = session_with_hazard(HazardKind::FireLane);
        let health = session.unit(&unit_id).unwrap().health;
        session
            .execute(Command::Move {
                unit_id: unit_id.clone(),
                to: destination,
            })
            .unwrap();
        assert_eq!(session.unit(&unit_id).unwrap().health, health - 2);
    }

    #[test]
    fn spore_bloom_damages_and_hinders_a_colonist() {
        let (mut session, unit_id, destination) = session_with_hazard(HazardKind::SporeBloom);
        session
            .execute(Command::Move {
                unit_id: unit_id.clone(),
                to: destination,
            })
            .unwrap();
        let unit = session.unit(&unit_id).unwrap();
        assert!(unit.has_status(StatusKind::Hindered));
        assert_eq!(unit.health, unit.max_health - 1);
    }

    #[test]
    fn static_rift_disrupts_a_colonist() {
        let (mut session, unit_id, destination) = session_with_hazard(HazardKind::StaticRift);
        session
            .execute(Command::Move {
                unit_id: unit_id.clone(),
                to: destination,
            })
            .unwrap();
        assert!(session
            .unit(&unit_id)
            .unwrap()
            .has_status(StatusKind::Disrupted));
    }

    #[test]
    fn a_faction_is_immune_to_its_own_hazard() {
        let data = GameData::load().unwrap();
        let mut session = GameSession::new(&data.config, &data.mission, &data.roster);
        session.tactical.phase = TacticalPhase::Enemy;
        let brood = session
            .tactical
            .units
            .iter_mut()
            .find(|unit| unit.faction.as_deref() == Some("brood"))
            .unwrap();
        brood.position = TilePos::new(6, 4);
        brood.action_points = 6;
        let id = brood.id.clone();
        let destination = TilePos::new(6, 5);
        session.tactical.hazards = vec![HazardTile {
            position: destination,
            kind: HazardKind::SporeBloom,
        }];
        let health = session.unit(&id).unwrap().health;
        session
            .execute(Command::Move {
                unit_id: id.clone(),
                to: destination,
            })
            .unwrap();
        assert_eq!(session.unit(&id).unwrap().health, health);
        assert!(!session.unit(&id).unwrap().has_status(StatusKind::Hindered));
    }
}
