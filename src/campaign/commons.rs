//! The Commons' once-per-operation social preparation action.

use super::{Availability, CampaignState, SQUAD_LIMIT};
use crate::colony::BuildingKind;

pub(crate) const COMMONS_MEAL_FOOD_COST: i32 = 4;

impl CampaignState {
    pub fn last_operation_had_commons_meal(&self) -> bool {
        self.operations_completed > 0
            && self.commons_meal_operation == Some(self.operations_completed - 1)
    }

    pub fn commons_meal_available(&self) -> bool {
        self.colony.has_facility(BuildingKind::Commons)
            && self.commons_meal_operation != Some(self.operations_completed)
            && self.colony.resources.food >= COMMONS_MEAL_FOOD_COST
            && self
                .roster
                .iter()
                .filter(|character| {
                    character.availability == Availability::Ready && character.deployment_selected
                })
                .take(SQUAD_LIMIT)
                .count()
                >= 2
    }

    pub fn host_commons_meal(&mut self) -> Result<String, String> {
        if !self.colony.has_facility(BuildingKind::Commons) {
            return Err("An operational Commons is required".to_owned());
        }
        if self.commons_meal_operation == Some(self.operations_completed) {
            return Err("The Commons has already hosted this operation's meal".to_owned());
        }
        let participants = self
            .roster
            .iter()
            .filter(|character| {
                character.availability == Availability::Ready && character.deployment_selected
            })
            .take(SQUAD_LIMIT)
            .map(|character| character.id.clone())
            .collect::<Vec<_>>();
        if participants.len() < 2 {
            return Err("At least two ready squad members must share the meal".to_owned());
        }
        if self.colony.resources.food < COMMONS_MEAL_FOOD_COST {
            return Err(format!(
                "The Commons meal requires {} food",
                COMMONS_MEAL_FOOD_COST
            ));
        }
        self.colony.resources.food -= COMMONS_MEAL_FOOD_COST;
        self.strengthen_event_participants(&participants);
        self.commons_meals_hosted = self.commons_meals_hosted.saturating_add(1);
        self.commons_meal_operation = Some(self.operations_completed);
        Ok(format!(
            "COMMONS MEAL // {} colonists shared the table",
            participants.len()
        ))
    }
}
