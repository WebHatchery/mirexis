//! Application integration for serialized first-hour guidance.

use super::{AppState, Game, TacticalTargeting};
use crate::data::{MissionDef, Team};
use crate::state::{Command, GameSession};
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
    pub(super) fn handle_objective_interaction(&mut self) {
        match self.session.interact_selected() {
            Ok(_) => {
                self.campaign.first_hour.touched_objective();
                if self.campaign.first_hour.lesson == crate::first_hour::TacticalLesson::Ability {
                    prepare_first_hour_ability_selection(&mut self.session);
                }
                self.notifications.success("Mission objective secured");
            }
            Err(_) => self
                .notifications
                .warning("A colonist must reach the objective"),
        }
    }

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

    pub(super) fn select_next_ready_for_first_hour(&mut self) -> bool {
        advance_first_hour_selection(
            &mut self.session,
            &mut self.campaign.first_hour,
            &mut self.targeting,
        )
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
                let (help_open, tactical_help_open, battle_log_open) =
                    first_hour_help_overlay_state(
                        self.campaign.first_hour.help_open,
                        self.show_tactical_help,
                        self.show_battle_log,
                    );
                self.campaign.first_hour.help_open = help_open;
                self.show_tactical_help = tactical_help_open;
                self.show_battle_log = battle_log_open;
                self.clear_colony_explorer_motion();
                "Field-guide state and session metrics autosaved"
            }
            UiAction::SkipFirstHourTutorial => {
                self.campaign.first_hour.guidance_enabled = false;
                self.campaign.first_hour.help_open = false;
                self.clear_colony_explorer_motion();
                "Tutorial prompts skipped; campaign goals preserved"
            }
            UiAction::RestartFirstHourTutorial => {
                self.campaign.first_hour.restart();
                self.clear_colony_explorer_motion();
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
        self.autosave_first_hour_action(save_message);
        true
    }

    fn autosave_first_hour_action(&mut self, success_message: &str) {
        if first_hour_action_needs_current_save(self.state) {
            self.autosave_current(success_message);
        } else {
            self.autosave_campaign_only(success_message);
        }
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
        if matches!(
            self.campaign.first_hour.stage,
            crate::first_hour::FirstHourStage::FirstReturn
                | crate::first_hour::FirstHourStage::FirstReturnColony
        ) && self.campaign.colony.resources.materials
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

fn first_hour_action_needs_current_save(state: AppState) -> bool {
    matches!(state, AppState::Tactical | AppState::Debrief)
}

pub(super) fn advance_first_hour_selection(
    session: &mut GameSession,
    progress: &mut crate::first_hour::FirstHourProgress,
    targeting: &mut Option<TacticalTargeting>,
) -> bool {
    *targeting = None;
    let selected = crate::phase_readiness::select_next(session).is_some();
    if selected {
        progress.selected();
    }
    selected
}

fn first_hour_help_overlay_state(
    help_open: bool,
    _tactical_help_open: bool,
    _battle_log_open: bool,
) -> (bool, bool, bool) {
    (!help_open, false, false)
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

pub(super) fn prepare_first_hour_ability_selection(session: &mut GameSession) {
    let selected_has_ability = session
        .tactical
        .selected_unit
        .as_deref()
        .is_some_and(|unit_id| first_hour_ability_available(session, unit_id));
    if selected_has_ability {
        return;
    }

    let candidates = session
        .tactical
        .units
        .iter()
        .filter(|unit| unit.team == Team::Colony && !unit.incapacitated)
        .map(|unit| (unit.id.clone(), unit.position))
        .collect::<Vec<_>>();
    if let Some((unit_id, position)) = candidates
        .into_iter()
        .find(|(unit_id, _)| first_hour_ability_available(session, unit_id))
    {
        session.tactical.selected_unit = Some(unit_id);
        session.tactical.selected_tile = position;
    }
}

fn first_hour_ability_available(session: &GameSession, unit_id: &str) -> bool {
    let Some(unit) = session.unit(unit_id) else {
        return false;
    };
    if unit.team != Team::Colony || unit.incapacitated {
        return false;
    }
    if session
        .validate(&Command::ActivateMutation {
            unit_id: unit_id.to_owned(),
        })
        .is_ok()
    {
        return true;
    }
    let class_action_available = if crate::class_actions::requires_target(&unit.class_id) {
        crate::class_actions::has_valid_target(session, unit_id)
    } else {
        session
            .validate(&Command::ActivateClassAction {
                unit_id: unit_id.to_owned(),
                target_id: None,
                target_tile: None,
            })
            .is_ok()
    };
    if class_action_available {
        return true;
    }
    crate::equipment_actions::available_action(session, unit_id).is_some_and(|equipment_id| {
        crate::equipment_actions::has_valid_target(session, unit_id, &equipment_id)
    })
}

#[cfg(test)]
mod tests;
