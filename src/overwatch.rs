//! Prepaid reaction fire resolved during hostile movement.

use crate::data::Team;
use crate::state::GameSession;
use crate::tactical::{manhattan, BattleEvent, CommandCost, RuleError};

const REACTION_ACCURACY_PENALTY: i32 = 15;

impl GameSession {
    pub fn set_selected_overwatch(&mut self) -> Result<Vec<BattleEvent>, RuleError> {
        let unit_id = self
            .tactical
            .selected_unit
            .clone()
            .ok_or(RuleError::UnknownUnit)?;
        self.execute(crate::tactical::Command::SetOverwatch { unit_id })
    }

    pub fn can_set_selected_overwatch(&self) -> bool {
        self.tactical.selected_unit.as_ref().is_some_and(|unit_id| {
            self.validate(&crate::tactical::Command::SetOverwatch {
                unit_id: unit_id.clone(),
            })
            .is_ok()
        })
    }
}

pub(crate) fn validate(session: &GameSession, unit_id: &str) -> Result<CommandCost, RuleError> {
    let unit = session.active_unit_for_phase(unit_id)?;
    if unit.team != Team::Colony || unit.overwatching {
        return Err(RuleError::OverwatchUnavailable);
    }
    if unit.action_points < unit.weapon_ap_cost {
        return Err(RuleError::InsufficientActionPoints);
    }
    Ok(CommandCost {
        action_points: unit.weapon_ap_cost,
    })
}

pub(crate) fn execute(session: &mut GameSession, unit_id: &str) -> Vec<BattleEvent> {
    let unit = session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == unit_id)
        .unwrap();
    unit.action_points -= unit.weapon_ap_cost;
    unit.overwatching = true;
    vec![BattleEvent::OverwatchSet {
        unit_id: unit_id.to_owned(),
    }]
}

pub(crate) fn resolve_after_hostile_move(
    session: &mut GameSession,
    target_id: &str,
) -> Vec<BattleEvent> {
    let Some(target) = session.unit(target_id) else {
        return Vec::new();
    };
    if target.team != Team::Hostile || target.incapacitated {
        return Vec::new();
    }
    let mut reactors = session
        .tactical
        .units
        .iter()
        .filter(|unit| unit.team == Team::Colony && unit.overwatching && !unit.incapacitated)
        .filter(|unit| {
            manhattan(unit.position, target.position) <= i32::from(unit.weapon_range)
                && session.has_line_of_fire(unit.position, target.position)
        })
        .map(|unit| unit.id.clone())
        .collect::<Vec<_>>();
    reactors.sort();

    let mut events = Vec::new();
    for attacker_id in reactors {
        if session
            .unit(target_id)
            .is_none_or(|target| target.incapacitated)
        {
            break;
        }
        events.extend(resolve_reaction(session, &attacker_id, target_id));
    }
    session.check_outcome(&mut events);
    events
}

