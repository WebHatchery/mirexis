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
    use crate::data::{CoverEdgeDef, EdgeDirection, HazardKind};
    use crate::tactical::{
        BattleEvent, DestructibleCover, HazardTile, ObscuringField, ReinforcementWave,
    };
    use macroquad_toolkit::grid::{FlatGrid, FogState, TilePos};
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
    use crate::data::{CoverEdgeDef, EdgeDirection, HazardKind};
    use crate::tactical::{
        BattleEvent, DestructibleCover, HazardTile, ObscuringField, ReinforcementWave,
    };
    use macroquad_toolkit::grid::TilePos;

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

#[test]
fn version_171_save_gains_outsider_runtime_defaults() {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let mut legacy = serde_json::to_value(session.to_save("1.71.0", &campaign)).unwrap();
    let campaign = legacy["campaign"].as_object_mut().unwrap();
    campaign.remove("outsider_arc_stage");
    campaign.remove("outsider_disagreements");
    campaign.remove("outsider_final_choice");
    campaign.remove("outsider_arc_states");
    campaign.remove("commons_meals_hosted");
    campaign.remove("commons_meal_operation");
    for character in campaign["roster"].as_array_mut().unwrap() {
        let character = character.as_object_mut().unwrap();
        character.remove("origin");
        character.remove("origin_description");
    }

    let migrated = migrate_save_value(Some("1.71.0".to_owned()), legacy, &data).unwrap();

    assert_eq!(migrated.version, data.config.version);
    assert_eq!(migrated.campaign.outsider_arc_stage, 0);
    assert_eq!(migrated.campaign.outsider_disagreements, 0);
    assert!(migrated.campaign.outsider_final_choice.is_empty());
    assert!(migrated.campaign.outsider_arc_states.is_empty());
    assert_eq!(migrated.campaign.commons_meals_hosted, 0);
    assert_eq!(migrated.campaign.commons_meal_operation, None);
    assert!(migrated
        .campaign
        .roster
        .iter()
        .all(|character| character.origin.is_empty() && character.origin_description.is_empty()));
}

#[test]
fn version_172_save_gains_commons_runtime_defaults() {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let mut legacy = serde_json::to_value(session.to_save("1.72.0", &campaign)).unwrap();
    let campaign = legacy["campaign"].as_object_mut().unwrap();
    campaign.remove("commons_meals_hosted");
    campaign.remove("commons_meal_operation");

    let migrated = migrate_save_value(Some("1.72.0".to_owned()), legacy, &data).unwrap();

    assert_eq!(migrated.version, data.config.version);
    assert_eq!(migrated.campaign.commons_meals_hosted, 0);
    assert_eq!(migrated.campaign.commons_meal_operation, None);
}

#[test]
fn version_173_save_gains_relay_runtime_defaults() {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let mut legacy = serde_json::to_value(session.to_save("1.73.0", &campaign)).unwrap();
    let campaign = legacy["campaign"].as_object_mut().unwrap();
    campaign.remove("relay_scans_used");
    campaign.remove("relay_scan_operation");

    let migrated = migrate_save_value(Some("1.73.0".to_owned()), legacy, &data).unwrap();

    assert_eq!(migrated.version, data.config.version);
    assert_eq!(migrated.campaign.relay_scans_used, 0);
    assert_eq!(migrated.campaign.relay_scan_operation, None);
}

