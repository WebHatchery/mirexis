use super::*;

#[test]
fn all_five_mutations_produce_gift_and_complication_traits() {
    let data = GameData::load().unwrap();
    assert!(data.mutations.len() >= 5);
    for mutation in &data.mutations {
        assert!(!mutation.gift.is_empty());
        assert!(!mutation.complication.is_empty());
        let mut traits = BTreeMap::new();
        apply_mutation(mutation, &mut traits);
        assert!(traits.len() >= 2);
    }
}
#[test]
fn aptitude_changes_base_class_training_cost() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    let psionic = data
        .classes
        .iter()
        .find(|class| class.id == "psionic")
        .unwrap();
    let cost = campaign.training_cost("mara_venn", psionic).unwrap();
    assert!(cost > campaign.training_cost("kira_voss", psionic).unwrap());
    campaign
        .switch_class("mara_venn", "psionic", &data)
        .unwrap();
    assert_eq!(campaign.roster[1].active_class, "psionic");
}
#[test]
fn advanced_training_requires_phase_level_and_both_disciplines() {
    let data = GameData::load().unwrap();
    let vanguard = data
        .classes
        .iter()
        .find(|class| class.id == "vanguard")
        .unwrap();
    let mut campaign = CampaignState::new(&data);
    assert_eq!(
        campaign.class_training_lock_reason("mara_venn", vanguard),
        Some("REQUIRES LEVEL 3".to_owned())
    );
    let mara = campaign
        .roster
        .iter_mut()
        .find(|character| character.id == "mara_venn")
        .unwrap();
    mara.level = 3;
    campaign.strategy.phase_id = "adaptation".to_owned();
    assert_eq!(
        campaign.class_training_lock_reason("mara_venn", vanguard),
        Some("MASTER SOLDIER".to_owned())
    );
    campaign
        .switch_class("mara_venn", "soldier", &data)
        .unwrap();
    assert_eq!(
        campaign.class_training_lock_reason("mara_venn", vanguard),
        None
    );
    campaign.colony.resources.materials = 999;
    campaign
        .train_character("mara_venn", "vanguard", &data)
        .unwrap();
    assert_eq!(
        campaign
            .roster
            .iter()
            .find(|character| character.id == "mara_venn")
            .unwrap()
            .active_class,
        "vanguard"
    );
}
#[test]
fn deployment_applies_class_mutation_and_equipment() {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let roster = campaign.deployment_roster(&data, &data.mission);
    let mara = roster.iter().find(|unit| unit.id == "mara_venn").unwrap();
    assert!(mara.armour >= 5);
    assert_eq!(mara.role, "Defender");
}
#[test]
fn primary_weapon_families_override_range_and_action_economy() {
    let data = GameData::load().unwrap();
    let mut scatter_campaign = CampaignState::new(&data);
    let kira = scatter_campaign
        .roster
        .iter_mut()
        .find(|character| character.id == "kira_voss")
        .unwrap();
    kira.equipment_ids.retain(|id| id != "frontier_rifle");
    kira.equipment_ids.push("breach_scattergun".to_owned());
    let scatter = scatter_campaign
        .deployment_roster(&data, &data.mission)
        .into_iter()
        .find(|unit| unit.id == "kira_voss")
        .unwrap();
    assert_eq!(scatter.weapon_range, 3);
    assert_eq!(scatter.weapon_ap_cost, 2);

    let kira = scatter_campaign
        .roster
        .iter_mut()
        .find(|character| character.id == "kira_voss")
        .unwrap();
    kira.equipment_ids.retain(|id| id != "breach_scattergun");
    kira.equipment_ids.push("needle_carbine".to_owned());
    let carbine = scatter_campaign
        .deployment_roster(&data, &data.mission)
        .into_iter()
        .find(|unit| unit.id == "kira_voss")
        .unwrap();
    assert_eq!(carbine.weapon_range, 6);
    assert_eq!(carbine.weapon_ap_cost, 1);
    assert!(carbine.weapon_damage < scatter.weapon_damage);
}
#[test]
fn deployment_uses_only_the_mission_factions_hostiles() {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    for faction in ["brood", "directorate", "ascendants"] {
        let mut mission = data.mission.clone();
        mission.hostile_faction = faction.to_owned();
        let roster = campaign.deployment_roster(&data, &mission);
        let hostiles = roster
            .iter()
            .filter(|unit| unit.team == Team::Hostile)
            .collect::<Vec<_>>();
        assert!(!hostiles.is_empty());
        assert!(hostiles
            .iter()
            .all(|unit| unit.faction.as_deref() == Some(faction)));
    }
}

