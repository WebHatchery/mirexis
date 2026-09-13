//! Focused test_part_1 integration tests.

use super::*;

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
    assert_eq!(campaign.lost_objectives.len(), 1);
    assert_eq!(campaign.lost_objectives[0].operation, 1);
    assert_eq!(campaign.lost_objectives[0].objective, mission.objective);
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
