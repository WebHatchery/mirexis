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
        assert!(variant.objective_tile[1] <= AUTHORED_MAX_Y);
        assert!(variant
            .blocked_tiles
            .iter()
            .all(|position| position[1] <= AUTHORED_MAX_Y));
    }
}

#[test]
fn larger_runtime_world_does_not_stretch_authored_recipe_variants() {
    let data = GameData::load().unwrap();
    assert!(data.config.world_height > (AUTHORED_MAX_Y + 1) as usize);
    for recipe in &data.campaign.map_recipes {
        let variant = materialize(recipe, &data, 3);
        let authored = materialize(recipe, &data, 2);
        assert_eq!(
            variant.objective_tile[1],
            AUTHORED_MAX_Y - authored.objective_tile[1]
        );
    }
}