#[test]
fn deployments_hold_opposite_ends_with_a_broader_hostile_front() {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    for faction in ["brood", "directorate", "ascendants"] {
        let mut mission = data.mission.clone();
        mission.hostile_faction = faction.to_owned();
        let roster = campaign.deployment_roster(&data, &mission);
        let allies = roster
            .iter()
            .filter(|unit| unit.team == Team::Colony)
            .collect::<Vec<_>>();
        let hostiles = roster
            .iter()
            .filter(|unit| unit.team == Team::Hostile)
            .collect::<Vec<_>>();

        let spread = |units: &[&UnitDef]| {
            units
                .iter()
                .enumerate()
                .flat_map(|(index, unit)| {
                    units.iter().skip(index + 1).map(move |other| {
                        (unit.position[0] - other.position[0]).abs()
                            + (unit.position[1] - other.position[1]).abs()
                    })
                })
                .max()
                .unwrap_or(0)
        };
        assert!(allies.iter().all(|unit| unit.position[0] <= 6));
        assert!(hostiles.iter().all(|unit| unit.position[0] >= 32));
        assert!(spread(&allies) <= 4);
        assert!(spread(&hostiles) >= 18);
        assert!(spread(&hostiles) > spread(&allies));
        assert!(allies.iter().all(|ally| hostiles.iter().all(|hostile| {
            (ally.position[0] - hostile.position[0]).abs()
                + (ally.position[1] - hostile.position[1]).abs()
                >= 26
        })));
        assert!(roster.iter().all(|unit| {
            unit.position[0] >= 0
                && unit.position[1] >= 0
                && unit.position[0] < data.config.world_width as i32
                && unit.position[1] < data.config.world_height as i32
        }));
    }
}

#[test]
fn hostile_front_falls_back_around_unsafe_large_world_entry_cells() {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let mut mission = data.mission.clone();
    mission.objective_tile = [35, 10];
    mission.blocked_tiles.push([33, 20]);
    mission.hazards.push(crate::data::HazardDef {
        position: [36, 30],
        kind: crate::data::HazardKind::SporeBloom,
    });

    let deployment = campaign.deployment_roster(&data, &mission);
    let hostiles = deployment
        .iter()
        .filter(|unit| unit.team == Team::Hostile)
        .collect::<Vec<_>>();
    let positions = hostiles
        .iter()
        .map(|unit| unit.position)
        .collect::<std::collections::HashSet<_>>();
    assert_eq!(positions.len(), hostiles.len());
    assert!(hostiles.iter().all(|unit| {
        unit.position[0] >= data.config.world_width as i32 / 2
            && unit.position[0] < data.config.world_width as i32
            && unit.position[1] >= 0
            && unit.position[1] < data.config.world_height as i32
            && unit.position != mission.objective_tile
            && !mission.blocked_tiles.contains(&unit.position)
            && !mission
                .hazards
                .iter()
                .any(|hazard| hazard.position == unit.position)
    }));
}
#[test]
fn three_knives_deploys_one_hostile_from_each_power() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign.strategy.phase_id = "escalation".to_owned();
    campaign.strategy.regenerate_missions(&data);
    let mission_id = campaign
        .strategy
        .mission_offers
        .iter()
        .find(|mission| mission.template_id == "escalation_three_knives")
        .unwrap()
        .id
        .clone();
    campaign.strategy.select_mission(&mission_id).unwrap();
    let mission = campaign
        .strategy
        .materialize_selected(&data, &campaign.colony);
    let hostile_factions = campaign
        .deployment_roster(&data, &mission)
        .into_iter()
        .filter(|unit| unit.team == Team::Hostile)
        .filter_map(|unit| unit.faction)
        .collect::<std::collections::HashSet<_>>();

    assert_eq!(hostile_factions.len(), 3);
    assert!(hostile_factions.contains("directorate"));
    assert!(hostile_factions.contains("brood"));
    assert!(hostile_factions.contains("ascendants"));
}
#[test]
fn squad_selection_enforces_reserves_and_a_three_colonist_limit() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    assert_eq!(campaign.selected_squad_count(), SQUAD_LIMIT);
    assert_eq!(
        campaign
            .deployment_roster(&data, &data.mission)
            .iter()
            .filter(|unit| unit.team == Team::Colony)
            .count(),
        SQUAD_LIMIT
    );
    assert!(campaign.toggle_deployment("sol_cairn").is_err());
    assert!(!campaign.toggle_deployment("kira_voss").unwrap());
    assert!(campaign.toggle_deployment("sol_cairn").unwrap());
    assert_eq!(campaign.selected_squad_count(), SQUAD_LIMIT);
    assert!(campaign
        .deployment_roster(&data, &data.mission)
        .iter()
        .any(|unit| unit.id == "sol_cairn"));
    assert!(!campaign
        .deployment_roster(&data, &data.mission)
        .iter()
        .any(|unit| unit.id == "kira_voss"));
}

