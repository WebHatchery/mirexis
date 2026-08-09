//! Deterministic tactical mechanic showcase scenes.

use super::Game;
use crate::data::{OperationModifier, Team};
use crate::state::{Command, TacticalPhase};
use macroquad_toolkit::grid::TilePos;

impl Game {
    pub(super) fn capture_enemy_ability(&mut self, template_id: &str, faction: &str, seed: u64) {
        self.capture_template_operation(template_id, seed, OperationModifier::None);
        let colonist = self
            .session
            .tactical
            .units
            .iter_mut()
            .find(|unit| unit.id == "kira_voss")
            .expect("ability capture includes Kira");
        colonist.position = TilePos::new(5, 3);
        let target_id = (faction == "directorate").then(|| colonist.id.clone());
        let enemy = self
            .session
            .tactical
            .units
            .iter_mut()
            .find(|unit| unit.faction.as_deref() == Some(faction))
            .expect("ability capture includes matching hostile");
        enemy.position = TilePos::new(5, 4);
        let enemy_id = enemy.id.clone();
        self.session.tactical.phase = TacticalPhase::Enemy;
        self.session
            .execute(Command::ActivateEnemyAbility {
                unit_id: enemy_id,
                target_id,
            })
            .expect("capture hostile can activate its faction ability");
        self.session.tactical.phase = TacticalPhase::Player;
        self.session.tactical.selected_tile = TilePos::new(5, 3);
    }

    pub(super) fn capture_hazard(&mut self) {
        self.capture_template_operation("sporefield_extraction", 4, OperationModifier::BroodFrenzy);
        let unit_id = self.session.tactical.selected_unit.clone().unwrap();
        self.session
            .tactical
            .units
            .iter_mut()
            .find(|unit| unit.id == unit_id)
            .unwrap()
            .position = TilePos::new(4, 3);
        let _ = self.session.execute(Command::Move {
            unit_id,
            to: TilePos::new(5, 3),
        });
    }

    pub(super) fn capture_enemy_intent(&mut self) {
        self.capture_template_operation("nest_suppression", 7, OperationModifier::BroodFrenzy);
        let hostile = self
            .session
            .tactical
            .units
            .iter()
            .find(|unit| unit.team == Team::Hostile)
            .unwrap();
        self.session.tactical.selected_tile = hostile.position;
    }
}
