use super::*;

#[test]
fn record_count_collects_scars_recovery_and_legacies() {
    let data = crate::data::GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    assert_eq!(record_count(&campaign), 0);

    {
        let character = campaign.roster.first_mut().unwrap();
        character.traumas.push(crate::trauma::TraumaRecord {
            id: "clouded_eye".to_owned(),
            name: "Clouded Eye".to_owned(),
            effect: "-8 ACC · +1 DMG".to_owned(),
        });
        character.injuries.push(crate::campaign::InjuryRecord {
            id: "fracture".to_owned(),
            name: "Fracture".to_owned(),
            recovery_operations: 1,
        });
        character
            .event_legacies
            .push(crate::campaign::CharacterLegacy {
                id: "legacy".to_owned(),
                name: "A Name Kept".to_owned(),
                stat: "armour".to_owned(),
                amount: 1,
            });
    }

    assert_eq!(record_count(&campaign), 3);
    assert_eq!(
        record_detail(campaign.roster.first().unwrap()),
        "SCAR // Clouded Eye (-8 ACC · +1 DMG) · RECOVERY // Fracture (1 OPS) · +1 MORE"
    );
}

#[test]
fn empty_record_detail_is_explicit() {
    let data = crate::data::GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let character = campaign.roster.first().unwrap();
    assert_eq!(record_detail(character), "NO RECORD DETAIL");
}

#[test]
fn record_count_includes_lost_objectives() {
    let data = crate::data::GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    campaign
        .lost_objectives
        .push(crate::campaign::LostObjectiveRecord {
            id: "lost_route".to_owned(),
            mission_name: "Cold Signal".to_owned(),
            objective: "Reach the relay before the signal collapses.".to_owned(),
            operation: 1,
        });

    assert_eq!(record_count(&campaign), 1);
}

#[test]
fn memorial_pages_keep_long_registers_reachable() {
    let data = crate::data::GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    for operation in 1..=7 {
        campaign
            .lost_objectives
            .push(crate::campaign::LostObjectiveRecord {
                id: format!("lost_route_{operation}"),
                mission_name: format!("Route {operation}"),
                objective: format!("Recover route {operation} before the signal collapses."),
                operation,
            });
    }

    assert_eq!(page_count(&campaign), 2);
    assert_eq!(previous_page(0), 0);
    assert_eq!(previous_page(1), 0);
    assert_eq!(next_page(0, 2), 1);
    assert_eq!(next_page(1, 2), 1);
    assert_eq!(next_page(99, 2), 1);
}
