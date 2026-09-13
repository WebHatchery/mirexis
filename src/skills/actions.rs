//! Technique execution actions and their tactical mutations.

use super::*;

pub fn execute(
    session: &mut GameSession,
    unit_id: &str,
    skill_id: &str,
    target_id: Option<&str>,
    target_tile: Option<TilePos>,
) -> Vec<BattleEvent> {
    mark_used(session, unit_id, skill_id);
    let mut events = vec![BattleEvent::SkillActivated {
        unit_id: unit_id.to_owned(),
        skill_id: skill_id.to_owned(),
    }];
    match skill_id {
        "controlled_burst" => execute_controlled_burst(session, unit_id, target_id, &mut events),
        "stabilise" => execute_stabilise(session, unit_id, target_id, &mut events),
        "armour_drill" | "interpose" | "anchor_point" | "spotters_mark" | "premonition"
        | "adaptive_secretion" | "combat_stimulant" | "overcharge" => {
            execute_status_skill(session, unit_id, skill_id, target_id, &mut events)
        }
        "slipstep" | "kinetic_draw" | "spore_veil" | "portable_cover" => execute_positioning_skill(
            session,
            unit_id,
            skill_id,
            target_id,
            target_tile,
            &mut events,
        ),
        _ => unreachable!("validated skill has an implementation"),
    }
    if skill_id != "controlled_burst" {
        session.check_outcome(&mut events);
    }
    events
}

fn execute_controlled_burst(
    session: &mut GameSession,
    unit_id: &str,
    target_id: Option<&str>,
    events: &mut Vec<BattleEvent>,
) {
    let target_id = target_id.expect("validated burst target");
    events.extend(session.execute_attack(unit_id, target_id));
    if session
        .unit(target_id)
        .is_some_and(|target| !target.incapacitated)
    {
        events.extend(session.execute_attack(unit_id, target_id));
    }
}

fn execute_stabilise(
    session: &mut GameSession,
    unit_id: &str,
    target_id: Option<&str>,
    events: &mut Vec<BattleEvent>,
) {
    spend_one_action_point(session, unit_id);
    stabilise_target(
        session,
        target_id.expect("validated stabilise target"),
        events,
    );
}

fn execute_status_skill(
    session: &mut GameSession,
    unit_id: &str,
    skill_id: &str,
    target_id: Option<&str>,
    events: &mut Vec<BattleEvent>,
) {
    spend_one_action_point(session, unit_id);
    match skill_id {
        "armour_drill" => {
            unit_mut(session, unit_id)
                .expect("validated skill user")
                .next_attack_ignores_armour = true;
        }
        "interpose" => {
            crate::class_actions::apply_status(
                session,
                target_id.expect("validated ally target"),
                StatusKind::Guarded,
                1,
                events,
            );
            crate::class_actions::apply_status(session, unit_id, StatusKind::Guarded, 1, events);
        }
        "anchor_point" => {
            crate::class_actions::apply_status(session, unit_id, StatusKind::Guarded, 1, events)
        }
        "spotters_mark" => crate::class_actions::apply_status(
            session,
            target_id.expect("validated mark target"),
            StatusKind::Marked,
            1,
            events,
        ),
        "premonition" => crate::class_actions::apply_status(
            session,
            target_id.expect("validated premonition target"),
            StatusKind::Disrupted,
            1,
            events,
        ),
        "adaptive_secretion" => {
            let target_id = target_id.expect("validated adaptive secretion target");
            let hazard = adaptive_hazard(session, target_id).expect("validated visible hazard");
            unit_mut(session, target_id)
                .expect("validated adaptive secretion target")
                .hazard_resistance = Some(hazard);
            crate::class_actions::apply_status(session, target_id, StatusKind::Adapted, 1, events);
        }
        "combat_stimulant" => {
            let target = unit_mut(session, target_id.expect("validated stimulant target"))
                .expect("validated stimulant target");
            target.action_points = target.action_points.saturating_add(2);
            crate::class_actions::apply_status(
                session,
                target_id.expect("validated stimulant target"),
                StatusKind::Hindered,
                2,
                events,
            );
        }
        "overcharge" => {
            unit_mut(session, unit_id)
                .expect("validated overcharge user")
                .next_equipment_overcharged = true;
        }
        _ => unreachable!("validated status skill"),
    }
}

fn execute_positioning_skill(
    session: &mut GameSession,
    unit_id: &str,
    skill_id: &str,
    target_id: Option<&str>,
    target_tile: Option<TilePos>,
    events: &mut Vec<BattleEvent>,
) {
    spend_one_action_point(session, unit_id);
    match skill_id {
        "slipstep" => events
            .extend(session.execute_move(unit_id, target_tile.expect("validated slipstep tile"))),
        "kinetic_draw" => execute_kinetic_draw(session, unit_id, target_id, events),
        "spore_veil" => session.tactical.obscuring_fields.push(ObscuringField {
            center: target_tile.expect("validated spore veil tile"),
            radius: 1,
            remaining_phases: 1,
        }),
        "portable_cover" => place_portable_cover(
            session,
            unit_id,
            target_tile.expect("validated portable cover tile"),
        ),
        _ => unreachable!("validated positioning skill"),
    }
}

fn execute_kinetic_draw(
    session: &mut GameSession,
    unit_id: &str,
    target_id: Option<&str>,
    events: &mut Vec<BattleEvent>,
) {
    let target_id = target_id.expect("validated kinetic draw target");
    let Some(destination) = kinetic_draw_destination(
        session,
        session.unit(unit_id).expect("validated draw user").position,
        session
            .unit(target_id)
            .expect("validated draw target")
            .position,
    ) else {
        return;
    };
    let target = unit_mut(session, target_id).expect("validated draw target");
    let from = target.position;
    target.position = destination;
    target.facing = UnitFacing::toward(from, destination);
    events.push(BattleEvent::UnitMoved {
        unit_id: target_id.to_owned(),
        path: vec![from, destination],
        cost: 0,
    });
    events.extend(crate::hazards::resolve_after_move(session, target_id));
}
