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
fn construction_affordance_closes_for_materials_and_unique_projects() {
    let mut colony = ColonyState::new();
    assert!(colony.can_start_construction(BuildingKind::Barricade));
    colony.resources.materials = BuildingKind::Barricade.material_cost() - 1;
    assert!(!colony.can_start_construction(BuildingKind::Barricade));

    colony.resources.materials = 120;
    assert!(colony.can_start_construction(BuildingKind::ResearchAnnex));
    colony
        .place_construction(BuildingKind::ResearchAnnex, [1, 1])
        .unwrap();
    assert!(!colony.can_start_construction(BuildingKind::ResearchAnnex));
}

#[test]
fn research_annex_is_unique_online_and_a_defense_objective() {
    let mut colony = ColonyState::new();
    let id = colony
        .place_construction(BuildingKind::ResearchAnnex, [1, 1])
        .unwrap();
    assert_eq!(colony.resources.materials, 60);
    colony.advance_operation();

    assert!(colony.has_facility(BuildingKind::ResearchAnnex));
    assert!(colony
        .place_construction(BuildingKind::ResearchAnnex, [1, 5])
        .is_err());
    let map = colony.defense_map();
    assert!(map.critical_objectives.contains(&TilePos::new(3, 2)));

    colony
        .buildings
        .iter_mut()
        .find(|building| building.id == id)
        .unwrap()
        .damaged = true;
    assert!(!colony.has_facility(BuildingKind::ResearchAnnex));
}

#[test]
fn salvage_yard_is_unique_online_and_a_defense_objective() {
    let mut colony = ColonyState::new();
    let id = colony
        .place_construction(BuildingKind::SalvageYard, [1, 1])
        .unwrap();
    assert_eq!(colony.resources.materials, 65);
    colony.advance_operation();

    assert!(colony.has_facility(BuildingKind::SalvageYard));
    assert!(colony
        .place_construction(BuildingKind::SalvageYard, [1, 5])
        .is_err());
    assert!(colony
        .defense_map()
        .critical_objectives
        .contains(&TilePos::new(3, 2)));

    colony
        .buildings
        .iter_mut()
        .find(|building| building.id == id)
        .unwrap()
        .damaged = true;
    assert!(!colony.has_facility(BuildingKind::SalvageYard));
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
    assert!(!map.cover_tiles.contains(&TilePos::new(2, 2)));
    assert!(!map.cover_tiles.contains(&TilePos::new(3, 1)));
    assert!(map.critical_objectives.contains(&TilePos::new(12, 11)));
}

#[test]
fn powered_watchtower_adds_strong_cover_until_the_grid_fails() {
    let mut colony = ColonyState::new();
    colony
        .place_construction(BuildingKind::Watchtower, [1, 1])
        .unwrap();
    colony.advance_operation();

    let tower_tile = TilePos::new(3, 2);
    let map = colony.defense_map();
    assert!(map.blocked_tiles.contains(&tower_tile));
    assert!(map.cover_tiles.contains(&tower_tile));
    assert!(map.watchtower_tiles.contains(&tower_tile));
    assert!(!map.critical_objectives.contains(&tower_tile));

    colony
        .buildings
        .iter_mut()
        .find(|building| building.kind == BuildingKind::PowerPlant)
        .unwrap()
        .damaged = true;
    colony.resources.power = 0;
    let offline_map = colony.defense_map();
    assert!(offline_map.blocked_tiles.contains(&tower_tile));
    assert!(!offline_map.cover_tiles.contains(&tower_tile));
    assert!(!offline_map.watchtower_tiles.contains(&tower_tile));
}

#[test]
fn doctrine_yard_adds_training_barricades_only_while_online() {
    let mut colony = ColonyState::new();
    let barracks_id = colony
        .buildings
        .iter()
        .find(|building| building.kind == BuildingKind::Barracks)
        .unwrap()
        .id
        .clone();
    colony
        .queue_facility_upgrade(&barracks_id, DOCTRINE_YARD_UPGRADE)
        .unwrap();
    colony.advance_operation();

    let rally_tile = TilePos::new(8, 9);
    let map = colony.defense_map();
    assert!(map.cover_tiles.contains(&TilePos::new(7, 10)));
    assert!(map.cover_tiles.contains(&TilePos::new(9, 10)));
    assert!(map.blocked_tiles.contains(&TilePos::new(7, 10)));
    assert!(map.blocked_tiles.contains(&TilePos::new(9, 10)));
    assert!(map.blocked_tiles.contains(&rally_tile));

    colony
        .buildings
        .iter_mut()
        .find(|building| building.id == barracks_id)
        .unwrap()
        .damaged = true;
    let offline_map = colony.defense_map();
    assert!(!offline_map.cover_tiles.contains(&TilePos::new(7, 10)));
    assert!(!offline_map.cover_tiles.contains(&TilePos::new(9, 10)));
}

