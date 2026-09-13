//! Colony bounds, legacy layouts, and unique-facility regressions.

use super::*;

#[test]
fn construction_clearance_must_fit_inside_the_colony_boundary() {
    let mut colony = ColonyState::new();
    for edge in [
        [1, 0],
        [0, 1],
        [COLONY_WIDTH - 1, 1],
        [1, COLONY_HEIGHT - 1],
    ] {
        assert!(colony
            .place_construction(BuildingKind::Barricade, edge)
            .is_err());
    }
    assert!(colony
        .place_construction(BuildingKind::PowerPlant, [1, 1])
        .is_ok());
}

#[test]
fn colony_build_area_extends_far_beyond_the_initial_settlement() {
    let mut colony = ColonyState::new();
    colony
        .place_construction(
            BuildingKind::Barricade,
            [COLONY_WIDTH - 2, COLONY_HEIGHT - 2],
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
fn legacy_colony_layout_migrates_founders_and_custom_work_without_collisions() {
    let mut colony = ColonyState::new();
    let legacy_founders = [
        ("command_centre", [3, 2]),
        ("barracks", [2, 3]),
        ("infirmary", [4, 3]),
        ("workshop", [3, 4]),
        ("hydroponics", [5, 3]),
        ("power_plant", [5, 4]),
    ];
    for (id, position) in legacy_founders {
        colony
            .buildings
            .iter_mut()
            .find(|building| building.id == id)
            .unwrap()
            .position = position;
    }
    colony.buildings.push(BuildingState {
        id: "barricade_7".to_owned(),
        kind: BuildingKind::Barricade,
        position: [0, 0],
        level: 1,
        damaged: true,
    });
    colony.construction_queue.push(ConstructionProject {
        id: "power_plant_8".to_owned(),
        kind: BuildingKind::PowerPlant,
        position: [7, 5],
        operations_remaining: 1,
    });

    assert!(colony.migrate_legacy_spatial_layout());
    assert!(!colony.migrate_legacy_spatial_layout());
    assert_eq!(
        colony
            .buildings
            .iter()
            .find(|building| building.id == "command_centre")
            .unwrap()
            .position,
        SETTLEMENT_CENTER
    );
    assert_eq!(
        colony
            .buildings
            .iter()
            .find(|building| building.id == "barricade_7")
            .unwrap()
            .position,
        [3, 4]
    );
    assert_eq!(colony.construction_queue[0].position, [17, 14]);
    let positions = colony
        .buildings
        .iter()
        .map(|building| building.position)
        .chain(
            colony
                .construction_queue
                .iter()
                .map(|project| project.position),
        )
        .collect::<std::collections::HashSet<_>>();
    assert_eq!(
        positions.len(),
        colony.buildings.len() + colony.construction_queue.len()
    );
    assert!(positions.into_iter().all(|position| {
        position[0] >= 0
            && position[1] >= 0
            && position[0] < COLONY_WIDTH
            && position[1] < COLONY_HEIGHT
    }));
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

#[test]
fn waystation_is_unique_and_online_on_the_redundant_starting_grid() {
    let mut colony = ColonyState::new();
    colony
        .place_construction(BuildingKind::Waystation, [2, 10])
        .unwrap();
    assert!(colony
        .place_construction(BuildingKind::Waystation, [2, 14])
        .is_err());
    colony.advance_operation();
    assert!(colony.has_facility(BuildingKind::Waystation));
    assert_eq!(colony.power_demand(), 8);
}

#[test]
fn commons_is_unique_and_online_on_the_redundant_starting_grid() {
    let mut colony = ColonyState::new();
    colony
        .place_construction(BuildingKind::Commons, [2, 10])
        .unwrap();
    assert!(colony
        .place_construction(BuildingKind::Commons, [2, 14])
        .is_err());
    colony.advance_operation();
    assert!(colony.has_facility(BuildingKind::Commons));
    assert_eq!(colony.power_demand(), 8);
}

#[test]
fn relay_mast_is_unique_and_online_on_the_redundant_starting_grid() {
    let mut colony = ColonyState::new();
    colony
        .place_construction(BuildingKind::RelayMast, [2, 10])
        .unwrap();
    assert!(colony
        .place_construction(BuildingKind::RelayMast, [2, 14])
        .is_err());
    colony.advance_operation();
    assert!(colony.has_facility(BuildingKind::RelayMast));
    assert_eq!(colony.power_demand(), 8);
}
