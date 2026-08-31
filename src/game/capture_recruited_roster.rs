//! Deterministic recruited-roster scenes for list-layout regression coverage.

use super::{AppState, Game, TacticalTargeting};
use crate::colony::{BuildingKind, BuildingState};
use macroquad_toolkit::grid::TilePos;

impl Game {
    pub(super) fn capture_ninth_recruitment(&mut self) {
        self.prepare_ninth_recruitment();
        self.colony_operations_open = true;
        self.state = AppState::Colony;
    }

    pub(super) fn capture_ninth_roster(&mut self) {
        self.prepare_ninth_recruitment();
        self.campaign
            .recruit_outsider(&self.data)
            .expect("Ninth roster capture recruits the severed Brood");
        self.campaign.selected_character_id = "ninth_voice_apart".to_owned();
        self.state = AppState::Roster;
    }

    pub(super) fn capture_ninth_resonance(&mut self) {
        self.prepare_ninth_recruitment();
        self.campaign
            .recruit_outsider(&self.data)
            .expect("Ninth tactical capture recruits the severed Brood");
        self.campaign
            .roster
            .iter_mut()
            .find(|character| character.deployment_selected)
            .expect("Ninth tactical capture has a selected colonist")
            .deployment_selected = false;
        self.campaign
            .roster
            .iter_mut()
            .find(|character| character.id == "ninth_voice_apart")
            .expect("Ninth tactical capture includes Ninth")
            .deployment_selected = true;
        self.reset_capture_session(AppState::Tactical);
        let ninth_position = self
            .session
            .unit("ninth_voice_apart")
            .expect("Ninth tactical capture deploys Ninth")
            .position;
        let hostile = self
            .session
            .tactical
            .units
            .iter_mut()
            .find(|unit| unit.team == crate::data::Team::Hostile)
            .expect("Ninth tactical capture has a hostile");
        hostile.position = TilePos::new(ninth_position.x + 3, ninth_position.y);
        self.session.tactical.selected_unit = Some("ninth_voice_apart".to_owned());
        self.session.tactical.selected_tile = hostile.position;
        self.targeting = Some(TacticalTargeting::Equipment {
            unit_id: "ninth_voice_apart".to_owned(),
            equipment_id: "severed_resonance".to_owned(),
        });
    }

    pub(super) fn capture_recruited_briefing(&mut self) {
        self.capture_recruited_roster(AppState::MissionBriefing);
    }

    pub(super) fn capture_recruited_roster_screen(&mut self) {
        self.capture_recruited_roster(AppState::Roster);
    }

    pub(super) fn capture_recruited_roster_info(&mut self) {
        self.capture_recruited_roster(AppState::Roster);
        self.roster_inspection_id = Some("mireborn_sense".to_owned());
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

    fn prepare_ninth_recruitment(&mut self) {
        self.campaign.strategy.contact_protocol_id = "brood_cultivation".to_owned();
        for event in &mut self.campaign.strategy.character_events {
            event.resolved = true;
        }
        self.campaign.colony.resources.biomass = 24;
        self.campaign.colony.buildings.push(BuildingState {
            id: "capture_ninth_waystation".to_owned(),
            kind: BuildingKind::Waystation,
            position: [2, 10],
            level: 1,
            damaged: false,
        });
    }
}
