use super::*;

#[test]
fn construction_reserves_resources_and_completes_after_an_operation() {
    let mut colony = ColonyState::new();
    colony
        .place_construction(BuildingKind::Barricade, [1, 1])
        .unwrap();
    assert_eq!(colony.resources.materials, 100);
    assert!(colony
        .place_construction(BuildingKind::Barricade, [1, 1])
        .is_err());
    colony.advance_operation();
    assert!(colony.buildings.iter().any(|building| {
        building.kind == BuildingKind::Barricade && building.position == [1, 1]
    }));
}

#[test]
fn physical_placement_generates_the_colony_defense_map() {
    let mut colony = ColonyState::new();
    colony
        .place_construction(BuildingKind::Barricade, [1, 1])
        .unwrap();
    colony.advance_operation();
    let map = colony.defense_map();
    assert!(map.cover_tiles.contains(&TilePos::new(3, 2)));
    assert!(map.critical_objectives.contains(&TilePos::new(12, 11)));
}

#[test]
fn failed_defense_damage_disables_a_facility_until_repaired() {
    let mut colony = ColonyState::new();
    let name = colony.damage_for_failed_defense(1).unwrap();
    let damaged = colony
        .buildings
        .iter()
        .find(|building| building.kind.name() == name)
        .unwrap();
    let kind = damaged.kind;
    let id = damaged.id.clone();
    assert!(!colony.has_facility(kind));
    let materials = colony.resources.materials;
    let (_, cost) = colony.repair_building(&id).unwrap();
    assert_eq!(colony.resources.materials, materials - cost);
    assert!(colony.has_facility(kind));
}

#[test]
fn hydroponics_production_and_power_load_are_derived_from_buildings() {
    let mut colony = ColonyState::new();
    assert_eq!(colony.power_supply(), 8);
    assert_eq!(colony.power_demand(), 7);
    let food = colony.resources.food;
    colony.advance_operation();
    assert_eq!(colony.resources.food, food + 3);

    colony
        .buildings
        .iter_mut()
        .find(|building| building.kind == BuildingKind::PowerPlant)
        .unwrap()
        .damaged = true;
    assert_eq!(colony.power_supply(), 4);
    assert_eq!(colony.power_demand(), 7);
    assert!(!colony.has_facility(BuildingKind::Workshop));
    assert!(!colony.has_facility(BuildingKind::Hydroponics));
    colony.advance_operation();
    assert_eq!(colony.resources.food, food + 4);
}

#[test]
fn power_plant_construction_adds_redundant_grid_capacity() {
    let mut colony = ColonyState::new();
    colony
        .place_construction(BuildingKind::PowerPlant, [1, 1])
        .unwrap();
    colony.advance_operation();
    assert_eq!(colony.power_supply(), 12);
    assert!(colony
        .buildings
        .iter()
        .any(|building| building.kind == BuildingKind::PowerPlant && building.position == [1, 1]));
}

#[test]
fn colony_build_area_extends_far_beyond_the_initial_settlement() {
    let mut colony = ColonyState::new();
    colony
        .place_construction(
            BuildingKind::Barricade,
            [COLONY_WIDTH - 1, COLONY_HEIGHT - 1],
        )
        .expect("the far colony frontier remains buildable");
}

#[test]
fn initial_settlement_is_centered_spacious_and_surrounded_by_frontier() {
    let colony = ColonyState::new();
    let positions = colony
        .buildings
        .iter()
        .map(|building| building.position)
        .collect::<Vec<_>>();
    assert!(positions.contains(&SETTLEMENT_CENTER));

    for (index, position) in positions.iter().enumerate() {
        assert!(position[0] >= 5 && position[0] <= COLONY_WIDTH - 5);
        assert!(position[1] >= 5 && position[1] <= COLONY_HEIGHT - 5);
        for other in positions.iter().skip(index + 1) {
            let distance = (position[0] - other[0]).abs() + (position[1] - other[1]).abs();
            assert!(
                distance >= 4,
                "buildings at {position:?} and {other:?} crowd each other"
            );
        }
    }

    for frontier in [
        [0, 0],
        [COLONY_WIDTH - 1, 0],
        [0, COLONY_HEIGHT - 1],
        [COLONY_WIDTH - 1, COLONY_HEIGHT - 1],
    ] {
        assert!(
            !colony.is_occupied(frontier),
            "frontier {frontier:?} must remain open"
        );
    }
}

#[test]
fn gene_lab_is_unique_and_requires_additional_power() {
    let mut colony = ColonyState::new();
    colony
        .place_construction(BuildingKind::GeneLab, [1, 1])
        .unwrap();
    assert!(colony
        .place_construction(BuildingKind::GeneLab, [1, 2])
        .is_err());
    colony.advance_operation();
    assert!(!colony.has_facility(BuildingKind::GeneLab));
    colony.resources.power += 2;
    assert!(colony.has_facility(BuildingKind::GeneLab));
}
