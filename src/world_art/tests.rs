use super::*;

#[test]
fn former_fauna_slot_selects_an_authored_flora_cutout() {
    assert!(open_tile_flora_cell(2).is_some());
}

#[test]
fn full_terrain_tile_slots_are_not_layered_over_open_tiles() {
    assert_eq!(open_tile_flora_cell(3), None);
    assert_eq!(open_tile_flora_cell(4), None);
}