#[test]
fn operation_xp_follows_the_deployed_squad() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    let deployed_id = campaign
        .roster
        .iter()
        .find(|character| character.deployment_selected)
        .unwrap()
        .id
        .clone();
    let reserve_id = campaign
        .roster
        .iter()
        .find(|character| !character.deployment_selected)
        .unwrap()
        .id
        .clone();
    let mission = campaign.strategy.selected_mission().unwrap().clone();
    let outcome = MissionOutcome {
        result: ObjectiveState::Victory,
        colonists_deployed: SQUAD_LIMIT,
        colonists_incapacitated: Vec::new(),
        hostiles_neutralised: 2,
        materials_awarded: 7,
        biomass_awarded: 0,
        power_awarded: 0,
    };

    campaign.apply_mission_outcome(&outcome, &mission, &data);

    assert_eq!(
        campaign
            .roster
            .iter()
            .find(|character| character.id == deployed_id)
            .unwrap()
            .experience,
        VICTORY_OPERATION_XP
    );
    assert_eq!(
        campaign
            .roster
            .iter()
            .find(|character| character.id == reserve_id)
            .unwrap()
            .experience,
        0
    );
}

#[test]
fn injury_blocks_deployment_until_operations_recover_it() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    let outcome = MissionOutcome {
        result: ObjectiveState::Failed,
        colonists_deployed: 4,
        colonists_incapacitated: vec![consequence("ilya_reed", "Ilya Reed")],
        hostiles_neutralised: 0,
        materials_awarded: 0,
        biomass_awarded: 0,
        power_awarded: 0,
    };
    let mission = campaign.strategy.selected_mission().unwrap().clone();
    campaign.apply_mission_outcome(&outcome, &mission, &data);
    assert_eq!(campaign.roster[2].availability, Availability::Recovering);
    assert_eq!(campaign.roster[2].traumas.len(), 1);
    assert!(!campaign
        .deployment_roster(&data, &data.mission)
        .iter()
        .any(|unit| unit.id == "ilya_reed"));
    for _ in 0..3 {
        campaign.advance_recovery();
    }
    assert_eq!(campaign.roster[2].availability, Availability::Ready);
    assert_eq!(campaign.roster[2].traumas.len(), 1);
}
#[test]
fn xeno_triage_shortens_new_injury_recovery() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign
        .strategy
        .research
        .iter_mut()
        .find(|research| research.id == "xeno_triage")
        .unwrap()
        .completed = true;
    let outcome = MissionOutcome {
        result: ObjectiveState::Failed,
        colonists_deployed: 4,
        colonists_incapacitated: vec![consequence("ilya_reed", "Ilya Reed")],
        hostiles_neutralised: 0,
        materials_awarded: 0,
        biomass_awarded: 0,
        power_awarded: 0,
    };
    let mission = campaign.strategy.selected_mission().unwrap().clone();
    campaign.apply_mission_outcome(&outcome, &mission, &data);
    assert_eq!(campaign.roster[2].injuries[0].recovery_operations, 2);
}
#[test]
fn character_events_leave_participant_legacies_in_later_deployments() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    let sol_record = campaign
        .roster
        .iter()
        .find(|character| character.id == "sol_cairn")
        .unwrap();
    let sol_base = data
        .roster
        .iter()
        .find(|unit| unit.id == "sol_cairn")
        .unwrap();
    let movement_before = derive_unit(sol_base, sol_record, &data).move_range;
    campaign.strategy.character_events[0].legacy_name.clear();
    campaign.strategy.character_events[0]
        .legacy_character_id
        .clear();
    campaign.strategy.character_events[0].legacy_stat.clear();
    campaign.strategy.character_events[0].legacy_amount = 0;
    campaign.resolve_first_character_event(&data).unwrap();
    let sol_after = campaign
        .roster
        .iter()
        .find(|character| character.id == "sol_cairn")
        .unwrap();
    assert_eq!(sol_after.event_legacies[0].name, "Survey Family Routes");
    assert_eq!(campaign.relationships.len(), 1);
    assert_eq!(campaign.relationships[0].bond, 2);
    assert_eq!(
        derive_unit(sol_base, sol_after, &data).move_range,
        movement_before + 1
    );

    let mara_base = data
        .roster
        .iter()
        .find(|unit| unit.id == "mara_venn")
        .unwrap();
    let mara_before = derive_unit(mara_base, &campaign.roster[1], &data).armour;
    campaign.resolve_first_character_event(&data).unwrap();
    assert_eq!(
        campaign.roster[1].event_legacies[0].name,
        "Documented Carapace"
    );
    assert_eq!(
        derive_unit(mara_base, &campaign.roster[1], &data).armour,
        mara_before + 1
    );
    assert_eq!(campaign.relationships.len(), 2);
}

