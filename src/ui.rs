//! Immediate-mode title and tactical presentation.

use crate::campaign::CampaignState;
use crate::data::{GameData, MissionDef, ObjectiveKind};
use crate::state::{GameSession, ObjectiveState, TacticalPhase};
use crate::ui_widgets::{action_status, button, event_summary};
use crate::visual_assets::VisualCatalog;
use macroquad::prelude::*;
use macroquad_toolkit::assets::AssetManager;
use macroquad_toolkit::prelude::*;
use macroquad_toolkit::ui::VirtualUi;

pub use crate::ui_action::UiAction;

pub const LOGICAL_WIDTH: f32 = 1280.0;
pub const LOGICAL_HEIGHT: f32 = 720.0;

// Tactical screens combine dense prose, meters, fitted controls, and notifications.
// Split ordinary labels and prose onto Macroquad's built-in atlas so neither font atlas
// is exhausted by a long-running operation or deterministic capture.
pub(crate) fn draw_ui_text_ex<'a>(
    text: &str,
    x: f32,
    y: f32,
    mut params: TextParams<'a>,
) -> TextDimensions {
    params.font = None;
    draw_text_ex(text, x, y, params)
}

pub(crate) fn set_ui_clip(ui: &VirtualUi, rect: Option<Rect>) {
    let clip = rect.map(|rect| {
        let dpi = screen_dpi_scale();
        let x = (ui.offset.x + rect.x * ui.scale) * dpi;
        let top = (ui.offset.y + rect.y * ui.scale) * dpi;
        let width = rect.w * ui.scale * dpi;
        let height = rect.h * ui.scale * dpi;
        (
            x.round() as i32,
            top.round() as i32,
            width.round() as i32,
            height.round() as i32,
        )
    });
    // Macroquad's scissor uses physical framebuffer pixels; callers use the
    // logical coordinate system established by `VirtualUi`.
    unsafe {
        get_internal_gl().quad_gl.scissor(clip);
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_text_block(
    text: &str,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    font_size: f32,
    line_gap: f32,
    color: Color,
) -> TextLayoutResult {
    draw_text_block_ex(
        text,
        x,
        y,
        w,
        h,
        TextStyle::new(font_size, color)
            .with_line_gap(line_gap)
            .with_macroquad_font(),
        12.0,
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetingView<'a> {
    Equipment {
        unit_id: &'a str,
        equipment_id: &'a str,
    },
    ClassAction {
        unit_id: &'a str,
    },
}

pub struct UiContext<'a> {
    pub feedback: &'a crate::combat_feedback::CombatFeedback,
    pub phase_replay: &'a crate::phase_replay::PhaseReplay,
    pub end_phase_armed: bool,
    pub data: &'a GameData,
    pub assets: &'a AssetManager,
    pub visuals: &'a VisualCatalog,
    pub mission: &'a MissionDef,
    pub session: &'a GameSession,
    pub save_exists: bool,
    pub loaded_assets: usize,
    pub ui: &'a VirtualUi,
    pub targeting: Option<TargetingView<'a>>,
    pub show_help: bool,
    pub show_battle_log: bool,
}

pub fn draw_title(
    data: &GameData,
    save_exists: bool,
    assets: &AssetManager,
    visuals: &VisualCatalog,
    ui: &VirtualUi,
    controller_focus_continue: Option<bool>,
    hover_preview: bool,
) -> Vec<UiAction> {
    crate::title_scene_ui::draw(
        data,
        save_exists,
        assets,
        visuals,
        ui,
        controller_focus_continue,
        hover_preview,
    )
}

pub fn draw_mission_briefing(
    data: &GameData,
    campaign: &CampaignState,
    mission: &MissionDef,
    formation: crate::formation::FormationKind,
    assets: &AssetManager,
    visuals: &VisualCatalog,
    ui: &VirtualUi,
) -> Vec<UiAction> {
    let mut actions = Vec::new();
    let mouse = ui.mouse_position();
    draw_rectangle(
        0.0,
        0.0,
        LOGICAL_WIDTH,
        LOGICAL_HEIGHT,
        Color::new(0.025, 0.04, 0.055, 1.0),
    );
    let panel = Rect::new(150.0, 90.0, 980.0, 540.0);
    draw_surface_with_title(
        panel,
        Some("MISSION BRIEFING // OUTER SETTLEMENT"),
        &SurfaceStyle::new(Color::new(0.045, 0.065, 0.075, 0.98))
            .with_border(1.0, Color::new(0.25, 0.58, 0.52, 0.9))
            .with_header(48.0, Color::new(0.07, 0.12, 0.13, 1.0)),
        TextStyle::new(18.0, dark::TEXT),
    );
    draw_ui_text_ex(
        &mission.name,
        200.0,
        182.0,
        TextStyle::new(34.0, dark::TEXT_BRIGHT).params(),
    );
    draw_text_block(
        &mission.briefing,
        200.0,
        220.0,
        430.0,
        92.0,
        18.0,
        7.0,
        dark::TEXT_DIM,
    );
    draw_mission_vignette(data, mission, assets, visuals);
    if mission.operation_modifier != crate::data::OperationModifier::None {
        draw_ui_text_ex(
            &format!(
                "PRESSURE MODIFIER // {} // {}",
                mission.operation_modifier.label(),
                mission.operation_modifier.description()
            ),
            200.0,
            326.0,
            TextStyle::new(15.0, dark::NEGATIVE).params(),
        );
    }
    draw_ui_text_ex(
        "MISSION RECOVERY",
        880.0,
        238.0,
        TextStyle::new(15.0, dark::ACCENT).params(),
    );
    for (index, line) in [
        format!("{} MATERIALS", mission.materials_reward),
        format!("{} BIOMASS", mission.biomass_reward),
        format!("{} POWER", mission.power_reward),
    ]
    .iter()
    .enumerate()
    {
        draw_ui_text_ex(
            line,
            880.0,
            262.0 + index as f32 * 22.0,
            TextStyle::new(15.0, dark::TEXT_DIM).params(),
        );
    }
    crate::briefing_intel_ui::draw(data, mission, vec2(880.0, 328.0));
    crate::briefing_loadout_ui::draw(campaign, data, assets, visuals, vec2(880.0, 454.0));
    crate::briefing_deployment_ui::draw(
        campaign,
        data,
        formation,
        assets,
        visuals,
        mouse,
        &mut actions,
    );
    draw_ui_text_ex(
        "PAD // D-PAD SELECT · A TOGGLE · X FORMATION · START DEPLOY · B COLONY  //  ENTER DEPLOY · ESC COLONY",
        200.0,
        624.0,
        TextStyle::new(10.0, dark::TEXT_DIM).params(),
    );
    actions
}

fn draw_mission_vignette(
    data: &GameData,
    mission: &MissionDef,
    assets: &AssetManager,
    visuals: &VisualCatalog,
) {
    let frame = Rect::new(660.0, 164.0, 190.0, 152.0);
    draw_rectangle(
        frame.x,
        frame.y,
        frame.w,
        frame.h,
        Color::new(0.01, 0.025, 0.03, 1.0),
    );
    let faction_cell = visuals.faction_terrain_cell(&mission.hostile_faction);
    visuals.draw_atlas_cell(assets, &visuals.terrain, faction_cell, frame, WHITE);
    draw_rectangle(
        frame.x,
        frame.y,
        frame.w,
        frame.h,
        Color::new(0.01, 0.03, 0.035, 0.32),
    );
    if let Some(enemy) = data.roster.iter().find(|unit| {
        unit.team == crate::data::Team::Hostile
            && (mission.hostile_unit_ids.is_empty() || mission.hostile_unit_ids.contains(&unit.id))
    }) {
        crate::portrait_ui::draw_character_portrait(
            assets,
            visuals,
            Rect::new(frame.right() - 78.0, frame.y + 38.0, 66.0, 94.0),
            &enemy.id,
            &enemy.name,
            Color::new(0.98, 0.35, 0.22, 1.0),
        );
    }
    draw_rectangle(
        frame.x,
        frame.bottom() - 24.0,
        frame.w,
        24.0,
        Color::new(0.015, 0.035, 0.04, 0.94),
    );
    draw_ui_text_ex(
        &format!(
            "{} // {} ROUNDS",
            mission.hostile_faction.to_uppercase(),
            mission.round_limit
        ),
        frame.x + 10.0,
        frame.bottom() - 8.0,
        TextStyle::new(11.0, dark::TEXT_BRIGHT).params(),
    );
    draw_rectangle_lines(
        frame.x,
        frame.y,
        frame.w,
        frame.h,
        2.0,
        Color::new(0.32, 0.78, 0.64, 0.9),
    );
}

pub fn draw_tactical(
    ctx: UiContext<'_>,
    camera: &mut crate::grid_ui::WorldCamera,
) -> Vec<UiAction> {
    let mut actions = Vec::new();
    let mouse = ctx.ui.mouse_position();
    crate::tactical_map_ui::draw(&ctx, camera, mouse, &mut actions);
    draw_header(&ctx);
    draw_sidebar(&ctx, mouse, &mut actions);
    draw_footer(&ctx, mouse, &mut actions);
    if ctx.show_help {
        actions.clear();
        crate::help_ui::draw(mouse, &mut actions);
    } else if ctx.show_battle_log {
        actions.clear();
        crate::battle_log_ui::draw(ctx.session, mouse, &mut actions);
    }
    if ctx.phase_replay.is_active() {
        actions.clear();
        ctx.phase_replay.draw();
    }
    actions
}

fn draw_header(ctx: &UiContext<'_>) {
    let rect = Rect::new(10.0, 10.0, LOGICAL_WIDTH - 20.0, 52.0);
    draw_surface(
        rect,
        &SurfaceStyle::new(Color::new(0.045, 0.075, 0.085, 0.98))
            .with_border(1.0, Color::new(0.22, 0.62, 0.56, 0.8))
            .with_left_accent(5.0, Color::new(0.34, 0.86, 0.68, 1.0)),
    );
    draw_ui_text_ex(
        "MIREXIS",
        rect.x + 22.0,
        rect.y + 34.0,
        TextStyle::new(24.0, dark::TEXT_BRIGHT).params(),
    );
    draw_ui_text_ex(
        &ctx.mission.name,
        rect.x + 190.0,
        rect.y + 31.0,
        TextStyle::new(15.0, dark::TEXT_DIM).params(),
    );
    draw_ui_text_ex(
        "DIRECTORATE // BROOD // ASCENDANT",
        rect.x + 190.0,
        rect.y + 46.0,
        TextStyle::new(10.0, Color::new(0.34, 0.58, 0.57, 1.0)).params(),
    );
    draw_badge(
        Rect::new(rect.right() - 280.0, rect.y + 11.0, 116.0, 28.0),
        &format!("ROUND {}", ctx.session.tactical.round),
        Color::new(0.12, 0.22, 0.24, 1.0),
        dark::TEXT,
    );
    let phase = match ctx.session.tactical.phase {
        TacticalPhase::Player => "COLONY PHASE",
        TacticalPhase::Enemy => "HOSTILE PHASE",
    };
    draw_badge(
        Rect::new(rect.right() - 152.0, rect.y + 11.0, 136.0, 28.0),
        phase,
        Color::new(0.13, 0.29, 0.24, 1.0),
        dark::TEXT,
    );
    draw_ui_text_ex(
        "TACTICAL VIEW  /  PLANNING",
        rect.x + 22.0,
        rect.y - 6.0,
        TextStyle::new(10.0, Color::new(0.36, 0.78, 0.69, 1.0)).params(),
    );
}

fn draw_sidebar(ctx: &UiContext<'_>, mouse: Vec2, actions: &mut Vec<UiAction>) {
    let panel = Rect::new(920.0, 74.0, 350.0, 608.0);
    draw_surface_with_title(
        panel,
        Some(ctx.mission.name.as_str()),
        &SurfaceStyle::new(Color::new(0.055, 0.07, 0.08, 0.98))
            .with_border(1.0, Color::new(0.25, 0.38, 0.40, 0.8))
            .with_inner_border(6.0, 1.0, Color::new(0.18, 0.28, 0.29, 0.65))
            .with_header(42.0, Color::new(0.08, 0.105, 0.115, 1.0)),
        TextStyle::new(17.0, dark::TEXT),
    );
    let x = panel.x + 18.0;
    draw_ui_text_ex(
        "COMMAND RAIL",
        x,
        panel.y - 6.0,
        TextStyle::new(10.0, Color::new(0.36, 0.78, 0.69, 1.0)).params(),
    );
    crate::objective_ui::draw_summary(ctx.session, ctx.mission, x, panel);
    let selected = ctx.session.selected_unit();
    if !crate::enemy_intent_ui::draw_inspector(
        ctx.session,
        ctx.data.config.max_action_points,
        panel,
        ctx.assets,
        ctx.visuals,
    ) {
        draw_ui_text_ex(
            "SELECTED COLONIST",
            x,
            panel.y + 164.0,
            TextStyle::new(15.0, Color::new(0.43, 0.83, 0.69, 1.0)).params(),
        );
        if let Some(unit) = selected {
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
                TextStyle::new(22.0, dark::TEXT_BRIGHT).params(),
            );
            draw_ui_text_ex(
                &format!("{}  //  {}", unit.role, unit.mutation),
                detail_x,
                panel.y + 222.0,
                TextStyle::new(13.0, dark::TEXT_DIM).params(),
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
                TextStyle::new(11.0, dark::ACCENT).params(),
            );
            meter(
                Rect::new(x, panel.y + 278.0, panel.w - 36.0, 20.0),
                unit.health as f32,
                unit.max_health as f32,
                dark::POSITIVE,
                Some(&format!("VITALS {}/{}", unit.health, unit.max_health)),
            );
            draw_ui_text_ex(
                &action_status(unit),
                x,
                panel.y + 340.0,
                TextStyle::new(11.0, dark::TEXT_DIM).params(),
            );
            meter(
                Rect::new(x, panel.y + 306.0, panel.w - 36.0, 20.0),
                unit.action_points as f32,
                ctx.data.config.max_action_points as f32,
                Color::new(0.33, 0.65, 0.92, 1.0),
                Some(&format!("ACTION POINTS {}", unit.action_points)),
            );
        } else {
            draw_ui_text_ex(
                "Select a colony unit",
                x,
                panel.y + 200.0,
                TextStyle::new(18.0, dark::TEXT_DIM).params(),
            );
        }
    }
    let objective_action = match ctx.mission.objective_kind {
        ObjectiveKind::SecureAndClear => "SECURE OBJECTIVE",
        ObjectiveKind::EliminateAll => "ELIMINATE ALL HOSTILES",
        ObjectiveKind::Holdout => "HOLD THE PERIMETER",
        ObjectiveKind::Extraction => "EXTRACT COLONIST",
        ObjectiveKind::SignalTrace
            if ctx.session.tactical.objective_state != ObjectiveState::Active =>
        {
            "SIGNAL RELAY ACTIVE"
        }
        ObjectiveKind::SignalTrace => "ACTIVATE SIGNAL RELAY",
        ObjectiveKind::DefendAsset => "PROTECT FIELD ASSET",
    };
    if button(
        Rect::new(x, panel.bottom() - 188.0, panel.w - 36.0, 38.0),
        objective_action,
        ctx.session.can_interact_selected(),
        mouse,
    ) {
        actions.push(UiAction::InteractObjective);
    }
    let mutation_label = selected.map_or("MUTATION GIFT", |unit| match unit.mutation.as_str() {
        "Neural Bloom" => "NEURAL FOCUS",
        "Chitinous Growth" => "HARDEN CARAPACE",
        "Regenerative Tissue" => "ACCELERATE TISSUE",
        "Elastic Musculature" => "COIL MUSCLE",
        "Symbiotic Organism" => "FEEDING FRENZY",
        _ => "MUTATION GIFT",
    });
    let action_width = (panel.w - 52.0) / 3.0;
    if button(
        Rect::new(x, panel.bottom() - 144.0, action_width, 38.0),
        mutation_label,
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
            panel.bottom() - 144.0,
            action_width,
            38.0,
        ),
        mouse,
        actions,
    );
    crate::equipment_ui::draw_action_button(
        ctx,
        selected,
        Rect::new(
            x + (action_width + 8.0) * 2.0,
            panel.bottom() - 144.0,
            action_width,
            38.0,
        ),
        mouse,
        actions,
    );
    let phase_width = (panel.w - 44.0) * 0.46;
    let ready = crate::phase_readiness::ready_count(ctx.session);
    let end_phase_label = if ctx.end_phase_armed {
        format!("CONFIRM END · {} READY", ready)
    } else if ready > 0 {
        format!("END PHASE · {} READY", ready)
    } else {
        "END COLONY PHASE".to_owned()
    };
    if button(
        Rect::new(x, panel.bottom() - 100.0, phase_width, 44.0),
        "OVERWATCH",
        ctx.session.can_set_selected_overwatch(),
        mouse,
    ) {
        actions.push(UiAction::SetOverwatch);
    }
    if button(
        Rect::new(
            x + phase_width + 8.0,
            panel.bottom() - 100.0,
            panel.w - 44.0 - phase_width,
            44.0,
        ),
        &end_phase_label,
        true,
        mouse,
    ) {
        actions.push(UiAction::EndPhase);
    }
    draw_ui_text_ex(
        &format!(
            "Materials: {}   Round limit: {}",
            ctx.session.tactical.materials, ctx.mission.round_limit
        ),
        x,
        panel.bottom() - 36.0,
        TextStyle::new(15.0, dark::TEXT_DIM).params(),
    );
    if let Some(event) = ctx.session.tactical.event_log.last() {
        draw_ui_text_ex(
            &event_summary(event),
            x,
            panel.bottom() - 14.0,
            TextStyle::new(13.0, dark::TEXT_DIM).params(),
        );
    }
}

