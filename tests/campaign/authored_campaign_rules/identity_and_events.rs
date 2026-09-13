//! Character identity, injury, squad, and event regressions.

use super::*;

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
    assert!(!campaign.can_toggle_deployment("sol_cairn"));
    assert!(campaign.can_toggle_deployment("kira_voss"));
    assert!(campaign.toggle_deployment("sol_cairn").is_err());
    assert!(!campaign.toggle_deployment("kira_voss").unwrap());
    assert!(campaign.can_toggle_deployment("sol_cairn"));
    campaign.roster[0].availability = Availability::Recovering;
    assert!(!campaign.can_toggle_deployment("kira_voss"));
    campaign.roster[0].availability = Availability::Ready;
    assert!(campaign.toggle_deployment("sol_cairn").unwrap());
    assert!(!campaign.can_toggle_deployment("kira_voss"));
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
    assert_eq!(campaign.colony_story.archived_notes().len(), 1);
    assert_eq!(
        campaign.colony_story.archived_notes()[0].speaker,
        "Mara Venn"
    );

    campaign.operations_completed = 1;
    campaign.first_hour.first_outcome_won = Some(true);
    campaign.acknowledge_colonist("mara_venn");
    assert!(campaign.colony_story.has_heard("mara_first_victory"));
    assert_eq!(campaign.colony_story.archived_notes().len(), 2);
}
