//! Campaign save migration at schema boundaries.

use crate::campaign::CampaignState;
use crate::colony::ColonyState;
use crate::data::{GameConfig, GameData};
use crate::state::SaveData;
use crate::strategy::StrategyState;
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
        assert_eq!(migrated.version, "0.9.0");
        assert_eq!(migrated.campaign.roster.len(), 4);
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
        assert_eq!(migrated.version, "0.9.0");
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
        assert_eq!(migrated.version, "0.9.0");
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
        assert_eq!(migrated.version, "0.9.0");
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
        assert_eq!(migrated.version, "0.9.0");
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
}
