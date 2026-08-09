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

pub(crate) fn select_next(session: &mut GameSession) -> Option<String> {
    let start = session
        .tactical
        .selected_unit
        .as_ref()
        .and_then(|selected| {
            session
                .tactical
                .units
                .iter()
                .position(|unit| &unit.id == selected)
        })
        .unwrap_or_else(|| session.tactical.units.len().saturating_sub(1));
    let candidate = (1..=session.tactical.units.len())
        .map(|offset| (start + offset) % session.tactical.units.len())
        .find_map(|index| {
            let unit = &session.tactical.units[index];
            (unit.team == Team::Colony && !unit.incapacitated && unit.action_points > 0)
                .then(|| (unit.id.clone(), unit.position))
        });
    if let Some((unit_id, position)) = candidate {
        session.tactical.selected_unit = Some(unit_id.clone());
        session.tactical.selected_tile = position;
        Some(unit_id)
    } else {
        None
    }
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
        session.tactical.selected_unit = Some("mara_venn".to_owned());
        assert_eq!(select_next(&mut session).as_deref(), Some("kira_voss"));
        assert_eq!(
            session.tactical.selected_tile,
            session.unit("kira_voss").unwrap().position
        );
        session.unit("kira_voss").unwrap();
        session
            .tactical
            .units
            .iter_mut()
            .find(|unit| unit.id == "kira_voss")
            .unwrap()
            .action_points = 0;
        assert!(select_next(&mut session).is_none());
    }
}