fn resolve_reaction(
    session: &mut GameSession,
    attacker_id: &str,
    target_id: &str,
) -> Vec<BattleEvent> {
    let attacker = session.unit(attacker_id).unwrap().clone();
    let target = session.unit(target_id).unwrap().clone();
    let hit_chance = (i32::from(session.hit_chance(&attacker, &target)) - REACTION_ACCURACY_PENALTY)
        .clamp(5, 95) as u8;
    let roll = session.tactical.rng.range_i32(1, 101) as u8;
    session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == attacker_id)
        .unwrap()
        .overwatching = false;

    let mut events = vec![
        BattleEvent::ReactionTriggered {
            attacker_id: attacker_id.to_owned(),
            target_id: target_id.to_owned(),
        },
        BattleEvent::AttackRolled {
            attacker_id: attacker_id.to_owned(),
            target_id: target_id.to_owned(),
            roll,
            hit_chance,
        },
    ];
    if roll > hit_chance {
        return events;
    }

    let critical = roll <= 10;
    let damage = (attacker.effective_weapon_damage() + i32::from(critical) * 2
        - target.effective_armour())
    .max(1);
    let target = session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == target_id)
        .unwrap();
    target.health = (target.health - damage).max(0);
    events.push(BattleEvent::DamageApplied {
        target_id: target_id.to_owned(),
        amount: damage,
        remaining: target.health,
    });
    if target.health == 0 {
        target.incapacitated = true;
        events.push(BattleEvent::UnitIncapacitated {
            unit_id: target_id.to_owned(),
        });
    }
    events
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::GameData;
    use crate::state::Command;
    use macroquad_toolkit::grid::TilePos;

    #[test]
    fn hostile_movement_triggers_one_prepaid_reaction() {
        let data = GameData::load().unwrap();
        let mut session = GameSession::new(&data.config, &data.mission, &data.roster);
        let kira = session
            .tactical
            .units
            .iter_mut()
            .find(|unit| unit.id == "kira_voss")
            .unwrap();
        kira.position = TilePos::new(6, 3);
        kira.accuracy = 100;
        let hostile = session
            .tactical
            .units
            .iter_mut()
            .find(|unit| unit.team == Team::Hostile)
            .unwrap();
        hostile.position = TilePos::new(8, 4);
        let hostile_id = hostile.id.clone();

        session
            .execute(Command::SetOverwatch {
                unit_id: "kira_voss".to_owned(),
            })
            .unwrap();
        session.tactical.phase = crate::tactical::TacticalPhase::Enemy;
        let events = session
            .execute(Command::Move {
                unit_id: hostile_id.clone(),
                to: TilePos::new(8, 3),
            })
            .unwrap();

        assert!(events.iter().any(|event| matches!(
            event,
            BattleEvent::ReactionTriggered { target_id, .. } if target_id == &hostile_id
        )));
        assert!(!session.unit("kira_voss").unwrap().overwatching);
    }

    #[test]
    fn reaction_waits_until_a_hostile_enters_the_weapon_envelope() {
        let data = GameData::load().unwrap();
        let mut session = GameSession::new(&data.config, &data.mission, &data.roster);
        let kira = session
            .tactical
            .units
            .iter_mut()
            .find(|unit| unit.id == "kira_voss")
            .unwrap();
        kira.position = TilePos::new(1, 1);
        kira.weapon_range = 2;
        session
            .execute(Command::SetOverwatch {
                unit_id: "kira_voss".to_owned(),
            })
            .unwrap();
        session.tactical.phase = crate::tactical::TacticalPhase::Enemy;
        let hostile = session
            .tactical
            .units
            .iter_mut()
            .find(|unit| unit.team == Team::Hostile)
            .unwrap();
        hostile.position = TilePos::new(7, 6);
        let hostile_id = hostile.id.clone();
        let events = session
            .execute(Command::Move {
                unit_id: hostile_id,
                to: TilePos::new(6, 6),
            })
            .unwrap();

        assert!(!events
            .iter()
            .any(|event| matches!(event, BattleEvent::ReactionTriggered { .. })));
        assert!(session.unit("kira_voss").unwrap().overwatching);
    }

    #[test]
    fn untriggered_overwatch_expires_at_the_next_player_phase() {
        let data = GameData::load().unwrap();
        let mut session = GameSession::new(&data.config, &data.mission, &data.roster);
        let kira = session
            .tactical
            .units
            .iter_mut()
            .find(|unit| unit.id == "kira_voss")
            .unwrap();
        kira.position = TilePos::new(0, 0);
        kira.weapon_range = 1;
        let attack_cost = kira.weapon_ap_cost;
        let starting_ap = kira.action_points;

        session.set_selected_overwatch().unwrap();
        assert_eq!(
            session.unit("kira_voss").unwrap().action_points,
            starting_ap - attack_cost
        );
        session.end_player_phase(&data.config);

        assert_eq!(
            session.tactical.phase,
            crate::tactical::TacticalPhase::Player
        );
        assert!(!session.unit("kira_voss").unwrap().overwatching);
    }
}
