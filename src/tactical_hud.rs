//! Minimal tactical chrome and the on-demand command drawer.

use crate::state::TacticalPhase;
use crate::ui::{draw_ui_text_ex, UiAction, UiContext};
use crate::ui_widgets::{action_status, button, event_summary};
use macroquad::prelude::*;
use macroquad_toolkit::prelude::{
    dark, draw_badge, draw_surface, draw_surface_with_title, meter, SurfaceStyle, TextStyle,
};

const EDGE_X: f32 = 1184.0;
const EDGE_WIDTH: f32 = 80.0;
const EDGE_BUTTON_HEIGHT: f32 = 38.0;
const EDGE_GAP: f32 = 6.0;

pub(crate) fn command_button_bounds() -> Rect {
    Rect::new(EDGE_X, 126.0, EDGE_WIDTH, EDGE_BUTTON_HEIGHT)
}

pub(crate) fn draw(ctx: &UiContext<'_>, mouse: Vec2, actions: &mut Vec<UiAction>) {
    draw_world_chrome(ctx);
    draw_edge_controls(ctx, mouse, actions);
    if ctx.tactical_panel_open {
        draw_command_panel(ctx, mouse, actions);
    }
}

fn draw_world_chrome(ctx: &UiContext<'_>) {
    let identity = Rect::new(24.0, 18.0, 350.0, 42.0);
    draw_surface(
        identity,
        &SurfaceStyle::new(Color::new(0.025, 0.055, 0.060, 0.88))
            .with_border(1.0, Color::new(0.28, 0.78, 0.66, 0.82))
            .with_left_accent(4.0, Color::new(0.34, 0.86, 0.68, 1.0)),
    );
    draw_ui_text_ex(
        "MIREXIS",
        identity.x + 16.0,
        identity.y + 27.0,
        TextStyle::new(17.0, dark::TEXT_BRIGHT).params(),
    );
    draw_ui_text_ex(
        &ctx.mission.name,
        identity.x + 112.0,
        identity.y + 19.0,
        TextStyle::new(12.0, dark::TEXT).params(),
    );
    draw_ui_text_ex(
        "TACTICAL FIELD // PLANNING",
        identity.x + 112.0,
        identity.y + 33.0,
        TextStyle::new(9.0, Color::new(0.38, 0.72, 0.67, 1.0)).params(),
    );

    let objective = Rect::new(24.0, 68.0, 500.0, 28.0);
    draw_surface(
        objective,
        &SurfaceStyle::new(Color::new(0.025, 0.055, 0.060, 0.76))
            .with_border(1.0, Color::new(0.20, 0.44, 0.42, 0.72)),
    );
    draw_ui_text_ex(
        &format!(
            "OBJECTIVE // {}",
            crate::objective_ui::progress(ctx.session, ctx.mission)
        ),
        objective.x + 10.0,
        objective.y + 19.0,
        TextStyle::new(11.0, Color::new(0.48, 0.90, 0.72, 1.0)).params(),
    );

    let phase = match ctx.session.tactical.phase {
        TacticalPhase::Player => "COLONY PHASE",
        TacticalPhase::Enemy => "HOSTILE PHASE",
    };
    draw_badge(
        Rect::new(864.0, 22.0, 112.0, 30.0),
        &format!("ROUND {}", ctx.session.tactical.round),
        Color::new(0.08, 0.18, 0.20, 0.92),
        dark::TEXT,
    );
    draw_badge(
        Rect::new(982.0, 22.0, 188.0, 30.0),
        phase,
        Color::new(0.08, 0.25, 0.21, 0.92),
        dark::TEXT,
    );
    if let Some(selected) = ctx.session.selected_unit() {
        draw_ui_text_ex(
            &format!(
                "SELECTED // {} · {} AP",
                selected.name, selected.action_points
            ),
            548.0,
            42.0,
            TextStyle::new(11.0, dark::TEXT_DIM).params(),
        );
    }
    crate::first_hour_investment_ui::draw_tactical_summary(ctx.first_hour, vec2(548.0, 56.0));
}

fn draw_edge_controls(ctx: &UiContext<'_>, mouse: Vec2, actions: &mut Vec<UiAction>) {
    draw_ui_text_ex(
        "VIEW",
        EDGE_X + 22.0,
        116.0,
        TextStyle::new(9.0, Color::new(0.40, 0.72, 0.68, 1.0)).params(),
    );
    edge_button(
        command_button_bounds(),
        if ctx.tactical_panel_open {
            "CLOSE"
        } else {
            "COMMAND"
        },
        mouse,
        actions,
        UiAction::ToggleTacticalPanel,
    );
    edge_button(
        edge_button_bounds(1),
        "LOG",
        mouse,
        actions,
        UiAction::ToggleBattleLog,
    );
    edge_button(
        edge_button_bounds(2),
        "GUIDE",
        mouse,
        actions,
        UiAction::ToggleTacticalHelp,
    );
    edge_button(
        edge_button_bounds(3),
        "AUDIO",
        mouse,
        actions,
        UiAction::ToggleSettings,
    );
    if button(
        edge_button_bounds(4),
        "NEXT",
        crate::phase_readiness::ready_count(ctx.session) > 0,
        mouse,
    ) {
        actions.push(UiAction::SelectNextReady);
    }

    if button(edge_button_bounds(5), "SAVE", true, mouse) {
        actions.push(UiAction::Save);
    }
    if button(edge_button_bounds(6), "LOAD", ctx.save_exists, mouse) {
        actions.push(UiAction::Load);
    }
    if button(edge_button_bounds(7), "TITLE", true, mouse) {
        actions.push(UiAction::ReturnToTitle);
    }
    if button(
        edge_button_bounds(8),
        if ctx.delete_save_armed {
            "CONFIRM"
        } else {
            "DELETE"
        },
        ctx.save_exists,
        mouse,
    ) {
        actions.push(UiAction::DeleteSave);
    }
    draw_ui_text_ex(
        "TAP WORLD TO INSPECT",
        EDGE_X + 3.0,
        688.0,
        TextStyle::new(8.0, dark::TEXT_DIM).params(),
    );
}

