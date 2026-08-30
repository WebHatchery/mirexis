use super::*;

#[test]
fn adaptation_evolution_applies_its_gift_and_complication() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign.strategy.contact_complete = true;
    assert!(campaign
        .choose_mutation_evolution("kira_voss", "expanded_cortex", &data)
        .is_err());
    campaign.colony.ensure_gene_lab();
    campaign.colony.resources.power += 2;
    let accuracy_before = campaign
        .deployment_roster(&data, &data.mission)
        .into_iter()
        .find(|unit| unit.id == "kira_voss")
        .unwrap()
        .accuracy;
    let food_before = campaign.deployment_food_cost(&data);
    assert_eq!(
        campaign
            .choose_mutation_evolution("kira_voss", "expanded_cortex", &data)
            .unwrap(),
        "Expanded Cortex"
    );
    let accuracy_after = campaign
        .deployment_roster(&data, &data.mission)
        .into_iter()
        .find(|unit| unit.id == "kira_voss")
        .unwrap()
        .accuracy;
    assert_eq!(accuracy_after, accuracy_before + 10);
    assert_eq!(campaign.deployment_food_cost(&data), food_before + 1);
    assert!(campaign
        .choose_mutation_evolution("kira_voss", "echo_mind", &data)
        .is_err());
}

#[test]
fn mara_can_trade_mobility_for_a_fortress_carapace() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign.strategy.contact_complete = true;
    campaign.colony.ensure_gene_lab();
    campaign.colony.resources.power += 2;
    let before = campaign
        .deployment_roster(&data, &data.mission)
        .into_iter()
        .find(|unit| unit.id == "mara_venn")
        .unwrap();
    assert_eq!(
        campaign
            .choose_mutation_evolution("mara_venn", "fortress_carapace", &data)
            .unwrap(),
        "Fortress Carapace"
    );
    let after = campaign
        .deployment_roster(&data, &data.mission)
        .into_iter()
        .find(|unit| unit.id == "mara_venn")
        .unwrap();
    assert_eq!(after.armour, before.armour + 2);
    assert_eq!(after.move_range, before.move_range - 1);
    assert!(campaign.roster[0].mutation_evolution_id.is_empty());
}

#[test]
fn clean_marrow_stabilizes_ilyas_injury_recovery_at_a_damage_cost() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign.strategy.contact_complete = true;
    campaign.colony.ensure_gene_lab();
    campaign.colony.resources.power += 2;
    let damage_before = campaign
        .deployment_roster(&data, &data.mission)
        .into_iter()
        .find(|unit| unit.id == "ilya_reed")
        .unwrap()
        .weapon_damage;
    campaign
        .choose_mutation_evolution("ilya_reed", "clean_marrow", &data)
        .unwrap();
    let damage_after = campaign
        .deployment_roster(&data, &data.mission)
        .into_iter()
        .find(|unit| unit.id == "ilya_reed")
        .unwrap()
        .weapon_damage;
    assert_eq!(damage_after, damage_before - 1);
    let outcome = MissionOutcome {
        result: ObjectiveState::Failed,
        colonists_deployed: 3,
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
fn load_bearing_fascia_restores_sols_armour_efficiency() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign.strategy.contact_complete = true;
    campaign.colony.ensure_gene_lab();
    campaign.colony.resources.power += 2;
    campaign.roster[3]
        .equipment_ids
        .push("chitin_plate".to_owned());
    let base = data
        .roster
        .iter()
        .find(|unit| unit.id == "sol_cairn")
        .unwrap();
    let before = derive_unit(base, &campaign.roster[3], &data);
    campaign
        .choose_mutation_evolution("sol_cairn", "load_bearing_fascia", &data)
        .unwrap();
    let after = derive_unit(base, &campaign.roster[3], &data);
    assert_eq!(after.armour, before.armour + 1);
    assert_eq!(after.weapon_damage, before.weapon_damage - 1);
}

#[test]
fn nadis_symbiote_evolves_toward_cooperation_or_predation() {
    let data = GameData::load().unwrap();
    let base = data
        .roster
        .iter()
        .find(|unit| unit.id == "nadi_vale")
        .unwrap();

    let mut cooperative = CampaignState::new(&data);
    cooperative.strategy.contact_complete = true;
    cooperative.colony.ensure_gene_lab();
    cooperative.colony.resources.power += 2;
    cooperative.colony.resources.biomass = 20;
    let nadi = cooperative
        .roster
        .iter()
        .find(|character| character.id == "nadi_vale")
        .unwrap();
    let before = derive_unit(base, nadi, &data);
    cooperative
        .choose_mutation_evolution("nadi_vale", "cooperative_symbiote", &data)
        .unwrap();
    let nadi = cooperative
        .roster
        .iter()
        .find(|character| character.id == "nadi_vale")
        .unwrap();
    let after = derive_unit(base, nadi, &data);
    assert_eq!(after.armour, before.armour + 2);
    assert_eq!(after.round_regeneration, before.round_regeneration + 1);
    assert_eq!(after.move_range, before.move_range - 1);

    let mut predatory = CampaignState::new(&data);
    predatory.strategy.contact_complete = true;
    predatory.colony.ensure_gene_lab();
    predatory.colony.resources.power += 2;
    predatory.colony.resources.biomass = 20;
    predatory
        .choose_mutation_evolution("nadi_vale", "predatory_symbiote", &data)
        .unwrap();
    let nadi = predatory
        .roster
        .iter()
        .find(|character| character.id == "nadi_vale")
        .unwrap();
    let after = derive_unit(base, nadi, &data);
    assert_eq!(after.weapon_damage, before.weapon_damage + 3);
    assert_eq!(after.accuracy, before.accuracy - 10);
    assert_eq!(derived_mutation_traits(nadi, &data)["food_upkeep"], 2);
}

