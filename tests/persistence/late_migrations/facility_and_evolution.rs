//! Facility and mutation evolution save migrations.

use super::*;

#[test]
fn gene_lab_save_gains_maras_unevolved_paths() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign.strategy.contact_complete = true;
    campaign.strategy.phase_id = "adaptation".to_owned();
    campaign.colony.ensure_gene_lab();
    campaign.roster[0].mutation_evolution_id = "expanded_cortex".to_owned();
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let legacy = serde_json::to_value(session.to_save("1.14.0", &campaign)).unwrap();
    let migrated = migrate_save_value(Some("1.14.0".to_owned()), legacy, &data).unwrap();
    let mara = migrated
        .campaign
        .roster
        .iter()
        .find(|character| character.id == "mara_venn")
        .unwrap();
    assert!(mara.mutation_evolution_id.is_empty());
    assert_eq!(
        data.mutations
            .iter()
            .find(|mutation| mutation.id == mara.mutation_id)
            .unwrap()
            .evolutions
            .len(),
        2
    );
}

#[test]
fn same_version_small_battle_save_projects_every_spatial_field_into_the_large_world() {
    use macroquad_toolkit::grid::{FlatGrid, FogState, TilePos};
    use mirexis::data::{CoverEdgeDef, EdgeDirection, HazardKind};
    use mirexis::tactical::{
        BattleEvent, DestructibleCover, HazardTile, ObscuringField, ReinforcementWave,
    };
    use std::collections::HashSet;

    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let mut session = GameSession::new(&data.config, &data.mission, &data.roster);
    let tactical = &mut session.tactical;
    tactical.fog = FlatGrid::new(12, 8, FogState::Visible);
    tactical.blocked = HashSet::from([TilePos::new(4, 2)]);
    tactical.terrain_costs = vec![(TilePos::new(5, 3), 2)];
    tactical.hazards = vec![HazardTile {
        position: TilePos::new(6, 4),
        kind: HazardKind::SporeBloom,
    }];
    tactical.cover_edges = vec![CoverEdgeDef {
        position: [7, 5],
        direction: EdgeDirection::West,
        strength: 25,
    }];
    tactical.destructible_cover = vec![DestructibleCover {
        position: TilePos::new(8, 6),
        health: 4,
        max_health: 6,
    }];
    tactical.obscuring_fields = vec![ObscuringField {
        center: TilePos::new(9, 7),
        radius: 1,
        remaining_phases: 1,
    }];
    for (index, unit) in tactical.units.iter_mut().enumerate() {
        unit.position = TilePos::new(index as i32 + 1, 2);
    }
    let mut reinforcement = tactical.units[0].clone();
    reinforcement.position = TilePos::new(2, 3);
    tactical.reinforcement_waves = vec![ReinforcementWave {
        round: 3,
        units: vec![reinforcement],
    }];
    tactical.selected_tile = TilePos::new(1, 2);
    tactical.objective_tile = TilePos::new(8, 4);
    tactical.event_log = vec![
        BattleEvent::UnitMoved {
            unit_id: "kira_voss".to_owned(),
            path: vec![TilePos::new(1, 1), TilePos::new(1, 2)],
            cost: 1,
        },
        BattleEvent::CoverDamaged {
            position: TilePos::new(8, 6),
            amount: 2,
            remaining: 4,
        },
        BattleEvent::HazardConverted {
            unit_id: "kira_voss".to_owned(),
            position: TilePos::new(9, 7),
            kind: HazardKind::SporeBloom,
        },
    ];

    let legacy = serde_json::to_value(session.to_save(&data.config.version, &campaign)).unwrap();
    let migrated = migrate_save_value(Some(data.config.version.clone()), legacy, &data).unwrap();
    let tactical = migrated.tactical.unwrap();
    assert_eq!((tactical.fog.width, tactical.fog.height), (40, 40));
    assert!(tactical.blocked.contains(&TilePos::new(18, 13)));
    assert_eq!(tactical.terrain_costs[0].0, TilePos::new(20, 17));
    assert_eq!(tactical.hazards[0].position, TilePos::new(22, 21));
    assert_eq!(tactical.cover_edges[0].position, [24, 25]);
    assert_eq!(
        tactical.destructible_cover[0].position,
        TilePos::new(26, 29)
    );
    assert_eq!(tactical.obscuring_fields[0].center, TilePos::new(28, 33));
    assert_eq!(tactical.units[0].position, TilePos::new(12, 13));
    assert_eq!(
        tactical.reinforcement_waves[0].units[0].position,
        TilePos::new(14, 17)
    );
    assert_eq!(tactical.selected_tile, TilePos::new(12, 13));
    assert_eq!(tactical.objective_tile, TilePos::new(26, 21));
    assert!(matches!(
        &tactical.event_log[0],
        BattleEvent::UnitMoved { path, .. }
            if path == &[TilePos::new(12, 9), TilePos::new(12, 13)]
    ));
    assert!(matches!(
        tactical.event_log[1],
        BattleEvent::CoverDamaged {
            position: TilePos { x: 26, y: 29 },
            ..
        }
    ));
    assert!(matches!(
        tactical.event_log[2],
        BattleEvent::HazardConverted {
            position: TilePos { x: 28, y: 33 },
            ..
        }
    ));
}

