use super::CampaignState;
use crate::colony::BuildingKind;
use crate::colony_story::{IdentityArcProgress, IdentityBuildingState};

impl CampaignState {
    pub(crate) fn identity_building_story_state(&self) -> Option<IdentityBuildingState> {
        let kind = BuildingKind::identity_for_path(&self.strategy.mirexis_path_id)?;
        let (building_id, damaged) = self
            .colony
            .buildings
            .iter()
            .find(|building| building.kind == kind)
            .map(|building| (building.id.clone(), building.damaged))?;
        Some(IdentityBuildingState {
            damaged,
            powered: self.colony.building_is_powered(&building_id),
        })
    }

    pub fn acknowledge_colonist(&mut self, character_id: &str) {
        self.first_hour.acknowledge_colonist(character_id);
        let beat = crate::colony_story::identity_arc_beat(
            &self.strategy.mirexis_path_id,
            character_id,
            self.strategy.campaign_complete,
            self.strategy.post_campaign_operations_completed,
            IdentityArcProgress {
                preparations_completed: self.identity_preparations_completed,
                stewardship_completed: self.identity_stewardship_completed,
            },
            &self.colony_story,
            self.identity_building_story_state(),
        )
        .or_else(|| {
            crate::colony_story::finale_beat(
                &self.strategy.mirexis_path_id,
                character_id,
                self.strategy.campaign_complete,
                &self.colony_story,
            )
        })
        .or_else(|| {
            crate::colony_story::contact_route_beat(
                &self.strategy.contact_protocol_id,
                character_id,
                self.strategy.contact_trace_completed,
                self.strategy.contact_complete,
            )
        })
        .or_else(|| crate::colony_story::phase_beat(&self.strategy.phase_id, character_id))
        .or_else(|| {
            self.last_operation_had_commons_meal()
                .then(|| crate::colony_story::commons_meal_beat(character_id))
                .flatten()
        })
        .or_else(|| {
            crate::colony_story::current_beat(
                character_id,
                self.operations_completed,
                self.first_hour.first_outcome_won,
                self.first_hour.second_outcome_won,
            )
        });
        let Some(beat) = beat else {
            return;
        };
        self.colony_story.acknowledge(beat.id);
    }
}
