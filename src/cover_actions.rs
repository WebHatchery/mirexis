//! Destructible battlefield-cover commands.

use crate::state::{BattleEvent, Command, CommandCost, GameSession, RuleError};
use crate::tactical::manhattan;
use macroquad_toolkit::grid::TilePos;

impl GameSession {
    pub fn can_attack_selected_cover(&self, position: TilePos) -> bool {
        self.tactical
            .selected_unit
            .as_ref()
            .is_some_and(|attacker_id| {
                self.validate(&Command::AttackCover {
                    attacker_id: attacker_id.clone(),
                    position,
                })
                .is_ok()
            })
    }

    pub fn attack_selected_cover(
        &mut self,
        position: TilePos,
    ) -> Result<Vec<BattleEvent>, RuleError> {
        let attacker_id = self
            .tactical
            .selected_unit
            .clone()
            .ok_or(RuleError::UnknownUnit)?;
        self.execute(Command::AttackCover {
            attacker_id,
            position,
        })
    }
}

pub(crate) fn validate(
    session: &GameSession,
    attacker_id: &str,
    position: TilePos,
) -> Result<CommandCost, RuleError> {
    let attacker = session.active_unit_for_phase(attacker_id)?;
    if !session
        .tactical
        .destructible_cover
        .iter()
        .any(|cover| cover.position == position && cover.health > 0)
    {
        return Err(RuleError::CoverUnavailable);
    }
    if manhattan(attacker.position, position) > i32::from(attacker.weapon_range) {
        return Err(RuleError::OutOfRange);
    }
    if !session.has_line_of_fire(attacker.position, position) {
        return Err(RuleError::NoLineOfFire);
    }
    if attacker.action_points < attacker.weapon_ap_cost {
        return Err(RuleError::InsufficientActionPoints);
    }
    Ok(CommandCost {
        action_points: attacker.weapon_ap_cost,
    })
}

pub(crate) fn execute(
    session: &mut GameSession,
    attacker_id: &str,
    position: TilePos,
) -> Vec<BattleEvent> {
    let attacker = session
        .unit(attacker_id)
        .expect("validated cover attacker exists")
        .clone();
    session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == attacker_id)
        .expect("validated cover attacker exists")
        .action_points -= attacker.weapon_ap_cost;
    let damage = attacker.effective_weapon_damage().max(1);
    let cover = session
        .tactical
        .destructible_cover
        .iter_mut()
        .find(|cover| cover.position == position)
        .expect("validated cover exists");
    cover.health = (cover.health - damage).max(0);
    let remaining = cover.health;
    let mut events = vec![BattleEvent::CoverDamaged {
        position,
        amount: damage,
        remaining,
    }];
    if remaining == 0 {
        session
            .tactical
            .destructible_cover
            .retain(|cover| cover.position != position);
        session.tactical.blocked.remove(&position);
        session
            .tactical
            .cover_edges
            .retain(|edge| edge.position != [position.x, position.y]);
        events.push(BattleEvent::CoverDestroyed { position });
    }
    events
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::campaign::CampaignState;
    use crate::data::GameData;

    #[test]
    fn destroying_cover_opens_the_blocked_tile() {
        let data = GameData::load().unwrap();
        let campaign = CampaignState::new(&data);
        let roster = campaign.deployment_roster(&data, &data.mission);
        let mut session = GameSession::new(&data.config, &data.mission, &roster);
        let attacker_position = session.selected_unit().unwrap().position;
        let position = session
            .tactical
            .destructible_cover
            .iter()
            .find(|cover| {
                cover.position.y == attacker_position.y
                    && session.can_attack_selected_cover(cover.position)
            })
            .expect("authored map has reachable cover")
            .position;
        let tile_beyond = TilePos::new(position.x + 1, position.y);
        assert!(!session.has_line_of_fire(attacker_position, tile_beyond));
        while session.tactical.blocked.contains(&position) {
            session
                .tactical
                .units
                .iter_mut()
                .find(|unit| Some(&unit.id) == session.tactical.selected_unit.as_ref())
                .unwrap()
                .action_points = data.config.max_action_points;
            session.attack_selected_cover(position).unwrap();
        }
        session
            .tactical
            .units
            .iter_mut()
            .find(|unit| Some(&unit.id) == session.tactical.selected_unit.as_ref())
            .unwrap()
            .action_points = 20;
        assert!(!session.tactical.blocked.contains(&position));
        assert!(session.has_line_of_fire(attacker_position, tile_beyond));
        let move_result = session.validate(&Command::Move {
            unit_id: session.tactical.selected_unit.clone().unwrap(),
            to: position,
        });
        assert!(
            move_result.is_ok(),
            "opened tile was not pathable: {move_result:?}"
        );
        assert!(!session
            .tactical
            .destructible_cover
            .iter()
            .any(|cover| cover.position == position));
        assert!(matches!(
            session.tactical.event_log.last(),
            Some(BattleEvent::CoverDestroyed { position: destroyed }) if *destroyed == position
        ));
    }
}