#[test]
fn adaptation_completion_requires_glass_nerve_and_two_evolutions() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign.strategy.contact_complete = true;
    campaign.strategy.phase_id = "adaptation".to_owned();
    campaign.colony.ensure_gene_lab();
    campaign.colony.resources.power += 2;
    campaign.colony.resources.biomass = 50;
    campaign
        .choose_mutation_evolution("kira_voss", "expanded_cortex", &data)
        .unwrap();
    campaign
        .choose_mutation_evolution("mara_venn", "fortress_carapace", &data)
        .unwrap();
    assert!(!campaign.strategy.adaptation_complete);
    let mission = campaign.strategy.selected_mission().unwrap().clone();
    assert_eq!(mission.template_id, "adaptation_glass_nerve");
    let outcome = MissionOutcome {
        result: ObjectiveState::Victory,
        colonists_deployed: 3,
        colonists_incapacitated: Vec::new(),
        hostiles_neutralised: 3,
        materials_awarded: mission.materials_reward,
        biomass_awarded: mission.biomass_reward,
        power_awarded: mission.power_reward,
    };
    campaign.apply_mission_outcome(&outcome, &mission, &data);
    assert!(campaign.strategy.adaptation_operation_completed);
    assert!(campaign.strategy.adaptation_complete);
    assert_eq!(campaign.strategy.phase_id, "escalation");
}

#[test]
fn escalation_completion_requires_the_matching_response_operation() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign.strategy.adaptation_complete = true;
    campaign.strategy.phase_id = "escalation".to_owned();
    campaign.strategy.escalation_operation_completed = true;
    campaign.colony.resources.biomass = 20;
    campaign
        .strategy
        .choose_escalation_response("living_decoy", &mut campaign.colony, &data)
        .unwrap();
    assert!(!campaign.strategy.escalation_complete);
    let mission = campaign.strategy.selected_mission().unwrap().clone();
    assert_eq!(mission.template_id, "escalation_living_false_heart");
    let outcome = MissionOutcome {
        result: ObjectiveState::Victory,
        colonists_deployed: 3,
        colonists_incapacitated: Vec::new(),
        hostiles_neutralised: 3,
        materials_awarded: mission.materials_reward,
        biomass_awarded: mission.biomass_reward,
        power_awarded: mission.power_reward,
    };
    campaign.apply_mission_outcome(&outcome, &mission, &data);

    assert!(campaign.strategy.escalation_branch_completed);
    assert!(campaign.strategy.escalation_complete);
    assert_eq!(campaign.strategy.phase_id, "mirexis");
}

#[test]
fn mirexis_paths_change_defense_supply_and_recovery() {
    let data = GameData::load().unwrap();

    let mut redoubt = CampaignState::new(&data);
    assert!(redoubt
        .strategy
        .choose_mirexis_path("human_redoubt", &mut redoubt.colony, &data)
        .is_err());
    redoubt.strategy.escalation_complete = true;
    redoubt.strategy.phase_id = "mirexis".to_owned();
    redoubt.colony.resources.materials = 100;
    redoubt
        .strategy
        .choose_mirexis_path("human_redoubt", &mut redoubt.colony, &data)
        .unwrap();
    assert!(redoubt
        .colony
        .buildings
        .iter()
        .any(|building| building.kind == crate::colony::BuildingKind::RedoubtArsenal));
    assert_eq!(
        redoubt.strategy.selected_mission().unwrap().template_id,
        "mirexis_redoubt_last_wall"
    );
    redoubt.strategy.threats[0].operations_until = 0;
    redoubt.strategy.regenerate_missions(&data);
    assert_eq!(
        redoubt
            .strategy
            .materialize_selected(&data, &redoubt.colony)
            .cover_integrity,
        data.mission.cover_integrity + 4
    );

    let mut commonwealth = CampaignState::new(&data);
    commonwealth.strategy.escalation_complete = true;
    commonwealth.strategy.phase_id = "mirexis".to_owned();
    commonwealth.colony.resources.biomass = 20;
    let food_before = commonwealth.deployment_food_cost(&data);
    commonwealth
        .strategy
        .choose_mirexis_path("living_commonwealth", &mut commonwealth.colony, &data)
        .unwrap();
    assert!(commonwealth
        .colony
        .buildings
        .iter()
        .any(|building| building.kind == crate::colony::BuildingKind::ChoirGarden));
    assert_eq!(commonwealth.deployment_food_cost(&data), food_before - 1);
    assert_eq!(
        commonwealth
            .strategy
            .selected_mission()
            .unwrap()
            .template_id,
        "mirexis_commonwealth_root_choir"
    );

    let mut threshold = CampaignState::new(&data);
    threshold.strategy.escalation_complete = true;
    threshold.strategy.phase_id = "mirexis".to_owned();
    threshold.colony.resources.power = 10;
    threshold
        .strategy
        .choose_mirexis_path("open_threshold", &mut threshold.colony, &data)
        .unwrap();
    assert!(threshold
        .colony
        .buildings
        .iter()
        .any(|building| building.kind == crate::colony::BuildingKind::ThresholdSpire));
    assert_eq!(
        threshold.strategy.selected_mission().unwrap().template_id,
        "mirexis_threshold_door_of_light"
    );
    let power_before = threshold.strategy.selected_mission().unwrap().power_reward;
    assert_eq!(
        threshold
            .strategy
            .materialize_selected(&data, &threshold.colony)
            .power_reward,
        power_before + 3
    );
    assert!(threshold
        .strategy
        .choose_mirexis_path("human_redoubt", &mut threshold.colony, &data)
        .is_err());
}

