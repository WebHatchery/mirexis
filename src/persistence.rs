//! Campaign save migration at schema boundaries.

use crate::campaign::{CampaignState, SQUAD_LIMIT};
use crate::colony::ColonyState;
use crate::data::{GameConfig, GameData};
use crate::state::SaveData;
use crate::strategy::StrategyState;
use crate::tactical::DestructibleCover;
use macroquad_toolkit::rng::SeededRng;
use serde_json::Value;

pub fn migrate_save_value(
    detected_version: Option<String>,
    value: Value,
    data: &GameData,
) -> Result<SaveData, String> {
    let mut payload = value.get("data").cloned().unwrap_or(value);
    if detected_version.as_deref() == Some("0.1.0") {
        migrate_phase_zero_payload(&mut payload, &data.config)?;
    }
    if matches!(detected_version.as_deref(), Some("0.1.0" | "0.2.0")) {
        let root = payload
            .as_object_mut()
            .ok_or_else(|| "Legacy save root is not an object".to_owned())?;
        root.insert(
            "campaign".to_owned(),
            serde_json::to_value(CampaignState::new(data))
                .map_err(|err| format!("Could not create migrated campaign: {}", err))?,
        );
        add_character_runtime_defaults(&mut payload)?;
    }
    if matches!(detected_version.as_deref(), Some("0.3.0" | "0.4.0")) {
        let campaign = payload
            .get_mut("campaign")
            .and_then(Value::as_object_mut)
            .ok_or_else(|| "Character save is missing campaign state".to_owned())?;
        campaign.entry("colony".to_owned()).or_insert(
            serde_json::to_value(ColonyState::new())
                .map_err(|err| format!("Could not create migrated colony: {}", err))?,
        );
        campaign.entry("strategy".to_owned()).or_insert(
            serde_json::to_value(StrategyState::new(data))
                .map_err(|err| format!("Could not create migrated strategy: {}", err))?,
        );
    }
    add_class_action_defaults(&mut payload)?;
    let mut save = serde_json::from_value::<SaveData>(payload)
        .map_err(|err| format!("Unsupported Mirexis save {:?}: {}", detected_version, err))?;
    save.campaign.ensure_roster_characters(data);
    save.campaign.strategy.ensure_character_events(data);
    save.campaign.colony.ensure_phase_one_infrastructure(
        detected_version.as_deref() != Some(data.config.version.as_str()),
    );
    save.campaign.refresh_contact_completion(data);
    if detected_version.as_deref() == Some("1.12.0")
        && save.campaign.strategy.contact_complete
        && save
            .campaign
            .roster
            .iter()
            .any(|character| !character.mutation_evolution_id.is_empty())
    {
        save.campaign.strategy.regenerate_missions(data);
    }
    if detected_version.as_deref() == Some("1.13.0")
        && save
            .campaign
            .roster
            .iter()
            .any(|character| !character.mutation_evolution_id.is_empty())
    {
        save.campaign.colony.ensure_gene_lab();
    }
    if detected_version.as_deref() == Some("1.18.0") && save.campaign.strategy.adaptation_complete {
        save.campaign.strategy.regenerate_missions(data);
    }
    if detected_version.as_deref() == Some("1.20.0")
        && !save.campaign.strategy.escalation_response_id.is_empty()
    {
        save.campaign.strategy.regenerate_missions(data);
    }
    if detected_version.as_deref() == Some("1.21.0")
        && save.campaign.strategy.phase_id == "escalation"
    {
        save.campaign.strategy.regenerate_missions(data);
    }
    if detected_version.as_deref() == Some("1.24.0")
        && !save.campaign.strategy.mirexis_path_id.is_empty()
    {
        save.campaign.strategy.regenerate_missions(data);
    }
    if detected_version.as_deref() != Some(data.config.version.as_str())
        && !save.campaign.strategy.isolation_complete
    {
        save.campaign.strategy.isolation_victories =
            save.campaign.operations_completed.min(3) as u8;
        save.campaign.strategy.first_assault_repulsed = save.campaign.operations_completed >= 4;
        save.campaign
            .strategy
            .refresh_isolation_completion(&mut save.campaign.colony);
    }
    let mut selected = 0;
    for character in &mut save.campaign.roster {
        if character.deployment_selected {
            if selected < SQUAD_LIMIT {
                selected += 1;
            } else {
                character.deployment_selected = false;
            }
        }
    }
    if !save
        .campaign
        .roster
        .iter()
        .any(|character| character.id == save.campaign.selected_character_id)
    {
        save.campaign.selected_character_id = save
            .campaign
            .roster
            .first()
            .map(|character| character.id.clone())
            .unwrap_or_default();
    }
    if detected_version.as_deref() != Some(data.config.version.as_str()) {
        if let Some(tactical) = &mut save.tactical {
            for unit in &mut tactical.units {
                if unit.equipment_ids.is_empty() {
                    if let Some(character) = save
                        .campaign
                        .roster
                        .iter()
                        .find(|character| character.id == unit.id)
                    {
                        unit.equipment_ids = character.equipment_ids.clone();
                    }
                }
            }
            if tactical.destructible_cover.is_empty() {
                tactical.destructible_cover = tactical
                    .blocked
                    .iter()
                    .copied()
                    .map(|position| DestructibleCover {
                        position,
                        health: 6,
                        max_health: 6,
                    })
                    .collect();
                tactical
                    .destructible_cover
                    .sort_by_key(|cover| (cover.position.y, cover.position.x));
            }
        }
    }
    save.version = data.config.version.clone();
    Ok(save)
}

