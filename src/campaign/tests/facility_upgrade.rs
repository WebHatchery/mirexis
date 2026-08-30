use super::*;
use crate::colony::{
    BuildingKind, BuildingState, ADAPTATION_CLINIC_UPGRADE, COMMUNITY_KITCHEN_UPGRADE,
    COUNTERINTELLIGENCE_CELL_UPGRADE, DRONE_BAY_UPGRADE, EVOLUTION_CHAMBER_UPGRADE,
    PRECISION_BENCH_UPGRADE, SIGNAL_CARTOGRAPHY_UPGRADE, SIMULATION_HALL_UPGRADE,
    STABILISATION_WING_UPGRADE, TRAUMA_WARD_UPGRADE,
};

fn campaign_with_online_gene_lab(data: &GameData) -> CampaignState {
    let mut campaign = CampaignState::new(data);
    campaign.strategy.contact_complete = true;
    campaign.colony.ensure_gene_lab();
    campaign.colony.resources.power += 2;
    campaign
}

#[test]
fn community_kitchen_makes_the_commons_meal_more_affordable() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign.colony.buildings.push(BuildingState {
        id: "commons_test".to_owned(),
        kind: BuildingKind::Commons,
        position: [2, 10],
        level: 1,
        damaged: false,
    });
    let hydroponics_id = campaign
        .colony
        .buildings
        .iter()
        .find(|building| building.kind == BuildingKind::Hydroponics)
        .unwrap()
        .id
        .clone();
    campaign
        .colony
        .queue_facility_upgrade(&hydroponics_id, COMMUNITY_KITCHEN_UPGRADE)
        .unwrap();
    campaign.colony.advance_operation();

    let food_before = campaign.colony.resources.food;
    assert_eq!(campaign.commons_meal_food_cost(), 2);
    campaign.host_commons_meal().unwrap();
    assert_eq!(campaign.colony.resources.food, food_before - 2);
}

#[test]
fn hot_core_adds_attention_to_the_most_visible_faction_after_an_operation() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    let plant_id = campaign
        .colony
        .buildings
        .iter()
        .find(|building| building.kind == crate::colony::BuildingKind::PowerPlant)
        .unwrap()
        .id
        .clone();
    campaign
        .colony
        .queue_facility_upgrade(&plant_id, crate::colony::HOT_CORE_UPGRADE)
        .unwrap();
    campaign.colony.advance_operation();

    let highest_id = campaign
        .strategy
        .factions
        .iter()
        .max_by_key(|faction| (faction.attention, faction.id.clone()))
        .unwrap()
        .id
        .clone();
    let before = campaign
        .strategy
        .factions
        .iter()
        .map(|faction| (faction.id.clone(), faction.attention))
        .collect::<Vec<_>>();
    let mission = campaign.strategy.selected_mission().unwrap().clone();
    let mission_faction = mission.faction_id.clone();
    let outcome = MissionOutcome {
        result: ObjectiveState::Victory,
        colonists_deployed: 0,
        colonists_incapacitated: Vec::new(),
        hostiles_neutralised: 0,
        materials_awarded: 0,
        biomass_awarded: 0,
        power_awarded: 0,
    };

    campaign.apply_mission_outcome(&outcome, &mission, &data);

    for faction in &campaign.strategy.factions {
        let initial = before
            .iter()
            .find(|(id, _)| id == &faction.id)
            .map(|(_, attention)| *attention)
            .unwrap();
        let expected = initial
            + i32::from(faction.id == highest_id)
            + i32::from(faction.id == mission_faction) * 8;
        assert_eq!(faction.attention, expected.min(100));
    }
}

#[test]
fn precision_bench_reduces_weapon_fabrication_cost() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    let workshop_id = campaign
        .colony
        .buildings
        .iter()
        .find(|building| building.kind == BuildingKind::Workshop)
        .unwrap()
        .id
        .clone();
    campaign
        .colony
        .queue_facility_upgrade(&workshop_id, PRECISION_BENCH_UPGRADE)
        .unwrap();
    campaign.colony.advance_operation();

    campaign.colony.resources.materials = 30;
    let cost = campaign
        .craft_equipment("kira_voss", "service_pistol", &data)
        .unwrap();
    assert_eq!(cost, 25);
    assert_eq!(campaign.colony.resources.materials, 5);
}

