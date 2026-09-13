//! Focused tests integration tests.

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
    let repair = colony.repair_building(&id).unwrap();
    let cost = repair.materials_spent;
    assert_eq!(colony.resources.materials, materials - cost);
    assert!(colony.has_facility(kind));
}

#[test]
fn repair_affordance_closes_when_materials_cannot_cover_the_effective_cost() {
    let mut colony = ColonyState::new();
    let id = colony
        .buildings
        .iter()
        .find(|building| building.kind == BuildingKind::Hydroponics)
        .unwrap()
        .id
        .clone();
    colony
        .buildings
        .iter_mut()
        .find(|building| building.id == id)
        .unwrap()
        .damaged = true;
    assert_eq!(colony.repair_cost_for(&id), Some(25));
    colony.resources.materials = 24;
    assert!(!colony.can_repair_building(&id));
    colony.resources.materials = 25;
    assert!(colony.can_repair_building(&id));
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
