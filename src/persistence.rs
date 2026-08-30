//! Campaign save migration at schema boundaries.

use crate::campaign::{CampaignState, SQUAD_LIMIT};
use crate::colony::ColonyState;
use crate::data::{GameConfig, GameData};
use crate::state::SaveData;
use crate::strategy::StrategyState;
use crate::tactical::{BattleEvent, DestructibleCover, TacticalState};
use macroquad_toolkit::grid::{FlatGrid, FogState, TilePos};
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
    add_campaign_runtime_defaults(&mut payload)?;
    let mut save = serde_json::from_value::<SaveData>(payload)
        .map_err(|err| format!("Unsupported Mirexis save {:?}: {}", detected_version, err))?;
    migrate_legacy_tactical_world(&mut save, &data.config)?;
    validate_tactical_world(&save)?;
    save.campaign.colony.migrate_legacy_spatial_layout();
    save.campaign.ensure_roster_characters(data);
    save.campaign.strategy.ensure_character_events(data);
    save.campaign.colony.ensure_phase_one_infrastructure(
        detected_version.as_deref() != Some(data.config.version.as_str()),
    );
    if !save.campaign.strategy.mirexis_path_id.is_empty() {
        save.campaign
            .colony
            .ensure_identity_building(&save.campaign.strategy.mirexis_path_id)?;
    }
    save.campaign.refresh_contact_completion(data);
    if detected_version.as_deref() == Some("1.12.0")
        && save.campaign.strategy.contact_complete
        && save
            .campaign
            .roster
            .iter()
            .any(|character| !character.mutation_evolution_id.is_empty())
    {
        save.campaign.refresh_mission_offers(data);
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
        save.campaign.refresh_mission_offers(data);
    }
    if detected_version.as_deref() == Some("1.20.0")
        && !save.campaign.strategy.escalation_response_id.is_empty()
    {
        save.campaign.refresh_mission_offers(data);
    }
    if detected_version.as_deref() == Some("1.21.0")
        && save.campaign.strategy.phase_id == "escalation"
    {
        save.campaign.refresh_mission_offers(data);
    }
    if detected_version.as_deref() == Some("1.24.0")
        && !save.campaign.strategy.mirexis_path_id.is_empty()
    {
        save.campaign.refresh_mission_offers(data);
    }
    if detected_version.as_deref() == Some("1.95.0")
        && !save.campaign.strategy.mirexis_path_id.is_empty()
    {
        save.campaign.refresh_mission_offers(data);
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
    if version_predates_first_hour(detected_version.as_deref()) {
        save.campaign
            .first_hour
            .migrate_from_operations(save.campaign.operations_completed);
    }
    if detected_version.as_deref() != Some(data.config.version.as_str()) {
        if let Some(tactical) = &mut save.tactical {
            for unit in &mut tactical.units {
                if unit.faction.is_none() {
                    unit.faction = data
                        .roster
                        .iter()
                        .chain(data.recruitable_roster.iter())
                        .find(|definition| definition.id == unit.id)
                        .and_then(|definition| definition.faction.clone());
                }
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
                if let Some(character) = save
                    .campaign
                    .roster
                    .iter()
                    .find(|character| character.id == unit.id)
                {
                    if unit.learned_skills.is_empty() {
                        unit.learned_skills = character.learned_skills.clone();
                    }
                    if unit.active_skills.is_empty() {
                        unit.active_skills = character.active_skills.clone();
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

fn version_predates_first_hour(version: Option<&str>) -> bool {
    let Some(version) = version else {
        return true;
    };
    let mut parts = version
        .split('.')
        .filter_map(|part| part.parse::<u32>().ok());
    let pair = (parts.next().unwrap_or(0), parts.next().unwrap_or(0));
    pair < (1, 64)
}

fn migrate_legacy_tactical_world(save: &mut SaveData, config: &GameConfig) -> Result<(), String> {
    let Some(tactical) = &mut save.tactical else {
        return Ok(());
    };
    if tactical.fog.width == config.world_width && tactical.fog.height == config.world_height {
        return Ok(());
    }
    if tactical.fog.width != 12
        || tactical.fog.height != 8
        || config.world_width != 40
        || config.world_height != 40
    {
        return Err(format!(
            "Cannot migrate tactical grid {}x{} to {}x{}",
            tactical.fog.width, tactical.fog.height, config.world_width, config.world_height
        ));
    }

    tactical.fog = FlatGrid::new(config.world_width, config.world_height, FogState::Visible);
    tactical.blocked = tactical.blocked.drain().map(project_legacy_tile).collect();
    for (position, _) in &mut tactical.terrain_costs {
        *position = project_legacy_tile(*position);
    }
    for hazard in &mut tactical.hazards {
        hazard.position = project_legacy_tile(hazard.position);
    }
    for edge in &mut tactical.cover_edges {
        edge.position = crate::map_variants::project_authored_position(edge.position);
    }
    for cover in &mut tactical.destructible_cover {
        cover.position = project_legacy_tile(cover.position);
    }
    for field in &mut tactical.obscuring_fields {
        field.center = project_legacy_tile(field.center);
    }
    for unit in &mut tactical.units {
        unit.position = project_legacy_tile(unit.position);
    }
    for wave in &mut tactical.reinforcement_waves {
        for unit in &mut wave.units {
            unit.position = project_legacy_tile(unit.position);
        }
    }
    tactical.selected_tile = project_legacy_tile(tactical.selected_tile);
    tactical.objective_tile = project_legacy_tile(tactical.objective_tile);
    migrate_positional_events(tactical);
    Ok(())
}

fn migrate_positional_events(tactical: &mut TacticalState) {
    for event in &mut tactical.event_log {
        match event {
            BattleEvent::UnitMoved { path, .. } => {
                for position in path {
                    *position = project_legacy_tile(*position);
                }
            }
            BattleEvent::CoverDamaged { position, .. }
            | BattleEvent::CoverDestroyed { position }
            | BattleEvent::HazardConverted { position, .. } => {
                *position = project_legacy_tile(*position);
            }
            _ => {}
        }
    }
}

fn project_legacy_tile(position: TilePos) -> TilePos {
    let projected = crate::map_variants::project_authored_position([position.x, position.y]);
    TilePos::new(projected[0], projected[1])
}

fn validate_tactical_world(save: &SaveData) -> Result<(), String> {
    let Some(tactical) = &save.tactical else {
        return Ok(());
    };
    let check = |label: &str, position: TilePos| {
        if tactical.fog.is_valid(position) {
            Ok(())
        } else {
            Err(format!(
                "Tactical save {label} ({}, {}) lies outside the {}x{} world",
                position.x, position.y, tactical.fog.width, tactical.fog.height
            ))
        }
    };
    check("selection", tactical.selected_tile)?;
    check("objective", tactical.objective_tile)?;
    for position in &tactical.blocked {
        check("blocked tile", *position)?;
    }
    for (position, _) in &tactical.terrain_costs {
        check("terrain tile", *position)?;
    }
    for hazard in &tactical.hazards {
        check("hazard", hazard.position)?;
    }
    for edge in &tactical.cover_edges {
        check(
            "cover edge",
            TilePos::new(edge.position[0], edge.position[1]),
        )?;
    }
    for cover in &tactical.destructible_cover {
        check("destructible cover", cover.position)?;
    }
    for field in &tactical.obscuring_fields {
        check("obscuring field", field.center)?;
    }
    for unit in &tactical.units {
        check("unit", unit.position)?;
    }
    for wave in &tactical.reinforcement_waves {
        for unit in &wave.units {
            check("reinforcement", unit.position)?;
        }
    }
    for event in &tactical.event_log {
        match event {
            BattleEvent::UnitMoved { path, .. } => {
                for position in path {
                    check("movement history", *position)?;
                }
            }
            BattleEvent::CoverDamaged { position, .. }
            | BattleEvent::CoverDestroyed { position } => check("cover history", *position)?,
            BattleEvent::HazardConverted { position, .. } => check("hazard history", *position)?,
            _ => {}
        }
    }
    Ok(())
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
            "Breacher" => "breacher",
            "Fortifier" => "fortifier",
            "Rescue Specialist" => "rescue_specialist",
            "Chorus Warden" => "chorus_warden",
            _ => "",
        };
        unit.entry("class_id".to_owned())
            .or_insert_with(|| serde_json::json!(class_id));
        unit.entry("class_action_used".to_owned())
            .or_insert_with(|| serde_json::json!(false));
        unit.entry("statuses".to_owned())
            .or_insert_with(|| serde_json::json!([]));
        unit.entry("learned_skills".to_owned())
            .or_insert_with(|| serde_json::json!([]));
        unit.entry("active_skills".to_owned())
            .or_insert_with(|| serde_json::json!([]));
        unit.entry("used_skill_ids".to_owned())
            .or_insert_with(|| serde_json::json!([]));
        unit.entry("next_attack_ignores_armour".to_owned())
            .or_insert_with(|| serde_json::json!(false));
        unit.entry("next_equipment_overcharged".to_owned())
            .or_insert_with(|| serde_json::json!(false));
    }
    Ok(())
}

fn add_campaign_runtime_defaults(value: &mut Value) -> Result<(), String> {
    let Some(campaign) = value.get_mut("campaign").and_then(Value::as_object_mut) else {
        return Ok(());
    };
    campaign
        .entry("outsider_arc_stage".to_owned())
        .or_insert_with(|| serde_json::json!(0));
    campaign
        .entry("outsider_disagreements".to_owned())
        .or_insert_with(|| serde_json::json!(0));
    campaign
        .entry("outsider_final_choice".to_owned())
        .or_insert_with(|| serde_json::json!(""));
    campaign
        .entry("outsider_arc_states".to_owned())
        .or_insert_with(|| serde_json::json!({}));
    campaign
        .entry("commons_meals_hosted".to_owned())
        .or_insert_with(|| serde_json::json!(0));
    campaign
        .entry("commons_meal_operation".to_owned())
        .or_insert_with(|| serde_json::Value::Null);
    campaign
        .entry("relay_scans_used".to_owned())
        .or_insert_with(|| serde_json::json!(0));
    campaign
        .entry("relay_scan_operation".to_owned())
        .or_insert_with(|| serde_json::Value::Null);
    campaign
        .entry("identity_stewardship_completed".to_owned())
        .or_insert_with(|| serde_json::json!(0));
    campaign
        .entry("identity_stewardship_operation".to_owned())
        .or_insert_with(|| serde_json::Value::Null);
    campaign
        .entry("identity_preparations_completed".to_owned())
        .or_insert_with(|| serde_json::json!(0));
    campaign
        .entry("identity_preparation_operation".to_owned())
        .or_insert_with(|| serde_json::Value::Null);
    campaign
        .entry("salvage_cache_count".to_owned())
        .or_insert_with(|| serde_json::json!(0));
    campaign
        .entry("salvage_yard_operation".to_owned())
        .or_insert_with(|| serde_json::Value::Null);
    campaign
        .entry("research_insight".to_owned())
        .or_insert_with(|| serde_json::json!(0));
    campaign
        .entry("salvage_prototypes".to_owned())
        .or_insert_with(|| serde_json::json!(0));
    if let Some(roster) = campaign.get_mut("roster").and_then(Value::as_array_mut) {
        for character in roster {
            let Some(character) = character.as_object_mut() else {
                continue;
            };
            character
                .entry("origin".to_owned())
                .or_insert_with(|| serde_json::json!(""));
            character
                .entry("origin_description".to_owned())
                .or_insert_with(|| serde_json::json!(""));
        }
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
    tactical.insert("hazards".to_owned(), serde_json::json!([]));
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
mod tests;
