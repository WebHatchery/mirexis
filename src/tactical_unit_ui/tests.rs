use super::*;

#[test]
fn unit_hover_bounds_include_the_visible_sprite_and_vitality_bar() {
    let tile = Rect::new(100.0, 100.0, 48.0, 24.0);
    let hit = unit_hit_bounds(tile);

    assert!(hit.contains(vec2(124.0, 54.0)), "sprite top must inspect");
    assert!(hit.contains(vec2(124.0, 87.0)), "vitality bar must inspect");
    assert!(hit.contains(tile.center()), "ground tile must inspect");
    assert!(!hit.contains(vec2(80.0, 80.0)));
}
