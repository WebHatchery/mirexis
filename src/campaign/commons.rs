//! The Commons' once-per-operation social preparation action.

use super::{Availability, CampaignState, SQUAD_LIMIT};
use crate::colony::{BuildingKind, COMMUNITY_KITCHEN_UPGRADE};

pub(crate) const COMMONS_MEAL_FOOD_COST: i32 = 4;

impl CampaignState {
    pub fn commons_meal_food_cost(&self) -> i32 {
        if self
            .colony
            .has_active_upgrade(BuildingKind::Hydroponics, COMMUNITY_KITCHEN_UPGRADE)
        {
            COMMONS_MEAL_FOOD_COST - 2
        } else {
            COMMONS_MEAL_FOOD_COST
        }
    }

    pub fn last_operation_had_commons_meal(&self) -> bool {
        self.operations_completed > 0
            && self.commons_meal_operation == Some(self.operations_completed - 1)
    }

    pub fn commons_meal_available(&self) -> bool {
        let food_cost = self.commons_meal_food_cost();
        self.colony.has_facility(BuildingKind::Commons)
            && self.commons_meal_operation != Some(self.operations_completed)
            && self.colony.resources.food >= food_cost
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
        let food_cost = self.commons_meal_food_cost();
        if self.colony.resources.food < food_cost {
            return Err(format!("The Commons meal requires {} food", food_cost));
        }
        self.colony.resources.food -= food_cost;
        self.strengthen_event_participants(&participants);
        self.commons_meals_hosted = self.commons_meals_hosted.saturating_add(1);
        self.commons_meal_operation = Some(self.operations_completed);
        Ok(format!(
            "COMMONS MEAL // {} colonists shared the table",
            participants.len()
        ))
    }
}
