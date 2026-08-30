use super::*;
use crate::colony::{BuildingKind, BuildingState};

fn campaign_with_relay(data: &GameData) -> CampaignState {
    let mut campaign = CampaignState::new(data);
    campaign.colony.buildings.push(BuildingState {
        id: "relay_test".to_owned(),
        kind: BuildingKind::RelayMast,
        position: [2, 10],
        level: 1,
        damaged: false,
    });
    campaign
}

#[test]
fn relay_scan_refreshes_routes_and_exposes_the_dominant_signal() {
    let data = GameData::load().unwrap();
    let mut campaign = campaign_with_relay(&data);
    let power_before = campaign.colony.resources.power;

    assert!(campaign.relay_scan_available());
    assert_eq!(campaign.strategy.mission_offers.len(), 1);
    assert!(campaign.run_relay_scan(&data).is_ok());

    assert_eq!(campaign.colony.resources.power, power_before - 2);
    assert_eq!(campaign.relay_scans_used, 1);
    assert_eq!(campaign.relay_scan_operation, Some(0));
    assert_eq!(
        campaign
            .strategy
            .factions
            .iter()
            .find(|faction| faction.id == "brood")
            .unwrap()
            .attention,
        22
    );
    assert_eq!(campaign.strategy.mission_offers.len(), 2);
    assert!(!campaign.relay_scan_available());
}

#[test]
fn relay_scan_can_be_used_again_after_the_operation_turns_over() {
    let data = GameData::load().unwrap();
    let mut campaign = campaign_with_relay(&data);
    campaign.run_relay_scan(&data).unwrap();
    assert!(campaign.run_relay_scan(&data).is_err());

    campaign.operations_completed = 1;
    assert!(campaign.relay_scan_available());
    campaign.run_relay_scan(&data).unwrap();
    assert_eq!(campaign.relay_scans_used, 2);
    assert_eq!(campaign.relay_scan_operation, Some(1));
}
