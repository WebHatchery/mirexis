use super::*;
use crate::colony::BuildingKind;

#[test]
fn tapped_destinations_preserve_sub_plot_position() {
    let colony = ColonyState::new();
    let mut explorer = ColonyExplorer::default();
    let destination = vec2(8.35, 10.25);
    explorer.request_walk(destination, &colony);
    for _ in 0..20 {
        explorer.update(0.05, &colony);
    }
    assert!(explorer.position.distance(destination) < 0.001);
    assert_ne!(explorer.position.x.fract(), 0.0);
}

#[test]
fn moving_player_stays_behind_the_next_terrain_row_until_crossing_it() {
    assert_eq!(player_draw_depth(vec2(8.0, 10.0)), 18);
    assert_eq!(player_draw_depth(vec2(8.9, 10.9)), 19);
    assert_eq!(player_draw_depth(vec2(9.0, 11.0)), 20);
}

#[test]
fn approaching_an_npc_ends_on_an_adjacent_open_plot() {
    let data = crate::data::GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let mut explorer = ColonyExplorer::default();
    let npc = &campaign.roster[1];
    let station = npc_position(&campaign, &npc.id).unwrap();
    explorer.request_approach(&npc.id, station, &campaign.colony);
    for _ in 0..60 {
        explorer.update(0.05, &campaign.colony);
        explorer.update_approach(&campaign, &data);
    }
    assert!(explorer.position.distance(station) <= INTERACTION_DISTANCE);
    assert!(explorer.is_talking());
}

#[test]
fn reset_closes_an_open_dialogue_before_the_next_scene() {
    let data = crate::data::GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let npc = &campaign.roster[1];
    let station = npc_position(&campaign, &npc.id).unwrap();
    let mut explorer = ColonyExplorer::default();

    explorer.request_approach(&npc.id, station, &campaign.colony);
    for _ in 0..60 {
        explorer.update(0.05, &campaign.colony);
        explorer.update_approach(&campaign, &data);
    }
    assert!(explorer.is_talking());

    explorer.reset();

    assert!(!explorer.is_talking());
    assert_eq!(explorer.position(), PLAYER_START);
}

#[test]
fn construction_plots_are_not_walkable() {
    let mut colony = ColonyState::new();
    colony
        .place_construction(BuildingKind::Barricade, [3, 3])
        .unwrap();
    assert!(!can_occupy(&colony, vec2(3.0, 3.0)));
    assert!(can_occupy(&colony, vec2(3.8, 3.0)));
}

#[test]
fn identity_contact_stands_at_the_chosen_identity_building() {
    let data = crate::data::GameData::load().unwrap();
    for (path_id, character_id, kind) in [
        ("human_redoubt", "mara_venn", BuildingKind::RedoubtArsenal),
        (
            "living_commonwealth",
            "nadi_vale",
            BuildingKind::ChoirGarden,
        ),
        ("open_threshold", "sol_cairn", BuildingKind::ThresholdSpire),
    ] {
        let mut campaign = CampaignState::new(&data);
        campaign.strategy.mirexis_path_id = path_id.to_owned();
        campaign.colony.ensure_identity_building(path_id).unwrap();
        let building_position = campaign
            .colony
            .buildings
            .iter()
            .find(|building| building.kind == kind)
            .map(|building| building.position)
            .unwrap();
        assert_eq!(
            npc_position(&campaign, character_id),
            Some(grid_vec(building_position))
        );
    }
}

#[test]
fn adapted_recruit_stands_at_the_gene_lab_when_it_exists() {
    let data = crate::data::GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign.strategy.phase_id = "adaptation".to_owned();
    campaign.strategy.contact_complete = true;
    campaign
        .colony
        .buildings
        .push(crate::colony::BuildingState {
            id: "waystation_sedge_exploration".to_owned(),
            kind: BuildingKind::Waystation,
            position: [2, 11],
            level: 1,
            damaged: false,
        });
    campaign
        .colony
        .buildings
        .push(crate::colony::BuildingState {
            id: "gene_lab_sedge_exploration".to_owned(),
            kind: BuildingKind::GeneLab,
            position: [16, 10],
            level: 1,
            damaged: false,
        });
    campaign.recruit_outsider(&data).unwrap();
    assert_eq!(npc_position(&campaign, "sedge"), Some(grid_vec([16, 10])));
}

