use super::*;
use crate::colony::BuildingKind;

#[test]
fn explorer_routes_around_colony_buildings() {
    let colony = ColonyState::new();
    let route = pathfind([9, 10], [11, 10], &colony, &[]).expect("route around command centre");
    assert!(!route.contains(&[10, 10]));
    assert_eq!(route.back(), Some(&[11, 10]));
}

#[test]
fn approaching_an_npc_ends_on_an_adjacent_open_plot() {
    let data = crate::data::GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let mut explorer = ColonyExplorer::default();
    let npc = &campaign.roster[1];
    let station = npc_position(&campaign, &npc.id).unwrap();
    explorer.request_approach(
        &npc.id,
        station,
        &campaign.colony,
        &npc_positions(&campaign),
    );
    while let Some(position) = explorer.route.pop_front() {
        explorer.position = position;
    }
    assert!(adjacent(explorer.position, station));
}

#[test]
fn construction_plots_are_not_walkable() {
    let mut colony = ColonyState::new();
    colony
        .place_construction(BuildingKind::Barricade, [3, 3])
        .unwrap();
    assert!(pathfind([9, 10], [3, 3], &colony, &[]).is_none());
}