#[test]
fn current_world_save_rejects_every_out_of_bounds_positional_family() {
    use macroquad_toolkit::grid::TilePos;
    use mirexis::data::{CoverEdgeDef, EdgeDirection, HazardKind};
    use mirexis::tactical::{
        BattleEvent, DestructibleCover, HazardTile, ObscuringField, ReinforcementWave,
    };

    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let base = session.to_save(&data.config.version, &campaign);
    let outside = TilePos::new(data.config.world_width as i32, 0);
    let mut cases = Vec::new();

    let mut save = base.clone();
    save.tactical.as_mut().unwrap().selected_tile = outside;
    cases.push(("selection", save));
    let mut save = base.clone();
    save.tactical.as_mut().unwrap().objective_tile = outside;
    cases.push(("objective", save));
    let mut save = base.clone();
    save.tactical.as_mut().unwrap().blocked.insert(outside);
    cases.push(("blocked tile", save));
    let mut save = base.clone();
    save.tactical.as_mut().unwrap().terrain_costs = vec![(outside, 2)];
    cases.push(("terrain tile", save));
    let mut save = base.clone();
    save.tactical.as_mut().unwrap().hazards = vec![HazardTile {
        position: outside,
        kind: HazardKind::SporeBloom,
    }];
    cases.push(("hazard", save));
    let mut save = base.clone();
    save.tactical.as_mut().unwrap().cover_edges = vec![CoverEdgeDef {
        position: [outside.x, outside.y],
        direction: EdgeDirection::West,
        strength: 25,
    }];
    cases.push(("cover edge", save));
    let mut save = base.clone();
    save.tactical.as_mut().unwrap().destructible_cover = vec![DestructibleCover {
        position: outside,
        health: 4,
        max_health: 6,
    }];
    cases.push(("destructible cover", save));
    let mut save = base.clone();
    save.tactical.as_mut().unwrap().obscuring_fields = vec![ObscuringField {
        center: outside,
        radius: 1,
        remaining_phases: 1,
    }];
    cases.push(("obscuring field", save));
    let mut save = base.clone();
    save.tactical.as_mut().unwrap().units[0].position = outside;
    cases.push(("unit", save));
    let mut save = base.clone();
    let mut reinforcement = save.tactical.as_ref().unwrap().units[0].clone();
    reinforcement.position = outside;
    save.tactical.as_mut().unwrap().reinforcement_waves = vec![ReinforcementWave {
        round: 2,
        units: vec![reinforcement],
    }];
    cases.push(("reinforcement", save));
    let mut save = base.clone();
    save.tactical.as_mut().unwrap().event_log = vec![BattleEvent::UnitMoved {
        unit_id: "mara_venn".to_owned(),
        path: vec![outside],
        cost: 1,
    }];
    cases.push(("movement history", save));
    let mut save = base.clone();
    save.tactical.as_mut().unwrap().event_log =
        vec![BattleEvent::CoverDestroyed { position: outside }];
    cases.push(("cover history", save));
    let mut save = base;
    save.tactical.as_mut().unwrap().event_log = vec![BattleEvent::HazardConverted {
        unit_id: "kira_voss".to_owned(),
        position: outside,
        kind: HazardKind::SporeBloom,
    }];
    cases.push(("hazard history", save));

    for (label, save) in cases {
        let value = serde_json::to_value(save).unwrap();
        let error =
            migrate_save_value(Some(data.config.version.clone()), value, &data).unwrap_err();
        assert!(error.contains(label), "{label}: {error}");
        assert!(
            error.contains("outside the 40x40 world"),
            "{label}: {error}"
        );
    }
}

#[test]
fn unknown_tactical_grid_dimensions_fail_with_a_clear_migration_error() {
    use macroquad_toolkit::grid::{FlatGrid, FogState};

    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let mut save = session.to_save(&data.config.version, &campaign);
    save.tactical.as_mut().unwrap().fog = FlatGrid::new(13, 8, FogState::Visible);
    let value = serde_json::to_value(save).unwrap();
    let error = migrate_save_value(Some(data.config.version.clone()), value, &data).unwrap_err();
    assert_eq!(error, "Cannot migrate tactical grid 13x8 to 40x40");
}
