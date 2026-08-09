//! Class-specific tactical actions and their status effects.

use crate::data::Team;
use crate::state::{BattleEvent, CommandCost, GameSession, RuleError};
use crate::tactical::{manhattan, StatusEffect, StatusKind};

pub(crate) fn action_name(class_id: &str) -> Option<&'static str> {
    match class_id {
        "soldier" => Some("STEADY AIM"),
        "defender" => Some("BRACE"),
        "scout" => Some("SURGE"),
        "medic" => Some("FIELD DRESSING"),
        "engineer" => Some("SHOCK DRONE"),
        "psionic" => Some("NEURAL DISRUPT"),
        "biotech" => Some("SPORE WARD"),
        _ => None,
    }
}

pub(crate) fn validate(session: &GameSession, unit_id: &str) -> Result<CommandCost, RuleError> {
    let unit = session.unit(unit_id).ok_or(RuleError::UnknownUnit)?;
    if session.tactical.phase != crate::state::TacticalPhase::Player {
        return Err(RuleError::WrongPhase);
    }
    if unit.team != Team::Colony {
        return Err(RuleError::WrongTeam);
    }
    if unit.incapacitated {
        return Err(RuleError::Incapacitated);
    }
    if unit.class_action_used || action_name(&unit.class_id).is_none() {
        return Err(RuleError::ClassActionUnavailable);
    }
    if unit.action_points < 1 {
        return Err(RuleError::InsufficientActionPoints);
    }
    let available = match unit.class_id.as_str() {
        "medic" => session.tactical.units.iter().any(|ally| {
            ally.team == Team::Colony && !ally.incapacitated && ally.health < ally.max_health
        }),
        "engineer" => nearest_hostile(session, unit_id, 4).is_some(),
        "psionic" => nearest_hostile(session, unit_id, 5).is_some(),
        _ => true,
    };
    available
        .then_some(CommandCost { action_points: 1 })
        .ok_or(RuleError::ClassActionUnavailable)
}

pub(crate) fn execute(session: &mut GameSession, unit_id: &str) -> Vec<BattleEvent> {
    let class_id = session.unit(unit_id).unwrap().class_id.clone();
    let action = action_name(&class_id).unwrap().to_owned();
    let unit = session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == unit_id)
        .unwrap();
    unit.action_points -= 1;
    unit.class_action_used = true;
    let mut events = vec![BattleEvent::ClassActionActivated {
        unit_id: unit_id.to_owned(),
        action,
    }];
    match class_id.as_str() {
        "soldier" => apply_status(session, unit_id, StatusKind::Focused, 1, &mut events),
        "defender" => apply_status(session, unit_id, StatusKind::Guarded, 2, &mut events),
        "scout" => {
            let unit = session
                .tactical
                .units
                .iter_mut()
                .find(|unit| unit.id == unit_id)
                .unwrap();
            unit.action_points = unit.action_points.saturating_add(3);
            apply_status(session, unit_id, StatusKind::Quickened, 1, &mut events);
        }
        "medic" => heal_most_wounded(session, &mut events),
        "engineer" => damage_nearest(session, unit_id, 4, 3, &mut events),
        "psionic" => {
            let target = nearest_hostile(session, unit_id, 5).unwrap();
            apply_status(session, &target, StatusKind::Disrupted, 1, &mut events);
        }
        "biotech" => {
            let allies = session
                .tactical
                .units
                .iter()
                .filter(|unit| unit.team == Team::Colony && !unit.incapacitated)
                .map(|unit| unit.id.clone())
                .collect::<Vec<_>>();
            for ally in allies {
                apply_status(session, &ally, StatusKind::Regenerating, 2, &mut events);
            }
        }
        _ => unreachable!("validated class has an action"),
    }
    session.check_outcome(&mut events);
    events
}

fn nearest_hostile(session: &GameSession, unit_id: &str, range: i32) -> Option<String> {
    let origin = session.unit(unit_id)?.position;
    session
        .tactical
        .units
        .iter()
        .filter(|unit| {
            unit.team == Team::Hostile
                && !unit.incapacitated
                && manhattan(origin, unit.position) <= range
        })
        .min_by_key(|unit| {
            (
                manhattan(origin, unit.position),
                unit.health,
                unit.id.clone(),
            )
        })
        .map(|unit| unit.id.clone())
}

fn apply_status(
    session: &mut GameSession,
    unit_id: &str,
    kind: StatusKind,
    phases: u8,
    events: &mut Vec<BattleEvent>,
) {
    let unit = session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == unit_id)
        .unwrap();
    if let Some(status) = unit.statuses.iter_mut().find(|status| status.kind == kind) {
        status.remaining_phases = status.remaining_phases.max(phases);
    } else {
        unit.statuses.push(StatusEffect {
            kind,
            remaining_phases: phases,
        });
    }
    events.push(BattleEvent::StatusApplied {
        unit_id: unit_id.to_owned(),
        status: kind,
    });
}