fn edge_button_bounds(index: usize) -> Rect {
    let first = command_button_bounds();
    Rect::new(
        EDGE_X,
        first.y + index as f32 * (EDGE_BUTTON_HEIGHT + EDGE_GAP),
        EDGE_WIDTH,
        EDGE_BUTTON_HEIGHT,
    )
}

fn edge_button(
    rect: Rect,
    label: &str,
    mouse: Vec2,
    actions: &mut Vec<UiAction>,
    action: UiAction,
) {
    if button(rect, label, true, mouse) {
        actions.push(action);
    }
}

fn draw_command_panel(ctx: &UiContext<'_>, mouse: Vec2, actions: &mut Vec<UiAction>) {
    let panel = crate::ui::tactical_command_panel_rect();
    draw_surface_with_title(
        panel,
        Some("COMMAND DRAWER"),
        &SurfaceStyle::new(Color::new(0.035, 0.065, 0.070, 0.985))
            .with_border(2.0, Color::new(0.32, 0.78, 0.66, 0.98))
            .with_inner_border(6.0, 1.0, Color::new(0.17, 0.33, 0.32, 0.75))
            .with_header(38.0, Color::new(0.07, 0.12, 0.125, 1.0)),
        TextStyle::new(16.0, dark::TEXT),
    );
    let close = Rect::new(panel.right() - 70.0, panel.y + 6.0, 56.0, 25.0);
    if button(close, "CLOSE", true, mouse) {
        actions.push(UiAction::ToggleTacticalPanel);
    }

    let x = panel.x + 18.0;
    crate::objective_ui::draw_summary(ctx.session, ctx.mission, x, panel);
    let selected = ctx.session.selected_unit();
    if !crate::enemy_intent_ui::draw_inspector(
        ctx.session,
        ctx.data.config.max_action_points,
        panel,
        ctx.assets,
        ctx.visuals,
    ) {
        draw_selected_unit(ctx, panel, x, selected);
    }

    let readiness = crate::phase_readiness::counts(ctx.session);
    draw_ui_text_ex(
        &format!(
            "ACTION STATUS // READY {} · SPENT {} · INCAP {}",
            readiness.ready, readiness.spent, readiness.incapacitated
        ),
        x,
        panel.y + 364.0,
        TextStyle::new(10.0, Color::new(0.46, 0.68, 0.66, 1.0)).params(),
    );
    draw_objective_action(ctx, panel, selected, mouse, actions);
    crate::skill_ui::draw_action_buttons(
        ctx,
        selected,
        Rect::new(x, panel.bottom() - 190.0, panel.w - 36.0, 34.0),
        mouse,
        actions,
    );
    draw_ability_actions(ctx, panel, selected, x, mouse, actions);
    draw_phase_actions(ctx, panel, selected, x, mouse, actions);
    draw_ui_text_ex(
        &format!(
            "MATERIALS {} // ROUND LIMIT {}",
            ctx.session.tactical.materials, ctx.mission.round_limit
        ),
        x,
        panel.bottom() - 36.0,
        TextStyle::new(12.0, dark::TEXT_DIM).params(),
    );
    if let Some(event) = ctx.session.tactical.event_log.last() {
        draw_ui_text_ex(
            &event_summary(event),
            x,
            panel.bottom() - 14.0,
            TextStyle::new(11.0, dark::TEXT_DIM).params(),
        );
    }
}

