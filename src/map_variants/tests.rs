use super::*;

#[test]
fn odd_seed_mirrors_safely_and_repeats_exactly() {
    let data = GameData::load().unwrap();
    for recipe in &data.campaign.map_recipes {
        let authored = materialize(recipe, &data, 2);
        let variant = materialize(recipe, &data, 3);
        assert_ne!(variant, authored, "{} did not vary", recipe.id);
        assert_eq!(variant, materialize(recipe, &data, 3));
        assert!(!variant.blocked_tiles.contains(&variant.objective_tile));
        assert!(data
            .roster
            .iter()
            .all(|unit| !variant.blocked_tiles.contains(&unit.position)));
        assert!(variant.objective_tile[1] >= BATTLEFIELD_Y_OFFSET);
        assert!(variant
            .blocked_tiles
            .iter()
            .all(|position| position[1] >= BATTLEFIELD_Y_OFFSET));
    }
}

#[test]
fn larger_runtime_world_projects_recipes_across_the_battlefield() {
    let data = GameData::load().unwrap();
    assert!(data.config.world_height > (AUTHORED_MAX_Y + 1) as usize);
    for recipe in &data.campaign.map_recipes {
        let variant = materialize(recipe, &data, 3);
        let authored = materialize(recipe, &data, 2);
        assert_eq!(
            variant.objective_tile[1],
            BATTLEFIELD_Y_OFFSET + AUTHORED_MAX_Y * BATTLEFIELD_Y_STRIDE
                - (authored.objective_tile[1] - BATTLEFIELD_Y_OFFSET)
        );
        assert!(authored.objective_tile[0] >= 20);
        assert!(authored
            .blocked_tiles
            .iter()
            .chain(authored.terrain_costs.iter().map(|entry| &entry.position))
            .any(|position| position[1] >= 25));
    }
}

#[test]
fn every_variant_is_bounded_and_reachable_from_the_allied_approach() {
    use std::collections::{HashSet, VecDeque};

    let data = GameData::load().unwrap();
    for recipe in &data.campaign.map_recipes {
        for seed in [2, 3] {
            let layout = materialize(recipe, &data, seed);
            let positions = layout
                .blocked_tiles
                .iter()
                .copied()
                .chain(std::iter::once(layout.objective_tile))
                .chain(layout.terrain_costs.iter().map(|entry| entry.position))
                .chain(layout.hazards.iter().map(|hazard| hazard.position))
                .chain(layout.cover_edges.iter().map(|cover| cover.position));
            assert!(positions.into_iter().all(|position| {
                position[0] >= 0
                    && position[1] >= 0
                    && position[0] < data.config.world_width as i32
                    && position[1] < data.config.world_height as i32
            }));

            let blocked = layout.blocked_tiles.iter().copied().collect::<HashSet<_>>();
            let mut frontier = VecDeque::from([[4, 19]]);
            let mut visited = HashSet::from([[4, 19]]);
            while let Some(position) = frontier.pop_front() {
                for next in [
                    [position[0] - 1, position[1]],
                    [position[0] + 1, position[1]],
                    [position[0], position[1] - 1],
                    [position[0], position[1] + 1],
                ] {
                    if next[0] >= 0
                        && next[1] >= 0
                        && next[0] < data.config.world_width as i32
                        && next[1] < data.config.world_height as i32
                        && !blocked.contains(&next)
                        && visited.insert(next)
                    {
                        frontier.push_back(next);
                    }
                }
            }
            assert!(
                visited.contains(&layout.objective_tile),
                "{} seed {seed} blocks its objective",
                recipe.id
            );
        }
    }
}
