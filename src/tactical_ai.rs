//! Deterministic hostile target selection and movement.

use crate::data::Team;
use crate::state::{Command, GameSession};
use crate::tactical::manhattan;
use macroquad_toolkit::grid::TilePos;

pub(crate) fn resolve_enemy_phase(session: &mut GameSession) {
    let mut enemies = session
        .tactical
        .units
        .iter()
        .filter(|unit| unit.team == Team::Hostile && !unit.incapacitated)
        .map(|unit| unit.id.clone())
        .collect::<Vec<_>>();
    enemies.sort();
    for enemy_id in enemies {
        if session.battle_is_over() {
            break;
        }
        if let Some(target_id) = best_attack_target(session, &enemy_id) {
            let _ = session.execute(Command::Attack {
                attacker_id: enemy_id.clone(),
                target_id,
            });
            continue;
        }
        if let Some(destination) = best_enemy_move(session, &enemy_id) {
            let _ = session.execute(Command::Move {
                unit_id: enemy_id.clone(),
                to: destination,
            });
        }
        if let Some(target_id) = best_attack_target(session, &enemy_id) {
            let _ = session.execute(Command::Attack {
                attacker_id: enemy_id.clone(),
                target_id,
            });
        }
    }
}

fn best_attack_target(session: &GameSession, attacker_id: &str) -> Option<String> {
    let mut targets = session
        .tactical
        .units
        .iter()
        .filter(|unit| {
            unit.team == Team::Colony
                && !unit.incapacitated
                && session
                    .validate(&Command::Attack {
                        attacker_id: attacker_id.to_owned(),
                        target_id: unit.id.clone(),
                    })
                    .is_ok()
        })
        .collect::<Vec<_>>();
    targets.sort_by_key(|unit| (unit.health, unit.id.clone()));
    targets.first().map(|unit| unit.id.clone())
}

fn best_enemy_move(session: &GameSession, enemy_id: &str) -> Option<TilePos> {
    let enemy = session.unit(enemy_id)?;
    let target = session
        .tactical
        .units
        .iter()
        .filter(|unit| unit.team == Team::Colony && !unit.incapacitated)
        .min_by_key(|unit| (manhattan(enemy.position, unit.position), unit.id.clone()))?;
    let mut candidates = enemy
        .position
        .neighbors_4way()
        .into_iter()
        .filter(|to| {
            session
                .validate(&Command::Move {
                    unit_id: enemy_id.to_owned(),
                    to: *to,
                })
                .is_ok()
        })
        .collect::<Vec<_>>();
    candidates.sort_by_key(|to| (manhattan(*to, target.position), to.y, to.x));
    candidates.first().copied()
}
