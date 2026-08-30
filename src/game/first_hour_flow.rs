//! Application integration for serialized first-hour guidance.

use super::{AppState, Game};
use crate::data::{MissionDef, Team};
use crate::state::GameSession;
use crate::ui::{self, UiAction};
use macroquad_toolkit::grid::TilePos;
use macroquad_toolkit::ui::VirtualUi;

const FIRST_HOUR_COVER_POSITION: [i32; 2] = [6, 18];
const FIRST_HOUR_ATTACK_POSITION: [i32; 2] = [12, 18];
const FIRST_HOUR_OBJECTIVE_POSITION: [i32; 2] = [18, 22];

fn first_hour_attack_position() -> TilePos {
    TilePos::new(FIRST_HOUR_ATTACK_POSITION[0], FIRST_HOUR_ATTACK_POSITION[1])
}

fn first_hour_objective_position() -> TilePos {
    TilePos::new(
        FIRST_HOUR_OBJECTIVE_POSITION[0],
        FIRST_HOUR_OBJECTIVE_POSITION[1],
    )
}

impl Game {
    pub(super) fn create_first_hour_session(
        &self,
        roster: &[crate::data::UnitDef],
    ) -> crate::state::GameSession {
        let mut session =
            crate::state::GameSession::new(&self.data.config, &self.active_mission, roster);
        if self.campaign.first_hour.stage == crate::first_hour::FirstHourStage::FirstOperation {
            prepare_first_hour_tactical_session(&mut session, &self.active_mission);
        }
        session
    }

    pub(super) fn record_first_hour_move(&mut self, tile: TilePos) {
        let on_cover =
            crate::cover_rules::is_cover_position(&self.session.tactical.cover_edges, tile);
        self.campaign.first_hour.moved(on_cover);
    }

    pub(super) fn draw_first_hour(&self, ui: &VirtualUi, actions: &mut Vec<UiAction>) {
        if self.state != AppState::Title
            && !(self.campaign.first_hour.stage == crate::first_hour::FirstHourStage::Complete
                && self.state != AppState::Colony)
        {
            crate::first_hour_ui::draw(
                &self.campaign.first_hour,
                ui::pointer_position(ui),
                actions,
            );
        }
    }

    pub(super) fn apply_first_hour_action(&mut self, action: &UiAction) -> bool {
        let save_message = match action {
            UiAction::AdvanceFirstHour => {
                self.campaign.first_hour.advance_arrival();
                "First-hour goal autosaved"
            }
            UiAction::AcknowledgeColonist(character_id) => {
                self.campaign.acknowledge_colonist(character_id);
                "Colony introduction autosaved"
            }
            UiAction::ToggleFirstHourHelp => {
                if !self.campaign.first_hour.help_open {
                    self.campaign.first_hour.metrics.opened_guide();
                }
                self.campaign.first_hour.help_open = !self.campaign.first_hour.help_open;
                "Field-guide state and session metrics autosaved"
            }
            UiAction::SkipFirstHourTutorial => {
                self.campaign.first_hour.guidance_enabled = false;
                self.campaign.first_hour.help_open = false;
                "Tutorial prompts skipped; campaign goals preserved"
            }
            UiAction::RestartFirstHourTutorial => {
                self.campaign
                    .first_hour
                    .restart(self.campaign.operations_completed);
                "First-hour guide restarted"
            }
            UiAction::ChooseFirstHourInvestment(investment_id) => {
                if self.campaign.first_hour.stage
                    != crate::first_hour::FirstHourStage::MakeInvestment
                {
                    self.notifications
                        .warning("The preparation window is closed");
                    return true;
                }
                let cost = crate::first_hour_investment_ui::INVESTMENT_COST;
                if self.campaign.colony.resources.materials < cost {
                    self.notifications
                        .warning("Recover 24 materials before choosing a preparation");
                    return true;
                }
                let label = crate::first_hour_investment_ui::label(investment_id);
                if label == "Unknown preparation" {
                    self.notifications.warning("Unknown first-hour preparation");
                    return true;
                }
                self.campaign.colony.resources.materials -= cost;
                self.campaign.first_hour.invested(investment_id.clone());
                self.notifications
                    .success(format!("{label} prepared for the second operation"));
                "First-hour investment autosaved"
            }
            _ => return false,
        };
        self.autosave_campaign_only(save_message);
        true
    }

    pub(super) fn record_first_hour_outcome(&mut self) {
        let won = self
            .last_outcome
            .as_ref()
            .is_some_and(|outcome| outcome.result == crate::state::ObjectiveState::Victory);
        self.campaign.first_hour.operation_resolved(
            self.campaign.operations_completed,
            won,
            self.session.tactical.round,
        );
    }

    pub(super) fn ensure_first_hour_recovery_reserve(&mut self) {
        if self.campaign.first_hour.stage == crate::first_hour::FirstHourStage::FirstReturn
            && self.campaign.colony.resources.materials
                < crate::first_hour_investment_ui::INVESTMENT_COST
        {
            self.campaign.colony.resources.materials =
                crate::first_hour_investment_ui::INVESTMENT_COST;
            self.notifications
                .info("Emergency stores restored 24 materials for one viable preparation");
        }
    }

    pub(super) fn first_hour_ability_success(
        &mut self,
        events: Vec<crate::tactical::BattleEvent>,
        fallback: &str,
    ) {
        self.campaign.first_hour.used_ability();
        self.notifications.success(
            events
                .first()
                .map(crate::ui_widgets::event_summary)
                .unwrap_or_else(|| fallback.to_owned()),
        );
    }
}

pub(super) fn prepare_first_hour_tactical_session(session: &mut GameSession, mission: &MissionDef) {
    if !session
        .tactical
        .cover_edges
        .iter()
        .any(|edge| edge.position == FIRST_HOUR_COVER_POSITION)
    {
        session
            .tactical
            .cover_edges
            .push(crate::data::CoverEdgeDef {
                position: FIRST_HOUR_COVER_POSITION,
                direction: crate::data::EdgeDirection::North,
                strength: 25,
            });
    }

    let Some(hostile_id) = session
        .tactical
        .units
        .iter()
        .find(|unit| {
            unit.team == Team::Hostile && unit.faction.as_deref() == Some(&mission.hostile_faction)
        })
        .map(|unit| unit.id.clone())
    else {
        return;
    };
    if session
        .tactical
        .units
        .iter()
        .any(|unit| unit.id != hostile_id && unit.position == first_hour_attack_position())
        || session
            .tactical
            .blocked
            .contains(&first_hour_attack_position())
        || session
            .tactical
            .hazards
            .iter()
            .any(|hazard| hazard.position == first_hour_attack_position())
        || session.tactical.objective_tile == first_hour_attack_position()
    {
        return;
    }
    if let Some(hostile) = session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == hostile_id)
    {
        hostile.position = first_hour_attack_position();
    }

    let objective = first_hour_objective_position();
    if session.tactical.fog.is_valid(objective)
        && !session.tactical.blocked.contains(&objective)
        && !session
            .tactical
            .hazards
            .iter()
            .any(|hazard| hazard.position == objective)
        && !session
            .tactical
            .units
            .iter()
            .any(|unit| unit.position == objective)
    {
        session.tactical.objective_tile = objective;
    }
}

#[cfg(test)]
mod tests;
