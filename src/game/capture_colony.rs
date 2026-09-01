//! Deterministic colony construction, damage, power, and repair showcase scenes.

use super::{AppState, Game};
use crate::colony::BuildingKind;

impl Game {
    pub(super) fn capture_colony_damage(&mut self) {
        self.campaign
            .colony
            .buildings
            .iter_mut()
            .find(|building| building.id == "workshop")
            .expect("capture colony has a workshop")
            .damaged = true;
        self.state = AppState::Colony;
    }

    pub(super) fn capture_colony_repair(&mut self) {
        self.capture_colony_damage();
        let (name, cost) = self
            .campaign
            .colony
            .repair_building("workshop")
            .expect("capture colony can repair its workshop");
        self.notifications
            .success(format!("{name} repaired for {cost} materials"));
    }

    pub(super) fn capture_power_outage(&mut self) {
        self.campaign
            .colony
            .buildings
            .iter_mut()
            .find(|building| building.id == "power_plant")
            .expect("capture colony has a power plant")
            .damaged = true;
        self.state = AppState::Colony;
    }

    pub(super) fn capture_power_construction(&mut self) {
        self.campaign
            .colony
            .select_construction(BuildingKind::PowerPlant)
            .expect("power plants are constructible");
        self.campaign
            .colony
            .place_construction(BuildingKind::PowerPlant, [10, 14])
            .expect("capture plot is open");
        self.state = AppState::Colony;
    }

    pub(super) fn capture_facility_upgrades(&mut self) {
        self.campaign.colony.ensure_gene_lab();
        self.campaign.colony.resources.materials = 120;
        self.campaign.colony.resources.power = 20;
        self.colony_operations_open = true;
        self.facility_upgrade_open = true;
        self.state = AppState::Colony;
    }
}
