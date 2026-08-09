//! Read-only remaining-colonist readiness used by the end-phase guard.

use crate::data::Team;
use crate::state::GameSession;

pub(crate) fn ready_count(session: &GameSession) -> usize {
    session
        .tactical
        .units
        .iter()
        .filter(|unit| unit.team == Team::Colony && !unit.incapacitated && unit.action_points > 0)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::GameData;

    #[test]
    fn readiness_counts_only_active_colonists_with_unspent_actions() {
        let data = GameData::load().unwrap();
        let mut session = GameSession::new(&data.config, &data.mission, &data.roster);
        assert_eq!(ready_count(&session), 5);
        for unit in &mut session.tactical.units {
            if unit.team == Team::Colony {
                unit.action_points = 0;
            }
        }
        session
            .tactical
            .units
            .iter_mut()
            .find(|unit| unit.id == "kira_voss")
            .unwrap()
            .action_points = 1;
        assert_eq!(ready_count(&session), 1);
    }
}
