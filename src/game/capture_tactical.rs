//! Deterministic tactical mechanic showcase scenes.

use super::{AppState, Game};
use crate::data::{CoverEdgeDef, EdgeDirection, OperationModifier, Team};
use crate::state::{BattleEvent, Command, GameSession, StatusKind, TacticalPhase};
use macroquad_toolkit::grid::TilePos;

impl Game {
    pub(super) fn capture_valid_shot(&mut self) {
        self.reset_capture_session(AppState::Tactical);
        self.session
            .tactical
            .units
            .iter_mut()
            .find(|unit| unit.id == "kira_voss")
            .unwrap()
            .position = TilePos::new(4, 3);
        let hostile = self
            .session
            .tactical
            .units
            .iter_mut()
            .find(|unit| unit.team == Team::Hostile)
            .unwrap();
        hostile.position = TilePos::new(6, 3);
        let hostile_tile = hostile.position;
        self.session.tactical.blocked.remove(&TilePos::new(5, 3));
        self.session
            .tactical
            .destructible_cover
            .retain(|cover| cover.position != TilePos::new(5, 3));
        self.session.tactical.cover_edges.push(CoverEdgeDef {
            position: [6, 3],
            direction: EdgeDirection::West,
            strength: 25,
        });
        self.session.tactical.selected_unit = Some("kira_voss".to_owned());
        self.session.tactical.selected_tile = hostile_tile;
    }

    pub(super) fn capture_invalid_command(&mut self) {
        self.reset_capture_session(AppState::Tactical);
        self.session
            .tactical
            .units
            .iter_mut()
            .find(|unit| unit.id == "kira_voss")
            .unwrap()
            .position = TilePos::new(4, 3);
        let hostile = self
            .session
            .tactical
            .units
            .iter_mut()
            .find(|unit| unit.team == Team::Hostile)
            .unwrap();
        hostile.position = TilePos::new(6, 3);
        let hostile_tile = hostile.position;
        self.session.tactical.blocked.insert(TilePos::new(5, 3));
        self.session.tactical.selected_unit = Some("kira_voss".to_owned());
        self.session.tactical.selected_tile = hostile_tile;
    }

    pub(super) fn capture_cover_edges(&mut self) {
        self.reset_capture_session(AppState::Tactical);
        self.session.tactical.cover_edges = vec![
            CoverEdgeDef {
                position: [3, 2],
                direction: EdgeDirection::North,
                strength: 20,
            },
            CoverEdgeDef {
                position: [4, 3],
                direction: EdgeDirection::East,
                strength: 25,
            },
            CoverEdgeDef {
                position: [3, 4],
                direction: EdgeDirection::South,
                strength: 30,
            },
            CoverEdgeDef {
                position: [2, 3],
                direction: EdgeDirection::West,
                strength: 35,
            },
        ];
        self.session.tactical.selected_tile = self.session.selected_unit().unwrap().position;
    }

    pub(super) fn capture_movement_route(&mut self) {
        self.reset_capture_session(AppState::Tactical);
        self.session.tactical.blocked.insert(TilePos::new(2, 2));
        self.session
            .tactical
            .terrain_costs
            .push((TilePos::new(2, 3), 2));
        self.session.tactical.selected_unit = Some("kira_voss".to_owned());
        self.session.tactical.selected_tile = TilePos::new(4, 3);
    }

    pub(super) fn capture_vitality_markers(&mut self) {
        self.reset_capture_session(AppState::Tactical);
        for unit in &mut self.session.tactical.units {
            match unit.id.as_str() {
                "kira_voss" => unit.health = (unit.max_health / 4).max(1),
                "mara_venn" => {
                    unit.health = (unit.max_health * 2 / 3).max(1);
                    unit.temporary_armour = 3;
                }
                "brood_stalker_a" => unit.health = (unit.max_health / 2).max(1),
                _ => {}
            }
        }
        self.session.tactical.selected_unit = Some("mara_venn".to_owned());
        self.session.tactical.selected_tile = self.session.unit("mara_venn").unwrap().position;
    }

    pub(super) fn capture_readiness_markers(&mut self) {
        self.reset_capture_session(AppState::Tactical);
        for unit in self
            .session
            .tactical
            .units
            .iter_mut()
            .filter(|unit| unit.team == Team::Colony)
        {
            unit.action_points = match unit.id.as_str() {
                "kira_voss" => 0,
                "mara_venn" => 2,
                _ => self.data.config.max_action_points,
            };
        }
        self.session.tactical.selected_unit = Some("mara_venn".to_owned());
        self.session.tactical.selected_tile = self.session.unit("mara_venn").unwrap().position;
    }

    pub(super) fn capture_line_formation(&mut self) {
        let mut roster = self
            .campaign
            .deployment_roster(&self.data, &self.active_mission);
        crate::formation::apply(
            &mut roster,
            &self.active_mission,
            &self.data.config,
            crate::formation::FormationKind::Line,
        );
        self.session = GameSession::new(&self.data.config, &self.active_mission, &roster);
        self.state = AppState::Tactical;
    }

    pub(super) fn capture_phase_replay(&mut self) {
        self.reset_capture_session(AppState::Tactical);
        self.phase_replay.hold_for_capture(&[
            BattleEvent::EnemyAbilityActivated {
                unit_id: "brood_stalker_a".to_owned(),
                ability: "Predatory Surge".to_owned(),
            },
            BattleEvent::UnitMoved {
                unit_id: "brood_stalker_a".to_owned(),
                path: vec![TilePos::new(9, 2), TilePos::new(8, 2)],
                cost: 2,
            },
            BattleEvent::AttackRolled {
                attacker_id: "brood_stalker_b".to_owned(),
                target_id: "kira_voss".to_owned(),
                roll: 41,
                hit_chance: 64,
            },
            BattleEvent::PhaseStarted {
                phase: TacticalPhase::Player,
                round: 2,
            },
        ]);
    }

    pub(super) fn capture_combat_feedback(&mut self) {
        self.reset_capture_session(AppState::Tactical);
        self.combat_feedback.record_for(
            &[
                BattleEvent::DamageApplied {
                    target_id: "kira_voss".to_owned(),
                    amount: 3,
                    remaining: 6,
                },
                BattleEvent::UnitHealed {
                    unit_id: "mara_venn".to_owned(),
                    amount: 2,
                    remaining: 12,
                },
                BattleEvent::StatusApplied {
                    unit_id: "ilya_reed".to_owned(),
                    status: StatusKind::Guarded,
                },
            ],
            100.0,
        );
    }

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

    pub(super) fn capture_player_action_preview(&mut self) {
        self.capture_template_operation("nest_suppression", 7, OperationModifier::BroodFrenzy);
        self.session
            .tactical
            .units
            .iter_mut()
            .find(|unit| unit.id == "kira_voss")
            .unwrap()
            .position = TilePos::new(4, 3);
        let hostile = self
            .session
            .tactical
            .units
            .iter_mut()
            .find(|unit| unit.team == Team::Hostile)
            .unwrap();
        hostile.position = TilePos::new(5, 3);
        self.session.tactical.selected_tile = hostile.position;
    }
}
