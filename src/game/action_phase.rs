//! Tactical phase, replay, log, and persistence actions.

use super::*;

pub(super) fn apply_phase_action(game: &mut Game, action: UiAction) -> bool {
    match action {
        UiAction::EndPhase => {
            let ready = crate::phase_readiness::ready_count(&game.session);
            if ready > 0 && !game.end_phase_armed {
                game.end_phase_armed = true;
                game.notifications.warning(format!(
                    "{} colonist{} still ready · end phase again to confirm",
                    ready,
                    if ready == 1 { "" } else { "s" }
                ));
                return true;
            }
            game.end_phase_armed = false;
            game.targeting = None;
            let first_event = game.session.tactical.event_log.len();
            game.session.end_player_phase(&game.data.config);
            game.campaign.first_hour.ended_phase();
            game.phase_replay
                .start(&game.session.tactical.event_log[first_event..]);
            game.notifications.info(format!(
                "Enemy activity resolved — round {}",
                game.session.tactical.round
            ));
            true
        }
        UiAction::SkipPhaseReplay => {
            game.phase_replay.clear();
            true
        }
        UiAction::ToggleTacticalHelp => {
            game.show_tactical_help = !game.show_tactical_help;
            game.show_battle_log = false;
            game.campaign.first_hour.help_open = false;
            game.targeting = None;
            true
        }
        UiAction::ToggleBattleLog => {
            game.show_battle_log = !game.show_battle_log;
            game.show_tactical_help = false;
            game.campaign.first_hour.help_open = false;
            game.targeting = None;
            true
        }
        UiAction::ToggleTacticalPanel => {
            game.tactical_panel_open = !game.tactical_panel_open;
            if !game.tactical_panel_open {
                game.targeting = None;
            }
            true
        }
        UiAction::SetBattleLogFilter(filter) => {
            game.battle_log_filter = filter;
            true
        }
        UiAction::Save => {
            game.save_game();
            true
        }
        UiAction::Load => {
            game.load_game();
            true
        }
        _ => false,
    }
}