#[test]
fn drone_bay_reduces_repairs_while_the_workshop_is_online() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    let workshop_id = campaign
        .colony
        .buildings
        .iter()
        .find(|building| building.kind == BuildingKind::Workshop)
        .unwrap()
        .id
        .clone();
    campaign
        .colony
        .queue_facility_upgrade(&workshop_id, DRONE_BAY_UPGRADE)
        .unwrap();
    campaign.colony.advance_operation();
    campaign
        .colony
        .buildings
        .iter_mut()
        .find(|building| building.kind == BuildingKind::Hydroponics)
        .unwrap()
        .damaged = true;
    campaign.colony.resources.materials = 20;

    let (_, cost) = campaign.colony.repair_building("hydroponics").unwrap();
    assert_eq!(cost, 15);
    assert!(campaign
        .colony
        .buildings
        .iter()
        .find(|building| building.kind == BuildingKind::Hydroponics)
        .is_some_and(|building| !building.damaged));
}

#[test]
fn stabilisation_wing_suppresses_evolution_complications_while_online() {
    let data = GameData::load().unwrap();
    let mut campaign = campaign_with_online_gene_lab(&data);
    let gene_lab_id = campaign
        .colony
        .buildings
        .iter()
        .find(|building| building.kind == BuildingKind::GeneLab)
        .unwrap()
        .id
        .clone();
    campaign
        .colony
        .queue_facility_upgrade(&gene_lab_id, STABILISATION_WING_UPGRADE)
        .unwrap();
    campaign.colony.advance_operation();

    let accuracy_before = campaign
        .derived_character_unit("kira_voss", &data)
        .unwrap()
        .accuracy;
    let food_before = campaign.deployment_food_cost(&data);
    campaign
        .choose_mutation_evolution("kira_voss", "expanded_cortex", &data)
        .unwrap();

    let kira = campaign.derived_character_unit("kira_voss", &data).unwrap();
    assert_eq!(kira.accuracy, accuracy_before + 10);
    assert_eq!(campaign.deployment_food_cost(&data), food_before);
}

#[test]
fn evolution_chamber_reduces_mutation_evolution_biomass_cost() {
    let data = GameData::load().unwrap();
    let mut campaign = campaign_with_online_gene_lab(&data);
    let gene_lab_id = campaign
        .colony
        .buildings
        .iter()
        .find(|building| building.kind == BuildingKind::GeneLab)
        .unwrap()
        .id
        .clone();
    campaign
        .colony
        .queue_facility_upgrade(&gene_lab_id, EVOLUTION_CHAMBER_UPGRADE)
        .unwrap();
    campaign.colony.advance_operation();
    campaign.colony.resources.biomass = 4;

    let evolution = data
        .mutations
        .iter()
        .find(|mutation| mutation.id == "neural_bloom")
        .unwrap()
        .evolutions
        .iter()
        .find(|evolution| evolution.id == "expanded_cortex")
        .unwrap();
    assert_eq!(campaign.mutation_evolution_cost(evolution), 4);
    campaign
        .choose_mutation_evolution("kira_voss", "expanded_cortex", &data)
        .unwrap();
    assert_eq!(campaign.colony.resources.biomass, 0);
}

#[test]
fn signal_cartography_reveals_a_third_mission_route() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    let command_centre_id = campaign
        .colony
        .buildings
        .iter()
        .find(|building| building.kind == BuildingKind::CommandCentre)
        .unwrap()
        .id
        .clone();
    campaign
        .colony
        .queue_facility_upgrade(&command_centre_id, SIGNAL_CARTOGRAPHY_UPGRADE)
        .unwrap();
    campaign.colony.advance_operation();

    campaign.refresh_mission_offers(&data);

    assert_eq!(campaign.strategy.mission_offers.len(), 3);
}

#[test]
fn counterintelligence_cell_reduces_mission_attention_pressure() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    let command_centre_id = campaign
        .colony
        .buildings
        .iter()
        .find(|building| building.kind == BuildingKind::CommandCentre)
        .unwrap()
        .id
        .clone();
    campaign
        .colony
        .queue_facility_upgrade(&command_centre_id, COUNTERINTELLIGENCE_CELL_UPGRADE)
        .unwrap();
    campaign.colony.advance_operation();
    let mission = campaign.strategy.selected_mission().unwrap().clone();
    let faction_id = mission.faction_id.clone();
    let attention_before = campaign
        .strategy
        .factions
        .iter()
        .find(|faction| faction.id == faction_id)
        .unwrap()
        .attention;
    let outcome = MissionOutcome {
        result: ObjectiveState::Victory,
        colonists_deployed: 3,
        colonists_incapacitated: Vec::new(),
        hostiles_neutralised: 3,
        materials_awarded: 0,
        biomass_awarded: 0,
        power_awarded: 0,
    };

    campaign.apply_mission_outcome(&outcome, &mission, &data);

    let attention_after = campaign
        .strategy
        .factions
        .iter()
        .find(|faction| faction.id == faction_id)
        .unwrap()
        .attention;
    assert_eq!(attention_after, attention_before + 6);
}

