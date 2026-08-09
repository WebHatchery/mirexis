//! Tactical stat changes imposed by strategic operation pressure.

use crate::data::{OperationModifier, Team};
use crate::tactical::UnitState;

pub(crate) fn apply(modifier: OperationModifier, unit: &mut UnitState) {
    match modifier {
        OperationModifier::DirectorateFireControl if unit.team == Team::Hostile => {
            unit.accuracy += 10;
        }
        OperationModifier::BroodFrenzy if unit.team == Team::Hostile => {
            unit.move_range = unit.move_range.saturating_add(1);
        }
        OperationModifier::AscendantInterference if unit.team == Team::Colony => {
            unit.accuracy -= 10;
        }
        OperationModifier::EscalationCrossfire if unit.team == Team::Hostile => {
            unit.accuracy += 5;
            unit.move_range = unit.move_range.saturating_add(1);
        }
        OperationModifier::EscalationCrossfire if unit.team == Team::Colony => {
            unit.accuracy -= 5;
        }
        _ => {}
    }
}
