use super::*;

#[test]
fn empty_tactical_tiles_only_select_terrain_or_flora_dressing() {
    for signature in 0..23 {
        let Some((atlas, _)) = open_tile_dressing(signature) else {
            continue;
        };
        assert!(
            matches!(atlas, "flora" | "terrain_dressing"),
            "empty tile selected non-terrain atlas {atlas}"
        );
    }
}

#[test]
fn former_fauna_slot_selects_authored_terrain_dressing() {
    assert!(matches!(
        open_tile_dressing(2),
        Some(("terrain_dressing", _))
    ));
}