#[test]
fn character_event_choice_puts_the_legacy_on_the_selected_participant() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    let kira_base = data
        .roster
        .iter()
        .find(|unit| unit.id == "kira_voss")
        .unwrap();
    let movement_before = derive_unit(kira_base, &campaign.roster[0], &data).move_range;
    let food_before = campaign.colony.resources.food;

    assert_eq!(
        campaign.resolve_first_character_event_for("not_a_participant", &data),
        Err("Choose one of the event participants".to_owned())
    );
    assert_eq!(campaign.colony.resources.food, food_before);

    campaign
        .resolve_first_character_event_for("kira_voss", &data)
        .unwrap();

    let kira = campaign
        .roster
        .iter()
        .find(|character| character.id == "kira_voss")
        .unwrap();
    let sol = campaign
        .roster
        .iter()
        .find(|character| character.id == "sol_cairn")
        .unwrap();
    assert_eq!(kira.event_legacies[0].name, "Survey Family Routes");
    assert!(sol.event_legacies.is_empty());
    assert_eq!(
        derive_unit(kira_base, kira, &data).move_range,
        movement_before + 1
    );
    assert_eq!(campaign.colony.resources.food, food_before - 2);
}

#[test]
fn colony_conversations_remember_each_heard_story_beat() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);

    campaign.acknowledge_colonist("mara_venn");
    campaign.acknowledge_colonist("mara_venn");
    assert!(campaign.colony_story.has_heard("mara_arrival"));

    campaign.operations_completed = 1;
    campaign.first_hour.first_outcome_won = Some(true);
    campaign.acknowledge_colonist("mara_venn");
    assert!(campaign.colony_story.has_heard("mara_first_victory"));
}

#[test]
fn identity_conversations_archive_the_current_path_beat() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign.strategy.mirexis_path_id = "human_redoubt".to_owned();
    campaign.strategy.campaign_complete = true;
    campaign
        .colony
        .ensure_identity_building("human_redoubt")
        .unwrap();

    campaign.acknowledge_colonist("mara_venn");

    assert!(campaign.colony_story.has_heard("identity_redoubt_ending"));

    campaign.acknowledge_colonist("mara_venn");

    assert!(campaign.colony_story.has_heard("post_ending_redoubt_mara"));

    campaign.strategy.post_campaign_operations_completed = 1;
    campaign.acknowledge_colonist("mara_venn");

    assert!(campaign
        .colony_story
        .has_heard("epilogue_operation_redoubt_mara"));

    campaign.acknowledge_colonist("ilya_reed");

    assert!(campaign.colony_story.has_heard("finale_redoubt_ilya"));
}