#[test]
fn same_version_corner_colony_save_moves_into_the_centered_frontier() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    let legacy_founders = [
        ("command_centre", [3, 2]),
        ("barracks", [2, 3]),
        ("infirmary", [4, 3]),
        ("workshop", [3, 4]),
        ("hydroponics", [5, 3]),
        ("power_plant", [5, 4]),
    ];
    for (id, position) in legacy_founders {
        campaign
            .colony
            .buildings
            .iter_mut()
            .find(|building| building.id == id)
            .unwrap()
            .position = position;
    }
    let legacy =
        serde_json::to_value(SaveData::campaign_only(&data.config.version, &campaign)).unwrap();

    let migrated = migrate_save_value(Some(data.config.version.clone()), legacy, &data).unwrap();
    let command_centre = migrated
        .campaign
        .colony
        .buildings
        .iter()
        .find(|building| building.id == "command_centre")
        .unwrap();
    assert_eq!(command_centre.position, crate::colony::SETTLEMENT_CENTER);
    assert!(migrated.campaign.colony.buildings.iter().all(|building| {
        building.position[0] >= 0
            && building.position[1] >= 0
            && building.position[0] < crate::colony::COLONY_WIDTH
            && building.position[1] < crate::colony::COLONY_HEIGHT
    }));
}

#[test]
fn chitin_save_gains_ilyas_unevolved_paths() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign.strategy.contact_complete = true;
    campaign.strategy.phase_id = "adaptation".to_owned();
    campaign.colony.ensure_gene_lab();
    campaign.roster[1].mutation_evolution_id = "fortress_carapace".to_owned();
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let legacy = serde_json::to_value(session.to_save("1.15.0", &campaign)).unwrap();
    let migrated = migrate_save_value(Some("1.15.0".to_owned()), legacy, &data).unwrap();
    let ilya = migrated
        .campaign
        .roster
        .iter()
        .find(|character| character.id == "ilya_reed")
        .unwrap();
    assert!(ilya.mutation_evolution_id.is_empty());
    assert_eq!(
        data.mutations
            .iter()
            .find(|mutation| mutation.id == ilya.mutation_id)
            .unwrap()
            .evolutions
            .len(),
        2
    );
}

#[test]
fn regeneration_save_gains_sols_unevolved_paths() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign.strategy.contact_complete = true;
    campaign.strategy.phase_id = "adaptation".to_owned();
    campaign.colony.ensure_gene_lab();
    campaign.roster[2].mutation_evolution_id = "clean_marrow".to_owned();
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let legacy = serde_json::to_value(session.to_save("1.16.0", &campaign)).unwrap();
    let migrated = migrate_save_value(Some("1.16.0".to_owned()), legacy, &data).unwrap();
    let sol = migrated
        .campaign
        .roster
        .iter()
        .find(|character| character.id == "sol_cairn")
        .unwrap();
    assert!(sol.mutation_evolution_id.is_empty());
    assert_eq!(
        data.mutations
            .iter()
            .find(|mutation| mutation.id == sol.mutation_id)
            .unwrap()
            .evolutions
            .len(),
        2
    );
}

#[test]
fn full_roster_save_gains_adaptation_completion_gates() {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let mut legacy = serde_json::to_value(session.to_save("1.17.0", &campaign)).unwrap();
    let strategy = legacy["campaign"]["strategy"].as_object_mut().unwrap();
    strategy.remove("adaptation_operation_completed");
    strategy.remove("adaptation_complete");
    let migrated = migrate_save_value(Some("1.17.0".to_owned()), legacy, &data).unwrap();
    assert_eq!(migrated.version, data.config.version);
    assert!(!migrated.campaign.strategy.adaptation_operation_completed);
    assert!(!migrated.campaign.strategy.adaptation_complete);
}

#[test]
fn adaptation_completion_save_gains_the_escalation_operation() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign.strategy.contact_complete = true;
    campaign.strategy.adaptation_operation_completed = true;
    campaign.strategy.adaptation_complete = true;
    campaign.strategy.phase_id = "escalation".to_owned();
    campaign.strategy.phase_name = "PHASE FOUR: ESCALATION".to_owned();
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let legacy = serde_json::to_value(session.to_save("1.18.0", &campaign)).unwrap();
    let migrated = migrate_save_value(Some("1.18.0".to_owned()), legacy, &data).unwrap();

    assert_eq!(migrated.version, data.config.version);
    assert!(migrated
        .campaign
        .strategy
        .mission_offers
        .iter()
        .any(|mission| mission.template_id == "escalation_three_knives"));
}

