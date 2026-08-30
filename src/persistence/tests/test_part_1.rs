use super::*;
use crate::data::Team;

#[test]
fn phase_one_save_gains_campaign_and_character_runtime_fields() {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let session = GameSession::new(
        &data.config,
        &data.mission,
        &campaign.deployment_roster(&data, &data.mission),
    );
    let mut legacy = serde_json::to_value(session.to_save("0.2.0", &campaign)).unwrap();
    legacy.as_object_mut().unwrap().remove("campaign");
    for unit in legacy["tactical"]["units"].as_array_mut().unwrap() {
        unit.as_object_mut().unwrap().remove("round_regeneration");
    }
    let migrated = migrate_save_value(Some("0.2.0".to_owned()), legacy, &data).unwrap();
    assert_eq!(migrated.version, data.config.version);
    assert_eq!(migrated.campaign.roster.len(), 5);
    assert!(migrated.tactical.is_some());
}
#[test]
fn character_save_gains_colony_without_losing_progress() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign.roster[0].experience = 44;
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let mut legacy = serde_json::to_value(session.to_save("0.3.0", &campaign)).unwrap();
    legacy["campaign"].as_object_mut().unwrap().remove("colony");
    legacy["campaign"]
        .as_object_mut()
        .unwrap()
        .remove("strategy");
    let migrated = migrate_save_value(Some("0.3.0".to_owned()), legacy, &data).unwrap();
    assert_eq!(migrated.campaign.roster[0].experience, 44);
    assert!(migrated
        .campaign
        .colony
        .has_facility(crate::colony::BuildingKind::Workshop));
    assert_eq!(migrated.campaign.strategy.phase_id, "isolation");
}
#[test]
fn colony_save_gains_isolation_strategy() {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let mut legacy = serde_json::to_value(session.to_save("0.4.0", &campaign)).unwrap();
    legacy["campaign"]
        .as_object_mut()
        .unwrap()
        .remove("strategy");
    let migrated = migrate_save_value(Some("0.4.0".to_owned()), legacy, &data).unwrap();
    assert_eq!(migrated.version, data.config.version);
    assert_eq!(migrated.campaign.strategy.factions.len(), 3);
}
#[test]
fn isolation_save_gains_tactical_mutation_runtime_fields() {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let mut legacy = serde_json::to_value(session.to_save("0.5.0", &campaign)).unwrap();
    for unit in legacy["tactical"]["units"].as_array_mut().unwrap() {
        let unit = unit.as_object_mut().unwrap();
        for key in [
            "mutation_gift_used",
            "temporary_armour",
            "temporary_accuracy",
            "temporary_move_range",
            "temporary_weapon_damage",
        ] {
            unit.remove(key);
        }
    }
    let migrated = migrate_save_value(Some("0.5.0".to_owned()), legacy, &data).unwrap();
    let tactical = migrated.tactical.as_ref().unwrap();
    assert_eq!(migrated.version, data.config.version);
    assert!(!tactical.units[0].mutation_gift_used);
    assert_eq!(tactical.units[0].temporary_armour, 0);
}
#[test]
fn mutation_save_gains_typed_objectives() {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let mut legacy = serde_json::to_value(session.to_save("0.6.0", &campaign)).unwrap();
    legacy["tactical"]
        .as_object_mut()
        .unwrap()
        .remove("objective_kind");
    for mission in legacy["campaign"]["strategy"]["mission_offers"]
        .as_array_mut()
        .unwrap()
    {
        mission.as_object_mut().unwrap().remove("objective_kind");
    }
    let migrated = migrate_save_value(Some("0.6.0".to_owned()), legacy, &data).unwrap();
    assert_eq!(migrated.version, data.config.version);
    assert_eq!(
        migrated.tactical.unwrap().objective_kind,
        crate::data::ObjectiveKind::SecureAndClear
    );
    assert_eq!(
        migrated.campaign.strategy.mission_offers[0].objective_kind,
        crate::data::ObjectiveKind::SecureAndClear
    );
}
#[test]
fn faction_save_gains_class_actions_and_statuses() {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let mut legacy = serde_json::to_value(session.to_save("0.8.0", &campaign)).unwrap();
    for unit in legacy["tactical"]["units"].as_array_mut().unwrap() {
        let unit = unit.as_object_mut().unwrap();
        unit.remove("class_id");
        unit.remove("class_action_used");
        unit.remove("statuses");
    }
    let migrated = migrate_save_value(Some("0.8.0".to_owned()), legacy, &data).unwrap();
    assert_eq!(migrated.version, data.config.version);
    let kira = migrated
        .tactical
        .unwrap()
        .units
        .into_iter()
        .find(|unit| unit.id == "kira_voss")
        .unwrap();
    assert_eq!(kira.class_id, "scout");
    assert!(!kira.class_action_used);
    assert!(kira.statuses.is_empty());
}