fn draw_footer(ctx: &UiContext<'_>, mouse: Vec2, actions: &mut Vec<UiAction>) {
    let y = 686.0;
    if button(Rect::new(10.0, y, 82.0, 28.0), "TITLE", true, mouse) {
        actions.push(UiAction::ReturnToTitle);
    }
    if button(Rect::new(98.0, y, 82.0, 28.0), "SAVE [S]", true, mouse) {
        actions.push(UiAction::Save);
    }
    if button(
        Rect::new(186.0, y, 82.0, 28.0),
        "LOAD [L]",
        ctx.save_exists,
        mouse,
    ) {
        actions.push(UiAction::Load);
    }
    if button(
        Rect::new(274.0, y, 110.0, 28.0),
        "DELETE SAVE",
        ctx.save_exists,
        mouse,
    ) {
        actions.push(UiAction::DeleteSave);
    }
    if button(Rect::new(390.0, y, 82.0, 28.0), "HELP [H]", true, mouse) {
        actions.push(UiAction::ToggleTacticalHelp);
    }
    if button(Rect::new(478.0, y, 78.0, 28.0), "LOG [B]", true, mouse) {
        actions.push(UiAction::ToggleBattleLog);
    }
    if button(
        Rect::new(562.0, y, 136.0, 28.0),
        "NEXT READY [TAB]",
        crate::phase_readiness::ready_count(ctx.session) > 0,
        mouse,
    ) {
        actions.push(UiAction::SelectNextReady);
    }
    draw_ui_text_ex(
        &format!(
            "{} campaign  //  {} assets  //  click a hostile to attack",
            if ctx.mission.name.starts_with("ESCALATION:") {
                "Escalation"
            } else if ctx.mission.name.starts_with("ADAPTATION:") {
                "Adaptation"
            } else if ctx.mission.name.starts_with("CONTACT:") {
                "Contact"
            } else {
                "Isolation"
            },
            ctx.loaded_assets
        ),
        710.0,
        y + 20.0,
        TextStyle::new(11.0, dark::TEXT_DIM).params(),
    );
}

pub fn tile_move_from_keys() -> Option<(i32, i32)> {
    if is_key_pressed(KeyCode::Up) {
        Some((0, -1))
    } else if is_key_pressed(KeyCode::Right) {
        Some((1, 0))
    } else if is_key_pressed(KeyCode::Down) {
        Some((0, 1))
    } else if is_key_pressed(KeyCode::Left) {
        Some((-1, 0))
    } else {
        None
    }
}
