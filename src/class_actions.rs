//! Class-specific tactical actions and their status effects.

use crate::data::Team;
use crate::state::{BattleEvent, Command, CommandCost, GameSession, RuleError};
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
        "vanguard" => Some("HOLD THE LINE"),
        "pathfinder" => Some("GHOST VECTOR"),
        "lifewright" => Some("VITAL CASCADE"),
        "null_adept" => Some("NULL LANCE"),
        _ => None,
    }
}

pub(crate) fn requires_target(class_id: &str) -> bool {
    matches!(
        class_id,
        "medic" | "engineer" | "psionic" | "lifewright" | "null_adept"
    )
}

impl GameSession {
    pub fn activate_selected_class_action(&mut self) -> Result<Vec<BattleEvent>, RuleError> {
        let unit_id = self
            .tactical
            .selected_unit
            .clone()
            .ok_or(RuleError::UnknownUnit)?;
        self.execute(Command::ActivateClassAction {
            unit_id,
            target_id: None,
        })
    }

    pub fn can_activate_selected_class_action(&self) -> bool {
        self.tactical.selected_unit.as_ref().is_some_and(|unit_id| {
            self.validate(&Command::ActivateClassAction {
                unit_id: unit_id.clone(),
                target_id: None,
            })
            .is_ok()
        })
    }

    pub fn can_target_class_action(&self, unit_id: &str, target_id: &str) -> bool {
        self.validate(&Command::ActivateClassAction {
            unit_id: unit_id.to_owned(),
            target_id: Some(target_id.to_owned()),
        })
        .is_ok()
    }

    pub fn activate_class_action_on(
        &mut self,
        unit_id: &str,
        target_id: &str,
    ) -> Result<Vec<BattleEvent>, RuleError> {
        self.execute(Command::ActivateClassAction {
            unit_id: unit_id.to_owned(),
            target_id: Some(target_id.to_owned()),
        })
    }
}

pub(crate) fn has_valid_target(session: &GameSession, unit_id: &str) -> bool {
    session
        .tactical
        .units
        .iter()
        .any(|target| session.can_target_class_action(unit_id, &target.id))
}

pub(crate) fn validate(
    session: &GameSession,
    unit_id: &str,
    target_id: Option<&str>,
) -> Result<CommandCost, RuleError> {
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
    let target_valid = match unit.class_id.as_str() {
        "medic" => target_matches(session, unit_id, target_id, Team::Colony, 4, |target| {
            target.health < target.max_health
        }),
        "engineer" => target_matches(session, unit_id, target_id, Team::Hostile, 4, |_| true),
        "psionic" => target_matches(session, unit_id, target_id, Team::Hostile, 5, |_| true),
        "lifewright" => target_matches(session, unit_id, target_id, Team::Colony, 5, |_| true),
        "null_adept" => target_matches(session, unit_id, target_id, Team::Hostile, 6, |_| true),
        _ => target_id.is_none(),
    };
    target_valid
        .then_some(CommandCost { action_points: 1 })
        .ok_or(RuleError::InvalidTarget)
}

pub(crate) fn execute(
    session: &mut GameSession,
    unit_id: &str,
    target_id: Option<&str>,
) -> Vec<BattleEvent> {
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
        "medic" => heal_target(session, target_id.unwrap(), &mut events),
        "engineer" => damage_target(session, target_id.unwrap(), 3, &mut events),
        "psionic" => {
            apply_status(
                session,
                target_id.unwrap(),
                StatusKind::Disrupted,
                1,
                &mut events,
            );
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
        "vanguard" => {
            let origin = session.unit(unit_id).unwrap().position;
            let allies = session
                .tactical
                .units
                .iter()
                .filter(|unit| {
                    unit.team == Team::Colony
                        && !unit.incapacitated
                        && manhattan(origin, unit.position) <= 2
                })
                .map(|unit| unit.id.clone())
                .collect::<Vec<_>>();
            for ally in allies {
                apply_status(session, &ally, StatusKind::Guarded, 2, &mut events);
            }
        }
        "pathfinder" => {
            let unit = session
                .tactical
                .units
                .iter_mut()
                .find(|unit| unit.id == unit_id)
                .unwrap();
            unit.action_points = unit.action_points.saturating_add(2);
            apply_status(session, unit_id, StatusKind::Quickened, 2, &mut events);
            apply_status(session, unit_id, StatusKind::Focused, 1, &mut events);
        }
        "lifewright" => {
            let target_id = target_id.unwrap();
            heal_target_amount(session, target_id, 3, &mut events);
            apply_status(session, target_id, StatusKind::Regenerating, 2, &mut events);
        }
        "null_adept" => {
            let target_id = target_id.unwrap();
            damage_target(session, target_id, 2, &mut events);
            if !session.unit(target_id).unwrap().incapacitated {
                apply_status(session, target_id, StatusKind::Disrupted, 2, &mut events);
            }
        }
        _ => unreachable!("validated class has an action"),
    }
    session.check_outcome(&mut events);
    events
}

fn target_matches(
    session: &GameSession,
    unit_id: &str,
    target_id: Option<&str>,
    team: Team,
    range: i32,
    extra: impl FnOnce(&crate::state::UnitState) -> bool,
) -> bool {
    let Some((unit, target)) = session
        .unit(unit_id)
        .zip(target_id.and_then(|target_id| session.unit(target_id)))
    else {
        return false;
    };
    target.team == team
        && !target.incapacitated
        && manhattan(unit.position, target.position) <= range
        && extra(target)
}