#[test]
fn matching_mirexis_operation_reveals_the_chosen_campaign_end() {
    let data = GameData::load().unwrap();
    for (path_id, template_id, post_template_id, ending_title, modifier, objective_kind) in [
        (
            "human_redoubt",
            "mirexis_redoubt_last_wall",
            "epilogue_redoubt_old_fire",
            "THE LAST WALL HOLDS",
            crate::data::OperationModifier::MirexisRedoubt,
            crate::data::ObjectiveKind::DefendAsset,
        ),
        (
            "living_commonwealth",
            "mirexis_commonwealth_root_choir",
            "epilogue_commonwealth_new_roots",
            "THE ROOT CHOIR ANSWERS",
            crate::data::OperationModifier::MirexisCommonwealth,
            crate::data::ObjectiveKind::SignalTrace,
        ),
        (
            "open_threshold",
            "mirexis_threshold_door_of_light",
            "epilogue_threshold_return",
            "THE DOOR OF LIGHT OPENS",
            crate::data::OperationModifier::MirexisThreshold,
            crate::data::ObjectiveKind::SecureAndClear,
        ),
    ] {
        let mut campaign = CampaignState::new(&data);
        campaign.strategy.escalation_complete = true;
        campaign.strategy.phase_id = "mirexis".to_owned();
        campaign.colony.resources.materials = 100;
        campaign.colony.resources.biomass = 30;
        campaign.colony.resources.power = 20;
        campaign
            .strategy
            .choose_mirexis_path(path_id, &mut campaign.colony, &data)
            .unwrap();
        let mission = campaign.strategy.selected_mission().unwrap().clone();
        assert_eq!(mission.template_id, template_id);
        assert_eq!(mission.operation_modifier, modifier);
        assert_eq!(mission.objective_kind, objective_kind);
        let outcome = MissionOutcome {
            result: ObjectiveState::Victory,
            colonists_deployed: 3,
            colonists_incapacitated: Vec::new(),
            hostiles_neutralised: 3,
            materials_awarded: mission.materials_reward,
            biomass_awarded: mission.biomass_reward,
            power_awarded: mission.power_reward,
        };
        campaign.apply_mission_outcome(&outcome, &mission, &data);

        assert!(campaign.strategy.mirexis_operation_completed);
        assert!(campaign.strategy.campaign_complete);
        assert_eq!(campaign.strategy.phase_name, ending_title);
        assert!(!campaign.strategy.phase_summary.is_empty());
        assert!(campaign
            .strategy
            .mission_offers
            .iter()
            .all(|offer| offer.template_id != template_id));
        assert_eq!(
            campaign.strategy.mission_offers[0].template_id,
            post_template_id
        );
        assert_eq!(
            campaign.strategy.mission_offers[0].operation_modifier,
            modifier
        );
        campaign.strategy.regenerate_missions(&data);
        assert_eq!(
            campaign.strategy.mission_offers[0].template_id,
            post_template_id
        );
        assert_eq!(
            campaign.strategy.mission_offers[0].operation_modifier,
            modifier
        );
        let epilogue = campaign.strategy.selected_mission().unwrap().clone();
        let epilogue_outcome = MissionOutcome {
            result: ObjectiveState::Victory,
            colonists_deployed: 3,
            colonists_incapacitated: Vec::new(),
            hostiles_neutralised: 3,
            materials_awarded: epilogue.materials_reward,
            biomass_awarded: epilogue.biomass_reward,
            power_awarded: epilogue.power_reward,
        };
        campaign.apply_mission_outcome(&epilogue_outcome, &epilogue, &data);
        assert_eq!(campaign.strategy.post_campaign_operations_completed, 1);
    }
}
