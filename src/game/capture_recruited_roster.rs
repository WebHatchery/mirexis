//! Deterministic recruited-roster scenes for list-layout regression coverage.

use super::{AppState, Game};
use crate::colony::{BuildingKind, BuildingState};

impl Game {
    pub(super) fn capture_recruited_briefing(&mut self) {
        self.capture_recruited_roster(AppState::MissionBriefing);
    }

    pub(super) fn capture_recruited_roster_screen(&mut self) {
        self.capture_recruited_roster(AppState::Roster);
    }

    pub(super) fn capture_recruited_gene_lab(&mut self) {
        self.capture_recruited_roster(AppState::GeneLab);
    }

    fn capture_recruited_roster(&mut self, state: AppState) {
        self.campaign.strategy.contact_protocol_id = "directorate_requisition".to_owned();
        self.campaign.colony.buildings.push(BuildingState {
            id: "capture_waystation".to_owned(),
            kind: BuildingKind::Waystation,
            position: [2, 10],
            level: 1,
            damaged: false,
        });
        self.campaign
            .recruit_outsider(&self.data)
            .expect("recruited roster capture includes Veya");
        self.campaign.strategy.phase_id = "adaptation".to_owned();
        self.campaign.strategy.contact_complete = true;
        self.campaign
            .recruit_outsider(&self.data)
            .expect("recruited roster capture includes Sedge");
        self.campaign.colony.resources.materials = 240;
        self.campaign.colony.resources.biomass = 20;
        self.campaign.colony.resources.power = 12;
        if state == AppState::GeneLab {
            self.campaign.colony.buildings.push(BuildingState {
                id: "capture_gene_lab".to_owned(),
                kind: BuildingKind::GeneLab,
                position: [1, 1],
                level: 1,
                damaged: false,
            });
        }
        self.campaign
            .roster
            .iter_mut()
            .find(|character| character.id == "kira_voss")
            .expect("recruited roster capture includes Kira")
            .deployment_selected = false;
        self.campaign
            .roster
            .iter_mut()
            .find(|character| character.id == "sedge")
            .expect("recruited roster capture includes Sedge")
            .deployment_selected = true;
        self.campaign.selected_character_id = "sedge".to_owned();
        self.state = state;
    }
}
