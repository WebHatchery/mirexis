//! Resource production, upgrades, and project-clearance regressions.

use super::*;

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