#[test]
fn simulation_hall_reduces_retraining_cost_while_online() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    let barracks_id = campaign
        .colony
        .buildings
        .iter()
        .find(|building| building.kind == BuildingKind::Barracks)
        .unwrap()
        .id
        .clone();
    campaign
        .colony
        .queue_facility_upgrade(&barracks_id, SIMULATION_HALL_UPGRADE)
        .unwrap();
    campaign.colony.advance_operation();
    let class = data
        .classes
        .iter()
        .find(|class| class.id == "scout")
        .unwrap();
    let normal_cost = {
        let baseline = CampaignState::new(&data);
        baseline.training_cost("kira_voss", class).unwrap()
    };

    assert_eq!(
        campaign.training_cost("kira_voss", class),
        Some(normal_cost - 20)
    );
}

#[test]
fn trauma_ward_shortens_injury_recovery_while_online() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    let infirmary_id = campaign
        .colony
        .buildings
        .iter()
        .find(|building| building.kind == BuildingKind::Infirmary)
        .unwrap()
        .id
        .clone();
    campaign
        .colony
        .queue_facility_upgrade(&infirmary_id, TRAUMA_WARD_UPGRADE)
        .unwrap();
    campaign.colony.advance_operation();

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

    let ilya = campaign
        .roster
        .iter()
        .find(|character| character.id == "ilya_reed")
        .unwrap();
    assert_eq!(ilya.injuries[0].recovery_operations, 2);
    assert_eq!(ilya.traumas.len(), 1);
}

#[test]
fn adaptation_clinic_makes_mutation_recovery_and_treatment_safer() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    let infirmary_id = campaign
        .colony
        .buildings
        .iter()
        .find(|building| building.kind == BuildingKind::Infirmary)
        .unwrap()
        .id
        .clone();
    campaign
        .colony
        .queue_facility_upgrade(&infirmary_id, ADAPTATION_CLINIC_UPGRADE)
        .unwrap();
    campaign.colony.advance_operation();

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
    assert_eq!(
        campaign
            .roster
            .iter()
            .find(|character| character.id == "ilya_reed")
            .unwrap()
            .injuries[0]
            .recovery_operations,
        2
    );

    campaign.colony.resources.biomass = 3;
    let treated = campaign.treat_first_injury().unwrap();
    assert_eq!(treated, "Ilya Reed");
    assert_eq!(campaign.colony.resources.biomass, 0);
}

#[test]
fn completed_infirmary_branch_unlocks_ilyas_matching_field_note() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    let infirmary_id = campaign
        .colony
        .buildings
        .iter()
        .find(|building| building.kind == BuildingKind::Infirmary)
        .unwrap()
        .id
        .clone();
    campaign
        .colony
        .queue_facility_upgrade(&infirmary_id, ADAPTATION_CLINIC_UPGRADE)
        .unwrap();
    campaign.colony.advance_operation();

    campaign.acknowledge_colonist("ilya_reed");
    assert!(campaign
        .colony_story
        .has_heard("facility_adaptation_clinic_ilya"));
}

#[test]
fn treatment_availability_matches_injury_biomass_and_infirmary_state() {
    let data = GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    assert!(!campaign.can_treat_first_injury());

    campaign
        .roster
        .iter_mut()
        .find(|character| character.id == "ilya_reed")
        .unwrap()
        .injuries
        .push(InjuryRecord {
            id: "test_trauma".to_owned(),
            name: "Test trauma".to_owned(),
            recovery_operations: 1,
        });
    assert!(campaign.can_treat_first_injury());

    campaign.colony.resources.biomass = 4;
    assert!(!campaign.can_treat_first_injury());
    campaign.colony.resources.biomass = 5;
    assert!(campaign.can_treat_first_injury());

    campaign
        .colony
        .buildings
        .iter_mut()
        .find(|building| building.kind == BuildingKind::Infirmary)
        .unwrap()
        .damaged = true;
    assert!(!campaign.can_treat_first_injury());
}
