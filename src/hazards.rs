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
mod tests;
