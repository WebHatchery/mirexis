use super::StrategyState;
use crate::colony::ColonyState;
use crate::data::{CoverEdgeDef, EdgeDirection, GameData, MissionDef};

impl StrategyState {
    pub fn materialize_selected(&self, data: &GameData, colony: &ColonyState) -> MissionDef {
        let base = &data.mission;
        let Some(instance) = self.selected_mission() else {
            return base.clone();
        };
        let defense = colony.defense_map();
        let colony_defense = instance.map_recipe == "colony_defense";
        let recipe = data
            .campaign
            .map_recipes
            .iter()
            .find(|recipe| recipe.id == instance.map_recipe);
        let layout =
            recipe.map(|recipe| crate::map_variants::materialize(recipe, data, instance.seed));
        let strategic_bonus = crate::strategy_rewards::bonus(
            &self.contact_protocol_id,
            &self.escalation_response_id,
            &self.mirexis_path_id,
            data,
        );
        let mirexis_cover_bonus = data
            .campaign
            .mirexis_paths
            .iter()
            .find(|path| path.id == self.mirexis_path_id)
            .map_or(0, |path| path.defense_cover_bonus);
        MissionDef {
            id: instance.id.clone(),
            name: instance.name.clone(),
            briefing: instance.briefing.clone(),
            objective: instance.objective.clone(),
            objective_kind: instance.objective_kind,
            hostile_faction: instance.faction_id.clone(),
            hostile_unit_ids: instance.hostile_unit_ids.clone(),
            round_limit: instance.round_limit,
            materials_reward: instance.materials_reward
                + if self.research_completed("salvage_doctrine") {
                    8
                } else {
                    0
                }
                + strategic_bonus.0,
            biomass_reward: instance.biomass_reward + strategic_bonus.1,
            power_reward: instance.power_reward + strategic_bonus.2,
            operation_modifier: instance.operation_modifier,
            cover_integrity: if colony_defense {
                (if self.research_completed("field_fortifications") {
                    10
                } else {
                    base.cover_integrity
                }) + mirexis_cover_bonus
            } else {
                base.cover_integrity
            },
            seed: instance.seed,
            blocked_tiles: if colony_defense {
                defense
                    .blocked_tiles
                    .iter()
                    .map(|tile| [tile.x, tile.y])
                    .collect()
            } else if let Some(layout) = &layout {
                layout.blocked_tiles.clone()
            } else {
                base.blocked_tiles.clone()
            },
            objective_tile: if colony_defense {
                defense
                    .critical_objectives
                    .first()
                    .map_or(base.objective_tile, |tile| [tile.x, tile.y])
            } else if let Some(layout) = &layout {
                layout.objective_tile
            } else {
                base.objective_tile
            },
            terrain_costs: layout.as_ref().map_or_else(
                || base.terrain_costs.clone(),
                |layout| layout.terrain_costs.clone(),
            ),
            hazards: if colony_defense {
                Vec::new()
            } else {
                layout
                    .as_ref()
                    .map_or_else(|| base.hazards.clone(), |layout| layout.hazards.clone())
            },
            cover_edges: if colony_defense {
                defense
                    .cover_tiles
                    .iter()
                    .map(|tile| CoverEdgeDef {
                        position: [tile.x, tile.y],
                        direction: EdgeDirection::West,
                        strength: if defense.watchtower_tiles.contains(tile) {
                            45
                        } else {
                            25
                        },
                    })
                    .collect()
            } else if let Some(layout) = &layout {
                layout.cover_edges.clone()
            } else {
                base.cover_edges.clone()
            },
        }
    }
}