#[test]
fn unchosen_directorate_outsider_stands_at_the_waystation_as_a_guest() {
    let data = crate::data::GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign.strategy.contact_protocol_id = "directorate_requisition".to_owned();
    campaign
        .colony
        .buildings
        .push(crate::colony::BuildingState {
            id: "waystation_guest_exploration".to_owned(),
            kind: BuildingKind::Waystation,
            position: [2, 10],
            level: 1,
            damaged: false,
        });

    assert!(!campaign
        .roster
        .iter()
        .any(|character| character.id == "veya_orn"));
    assert_eq!(npc_position(&campaign, "veya_orn"), Some(grid_vec([2, 10])));
    let guest = npc(&campaign, &data, "veya_orn").unwrap();
    assert!(guest.guest);
    assert_eq!(guest.character.name, "Veya Orn");
    assert_eq!(
        npc_action_label(&guest.character, guest.guest),
        "RECRUIT CONTACT"
    );
    assert_eq!(
        npc_action_button_label(&campaign, &data, &guest.character, guest.guest),
        "RECRUIT // 30 MATERIALS"
    );
    assert_eq!(
        npc_action(&guest.character, guest.guest),
        Some(UiAction::RecruitOutsider)
    );
    let recruitment_cost = campaign.available_outsider(&data).unwrap().recruitment_cost;
    let action = npc_action(&guest.character, guest.guest).unwrap();
    assert!(npc_action_enabled(&campaign, &data, &action, guest.guest));
    campaign.colony.resources.materials = recruitment_cost - 1;
    assert!(!npc_action_enabled(&campaign, &data, &action, guest.guest));
    assert_eq!(
        npc_action_button_label(&campaign, &data, &guest.character, guest.guest),
        "RECRUIT // NEED 30 MATERIALS"
    );

    let mut explorer = ColonyExplorer::default();
    let station = npc_position(&campaign, "veya_orn").unwrap();
    explorer.request_approach("veya_orn", station, &campaign.colony);
    for _ in 0..60 {
        explorer.update(0.05, &campaign.colony);
        explorer.update_approach(&campaign, &data);
    }
    assert!(explorer.is_talking());
}

#[test]
fn unchosen_mireborn_outsider_uses_the_waystation_before_recruitment() {
    let data = crate::data::GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign.strategy.contact_protocol_id = "brood_adaptation".to_owned();
    campaign.strategy.phase_id = "adaptation".to_owned();
    campaign.strategy.contact_complete = true;
    campaign
        .colony
        .buildings
        .push(crate::colony::BuildingState {
            id: "waystation_mireborn_guest".to_owned(),
            kind: BuildingKind::Waystation,
            position: [3, 9],
            level: 1,
            damaged: false,
        });

    assert_eq!(
        campaign
            .available_outsider(&data)
            .map(|definition| definition.id.as_str()),
        Some("sedge")
    );
    assert_eq!(npc_position(&campaign, "sedge"), Some(grid_vec([3, 9])));
    assert!(npc(&campaign, &data, "sedge").is_some_and(|guest| guest.guest));
}

#[test]
fn treatment_dialogue_action_requires_an_injury_and_prerequisites() {
    let data = crate::data::GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    let ilya = campaign
        .roster
        .iter()
        .find(|character| character.id == "ilya_reed")
        .cloned()
        .unwrap();
    assert_eq!(npc_action(&ilya, false), None);

    campaign
        .roster
        .iter_mut()
        .find(|character| character.id == "ilya_reed")
        .unwrap()
        .injuries
        .push(crate::campaign::InjuryRecord {
            id: "test_trauma".to_owned(),
            name: "Test trauma".to_owned(),
            recovery_operations: 1,
        });
    let injured = campaign
        .roster
        .iter()
        .find(|character| character.id == "ilya_reed")
        .unwrap();
    let action = npc_action(injured, false).unwrap();
    assert_eq!(action, UiAction::TreatInjury);
    assert_eq!(
        npc_action_button_label(&campaign, &data, injured, false),
        "TREAT // 5 BIO"
    );
    assert!(npc_action_enabled(&campaign, &data, &action, false));

    campaign.colony.resources.biomass = 0;
    assert!(!npc_action_enabled(&campaign, &data, &action, false));
    assert_eq!(
        npc_action_button_label(&campaign, &data, injured, false),
        "TREAT // NEED 5 BIO"
    );
}

#[test]
fn gene_lab_dialogue_action_is_disabled_until_the_lab_is_online() {
    let data = crate::data::GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let nadi = campaign
        .roster
        .iter()
        .find(|character| character.id == "nadi_vale")
        .unwrap();
    let action = npc_action(nadi, false).unwrap();
    assert_eq!(action, UiAction::OpenGeneLab);
    assert_eq!(
        npc_action_button_label(&campaign, &data, nadi, false),
        "GENE LAB // NEED ADAPTATION"
    );
    assert!(!npc_action_enabled(&campaign, &data, &action, false));
}