#[test]
fn trauma_ward_adds_a_stabilization_barricade_only_while_online() {
    let mut colony = ColonyState::new();
    let infirmary_id = colony
        .buildings
        .iter()
        .find(|building| building.kind == BuildingKind::Infirmary)
        .unwrap()
        .id
        .clone();
    colony
        .queue_facility_upgrade(&infirmary_id, TRAUMA_WARD_UPGRADE)
        .unwrap();
    colony.advance_operation();

    let stabilization_tile = TilePos::new(13, 8);
    let map = colony.defense_map();
    assert!(map.blocked_tiles.contains(&stabilization_tile));
    assert!(map.cover_tiles.contains(&stabilization_tile));

    colony
        .buildings
        .iter_mut()
        .find(|building| building.id == infirmary_id)
        .unwrap()
        .damaged = true;
    let offline_map = colony.defense_map();
    assert!(!offline_map.cover_tiles.contains(&stabilization_tile));
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
fn power_plant_level_two_branches_queue_complete_and_change_the_grid() {
    let mut redundant = ColonyState::new();
    let plant_id = redundant
        .buildings
        .iter()
        .find(|building| building.kind == BuildingKind::PowerPlant)
        .unwrap()
        .id
        .clone();
    let materials = redundant.resources.materials;
    redundant
        .queue_facility_upgrade(&plant_id, REDUNDANT_GRID_UPGRADE)
        .unwrap();
    assert_eq!(redundant.resources.materials, materials - 55);
    assert!(redundant
        .queue_facility_upgrade(&plant_id, HOT_CORE_UPGRADE)
        .is_err());
    redundant.advance_operation();
    assert_eq!(
        redundant
            .buildings
            .iter()
            .find(|building| building.id == plant_id)
            .unwrap()
            .level,
        2
    );
    assert!(redundant.has_upgrade(&plant_id, REDUNDANT_GRID_UPGRADE));
    redundant
        .buildings
        .iter_mut()
        .find(|building| building.id == plant_id)
        .unwrap()
        .damaged = true;
    assert_eq!(redundant.power_supply(), 6);

    let mut hot_core = ColonyState::new();
    let plant_id = hot_core
        .buildings
        .iter()
        .find(|building| building.kind == BuildingKind::PowerPlant)
        .unwrap()
        .id
        .clone();
    hot_core
        .queue_facility_upgrade(&plant_id, HOT_CORE_UPGRADE)
        .unwrap();
    hot_core.advance_operation();
    assert!(hot_core.has_upgrade(&plant_id, HOT_CORE_UPGRADE));
    assert_eq!(hot_core.power_supply(), 11);
}

#[test]
fn hydroponics_level_two_branches_change_food_and_biomass_yields() {
    let mut kitchen = ColonyState::new();
    let hydroponics_id = kitchen
        .buildings
        .iter()
        .find(|building| building.kind == BuildingKind::Hydroponics)
        .unwrap()
        .id
        .clone();
    kitchen
        .queue_facility_upgrade(&hydroponics_id, COMMUNITY_KITCHEN_UPGRADE)
        .unwrap();
    kitchen.advance_operation();
    assert!(kitchen.has_active_upgrade(BuildingKind::Hydroponics, COMMUNITY_KITCHEN_UPGRADE));
    assert_eq!(kitchen.resources.food, 29);
    assert_eq!(kitchen.resources.biomass, 12);

    let mut culture_beds = ColonyState::new();
    let hydroponics_id = culture_beds
        .buildings
        .iter()
        .find(|building| building.kind == BuildingKind::Hydroponics)
        .unwrap()
        .id
        .clone();
    culture_beds
        .queue_facility_upgrade(&hydroponics_id, CULTURE_BEDS_UPGRADE)
        .unwrap();
    culture_beds.advance_operation();
    assert!(culture_beds.has_active_upgrade(BuildingKind::Hydroponics, CULTURE_BEDS_UPGRADE));
    assert_eq!(culture_beds.resources.food, 27);
    assert_eq!(culture_beds.resources.biomass, 14);
    culture_beds
        .buildings
        .iter_mut()
        .find(|building| building.id == hydroponics_id)
        .unwrap()
        .damaged = true;
    culture_beds.advance_operation();
    assert_eq!(culture_beds.resources.biomass, 14);
}

#[test]
fn identity_projects_are_path_specific_and_change_defense_behavior() {
    let mut redoubt = ColonyState::new();
    redoubt.ensure_identity_building("human_redoubt").unwrap();
    let redoubt_tile = redoubt
        .buildings
        .iter()
        .find(|building| building.kind == BuildingKind::RedoubtArsenal)
        .map(|building| TilePos::new(building.position[0] + 2, building.position[1] + 1))
        .unwrap();
    let redoubt_map = redoubt.defense_map();
    assert_eq!(redoubt_map.critical_objectives[0], redoubt_tile);
    assert!(redoubt_map.cover_tiles.contains(&redoubt_tile));
    assert!(redoubt
        .ensure_identity_building("living_commonwealth")
        .is_err());

    let mut choir = ColonyState::new();
    choir
        .ensure_identity_building("living_commonwealth")
        .unwrap();
    let biomass = choir.resources.biomass;
    choir.advance_operation();
    assert_eq!(choir.resources.biomass, biomass + 1);
    let choir_building = choir
        .buildings
        .iter()
        .find(|building| building.kind == BuildingKind::ChoirGarden)
        .unwrap();
    let choir_tile = TilePos::new(
        choir_building.position[0] + 2,
        choir_building.position[1] + 1,
    );
    assert!(choir.defense_map().cover_tiles.contains(&choir_tile));

    let mut threshold = ColonyState::new();
    threshold
        .ensure_identity_building("open_threshold")
        .unwrap();
    assert!(threshold.defense_map().shield_tiles.is_empty());
    threshold.resources.power += 2;
    let threshold_building = threshold
        .buildings
        .iter()
        .find(|building| building.kind == BuildingKind::ThresholdSpire)
        .unwrap();
    let threshold_tile = TilePos::new(
        threshold_building.position[0] + 2,
        threshold_building.position[1] + 1,
    );
    assert!(threshold
        .defense_map()
        .shield_tiles
        .contains(&threshold_tile));
    threshold
        .buildings
        .iter_mut()
        .find(|building| building.kind == BuildingKind::ThresholdSpire)
        .unwrap()
        .damaged = true;
    assert!(threshold.defense_map().shield_tiles.is_empty());
}

#[test]
fn buildable_projects_own_only_their_anchor_but_require_surrounding_clearance() {
    for kind in [
        BuildingKind::Barricade,
        BuildingKind::PowerPlant,
        BuildingKind::GeneLab,
        BuildingKind::Waystation,
        BuildingKind::Commons,
        BuildingKind::RelayMast,
        BuildingKind::Watchtower,
    ] {
        let mut colony = ColonyState::new();
        colony.place_construction(kind, [3, 3]).unwrap();
        assert_eq!(
            colony.project_at([3, 3]).map(|project| project.kind),
            Some(kind)
        );
        for candidate in clearance_conflict_positions([3, 3]) {
            assert!(colony.project_at(candidate).is_none());
            assert!(
                colony
                    .place_construction(BuildingKind::Barricade, candidate)
                    .is_err(),
                "construction zone overlapped queued {} at {candidate:?}",
                kind.name()
            );
        }

        colony.advance_operation();
        assert_eq!(
            colony.building_at([3, 3]).map(|building| building.kind),
            Some(kind)
        );
        for neighbour in clearance_positions([3, 3]).filter(|tile| *tile != [3, 3]) {
            assert!(colony.building_at(neighbour).is_none());
        }
        assert!(colony
            .place_construction(BuildingKind::Barricade, [5, 3])
            .is_err());
        colony
            .place_construction(BuildingKind::Barricade, [6, 3])
            .expect("non-overlapping 3x3 placement zones satisfy clearance");
    }
}

#[test]
fn every_building_kind_owns_only_its_anchor_tile() {
    for kind in [
        BuildingKind::CommandCentre,
        BuildingKind::Barracks,
        BuildingKind::Infirmary,
        BuildingKind::Workshop,
        BuildingKind::Barricade,
        BuildingKind::Hydroponics,
        BuildingKind::PowerPlant,
        BuildingKind::GeneLab,
        BuildingKind::Waystation,
        BuildingKind::Commons,
        BuildingKind::RelayMast,
        BuildingKind::Watchtower,
        BuildingKind::RedoubtArsenal,
        BuildingKind::ChoirGarden,
        BuildingKind::ThresholdSpire,
    ] {
        let mut colony = ColonyState::new();
        colony.buildings.clear();
        colony.buildings.push(BuildingState {
            id: "footprint_probe".to_owned(),
            kind,
            position: [3, 3],
            level: 1,
            damaged: false,
        });
        assert_eq!(
            colony.building_at([3, 3]).map(|found| found.id.as_str()),
            Some("footprint_probe")
        );
        assert!(colony.validate_construction_site([3, 3]).is_err());
        for neighbour in clearance_positions([3, 3]).filter(|tile| *tile != [3, 3]) {
            assert!(
                colony.building_at(neighbour).is_none(),
                "{} incorrectly claimed clearance tile {neighbour:?}",
                kind.name()
            );
        }
        for candidate in clearance_conflict_positions([3, 3]) {
            assert!(
                colony.validate_construction_site(candidate).is_err(),
                "{} allowed an overlapping 3x3 zone at {candidate:?}",
                kind.name()
            );
        }
        assert!(colony.validate_construction_site([6, 3]).is_ok());
        assert_eq!(kind.footprint(), &[[0, 0]]);
    }
}

fn clearance_conflict_positions(anchor: [i32; 2]) -> impl Iterator<Item = [i32; 2]> {
    (-2..=2)
        .flat_map(move |dy| (-2..=2).map(move |dx| [anchor[0] + dx, anchor[1] + dy]))
        .filter(move |position| *position != anchor)
}

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
