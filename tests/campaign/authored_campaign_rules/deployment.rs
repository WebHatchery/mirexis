//! Deployment, training, and content-materialization regressions.

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
fn campaign_construction_affordance_tracks_unlocks_and_materials() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    assert!(!campaign.construction_available(BuildingKind::GeneLab));
    campaign.strategy.contact_complete = true;
    assert!(campaign.construction_available(BuildingKind::GeneLab));
    campaign.colony.resources.materials = 0;
    assert!(!campaign.construction_available(BuildingKind::GeneLab));
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
    mission.hazards.push(mirexis::data::HazardDef {
        position: [36, 30],
        kind: mirexis::data::HazardKind::SporeBloom,
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