fn add_class_action_defaults(value: &mut Value) -> Result<(), String> {
    let Some(units) = value
        .get_mut("tactical")
        .and_then(Value::as_object_mut)
        .and_then(|tactical| tactical.get_mut("units"))
        .and_then(Value::as_array_mut)
    else {
        return Ok(());
    };
    for unit in units {
        let unit = unit
            .as_object_mut()
            .ok_or_else(|| "Tactical unit is not an object".to_owned())?;
        let class_id = match unit.get("role").and_then(Value::as_str).unwrap_or("") {
            "Soldier" => "soldier",
            "Defender" => "defender",
            "Scout" => "scout",
            "Medic" => "medic",
            "Engineer" => "engineer",
            "Psionic" => "psionic",
            "Biotech Specialist" => "biotech",
            "Vanguard" => "vanguard",
            "Pathfinder" => "pathfinder",
            "Lifewright" => "lifewright",
            "Null Adept" => "null_adept",
            _ => "",
        };
        unit.entry("class_id".to_owned())
            .or_insert_with(|| serde_json::json!(class_id));
        unit.entry("class_action_used".to_owned())
            .or_insert_with(|| serde_json::json!(false));
        unit.entry("statuses".to_owned())
            .or_insert_with(|| serde_json::json!([]));
    }
    Ok(())
}

fn add_character_runtime_defaults(value: &mut Value) -> Result<(), String> {
    let units = value
        .get_mut("tactical")
        .and_then(|tactical| tactical.get_mut("units"))
        .and_then(Value::as_array_mut)
        .ok_or_else(|| "Legacy save is missing tactical units".to_owned())?;
    for unit in units {
        if let Some(unit) = unit.as_object_mut() {
            unit.entry("round_regeneration".to_owned())
                .or_insert_with(|| serde_json::json!(0));
        }
    }
    Ok(())
}

fn migrate_phase_zero_payload(value: &mut Value, config: &GameConfig) -> Result<(), String> {
    let tactical = value
        .get_mut("tactical")
        .and_then(Value::as_object_mut)
        .ok_or_else(|| "Phase 0 save is missing tactical state".to_owned())?;
    tactical.insert("terrain_costs".to_owned(), serde_json::json!([]));
    tactical.insert("cover_edges".to_owned(), serde_json::json!([]));
    tactical.insert(
        "objective_tile".to_owned(),
        serde_json::json!({ "x": 10, "y": 4 }),
    );
    tactical.insert("objective_state".to_owned(), serde_json::json!("Active"));
    tactical.insert("round_limit".to_owned(), serde_json::json!(8));
    tactical.insert(
        "rng".to_owned(),
        serde_json::to_value(SeededRng::new(config.battle_seed))
            .map_err(|err| format!("Could not seed migrated battle: {}", err))?,
    );
    tactical.insert("event_log".to_owned(), serde_json::json!([]));
    let units = tactical
        .get_mut("units")
        .and_then(Value::as_array_mut)
        .ok_or_else(|| "Phase 0 save is missing tactical units".to_owned())?;
    for unit in units {
        let Some(unit) = unit.as_object_mut() else {
            continue;
        };
        let hostile = unit.get("team").and_then(Value::as_str) == Some("hostile");
        unit.insert(
            "armour".to_owned(),
            serde_json::json!(if hostile { 1 } else { 0 }),
        );
        unit.insert(
            "accuracy".to_owned(),
            serde_json::json!(if hostile { 60 } else { 68 }),
        );
        unit.insert(
            "weapon_range".to_owned(),
            serde_json::json!(if hostile { 1 } else { 5 }),
        );
        unit.insert("weapon_damage".to_owned(), serde_json::json!(4));
        unit.insert("weapon_ap_cost".to_owned(), serde_json::json!(2));
        unit.insert("round_regeneration".to_owned(), serde_json::json!(0));
        unit.insert("incapacitated".to_owned(), serde_json::json!(false));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::GameSession;

    #[test]
    fn phase_one_save_gains_campaign_and_character_runtime_fields() {
        let data = GameData::load().unwrap();
        let campaign = CampaignState::new(&data);
        let session = GameSession::new(&data.config, &data.mission, &data.roster);
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
}