#[test]
fn successful_operations_apply_all_recovered_resources() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    let before = campaign.colony.resources.clone();
    let outcome = MissionOutcome {
        result: ObjectiveState::Victory,
        colonists_deployed: 3,
        colonists_incapacitated: Vec::new(),
        hostiles_neutralised: 2,
        materials_awarded: 7,
        biomass_awarded: 5,
        power_awarded: 3,
    };
    let mission = campaign.strategy.selected_mission().unwrap().clone();
    campaign.apply_mission_outcome(&outcome, &mission, &data);
    assert_eq!(campaign.colony.resources.materials, before.materials + 7);
    assert_eq!(campaign.colony.resources.biomass, before.biomass + 5);
    assert_eq!(campaign.colony.resources.power, before.power + 3);
    assert_eq!(campaign.relationships.len(), 3);
    assert!(campaign
        .relationships
        .iter()
        .all(|relationship| relationship.bond == 1 && relationship.shared_victories == 1));
}
#[test]
fn failed_colony_defense_damages_a_saved_facility() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    let mut mission = campaign.strategy.selected_mission().unwrap().clone();
    mission.map_recipe = "colony_defense".to_owned();
    let outcome = MissionOutcome {
        result: ObjectiveState::Failed,
        colonists_deployed: 3,
        colonists_incapacitated: Vec::new(),
        hostiles_neutralised: 0,
        materials_awarded: 0,
        biomass_awarded: 0,
        power_awarded: 0,
    };
    campaign.apply_mission_outcome(&outcome, &mission, &data);
    assert!(campaign.relationships.is_empty());
    assert!(campaign
        .colony
        .buildings
        .iter()
        .any(|building| building.damaged));
}
#[test]
fn deployment_rations_include_mutation_upkeep_and_hydroponics_recovery() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign.roster[0].mutation_id = "symbiotic_organism".to_owned();
    let starting_food = campaign.colony.resources.food;
    assert_eq!(campaign.deployment_food_cost(&data), 4);
    assert_eq!(campaign.prepare_deployment(&data).unwrap(), 4);
    let outcome = MissionOutcome {
        result: ObjectiveState::Victory,
        colonists_deployed: 3,
        colonists_incapacitated: Vec::new(),
        hostiles_neutralised: 3,
        materials_awarded: 0,
        biomass_awarded: 0,
        power_awarded: 0,
    };
    let mission = campaign.strategy.selected_mission().unwrap().clone();
    campaign.apply_mission_outcome(&outcome, &mission, &data);
    assert_eq!(campaign.colony.resources.food, starting_food - 1);
}
#[test]
fn facilities_gate_training_treatment_and_crafting() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    let cost = campaign
        .train_character("kira_voss", "soldier", &data)
        .unwrap();
    assert!(cost >= 60);
    assert_eq!(campaign.roster[0].active_class, "soldier");
    campaign
        .craft_equipment("kira_voss", "chitin_plate", &data)
        .unwrap();
    assert!(campaign.roster[0]
        .equipment_ids
        .iter()
        .any(|id| id == "chitin_plate"));
    campaign.colony.resources.materials += 30;
    campaign
        .craft_equipment("kira_voss", "service_pistol", &data)
        .unwrap();
    assert!(campaign.roster[0]
        .equipment_ids
        .iter()
        .any(|id| id == "service_pistol"));
    assert!(!campaign.roster[0]
        .equipment_ids
        .iter()
        .any(|id| id == "frontier_rifle"));
}
#[test]
fn contact_prototypes_require_the_matching_completed_trace() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    assert!(campaign
        .craft_equipment("kira_voss", "directorate_smartlink", &data)
        .is_err());
    campaign.strategy.isolation_victories = 3;
    campaign.strategy.first_assault_repulsed = true;
    campaign.strategy.research[0].completed = true;
    campaign
        .strategy
        .refresh_isolation_completion(&mut campaign.colony);
    campaign
        .strategy
        .choose_contact_protocol("directorate_requisition", &mut campaign.colony, &data)
        .unwrap();
    assert!(campaign
        .craft_equipment("kira_voss", "directorate_smartlink", &data)
        .is_err());
    campaign.strategy.contact_trace_completed = true;
    campaign
        .craft_equipment("kira_voss", "directorate_smartlink", &data)
        .unwrap();
    assert!(campaign.roster[0]
        .equipment_ids
        .iter()
        .any(|id| id == "directorate_smartlink"));
    assert!(campaign
        .craft_equipment("kira_voss", "brood_living_plate", &data)
        .is_err());
}
#[test]
fn contact_aftermath_event_changes_its_faction_and_character() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign.strategy.isolation_victories = 3;
    campaign.strategy.first_assault_repulsed = true;
    campaign.strategy.research[0].completed = true;
    campaign
        .strategy
        .refresh_isolation_completion(&mut campaign.colony);
    campaign
        .strategy
        .choose_contact_protocol("ascendant_capacitor", &mut campaign.colony, &data)
        .unwrap();
    campaign.strategy.contact_trace_completed = true;
    campaign.resolve_first_character_event(&data).unwrap();
    campaign.resolve_first_character_event(&data).unwrap();
    let attention_before = campaign
        .strategy
        .factions
        .iter()
        .find(|faction| faction.id == "ascendants")
        .unwrap()
        .attention;
    assert_eq!(
        campaign.resolve_first_character_event(&data).unwrap(),
        "The Light Between Seconds"
    );
    assert_eq!(
        campaign
            .strategy
            .factions
            .iter()
            .find(|faction| faction.id == "ascendants")
            .unwrap()
            .attention,
        (attention_before - 5).max(0)
    );
    assert!(campaign
        .roster
        .iter()
        .find(|character| character.id == "sol_cairn")
        .unwrap()
        .event_legacies
        .iter()
        .any(|legacy| legacy.id == "ascendant_contact_aftermath"));
    campaign
        .craft_equipment("sol_cairn", "ascendant_phase_lens", &data)
        .unwrap();
    assert!(campaign.strategy.contact_complete);
    assert_eq!(campaign.strategy.phase_id, "adaptation");
}

