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
fn approaching_an_npc_ends_on_an_adjacent_open_plot() {
    let data = crate::data::GameData::load().unwrap();
    let campaign = CampaignState::new(&data);
    let mut explorer = ColonyExplorer::default();
    let npc = &campaign.roster[1];
    let station = npc_position(&campaign, &npc.id).unwrap();
    explorer.request_approach(&npc.id, station, &campaign.colony);
    for _ in 0..60 {
        explorer.update(0.05, &campaign.colony);
        explorer.update_approach(&campaign);
    }
    assert!(explorer.position.distance(station) <= INTERACTION_DISTANCE);
    assert!(explorer.is_talking());
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
