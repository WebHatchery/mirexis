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
        "controlled_burst" => {
            let target_id = target_id.expect("validated burst target");
            events.extend(session.execute_attack(unit_id, target_id));
            if session
                .unit(target_id)
                .is_some_and(|target| !target.incapacitated)
            {
                events.extend(session.execute_attack(unit_id, target_id));
            }
        }
        "armour_drill" => {
            spend_one_action_point(session, unit_id);
            unit_mut(session, unit_id)
                .expect("validated skill user")
                .next_attack_ignores_armour = true;
        }
        "interpose" => {
            spend_one_action_point(session, unit_id);
            crate::class_actions::apply_status(
                session,
                target_id.expect("validated ally target"),
                StatusKind::Guarded,
                1,
                &mut events,
            );
            crate::class_actions::apply_status(
                session,
                unit_id,
                StatusKind::Guarded,
                1,
                &mut events,
            );
        }
        "anchor_point" => {
            spend_one_action_point(session, unit_id);
            crate::class_actions::apply_status(
                session,
                unit_id,
                StatusKind::Guarded,
                1,
                &mut events,
            );
        }
        "slipstep" => {
            spend_one_action_point(session, unit_id);
            events.extend(
                session.execute_move(unit_id, target_tile.expect("validated slipstep tile")),
            );
        }
        "spotters_mark" => {
            spend_one_action_point(session, unit_id);
            crate::class_actions::apply_status(
                session,
                target_id.expect("validated mark target"),
                StatusKind::Marked,
                1,
                &mut events,
            );
        }
        "kinetic_draw" => {
            spend_one_action_point(session, unit_id);
            let target_id = target_id.expect("validated kinetic draw target");
            if let Some(destination) = kinetic_draw_destination(
                session,
                session.unit(unit_id).expect("validated draw user").position,
                session
                    .unit(target_id)
                    .expect("validated draw target")
                    .position,
            ) {
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
        }
        "premonition" => {
            spend_one_action_point(session, unit_id);
            crate::class_actions::apply_status(
                session,
                target_id.expect("validated premonition target"),
                StatusKind::Disrupted,
                1,
                &mut events,
            );
        }
        "adaptive_secretion" => {
            spend_one_action_point(session, unit_id);
            let target_id = target_id.expect("validated adaptive secretion target");
            let hazard = adaptive_hazard(session, target_id).expect("validated visible hazard");
            unit_mut(session, target_id)
                .expect("validated adaptive secretion target")
                .hazard_resistance = Some(hazard);
            crate::class_actions::apply_status(
                session,
                target_id,
                StatusKind::Adapted,
                1,
                &mut events,
            );
        }
        "spore_veil" => {
            spend_one_action_point(session, unit_id);
            session.tactical.obscuring_fields.push(ObscuringField {
                center: target_tile.expect("validated spore veil tile"),
                radius: 1,
                remaining_phases: 1,
            });
        }
        "stabilise" => {
            spend_one_action_point(session, unit_id);
            stabilise_target(
                session,
                target_id.expect("validated stabilise target"),
                &mut events,
            );
        }
        "combat_stimulant" => {
            spend_one_action_point(session, unit_id);
            let target_id = target_id.expect("validated stimulant target");
            let target = unit_mut(session, target_id).expect("validated stimulant target");
            target.action_points = target.action_points.saturating_add(2);
            crate::class_actions::apply_status(
                session,
                target_id,
                StatusKind::Hindered,
                2,
                &mut events,
            );
        }
        "portable_cover" => {
            spend_one_action_point(session, unit_id);
            place_portable_cover(
                session,
                unit_id,
                target_tile.expect("validated portable cover tile"),
            );
        }
        "overcharge" => {
            spend_one_action_point(session, unit_id);
            unit_mut(session, unit_id)
                .expect("validated overcharge user")
                .next_equipment_overcharged = true;
        }
        _ => unreachable!("validated skill has an implementation"),
    }
    if skill_id != "controlled_burst" {
        session.check_outcome(&mut events);
    }
    events
}
