use super::*;
use crate::relationships::RelationshipRecord;
use crate::trauma::TraumaRecord;

#[test]
fn epilogue_waits_for_a_completed_campaign() {
    let data = crate::data::GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    assert!(derive(&campaign).is_none());
}

#[test]
fn epilogue_records_the_civic_state_and_people_who_carried_it() {
    let data = crate::data::GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign.strategy.campaign_complete = true;
    campaign.strategy.mirexis_path_id = "human_redoubt".to_owned();
    campaign.strategy.escalation_response_id = "bastion_beacon".to_owned();
    campaign.strategy.character_events[0].resolved = true;
    campaign.strategy.post_campaign_operations_completed = 2;
    campaign.identity_stewardship_completed = 3;
    campaign
        .strategy
        .factions
        .iter_mut()
        .find(|faction| faction.id == "directorate")
        .unwrap()
        .attention = 29;
    campaign
        .colony
        .ensure_identity_building("human_redoubt")
        .unwrap();
    let first_id = campaign.roster[0].id.clone();
    let second_id = campaign.roster[1].id.clone();
    campaign.roster[0].mutation_evolution_id = "expanded_cortex".to_owned();
    campaign.roster[1].traumas.push(TraumaRecord {
        id: "clouded_eye".to_owned(),
        name: "Clouded Eye".to_owned(),
        effect: "-8 ACC · +1 DMG".to_owned(),
    });
    campaign.relationships.push(RelationshipRecord {
        first_id,
        second_id,
        bond: 3,
        shared_victories: 2,
    });

    let dossier = derive(&campaign).unwrap();
    let lines = dossier.lines();
    assert_eq!(dossier.institution, "Redoubt Arsenal");
    assert!(lines[0].contains("REDOUBT ARSENAL"));
    assert!(lines[1].contains("COLONISTS"));
    assert!(lines[2].contains("TRUSTED+"));
    assert!(lines[2].contains("+") && !lines[2].contains("NO TRUSTED"));
    assert!(lines[3].contains("CARRIED"));
    assert!(lines[4].contains("EVOLUTION"));
    assert!(lines[6].contains("NAVIGATOR // Kira Voss"));
    assert!(lines[7].contains("ENGINE // HUMAN BOUNDARY / ARMOURED"));
    assert!(lines[7].contains("DIRECTORATE 29"));
    assert!(lines[7].contains("MERCY 1"));
    assert!(lines[7].contains("EPILOGUE WORK 2"));
    assert!(lines[7].contains("CIVIC WORK 3"));
    let summary = dossier.debrief_summary_lines();
    assert!(summary[0].contains("REDOUBT ARSENAL ONLINE"));
    assert!(summary[0].contains("5 COLONISTS"));
    assert!(summary[1].contains("READY 5"));
    assert!(summary[1].contains("TRUSTED+ 1"));
    assert!(summary[1].contains("SCARS 1"));
    assert!(summary[1].contains("EVOLVED 1"));
    assert!(summary[2].contains("ENGINE HUMAN BOUNDARY / ARMOURED"));
    assert!(summary[2].contains("EPILOGUE 2"));
    assert!(summary[2].contains("CIVIC 3"));
}

#[test]
fn epilogue_adds_an_authored_voice_for_each_identity_path() {
    for (
        path_id,
        response_id,
        character_id,
        character_name,
        ready_line,
        recovering_line,
        engine_relationship,
        engine_response,
    ) in [
        (
            "human_redoubt",
            "bastion_beacon",
            "mara_venn",
            "Mara Venn",
            "THE ARSENAL OPENS FROM THE INSIDE.",
            "THE ARSENAL KEEPS THE INJURED WITHIN ITS LIGHT.",
            "HUMAN BOUNDARY",
            "ARMOURED",
        ),
        (
            "living_commonwealth",
            "living_decoy",
            "nadi_vale",
            "Nadi Vale",
            "WE DID NOT BECOME ONE BODY; WE MADE ROOM FOR DIFFERENCE.",
            "THE GARDEN HAS ROOM FOR THE BODY THAT NEEDS TIME.",
            "LIVING ACCORD",
            "SHELTERED",
        ),
        (
            "open_threshold",
            "weaponized_lattice",
            "sol_cairn",
            "Sol Cairn",
            "EVERY OPEN ROUTE NEEDS A WAY HOME.",
            "THE THRESHOLD WAITS; RETURN IS PART OF THE JOURNEY.",
            "OPEN RECIPROCITY",
            "DIRECTED",
        ),
    ] {
        let data = crate::data::GameData::load().unwrap();
        let mut campaign = CampaignState::new(&data);
        campaign.strategy.campaign_complete = true;
        campaign.strategy.mirexis_path_id = path_id.to_owned();
        campaign.strategy.escalation_response_id = response_id.to_owned();
        campaign.strategy.character_events[0].resolved = true;

        let ready = derive(&campaign).unwrap().lines();
        assert!(ready[5].contains(character_name));
        assert!(ready[5].contains(ready_line));
        assert!(ready[6].contains("NAVIGATOR // Kira Voss"));
        assert!(ready[7].contains(engine_relationship));
        assert!(ready[7].contains(engine_response));
        assert!(ready[7].contains("MERCY 1"));

        campaign
            .roster
            .iter_mut()
            .find(|character| character.id == character_id)
            .unwrap()
            .availability = Availability::Recovering;
        let recovering = derive(&campaign).unwrap().lines();
        assert!(recovering[5].contains(recovering_line));
        assert_ne!(ready[5], recovering[5]);
    }
}

#[test]
fn epilogue_names_the_mireborn_timeline_when_sedge_survives() {
    let data = crate::data::GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign.strategy.campaign_complete = true;
    campaign.strategy.mirexis_path_id = "open_threshold".to_owned();
    campaign.strategy.phase_id = "adaptation".to_owned();
    campaign.strategy.contact_complete = true;
    campaign
        .colony
        .buildings
        .push(crate::colony::BuildingState {
            id: "waystation_epilogue_sedge".to_owned(),
            kind: crate::colony::BuildingKind::Waystation,
            position: [2, 11],
            level: 1,
            damaged: false,
        });
    campaign.recruit_outsider(&data).unwrap();

    let voice = derive(&campaign).unwrap().lines()[5].clone();
    assert!(voice.contains("SEDGE"));
    assert!(voice.contains("OLD BODY WAS HERE FIRST"));
}