#[test]
fn escalation_operation_save_gains_response_state() {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let mut legacy = serde_json::to_value(session.to_save("1.19.0", &campaign)).unwrap();
    let strategy = legacy["campaign"]["strategy"].as_object_mut().unwrap();
    strategy.remove("escalation_operation_completed");
    strategy.remove("escalation_response_id");
    let migrated = migrate_save_value(Some("1.19.0".to_owned()), legacy, &data).unwrap();

    assert_eq!(migrated.version, data.config.version);
    assert!(!migrated.campaign.strategy.escalation_operation_completed);
    assert!(migrated.campaign.strategy.escalation_response_id.is_empty());
}

#[test]
fn convergence_response_save_gains_its_branch_operation() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign.strategy.phase_id = "escalation".to_owned();
    campaign.strategy.adaptation_complete = true;
    campaign.strategy.escalation_operation_completed = true;
    campaign.strategy.escalation_response_id = "bastion_beacon".to_owned();
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let legacy = serde_json::to_value(session.to_save("1.20.0", &campaign)).unwrap();
    let migrated = migrate_save_value(Some("1.20.0".to_owned()), legacy, &data).unwrap();

    assert_eq!(migrated.version, data.config.version);
    assert!(migrated
        .campaign
        .strategy
        .mission_offers
        .iter()
        .any(|mission| mission.template_id == "escalation_bastion_breakwater"));
}

#[test]
fn escalation_save_gains_mixed_power_deployment() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign.strategy.phase_id = "escalation".to_owned();
    campaign.strategy.adaptation_complete = true;
    campaign.strategy.regenerate_missions(&data);
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let mut legacy = serde_json::to_value(session.to_save("1.21.0", &campaign)).unwrap();
    for offer in legacy["campaign"]["strategy"]["mission_offers"]
        .as_array_mut()
        .unwrap()
    {
        offer.as_object_mut().unwrap().remove("hostile_unit_ids");
    }
    let migrated = migrate_save_value(Some("1.21.0".to_owned()), legacy, &data).unwrap();
    let three_knives = migrated
        .campaign
        .strategy
        .mission_offers
        .iter()
        .find(|mission| mission.template_id == "escalation_three_knives")
        .unwrap();

    assert_eq!(migrated.version, data.config.version);
    assert_eq!(three_knives.hostile_unit_ids.len(), 3);
}

#[test]
fn mixed_power_save_gains_escalation_completion_gates() {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let mut legacy = serde_json::to_value(session.to_save("1.22.0", &campaign)).unwrap();
    let strategy = legacy["campaign"]["strategy"].as_object_mut().unwrap();
    strategy.remove("escalation_branch_completed");
    strategy.remove("escalation_complete");
    let migrated = migrate_save_value(Some("1.22.0".to_owned()), legacy, &data).unwrap();

    assert_eq!(migrated.version, data.config.version);
    assert!(!migrated.campaign.strategy.escalation_branch_completed);
    assert!(!migrated.campaign.strategy.escalation_complete);
}

#[test]
fn phase_five_save_gains_an_uncommitted_mirexis_path() {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let mut legacy = serde_json::to_value(session.to_save("1.23.0", &campaign)).unwrap();
    legacy["campaign"]["strategy"]
        .as_object_mut()
        .unwrap()
        .remove("mirexis_path_id");
    let migrated = migrate_save_value(Some("1.23.0".to_owned()), legacy, &data).unwrap();

    assert_eq!(migrated.version, data.config.version);
    assert!(migrated.campaign.strategy.mirexis_path_id.is_empty());
}

