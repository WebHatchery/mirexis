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
    assert!(dossier.debrief_line().contains("COLONY LEGACY"));
}

#[test]
fn epilogue_adds_an_authored_voice_for_each_identity_path() {
    for (path_id, character_id, character_name, ready_line, recovering_line) in [
        (
            "human_redoubt",
            "mara_venn",
            "Mara Venn",
            "THE ARSENAL OPENS FROM THE INSIDE.",
            "THE ARSENAL KEEPS THE INJURED WITHIN ITS LIGHT.",
        ),
        (
            "living_commonwealth",
            "nadi_vale",
            "Nadi Vale",
            "WE DID NOT BECOME ONE BODY; WE MADE ROOM FOR DIFFERENCE.",
            "THE GARDEN HAS ROOM FOR THE BODY THAT NEEDS TIME.",
        ),
        (
            "open_threshold",
            "sol_cairn",
            "Sol Cairn",
            "EVERY OPEN ROUTE NEEDS A WAY HOME.",
            "THE THRESHOLD WAITS; RETURN IS PART OF THE JOURNEY.",
        ),
    ] {
        let data = crate::data::GameData::load().unwrap();
        let mut campaign = CampaignState::new(&data);
        campaign.strategy.campaign_complete = true;
        campaign.strategy.mirexis_path_id = path_id.to_owned();

        let ready = derive(&campaign).unwrap().lines();
        assert!(ready[5].contains(character_name));
        assert!(ready[5].contains(ready_line));

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
