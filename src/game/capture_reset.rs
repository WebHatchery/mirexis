//! Fresh capture state while retaining the process-owned render assets.

use super::Game;
use crate::campaign::CampaignState;
use crate::combat_feedback::CombatFeedback;
use crate::formation::FormationKind;
use crate::grid_ui::WorldCamera;
use crate::phase_replay::PhaseReplay;
use crate::state::GameSession;
use macroquad_toolkit::events::EventBus;
use macroquad_toolkit::notifications::NotificationManager;

impl Game {
    pub(super) fn reset_capture_world(&mut self) {
        let campaign = CampaignState::new(&self.data);
        let active_mission = campaign
            .strategy
            .materialize_selected(&self.data, &campaign.colony);
        let session = GameSession::new(
            &self.data.config,
            &active_mission,
            &campaign.deployment_roster(&self.data, &active_mission),
        );

        self.session = session;
        self.campaign = campaign;
        self.campaign.first_hour.stage = crate::first_hour::FirstHourStage::Complete;
        self.active_mission = active_mission;
        self.notifications = NotificationManager::new();
        self.events = EventBus::new();
        self.last_outcome = None;
        self.autosave = Default::default();
        self.targeting = None;
        self.show_tactical_help = false;
        self.show_battle_log = false;
        self.combat_feedback = CombatFeedback::default();
        self.observed_event_count = 0;
        self.phase_replay = PhaseReplay::default();
        self.deployment_formation = FormationKind::default();
        self.end_phase_armed = false;
        self.title_hover_preview = false;
        self.tactical_camera = WorldCamera::tactical_start(self.session.tactical.selected_tile);
        self.colony_camera = WorldCamera::colony_start(crate::colony::SETTLEMENT_CENTER);
    }
}