#[test]
fn mirexis_path_save_gains_its_phase_five_operation() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign.strategy.phase_id = "mirexis".to_owned();
    campaign.strategy.escalation_complete = true;
    campaign.strategy.mirexis_path_id = "open_threshold".to_owned();
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let legacy = serde_json::to_value(session.to_save("1.24.0", &campaign)).unwrap();
    let migrated = migrate_save_value(Some("1.24.0".to_owned()), legacy, &data).unwrap();

    assert_eq!(migrated.version, data.config.version);
    assert!(migrated
        .campaign
        .strategy
        .mission_offers
        .iter()
        .any(|mission| mission.template_id == "mirexis_threshold_door_of_light"));
}

#[test]
fn phase_five_operation_save_gains_campaign_completion_gates() {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let mut legacy = serde_json::to_value(session.to_save("1.25.0", &campaign)).unwrap();
    let strategy = legacy["campaign"]["strategy"].as_object_mut().unwrap();
    strategy.remove("mirexis_operation_completed");
    strategy.remove("campaign_complete");
    let migrated = migrate_save_value(Some("1.25.0".to_owned()), legacy, &data).unwrap();

    assert_eq!(migrated.version, data.config.version);
    assert!(!migrated.campaign.strategy.mirexis_operation_completed);
    assert!(!migrated.campaign.strategy.campaign_complete);
}

#[test]
fn completed_campaign_save_gains_nadi_as_an_unevolved_reserve() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign
        .roster
        .retain(|character| character.id != "nadi_vale");
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let legacy = serde_json::to_value(session.to_save("1.26.0", &campaign)).unwrap();
    let migrated = migrate_save_value(Some("1.26.0".to_owned()), legacy, &data).unwrap();
    let nadi = migrated
        .campaign
        .roster
        .iter()
        .find(|character| character.id == "nadi_vale")
        .expect("Nadi joins an existing campaign as a reserve");

    assert_eq!(migrated.version, data.config.version);
    assert!(!nadi.deployment_selected);
    assert!(nadi.mutation_evolution_id.is_empty());
}

#[test]
fn nadi_roster_save_preserves_class_mastery_for_advanced_training() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    let mara = campaign
        .roster
        .iter_mut()
        .find(|character| character.id == "mara_venn")
        .unwrap();
    mara.level = 3;
    mara.class_history.push("soldier".to_owned());
    campaign.strategy.phase_id = "adaptation".to_owned();
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let legacy = serde_json::to_value(session.to_save("1.27.0", &campaign)).unwrap();
    let migrated = migrate_save_value(Some("1.27.0".to_owned()), legacy, &data).unwrap();
    let vanguard = data
        .classes
        .iter()
        .find(|class| class.id == "vanguard")
        .unwrap();

    assert_eq!(migrated.version, data.config.version);
    assert_eq!(
        migrated
            .campaign
            .class_training_lock_reason("mara_venn", vanguard),
        None
    );
}

#[test]
fn advanced_class_save_preserves_new_weapon_profiles() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    let kira = campaign
        .roster
        .iter_mut()
        .find(|character| character.id == "kira_voss")
        .unwrap();
    kira.equipment_ids.retain(|id| id != "frontier_rifle");
    kira.equipment_ids.push("needle_carbine".to_owned());
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let legacy = serde_json::to_value(session.to_save("1.28.0", &campaign)).unwrap();
    let migrated = migrate_save_value(Some("1.28.0".to_owned()), legacy, &data).unwrap();
    let kira = migrated
        .campaign
        .roster
        .iter()
        .find(|character| character.id == "kira_voss")
        .unwrap();

    assert_eq!(migrated.version, data.config.version);
    assert!(kira.equipment_ids.contains(&"needle_carbine".to_owned()));
}

#[test]
fn weapon_profile_save_gains_empty_relationship_history() {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let mut legacy = serde_json::to_value(session.to_save("1.29.0", &campaign)).unwrap();
    legacy["campaign"]
        .as_object_mut()
        .unwrap()
        .remove("relationships");
    let migrated = migrate_save_value(Some("1.29.0".to_owned()), legacy, &data).unwrap();

    assert_eq!(migrated.version, data.config.version);
    assert!(migrated.campaign.relationships.is_empty());
}