pub(crate) fn apply_status(
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

fn heal_target(session: &mut GameSession, target_id: &str, events: &mut Vec<BattleEvent>) {
    heal_target_amount(session, target_id, 4, events);
}

fn heal_target_amount(
    session: &mut GameSession,
    target_id: &str,
    amount: i32,
    events: &mut Vec<BattleEvent>,
) {
    let target = session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == target_id)
        .unwrap();
    let before = target.health;
    target.health = (target.health + amount).min(target.max_health);
    events.push(BattleEvent::UnitHealed {
        unit_id: target_id.to_owned(),
        amount: target.health - before,
        remaining: target.health,
    });
}

fn damage_target(
    session: &mut GameSession,
    target_id: &str,
    damage: i32,
    events: &mut Vec<BattleEvent>,
) {
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

    fn engineer_session() -> GameSession {
        let data = GameData::load().unwrap();
        let mut campaign = CampaignState::new(&data);
        campaign.toggle_deployment("kira_voss").unwrap();
        campaign.toggle_deployment("sol_cairn").unwrap();
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
        medic
            .activate_class_action_on("ilya_reed", "mara_venn")
            .unwrap();
        assert_eq!(medic.unit("mara_venn").unwrap().health, 7);

        let mut engineer = engineer_session();
        engineer
            .tactical
            .units
            .iter_mut()
            .find(|unit| unit.id == "sol_cairn")
            .unwrap()
            .position = TilePos::new(7, 2);
        let before_health = engineer.unit("brood_stalker_a").unwrap().health;
        engineer.tactical.selected_unit = Some("sol_cairn".into());
        engineer
            .activate_class_action_on("sol_cairn", "brood_stalker_a")
            .unwrap();
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
            if requires_target(class_id) {
                session
                    .activate_class_action_on("kira_voss", "brood_stalker_a")
                    .unwrap();
            } else {
                session.activate_selected_class_action().unwrap();
            }
            assert!(session
                .tactical
                .units
                .iter()
                .any(|unit| unit.has_status(expected_status)));
        }
    }

    #[test]
    fn targeted_class_action_changes_only_the_chosen_valid_unit() {
        let mut medic = session();
        for id in ["kira_voss", "mara_venn"] {
            medic
                .tactical
                .units
                .iter_mut()
                .find(|unit| unit.id == id)
                .unwrap()
                .health -= 4;
        }
        let kira_before = medic.unit("kira_voss").unwrap().health;
        let mara_before = medic.unit("mara_venn").unwrap().health;
        medic
            .activate_class_action_on("ilya_reed", "kira_voss")
            .unwrap();
        assert_eq!(medic.unit("kira_voss").unwrap().health, kira_before + 4);
        assert_eq!(medic.unit("mara_venn").unwrap().health, mara_before);
        assert!(!medic.can_target_class_action("ilya_reed", "mara_venn"));
    }

    #[test]
    fn advanced_actions_create_hybrid_tactical_roles() {
        let mut vanguard = session();
        vanguard
            .tactical
            .units
            .iter_mut()
            .find(|unit| unit.id == "mara_venn")
            .unwrap()
            .class_id = "vanguard".to_owned();
        vanguard
            .tactical
            .units
            .iter_mut()
            .find(|unit| unit.id == "kira_voss")
            .unwrap()
            .position = TilePos::new(2, 3);
        vanguard.tactical.selected_unit = Some("mara_venn".into());
        vanguard.activate_selected_class_action().unwrap();
        assert!(vanguard
            .unit("mara_venn")
            .unwrap()
            .has_status(StatusKind::Guarded));
        assert!(vanguard
            .unit("kira_voss")
            .unwrap()
            .has_status(StatusKind::Guarded));

        let mut lifewright = session();
        lifewright
            .tactical
            .units
            .iter_mut()
            .find(|unit| unit.id == "ilya_reed")
            .unwrap()
            .class_id = "lifewright".to_owned();
        lifewright
            .tactical
            .units
            .iter_mut()
            .find(|unit| unit.id == "mara_venn")
            .unwrap()
            .health -= 4;
        let before = lifewright.unit("mara_venn").unwrap().health;
        lifewright
            .activate_class_action_on("ilya_reed", "mara_venn")
            .unwrap();
        assert_eq!(lifewright.unit("mara_venn").unwrap().health, before + 3);
        assert!(lifewright
            .unit("mara_venn")
            .unwrap()
            .has_status(StatusKind::Regenerating));

        let mut null_adept = session();
        let kira = null_adept
            .tactical
            .units
            .iter_mut()
            .find(|unit| unit.id == "kira_voss")
            .unwrap();
        kira.class_id = "null_adept".to_owned();
        kira.position = TilePos::new(7, 2);
        let before = null_adept.unit("brood_stalker_a").unwrap().health;
        null_adept
            .activate_class_action_on("kira_voss", "brood_stalker_a")
            .unwrap();
        assert_eq!(
            null_adept.unit("brood_stalker_a").unwrap().health,
            before - 2
        );
        assert!(null_adept
            .unit("brood_stalker_a")
            .unwrap()
            .has_status(StatusKind::Disrupted));
    }
}
