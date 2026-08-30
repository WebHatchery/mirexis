//! Application-owned formation selection and Doctrine Yard availability.

use super::Game;

impl Game {
    pub(super) fn normalize_deployment_formation(&mut self) {
        if !self.doctrine_yard_active() {
            self.deployment_formation = self.deployment_formation.without_doctrine_yard();
        }
    }

    pub(super) fn cycle_deployment_formation(&mut self) {
        self.deployment_formation = self
            .deployment_formation
            .next_with_doctrine_yard(self.doctrine_yard_active());
        self.notifications.info(format!(
            "Deployment formation: {}",
            self.deployment_formation.label()
        ));
    }

    fn doctrine_yard_active(&self) -> bool {
        self.campaign.colony.has_active_upgrade(
            crate::colony::BuildingKind::Barracks,
            crate::colony::DOCTRINE_YARD_UPGRADE,
        )
    }
}
