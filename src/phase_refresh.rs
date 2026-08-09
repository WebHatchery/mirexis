//! Per-team phase refresh for action economy and temporary combat state.

use crate::data::Team;
use crate::state::GameSession;
use crate::tactical::StatusKind;

impl GameSession {
    pub(crate) fn refresh_team(&mut self, team: Team, action_points: u8) {
        for unit in &mut self.tactical.units {
            if unit.team == team && !unit.incapacitated {
                unit.overwatching = false;
                unit.action_points = action_points;
                unit.mutation_gift_used = false;
                unit.class_action_used = false;
                unit.enemy_ability_used = false;
                unit.temporary_armour = 0;
                unit.temporary_accuracy = 0;
                unit.temporary_move_range = 0;
                unit.temporary_weapon_damage = 0;
                if unit.round_regeneration > 0 {
                    unit.health = (unit.health + unit.round_regeneration).min(unit.max_health);
                }
                if unit.has_status(StatusKind::Regenerating) {
                    unit.health = (unit.health + 1).min(unit.max_health);
                }
            }
        }
    }
}
