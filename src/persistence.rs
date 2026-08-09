//! Campaign save migration at schema boundaries.

use crate::data::GameConfig;
use crate::state::SaveData;
use macroquad_toolkit::rng::SeededRng;
use serde_json::Value;

pub fn migrate_save_value(
    detected_version: Option<String>,
    value: Value,
    config: &GameConfig,
) -> Result<SaveData, String> {
    let mut payload = value.get("data").cloned().unwrap_or(value);
    if detected_version.as_deref() == Some("0.1.0") {
        migrate_phase_zero_payload(&mut payload, config)?;
    }
    let mut save = serde_json::from_value::<SaveData>(payload)
        .map_err(|err| format!("Unsupported Mirexis save {:?}: {}", detected_version, err))?;
    save.version = config.version.clone();
    Ok(save)
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
        unit.insert("incapacitated".to_owned(), serde_json::json!(false));
    }
    Ok(())
}