#[test]
fn relationship_save_gains_disarmed_tactical_overwatch() {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let mut legacy = serde_json::to_value(session.to_save("1.30.0", &campaign)).unwrap();
    for unit in legacy["tactical"]["units"].as_array_mut().unwrap() {
        unit.as_object_mut().unwrap().remove("overwatching");
    }
    let migrated = migrate_save_value(Some("1.30.0".to_owned()), legacy, &data).unwrap();

    assert_eq!(migrated.version, data.config.version);
    assert!(migrated
        .tactical
        .unwrap()
        .units
        .iter()
        .all(|unit| !unit.overwatching));
}

#[test]
fn overwatch_save_gains_empty_character_trauma_histories() {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let mut legacy = serde_json::to_value(session.to_save("1.31.0", &campaign)).unwrap();
    for character in legacy["campaign"]["roster"].as_array_mut().unwrap() {
        character.as_object_mut().unwrap().remove("traumas");
    }
    let migrated = migrate_save_value(Some("1.31.0".to_owned()), legacy, &data).unwrap();

    assert_eq!(migrated.version, data.config.version);
    assert!(migrated
        .campaign
        .roster
        .iter()
        .all(|character| character.traumas.is_empty()));
}

#[test]
fn trauma_save_gains_inactive_defense_asset_integrity() {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let mut legacy = serde_json::to_value(session.to_save("1.32.0", &campaign)).unwrap();
    let tactical = legacy["tactical"].as_object_mut().unwrap();
    tactical.remove("objective_integrity");
    tactical.remove("objective_max_integrity");
    let migrated = migrate_save_value(Some("1.32.0".to_owned()), legacy, &data).unwrap();
    let tactical = migrated.tactical.unwrap();

    assert_eq!(migrated.version, data.config.version);
    assert_eq!(tactical.objective_integrity, 0);
    assert_eq!(tactical.objective_max_integrity, 0);
}

#[test]
fn defense_objective_save_gains_hostile_factions_and_ready_abilities() {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let mut legacy = serde_json::to_value(session.to_save("1.33.0", &campaign)).unwrap();
    for unit in legacy["tactical"]["units"].as_array_mut().unwrap() {
        let unit = unit.as_object_mut().unwrap();
        unit.remove("faction");
        unit.remove("enemy_ability_used");
    }
    let migrated = migrate_save_value(Some("1.33.0".to_owned()), legacy, &data).unwrap();
    let tactical = migrated.tactical.unwrap();

    assert_eq!(migrated.version, data.config.version);
    assert!(tactical
        .units
        .iter()
        .filter(|unit| unit.team == crate::data::Team::Hostile)
        .all(|unit| unit.faction.is_some() && !unit.enemy_ability_used));
}

#[test]
fn faction_ability_save_gains_empty_realized_hazard_tiles() {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let mut legacy = serde_json::to_value(session.to_save("1.34.0", &campaign)).unwrap();
    legacy["tactical"]
        .as_object_mut()
        .unwrap()
        .remove("hazards");
    let migrated = migrate_save_value(Some("1.34.0".to_owned()), legacy, &data).unwrap();

    assert_eq!(migrated.version, data.config.version);
    assert!(migrated.tactical.unwrap().hazards.is_empty());
}

#[test]
fn hazard_save_keeps_realized_tiles_when_intent_preview_is_added() {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let mut session = GameSession::new(&data.config, &data.mission, &data.roster);
    session.tactical.hazards.push(crate::tactical::HazardTile {
        position: macroquad_toolkit::grid::TilePos::new(2, 2),
        kind: crate::data::HazardKind::StaticRift,
    });
    let legacy = serde_json::to_value(session.to_save("1.35.0", &campaign)).unwrap();
    let migrated = migrate_save_value(Some("1.35.0".to_owned()), legacy, &data).unwrap();

    assert_eq!(migrated.version, data.config.version);
    assert_eq!(migrated.tactical.unwrap().hazards, session.tactical.hazards);
}