fn draw_selected_unit(
    ctx: &UiContext<'_>,
    panel: Rect,
    x: f32,
    selected: Option<&crate::state::UnitState>,
) {
    draw_ui_text_ex(
        "SELECTED COLONIST",
        x,
        panel.y + 164.0,
        TextStyle::new(14.0, Color::new(0.43, 0.83, 0.69, 1.0)).params(),
    );
    let Some(unit) = selected else {
        draw_ui_text_ex(
            "Select a colonist on the field",
            x,
            panel.y + 210.0,
            TextStyle::new(16.0, dark::TEXT_DIM).params(),
        );
        return;
    };
    let weapon_name = unit
        .equipment_ids
        .iter()
        .filter_map(|id| ctx.data.equipment.iter().find(|item| &item.id == id))
        .find(|item| item.slot == "primary")
        .map_or("Unarmed", |item| item.name.as_str());
    crate::portrait_ui::draw_character_portrait(
        ctx.assets,
        ctx.visuals,
        Rect::new(x, panel.y + 174.0, 88.0, 96.0),
        &unit.id,
        &unit.name,
        Color::new(0.28, 0.88, 0.72, 1.0),
    );
    let detail_x = x + 102.0;
    draw_ui_text_ex(
        &unit.name,
        detail_x,
        panel.y + 198.0,
        TextStyle::new(21.0, dark::TEXT_BRIGHT).params(),
    );
    draw_ui_text_ex(
        &format!("{} // {}", unit.role, unit.mutation),
        detail_x,
        panel.y + 222.0,
        TextStyle::new(12.0, dark::TEXT_DIM).params(),
    );
    draw_ui_text_ex(
        &format!(
            "{} // {} DMG · R{} · {} AP · A{} · M{}",
            weapon_name,
            unit.effective_weapon_damage(),
            unit.weapon_range,
            unit.weapon_ap_cost,
            unit.effective_armour(),
            unit.move_range
        ),
        detail_x,
        panel.y + 242.0,
        TextStyle::new(10.0, dark::ACCENT).params(),
    );
    meter(
        Rect::new(x, panel.y + 278.0, panel.w - 36.0, 20.0),
        unit.health as f32,
        unit.max_health as f32,
        dark::POSITIVE,
        Some(&format!("VITALS {}/{}", unit.health, unit.max_health)),
    );
    meter(
        Rect::new(x, panel.y + 306.0, panel.w - 36.0, 20.0),
        unit.action_points as f32,
        ctx.data.config.max_action_points as f32,
        Color::new(0.33, 0.65, 0.92, 1.0),
        Some(&format!("ACTION POINTS {}", unit.action_points)),
    );
    draw_ui_text_ex(
        &action_status(unit),
        x,
        panel.y + 340.0,
        TextStyle::new(10.0, dark::TEXT_DIM).params(),
    );
}

fn draw_objective_action(
    ctx: &UiContext<'_>,
    panel: Rect,
    selected: Option<&crate::state::UnitState>,
    mouse: Vec2,
    actions: &mut Vec<UiAction>,
) {
    let enabled = ctx.session.can_interact_selected();
    let label = crate::objective_ui::interaction_label(
        ctx.mission.objective_kind,
        ctx.session.tactical.objective_state,
        selected,
        selected.is_some_and(|unit| {
            crate::tactical::manhattan(unit.position, ctx.session.tactical.objective_tile) <= 1
        }),
        enabled,
    );
    if button(
        crate::objective_ui::action_button_bounds(panel),
        label,
        enabled,
        mouse,
    ) {
        actions.push(UiAction::InteractObjective);
    }
}

fn draw_ability_actions(
    ctx: &UiContext<'_>,
    panel: Rect,
    selected: Option<&crate::state::UnitState>,
    x: f32,
    mouse: Vec2,
    actions: &mut Vec<UiAction>,
) {
    let action_width = (panel.w - 52.0) / 3.0;
    if button(
        Rect::new(x, panel.bottom() - 148.0, action_width, 34.0),
        crate::ui::mutation_button_label(selected, ctx.session.can_activate_selected_mutation()),
        ctx.session.can_activate_selected_mutation(),
        mouse,
    ) {
        actions.push(UiAction::ActivateMutation);
    }
    crate::class_action_ui::draw_action_button(
        ctx,
        selected,
        Rect::new(
            x + action_width + 8.0,
            panel.bottom() - 148.0,
            action_width,
            34.0,
        ),
        mouse,
        actions,
    );
    crate::equipment_ui::draw_action_button(
        ctx,
        selected,
        Rect::new(
            x + (action_width + 8.0) * 2.0,
            panel.bottom() - 148.0,
            action_width,
            34.0,
        ),
        mouse,
        actions,
    );
}

fn draw_phase_actions(
    ctx: &UiContext<'_>,
    panel: Rect,
    selected: Option<&crate::state::UnitState>,
    x: f32,
    mouse: Vec2,
    actions: &mut Vec<UiAction>,
) {
    let phase_width = (panel.w - 44.0) * 0.46;
    let ready = crate::phase_readiness::ready_count(ctx.session);
    let end_phase_label = if ctx.end_phase_armed {
        format!("CONFIRM END · {} READY", ready)
    } else if ready > 0 {
        format!("END PHASE · {} READY", ready)
    } else {
        "END COLONY PHASE".to_owned()
    };
    let overwatch_enabled = ctx.session.can_set_selected_overwatch();
    if button(
        Rect::new(x, panel.bottom() - 104.0, phase_width, 44.0),
        crate::overwatch::overwatch_button_label(selected, overwatch_enabled),
        overwatch_enabled,
        mouse,
    ) {
        actions.push(UiAction::SetOverwatch);
    }
    if button(
        crate::ui::end_phase_button_bounds(panel),
        &end_phase_label,
        true,
        mouse,
    ) {
        actions.push(UiAction::EndPhase);
    }
}
