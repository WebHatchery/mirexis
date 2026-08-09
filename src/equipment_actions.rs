//! Targeted tactical actions granted by carried field equipment.

use crate::data::Team;
use crate::state::{
    BattleEvent, Command, CommandCost, GameSession, RuleError, StatusKind, TacticalPhase,
};
use crate::tactical::manhattan;

pub(crate) fn action_name(equipment_id: &str) -> Option<&'static str> {
    match equipment_id {
        "field_medkit" => Some("FIELD PATCH"),
        "field_toolkit" => Some("FIELD FORTIFY"),
        "survey_harness" => Some("MARK HOSTILE"),
        _ => None,
    }
}

impl GameSession {
    pub fn can_use_equipment(&self, unit_id: &str, equipment_id: &str, target_id: &str) -> bool {
        self.validate(&Command::UseEquipment {
            unit_id: unit_id.to_owned(),
            equipment_id: equipment_id.to_owned(),
            target_id: target_id.to_owned(),
        })
        .is_ok()
    }

    pub fn use_equipment(
        &mut self,
        unit_id: &str,
        equipment_id: &str,
        target_id: &str,
    ) -> Result<Vec<BattleEvent>, RuleError> {
        self.execute(Command::UseEquipment {
            unit_id: unit_id.to_owned(),
            equipment_id: equipment_id.to_owned(),
            target_id: target_id.to_owned(),
        })
    }
}

pub(crate) fn available_action(session: &GameSession, unit_id: &str) -> Option<String> {
    let unit = session.unit(unit_id)?;
    unit.equipment_ids
        .iter()
        .find(|equipment_id| {
            action_name(equipment_id).is_some() && !unit.used_equipment_ids.contains(equipment_id)
        })
        .cloned()
}

pub(crate) fn has_valid_target(session: &GameSession, unit_id: &str, equipment_id: &str) -> bool {
    session
        .tactical
        .units
        .iter()
        .any(|target| validate(session, unit_id, equipment_id, &target.id).is_ok())
}

pub(crate) fn validate(
    session: &GameSession,
    unit_id: &str,
    equipment_id: &str,
    target_id: &str,
) -> Result<CommandCost, RuleError> {
    let unit = session.unit(unit_id).ok_or(RuleError::UnknownUnit)?;
    let target = session.unit(target_id).ok_or(RuleError::UnknownUnit)?;
    if session.tactical.phase != TacticalPhase::Player {
        return Err(RuleError::WrongPhase);
    }
    if unit.team != Team::Colony {
        return Err(RuleError::WrongTeam);
    }
    if unit.incapacitated || target.incapacitated {
        return Err(RuleError::Incapacitated);
    }
    if unit.action_points < 1 {
        return Err(RuleError::InsufficientActionPoints);
    }
    if !unit.equipment_ids.iter().any(|id| id == equipment_id)
        || unit.used_equipment_ids.iter().any(|id| id == equipment_id)
        || action_name(equipment_id).is_none()
    {
        return Err(RuleError::EquipmentUnavailable);
    }
    let distance = manhattan(unit.position, target.position);
    let valid_target = match equipment_id {
        "field_medkit" => {
            target.team == Team::Colony && target.health < target.max_health && distance <= 3
        }
        "field_toolkit" => {
            target.team == Team::Colony && !target.has_status(StatusKind::Guarded) && distance <= 3
        }
        "survey_harness" => {
            target.team == Team::Hostile
                && !target.has_status(StatusKind::Disrupted)
                && distance <= 6
        }
        _ => false,
    };
    valid_target
        .then_some(CommandCost { action_points: 1 })
        .ok_or(RuleError::InvalidTarget)
}

pub(crate) fn execute(
    session: &mut GameSession,
    unit_id: &str,
    equipment_id: &str,
    target_id: &str,
) -> Vec<BattleEvent> {
    let unit = session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == unit_id)
        .expect("validated equipment user exists");
    unit.action_points -= 1;
    unit.used_equipment_ids.push(equipment_id.to_owned());
    let mut events = vec![BattleEvent::EquipmentUsed {
        unit_id: unit_id.to_owned(),
        equipment_id: equipment_id.to_owned(),
        target_id: target_id.to_owned(),
    }];
    match equipment_id {
        "field_medkit" => {
            let target = session
                .tactical
                .units
                .iter_mut()
                .find(|unit| unit.id == target_id)
                .expect("validated equipment target exists");
            let before = target.health;
            target.health = (target.health + 5).min(target.max_health);
            events.push(BattleEvent::UnitHealed {
                unit_id: target_id.to_owned(),
                amount: target.health - before,
                remaining: target.health,
            });
        }
        "field_toolkit" => crate::class_actions::apply_status(
            session,
            target_id,
            StatusKind::Guarded,
            2,
            &mut events,
        ),
        "survey_harness" => crate::class_actions::apply_status(
            session,
            target_id,
            StatusKind::Disrupted,
            2,
            &mut events,
        ),
        _ => unreachable!("validated equipment has an action"),
    }
    session.check_outcome(&mut events);
    events
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::campaign::CampaignState;
    use crate::data::GameData;
    use macroquad_toolkit::grid::TilePos;

    fn session() -> GameSession {
        let data = GameData::load().unwrap();
        let campaign = CampaignState::new(&data);
        let roster = campaign.deployment_roster(&data, &data.mission);
        GameSession::new(&data.config, &data.mission, &roster)
    }

    #[test]
    fn carried_field_items_target_each_team_and_are_spent_once() {
        let mut session = session();
        let hostile = "brood_stalker_a";
        let kira = session.unit("kira_voss").unwrap().position;
        session
            .tactical
            .units
            .iter_mut()
            .find(|unit| unit.id == hostile)
            .unwrap()
            .position = TilePos::new(kira.x + 1, kira.y);
        assert!(validate(&session, "kira_voss", "survey_harness", hostile).is_ok());
        execute(&mut session, "kira_voss", "survey_harness", hostile);
        assert!(session
            .unit(hostile)
            .unwrap()
            .has_status(StatusKind::Disrupted));
        assert_eq!(
            validate(&session, "kira_voss", "survey_harness", hostile),
            Err(RuleError::EquipmentUnavailable)
        );

        let ally = session.unit("mara_venn").unwrap().position;
        session
            .tactical
            .units
            .iter_mut()
            .find(|unit| unit.id == "ilya_reed")
            .unwrap()
            .position = TilePos::new(ally.x + 1, ally.y);
        let mara = session
            .tactical
            .units
            .iter_mut()
            .find(|unit| unit.id == "mara_venn")
            .unwrap();
        mara.health -= 5;
        let before = mara.health;
        assert!(validate(&session, "ilya_reed", "field_medkit", "mara_venn").is_ok());
        execute(&mut session, "ilya_reed", "field_medkit", "mara_venn");
        assert_eq!(session.unit("mara_venn").unwrap().health, before + 5);
    }
}