fn heal_most_wounded(session: &mut GameSession, events: &mut Vec<BattleEvent>) {
    let target_id = session
        .tactical
        .units
        .iter()
        .filter(|unit| {
            unit.team == Team::Colony && !unit.incapacitated && unit.health < unit.max_health
        })
        .min_by_key(|unit| (unit.health * 100 / unit.max_health.max(1), unit.id.clone()))
        .unwrap()
        .id
        .clone();
    let target = session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == target_id)
        .unwrap();
    let before = target.health;
    target.health = (target.health + 4).min(target.max_health);
    events.push(BattleEvent::UnitHealed {
        unit_id: target_id,
        amount: target.health - before,
        remaining: target.health,
    });
}

fn damage_nearest(
    session: &mut GameSession,
    unit_id: &str,
    range: i32,
    damage: i32,
    events: &mut Vec<BattleEvent>,
) {
    let target_id = nearest_hostile(session, unit_id, range).unwrap();
    let target = session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == target_id)
        .unwrap();
    target.health = (target.health - damage).max(0);
    events.push(BattleEvent::DamageApplied {
        target_id: target_id.clone(),
        amount: damage,
        remaining: target.health,
    });
    if target.health == 0 {
        target.incapacitated = true;
        events.push(BattleEvent::UnitIncapacitated { unit_id: target_id });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::campaign::CampaignState;
    use crate::data::GameData;
    use crate::state::GameSession;
    use macroquad_toolkit::grid::TilePos;

    fn session() -> GameSession {
        let data = GameData::load().unwrap();
        let campaign = CampaignState::new(&data);
        let roster = campaign.deployment_roster(&data, &data.mission);
        GameSession::new(&data.config, &data.mission, &roster)
    }

    #[test]
    fn starting_classes_each_change_the_battle_state() {
        let mut scout = session();
        let before_ap = scout.unit("kira_voss").unwrap().action_points;
        scout.tactical.selected_unit = Some("kira_voss".into());
        scout.activate_selected_class_action().unwrap();
        let kira = scout.unit("kira_voss").unwrap();
        assert_eq!(kira.action_points, before_ap + 2);
        assert!(kira.has_status(StatusKind::Quickened));
        assert!(!scout.can_activate_selected_class_action());

        let mut defender = session();
        let base_armour = defender.unit("mara_venn").unwrap().effective_armour();
        defender.tactical.selected_unit = Some("mara_venn".into());
        defender.activate_selected_class_action().unwrap();
        assert_eq!(
            defender.unit("mara_venn").unwrap().effective_armour(),
            base_armour + 2
        );
        let config = GameData::load().unwrap().config;
        defender.end_player_phase(&config);
        assert!(defender
            .unit("mara_venn")
            .unwrap()
            .has_status(StatusKind::Guarded));

        let mut medic = session();
        medic
            .tactical
            .units
            .iter_mut()
            .find(|unit| unit.id == "mara_venn")
            .unwrap()
            .health = 3;
        medic.tactical.selected_unit = Some("ilya_reed".into());
        medic.activate_selected_class_action().unwrap();
        assert_eq!(medic.unit("mara_venn").unwrap().health, 7);

        let mut engineer = session();
        engineer
            .tactical
            .units
            .iter_mut()
            .find(|unit| unit.id == "sol_cairn")
            .unwrap()
            .position = TilePos::new(7, 2);
        let before_health = engineer.unit("brood_stalker_a").unwrap().health;
        engineer.tactical.selected_unit = Some("sol_cairn".into());
        engineer.activate_selected_class_action().unwrap();
        assert_eq!(
            engineer.unit("brood_stalker_a").unwrap().health,
            before_health - 3
        );
    }

    #[test]
    fn retrained_classes_expose_their_own_status_actions() {
        for (class_id, expected_status) in [
            ("soldier", StatusKind::Focused),
            ("psionic", StatusKind::Disrupted),
            ("biotech", StatusKind::Regenerating),
        ] {
            let mut session = session();
            let kira = session
                .tactical
                .units
                .iter_mut()
                .find(|unit| unit.id == "kira_voss")
                .unwrap();
            kira.class_id = class_id.to_owned();
            kira.position = TilePos::new(7, 2);
            session.tactical.selected_unit = Some("kira_voss".into());
            session.activate_selected_class_action().unwrap();
            assert!(session
                .tactical
                .units
                .iter()
                .any(|unit| unit.has_status(expected_status)));
        }
    }
}
