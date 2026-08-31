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
}

#[test]
fn empty_register_stays_on_one_page() {
    let data = crate::data::GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    assert_eq!(page_count(&campaign), 1);
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

#[test]
fn memorial_stream_keeps_each_character_record_separate() {
    let data = crate::data::GameData::load().unwrap();
    let mut campaign = CampaignState::new(&data);
    let character = campaign.roster.first_mut().unwrap();
    character.traumas.extend([
        crate::trauma::TraumaRecord {
            id: "clouded_eye".to_owned(),
            name: "Clouded Eye".to_owned(),
            effect: "-8 ACC".to_owned(),
        },
        crate::trauma::TraumaRecord {
            id: "shallow_breath".to_owned(),
            name: "Shallow Breath".to_owned(),
            effect: "-1 AP".to_owned(),
        },
    ]);
    character
        .event_legacies
        .push(crate::campaign::CharacterLegacy {
            id: "route_held".to_owned(),
            name: "The Route Held".to_owned(),
            stat: "movement".to_owned(),
            amount: 1,
        });

    let entries = memorial_entries(&campaign);
    assert_eq!(entries.len(), 3);
    assert!(matches!(
        entries[0],
        MemorialEntry::Character {
            detail: CharacterDetail::Trauma(_),
            ..
        }
    ));
    assert!(matches!(
        entries[1],
        MemorialEntry::Character {
            detail: CharacterDetail::Trauma(_),
            ..
        }
    ));
    assert!(matches!(
        entries[2],
        MemorialEntry::Character {
            detail: CharacterDetail::Legacy(_),
            ..
        }
    ));
}
