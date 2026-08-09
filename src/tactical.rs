//! Serializable tactical types shared by simulation, presentation, and saves.

use crate::data::{CoverEdgeDef, ObjectiveKind, Team, UnitDef};
use macroquad_toolkit::grid::{FlatGrid, FogState, TilePos};
use macroquad_toolkit::rng::SeededRng;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TacticalPhase {
    Player,
    Enemy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObjectiveState {
    Active,
    Secured,
    Victory,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StatusKind {
    Focused,
    Guarded,
    Quickened,
    Disrupted,
    Hindered,
    Regenerating,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatusEffect {
    pub kind: StatusKind,
    pub remaining_phases: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReinforcementWave {
    pub round: u32,
    pub units: Vec<UnitState>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DestructibleCover {
    pub position: TilePos,
    pub health: i32,
    pub max_health: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitState {
    pub id: String,
    pub name: String,
    pub role: String,
    #[serde(default)]
    pub class_id: String,
    #[serde(default)]
    pub equipment_ids: Vec<String>,
    pub mutation: String,
    pub team: Team,
    pub position: TilePos,
    pub health: i32,
    pub max_health: i32,
    pub move_range: u8,
    pub action_points: u8,
    pub armour: i32,
    pub accuracy: i32,
    pub weapon_range: u8,
    pub weapon_damage: i32,
    pub weapon_ap_cost: u8,
    pub round_regeneration: i32,
    pub incapacitated: bool,
    #[serde(default)]
    pub mutation_gift_used: bool,
    #[serde(default)]
    pub temporary_armour: i32,
    #[serde(default)]
    pub temporary_accuracy: i32,
    #[serde(default)]
    pub temporary_move_range: u8,
    #[serde(default)]
    pub temporary_weapon_damage: i32,
    #[serde(default)]
    pub class_action_used: bool,
    #[serde(default)]
    pub used_equipment_ids: Vec<String>,
    #[serde(default)]
    pub statuses: Vec<StatusEffect>,
    #[serde(default)]
    pub overwatching: bool,
}

impl UnitState {
    pub(crate) fn from_def(def: &UnitDef, action_points: u8) -> Self {
        Self {
            id: def.id.clone(),
            name: def.name.clone(),
            role: def.role.clone(),
            class_id: def.class_id.clone(),
            equipment_ids: def.equipment_ids.clone(),
            mutation: def.mutation.clone(),
            team: def.team,
            position: TilePos::new(def.position[0], def.position[1]),
            health: def.max_health,
            max_health: def.max_health,
            move_range: def.move_range,
            action_points,
            armour: def.armour,
            accuracy: def.accuracy,
            weapon_range: def.weapon_range,
            weapon_damage: def.weapon_damage,
            weapon_ap_cost: def.weapon_ap_cost,
            round_regeneration: def.round_regeneration,
            incapacitated: false,
            mutation_gift_used: false,
            temporary_armour: 0,
            temporary_accuracy: 0,
            temporary_move_range: 0,
            temporary_weapon_damage: 0,
            class_action_used: false,
            used_equipment_ids: Vec::new(),
            statuses: Vec::new(),
            overwatching: false,
        }
    }

    pub fn effective_armour(&self) -> i32 {
        self.armour
            + self.temporary_armour
            + if self.has_status(StatusKind::Guarded) {
                2
            } else {
                0
            }
    }

    pub fn effective_accuracy(&self) -> i32 {
        self.accuracy
            + self.temporary_accuracy
            + if self.has_status(StatusKind::Focused) {
                15
            } else {
                0
            }
            - if self.has_status(StatusKind::Disrupted) {
                20
            } else {
                0
            }
    }

    pub fn effective_move_range(&self) -> u8 {
        self.move_range
            .saturating_add(self.temporary_move_range)
            .saturating_add(if self.has_status(StatusKind::Quickened) {
                2
            } else {
                0
            })
            .saturating_sub(if self.has_status(StatusKind::Hindered) {
                2
            } else {
                0
            })
    }

    pub fn effective_weapon_damage(&self) -> i32 {
        self.weapon_damage + self.temporary_weapon_damage
    }

    pub fn has_status(&self, kind: StatusKind) -> bool {
        self.statuses.iter().any(|status| status.kind == kind)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Command {
    Move {
        unit_id: String,
        to: TilePos,
    },
    Attack {
        attacker_id: String,
        target_id: String,
    },
    Interact {
        unit_id: String,
    },
    ActivateMutation {
        unit_id: String,
    },
    ActivateClassAction {
        unit_id: String,
        target_id: Option<String>,
    },
    UseEquipment {
        unit_id: String,
        equipment_id: String,
        target_id: String,
    },
    AttackCover {
        attacker_id: String,
        position: TilePos,
    },
    AttackObjective {
        attacker_id: String,
    },
    SetOverwatch {
        unit_id: String,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommandCost {
    pub action_points: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuleError {
    WrongPhase,
    UnknownUnit,
    WrongTeam,
    Incapacitated,
    Occupied,
    NoPath,
    NoLineOfFire,
    OutOfRange,
    InsufficientActionPoints,
    InvalidTarget,
    ObjectiveUnavailable,
    MutationUnavailable,
    ClassActionUnavailable,
    EquipmentUnavailable,
    CoverUnavailable,
    OverwatchUnavailable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BattleEvent {
    UnitMoved {
        unit_id: String,
        path: Vec<TilePos>,
        cost: u8,
    },
    AttackRolled {
        attacker_id: String,
        target_id: String,
        roll: u8,
        hit_chance: u8,
    },
    DamageApplied {
        target_id: String,
        amount: i32,
        remaining: i32,
    },
    UnitIncapacitated {
        unit_id: String,
    },
    ObjectiveSecured {
        unit_id: String,
    },
    ObjectiveDamaged {
        amount: i32,
        remaining: i32,
    },
    ObjectiveDestroyed,
    ExtractionCompleted {
        unit_id: String,
    },
    MutationActivated {
        unit_id: String,
        gift: String,
    },
    ClassActionActivated {
        unit_id: String,
        action: String,
    },
    EquipmentUsed {
        unit_id: String,
        equipment_id: String,
        target_id: String,
    },
    CoverDamaged {
        position: TilePos,
        amount: i32,
        remaining: i32,
    },
    CoverDestroyed {
        position: TilePos,
    },
    OverwatchSet {
        unit_id: String,
    },
    ReactionTriggered {
        attacker_id: String,
        target_id: String,
    },
    StatusApplied {
        unit_id: String,
        status: StatusKind,
    },
    ReinforcementsArrived {
        round: u32,
        count: usize,
    },
    UnitHealed {
        unit_id: String,
        amount: i32,
        remaining: i32,
    },
    PhaseStarted {
        phase: TacticalPhase,
        round: u32,
    },
    BattleEnded {
        outcome: ObjectiveState,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TacticalState {
    pub fog: FlatGrid<FogState>,
    pub blocked: HashSet<TilePos>,
    pub terrain_costs: Vec<(TilePos, u8)>,
    pub cover_edges: Vec<CoverEdgeDef>,
    #[serde(default)]
    pub destructible_cover: Vec<DestructibleCover>,
    pub units: Vec<UnitState>,
    pub selected_unit: Option<String>,
    pub selected_tile: TilePos,
    pub objective_tile: TilePos,
    #[serde(default)]
    pub objective_kind: ObjectiveKind,
    pub objective_state: ObjectiveState,
    #[serde(default)]
    pub objective_integrity: i32,
    #[serde(default)]
    pub objective_max_integrity: i32,
    pub phase: TacticalPhase,
    pub round: u32,
    pub round_limit: u32,
    pub materials: i32,
    pub rng: SeededRng,
    pub event_log: Vec<BattleEvent>,
    #[serde(default)]
    pub reinforcement_waves: Vec<ReinforcementWave>,
}

pub(crate) fn terrain_cost(position: TilePos, costs: &[(TilePos, u8)]) -> u8 {
    costs
        .iter()
        .find(|(tile, _)| *tile == position)
        .map_or(1, |(_, cost)| *cost)
}

pub(crate) fn path_cost(path: &[TilePos], costs: &[(TilePos, u8)]) -> u8 {
    path.iter()
        .skip(1)
        .map(|tile| terrain_cost(*tile, costs))
        .sum()
}

pub(crate) fn manhattan(a: TilePos, b: TilePos) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

pub(crate) fn line_between(from: TilePos, to: TilePos) -> Vec<TilePos> {
    let mut positions = Vec::new();
    let (mut x, mut y) = (from.x, from.y);
    let dx = (to.x - from.x).abs();
    let sx = if from.x < to.x { 1 } else { -1 };
    let dy = -(to.y - from.y).abs();
    let sy = if from.y < to.y { 1 } else { -1 };
    let mut error = dx + dy;
    loop {
        if (x != from.x || y != from.y) && (x != to.x || y != to.y) {
            positions.push(TilePos::new(x, y));
        }
        if x == to.x && y == to.y {
            break;
        }
        let doubled = error * 2;
        if doubled >= dy {
            error += dy;
            x += sx;
        }
        if doubled <= dx {
            error += dx;
            y += sy;
        }
    }
    positions
}