#[test]
fn version_170_save_gains_the_remaining_hybrid_class_ids() {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let mut legacy = serde_json::to_value(session.to_save("1.70.0", &campaign)).unwrap();
    for (id, role) in [
        ("mara_venn", "Rescue Specialist"),
        ("kira_voss", "Chorus Warden"),
    ] {
        let unit = legacy["tactical"]["units"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .find(|unit| unit["id"] == id)
            .unwrap()
            .as_object_mut()
            .unwrap();
        unit.insert("role".to_owned(), serde_json::json!(role));
        unit.remove("class_id");
    }

    let migrated = migrate_save_value(Some("1.70.0".to_owned()), legacy, &data).unwrap();
    assert_eq!(migrated.version, data.config.version);
    let units = migrated.tactical.unwrap().units;
    assert_eq!(
        units
            .iter()
            .find(|unit| unit.id == "mara_venn")
            .unwrap()
            .class_id,
        "rescue_specialist"
    );
    assert_eq!(
        units
            .iter()
            .find(|unit| unit.id == "kira_voss")
            .unwrap()
            .class_id,
        "chorus_warden"
    );
}

#[test]
fn tactical_save_gains_technique_runtime_fields() {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let session = GameSession::new(
        &data.config,
        &data.mission,
        &campaign.deployment_roster(&data, &data.mission),
    );
    let mut legacy = serde_json::to_value(session.to_save("1.66.0", &campaign)).unwrap();
    legacy["tactical"]
        .as_object_mut()
        .unwrap()
        .remove("obscuring_fields");
    for unit in legacy["tactical"]["units"].as_array_mut().unwrap() {
        let unit = unit.as_object_mut().unwrap();
        for key in [
            "learned_skills",
            "active_skills",
            "used_skill_ids",
            "next_attack_ignores_armour",
            "next_equipment_overcharged",
            "hazard_resistance",
        ] {
            unit.remove(key);
        }
    }
    let migrated = migrate_save_value(Some("1.66.0".to_owned()), legacy, &data).unwrap();
    let tactical = migrated.tactical.unwrap();
    assert!(tactical.obscuring_fields.is_empty());
    let colonist = tactical
        .units
        .into_iter()
        .find(|unit| unit.team == Team::Colony)
        .unwrap();
    assert!(colonist
        .learned_skills
        .iter()
        .any(|skill| skill.ends_with("_fundamentals")));
    assert!(colonist
        .active_skills
        .iter()
        .any(|skill| skill.ends_with("_fundamentals")));
    assert!(colonist.used_skill_ids.is_empty());
    assert!(!colonist.next_attack_ignores_armour);
    assert!(!colonist.next_equipment_overcharged);
    assert!(colonist.hazard_resistance.is_none());
}
#[test]
fn class_action_save_gains_reinforcement_queue() {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let mut legacy = serde_json::to_value(session.to_save("0.9.0", &campaign)).unwrap();
    legacy["tactical"]
        .as_object_mut()
        .unwrap()
        .remove("reinforcement_waves");
    let migrated = migrate_save_value(Some("0.9.0".to_owned()), legacy, &data).unwrap();
    assert_eq!(migrated.version, data.config.version);
    assert!(migrated.tactical.unwrap().reinforcement_waves.is_empty());
}
#[test]
fn reinforcement_save_gains_a_bounded_deployment_squad() {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let mut legacy = serde_json::to_value(session.to_save("1.0.0", &campaign)).unwrap();
    for character in legacy["campaign"]["roster"].as_array_mut().unwrap() {
        character
            .as_object_mut()
            .unwrap()
            .remove("deployment_selected");
    }
    let migrated = migrate_save_value(Some("1.0.0".to_owned()), legacy, &data).unwrap();
    assert_eq!(migrated.version, data.config.version);
    assert_eq!(
        migrated
            .campaign
            .roster
            .iter()
            .filter(|character| character.deployment_selected)
            .count(),
        SQUAD_LIMIT
    );
}
#[test]
fn squad_save_gains_a_selected_roster_character() {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let mut legacy = serde_json::to_value(session.to_save("1.1.0", &campaign)).unwrap();
    legacy["campaign"]
        .as_object_mut()
        .unwrap()
        .remove("selected_character_id");
    let migrated = migrate_save_value(Some("1.1.0".to_owned()), legacy, &data).unwrap();
    assert_eq!(migrated.version, data.config.version);
    assert_eq!(migrated.campaign.selected_character_id, "kira_voss");
}
#[test]
fn roster_save_gains_tactical_equipment_actions() {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let roster = campaign.deployment_roster(&data, &data.mission);
    let session = GameSession::new(&data.config, &data.mission, &roster);
    let mut legacy = serde_json::to_value(session.to_save("1.2.0", &campaign)).unwrap();
    for unit in legacy["tactical"]["units"].as_array_mut().unwrap() {
        let unit = unit.as_object_mut().unwrap();
        unit.remove("equipment_ids");
        unit.remove("used_equipment_ids");
    }
    let migrated = migrate_save_value(Some("1.2.0".to_owned()), legacy, &data).unwrap();
    assert_eq!(migrated.version, data.config.version);
    let kira = migrated
        .tactical
        .unwrap()
        .units
        .into_iter()
        .find(|unit| unit.id == "kira_voss")
        .unwrap();
    assert!(kira.equipment_ids.contains(&"survey_harness".to_owned()));
    assert!(kira.used_equipment_ids.is_empty());
}
#[test]
fn equipment_save_gains_destructible_cover_integrity() {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let roster = campaign.deployment_roster(&data, &data.mission);
    let session = GameSession::new(&data.config, &data.mission, &roster);
    let mut legacy = serde_json::to_value(session.to_save("1.3.0", &campaign)).unwrap();
    legacy["tactical"]
        .as_object_mut()
        .unwrap()
        .remove("destructible_cover");
    let migrated = migrate_save_value(Some("1.3.0".to_owned()), legacy, &data).unwrap();
    assert_eq!(migrated.version, data.config.version);
    let tactical = migrated.tactical.unwrap();
    assert_eq!(tactical.destructible_cover.len(), tactical.blocked.len());
    assert!(tactical
        .destructible_cover
        .iter()
        .all(|cover| cover.health == 6 && cover.max_health == 6));
}
#[test]
fn doctrine_save_gains_sustainable_colony_infrastructure() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign.colony.buildings.retain(|building| {
        !matches!(
            building.kind,
            crate::colony::BuildingKind::Hydroponics | crate::colony::BuildingKind::PowerPlant
        )
    });
    campaign.colony.resources.power = 8;
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let mut legacy = serde_json::to_value(session.to_save("1.4.0", &campaign)).unwrap();
    legacy["campaign"]["colony"]
        .as_object_mut()
        .unwrap()
        .remove("planned_construction");
    let migrated = migrate_save_value(Some("1.4.0".to_owned()), legacy, &data).unwrap();
    assert_eq!(migrated.version, data.config.version);
    assert!(migrated
        .campaign
        .colony
        .has_facility(crate::colony::BuildingKind::Hydroponics));
    assert!(migrated
        .campaign
        .colony
        .has_facility(crate::colony::BuildingKind::PowerPlant));
    assert_eq!(migrated.campaign.colony.resources.power, 4);
    assert_eq!(migrated.campaign.colony.power_supply(), 8);
}
#[test]
fn economy_save_recovers_isolation_phase_progress() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign.operations_completed = 4;
    campaign.strategy.research[0].completed = true;
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let mut legacy = serde_json::to_value(session.to_save("1.5.0", &campaign)).unwrap();
    let strategy = legacy["campaign"]["strategy"].as_object_mut().unwrap();
    strategy.remove("isolation_victories");
    strategy.remove("first_assault_repulsed");
    strategy.remove("isolation_complete");
    let migrated = migrate_save_value(Some("1.5.0".to_owned()), legacy, &data).unwrap();
    assert_eq!(migrated.version, data.config.version);
    assert!(migrated.campaign.strategy.isolation_complete);
    assert_eq!(migrated.campaign.strategy.phase_id, "contact");
    assert_eq!(migrated.campaign.colony.resources.alien_components, 2);
}
#[test]
fn contact_save_gains_an_uncommitted_protocol_choice() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign.strategy.isolation_victories = 3;
    campaign.strategy.first_assault_repulsed = true;
    campaign.strategy.research[0].completed = true;
    campaign
        .strategy
        .refresh_isolation_completion(&mut campaign.colony);
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let mut legacy = serde_json::to_value(session.to_save("1.6.0", &campaign)).unwrap();
    legacy["campaign"]["strategy"]
        .as_object_mut()
        .unwrap()
        .remove("contact_protocol_id");
    let migrated = migrate_save_value(Some("1.6.0".to_owned()), legacy, &data).unwrap();
    assert_eq!(migrated.version, data.config.version);
    assert!(migrated.campaign.strategy.contact_protocol_id.is_empty());
    assert_eq!(migrated.campaign.colony.resources.alien_components, 2);
}
#[test]
fn protocol_save_preserves_its_contact_operation() {
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
        .choose_contact_protocol("brood_cultivation", &mut campaign.colony, &data)
        .unwrap();
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let legacy = serde_json::to_value(session.to_save("1.7.0", &campaign)).unwrap();
    let migrated = migrate_save_value(Some("1.7.0".to_owned()), legacy, &data).unwrap();
    assert_eq!(migrated.version, data.config.version);
    assert_eq!(
        migrated.campaign.strategy.contact_protocol_id,
        "brood_cultivation"
    );
    assert_eq!(
        migrated
            .campaign
            .strategy
            .selected_mission()
            .unwrap()
            .template_id,
        "brood_contact_trace"
    );
}
#[test]
fn signal_trace_save_gains_an_incomplete_aftermath() {
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
        .choose_contact_protocol("directorate_requisition", &mut campaign.colony, &data)
        .unwrap();
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let mut legacy = serde_json::to_value(session.to_save("1.8.0", &campaign)).unwrap();
    legacy["campaign"]["strategy"]
        .as_object_mut()
        .unwrap()
        .remove("contact_trace_completed");
    let migrated = migrate_save_value(Some("1.8.0".to_owned()), legacy, &data).unwrap();
    assert_eq!(migrated.version, data.config.version);
    assert!(!migrated.campaign.strategy.contact_trace_completed);
}
#[test]
fn prototype_save_gains_protocol_aftermath_events() {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let mut legacy = serde_json::to_value(session.to_save("1.9.0", &campaign)).unwrap();
    legacy["campaign"]["strategy"]["character_events"]
        .as_array_mut()
        .unwrap()
        .retain(|event| {
            event["required_protocol"]
                .as_str()
                .is_none_or(str::is_empty)
        });
    let migrated = migrate_save_value(Some("1.9.0".to_owned()), legacy, &data).unwrap();
    assert_eq!(migrated.version, data.config.version);
    assert_eq!(
        migrated.campaign.strategy.character_events.len(),
        data.campaign.events.len()
    );
    assert!(migrated
        .campaign
        .strategy
        .character_events
        .iter()
        .any(|event| event.id == "brood_contact_aftermath"));
}
#[test]
fn aftermath_save_recovers_adaptation_completion() {
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
        .choose_contact_protocol("directorate_requisition", &mut campaign.colony, &data)
        .unwrap();
    campaign.strategy.contact_trace_completed = true;
    campaign.resolve_first_character_event(&data).unwrap();
    campaign.resolve_first_character_event(&data).unwrap();
    campaign.resolve_first_character_event(&data).unwrap();
    campaign
        .craft_equipment("kira_voss", "directorate_smartlink", &data)
        .unwrap();
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let mut legacy = serde_json::to_value(session.to_save("1.10.0", &campaign)).unwrap();
    legacy["campaign"]["strategy"]
        .as_object_mut()
        .unwrap()
        .remove("contact_complete");
    let migrated = migrate_save_value(Some("1.10.0".to_owned()), legacy, &data).unwrap();
    assert_eq!(migrated.version, data.config.version);
    assert!(migrated.campaign.strategy.contact_complete);
    assert_eq!(migrated.campaign.strategy.phase_id, "adaptation");
}
#[test]
fn adaptation_save_gains_unevolved_character_mutations() {
    let data = GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let mut legacy = serde_json::to_value(session.to_save("1.11.0", &campaign)).unwrap();
    for character in legacy["campaign"]["roster"].as_array_mut().unwrap() {
        character
            .as_object_mut()
            .unwrap()
            .remove("mutation_evolution_id");
    }
    let migrated = migrate_save_value(Some("1.11.0".to_owned()), legacy, &data).unwrap();
    assert_eq!(migrated.version, data.config.version);
    assert!(migrated
        .campaign
        .roster
        .iter()
        .all(|character| character.mutation_evolution_id.is_empty()));
}
#[test]
fn evolution_save_gains_the_adaptation_operation() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign.strategy.contact_complete = true;
    campaign.strategy.phase_id = "adaptation".to_owned();
    campaign.roster[0].mutation_evolution_id = "expanded_cortex".to_owned();
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let legacy = serde_json::to_value(session.to_save("1.12.0", &campaign)).unwrap();
    let migrated = migrate_save_value(Some("1.12.0".to_owned()), legacy, &data).unwrap();
    assert_eq!(migrated.version, data.config.version);
    assert!(migrated
        .campaign
        .strategy
        .mission_offers
        .iter()
        .any(|mission| mission.template_id == "adaptation_glass_nerve"));
}
#[test]
fn adaptation_operation_save_gains_an_existing_gene_lab() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign.strategy.contact_complete = true;
    campaign.strategy.phase_id = "adaptation".to_owned();
    campaign.roster[0].mutation_evolution_id = "expanded_cortex".to_owned();
    let session = GameSession::new(&data.config, &data.mission, &data.roster);
    let legacy = serde_json::to_value(session.to_save("1.13.0", &campaign)).unwrap();
    let migrated = migrate_save_value(Some("1.13.0".to_owned()), legacy, &data).unwrap();
    assert_eq!(migrated.version, data.config.version);
    assert!(migrated
        .campaign
        .colony
        .buildings
        .iter()
        .any(|building| building.kind == crate::colony::BuildingKind::GeneLab));
}