#[test]
fn phase_conversations_archive_the_current_phase_beat() {
    let data = GameData::load().unwrap();
    for (phase_id, character_id, beat_id) in [
        ("adaptation", "mara_venn", "phase_adaptation_mara"),
        ("adaptation", "ilya_reed", "phase_adaptation_ilya"),
        ("adaptation", "sol_cairn", "phase_adaptation_sol"),
        ("adaptation", "nadi_vale", "phase_adaptation_nadi"),
        ("escalation", "mara_venn", "phase_escalation_mara"),
        ("escalation", "ilya_reed", "phase_escalation_ilya"),
        ("escalation", "sol_cairn", "phase_escalation_sol"),
        ("escalation", "nadi_vale", "phase_escalation_nadi"),
    ] {
        let mut campaign = CampaignState::new(&data);
        campaign.strategy.phase_id = phase_id.to_owned();
        campaign.acknowledge_colonist(character_id);
        assert!(campaign.colony_story.has_heard(beat_id));
    }
}

#[test]
fn contact_route_conversations_archive_each_stage_for_the_chosen_protocol() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign.strategy.contact_protocol_id = "brood_cultivation".to_owned();

    campaign.acknowledge_colonist("nadi_vale");
    assert!(campaign.colony_story.has_heard("contact_brood_witness"));

    campaign.strategy.contact_trace_completed = true;
    campaign.acknowledge_colonist("ilya_reed");
    assert!(campaign
        .colony_story
        .has_heard("contact_brood_contradiction"));

    campaign.strategy.contact_complete = true;
    campaign.acknowledge_colonist("mara_venn");
    assert!(campaign.colony_story.has_heard("contact_brood_aftermath"));
}
