//! Immediate-mode title and tactical presentation.

use crate::campaign::CampaignState;
use crate::data::{GameData, MissionDef};
use crate::state::GameSession;
use crate::visual_assets::VisualCatalog;
use macroquad::prelude::*;
use macroquad_toolkit::assets::AssetManager;
use macroquad_toolkit::prelude::*;
use macroquad_toolkit::ui::VirtualUi;

pub use crate::ui_action::UiAction;

pub const LOGICAL_WIDTH: f32 = 1280.0;
pub const LOGICAL_HEIGHT: f32 = 720.0;
const TACTICAL_WORLD_X: f32 = 10.0;
const TACTICAL_WORLD_Y: f32 = 10.0;
const TACTICAL_WORLD_WIDTH: f32 = LOGICAL_WIDTH - 20.0;
const TACTICAL_WORLD_HEIGHT: f32 = LOGICAL_HEIGHT - 20.0;
const TACTICAL_COMMAND_PANEL_X: f32 = 832.0;
const TACTICAL_COMMAND_PANEL_Y: f32 = 76.0;
const TACTICAL_COMMAND_PANEL_WIDTH: f32 = 344.0;
const TACTICAL_COMMAND_PANEL_HEIGHT: f32 = 626.0;
const BRIEFING_MODIFIER_X: f32 = 200.0;
const BRIEFING_MODIFIER_Y: f32 = 326.0;
const BRIEFING_MODIFIER_WIDTH: f32 = 430.0;
const BRIEFING_MODIFIER_HEIGHT: f32 = 32.0;

pub(crate) fn pointer_position(ui: &VirtualUi) -> Vec2 {
    pointer_position_for_capture(ui, macroquad_toolkit::capture::capture_requested("MIREXIS"))
}

pub(crate) fn tactical_world_rect() -> Rect {
    Rect::new(
        TACTICAL_WORLD_X,
        TACTICAL_WORLD_Y,
        TACTICAL_WORLD_WIDTH,
        TACTICAL_WORLD_HEIGHT,
    )
}

pub(crate) fn tactical_command_panel_rect() -> Rect {
    Rect::new(
        TACTICAL_COMMAND_PANEL_X,
        TACTICAL_COMMAND_PANEL_Y,
        TACTICAL_COMMAND_PANEL_WIDTH,
        TACTICAL_COMMAND_PANEL_HEIGHT,
    )
}

fn pointer_position_for_capture(ui: &VirtualUi, capturing: bool) -> Vec2 {
    if capturing {
        vec2(-1_000.0, -1_000.0)
    } else {
        ui.mouse_position()
    }
}

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
    let clip = rect.map(|rect| ui_clip_pixels(ui, rect, screen_dpi_scale()));
    // Macroquad's scissor uses physical framebuffer pixels; callers use the
    // logical coordinate system established by `VirtualUi`.
    unsafe {
        get_internal_gl().quad_gl.scissor(clip);
    }
}

pub(crate) fn end_phase_button_bounds(panel: Rect) -> Rect {
    let phase_width = (panel.w - 44.0) * 0.46;
    Rect::new(
        panel.x + 18.0 + phase_width + 8.0,
        panel.bottom() - 104.0,
        panel.w - 44.0 - phase_width,
        44.0,
    )
}

fn ui_clip_pixels(ui: &VirtualUi, rect: Rect, dpi: f32) -> (i32, i32, i32, i32) {
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
        target_kind: crate::data::TechniqueTarget,
    },
    Skill {
        unit_id: &'a str,
        skill_id: &'a str,
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
    pub delete_save_armed: bool,
    pub first_hour: &'a crate::first_hour::FirstHourProgress,
    pub ui: &'a VirtualUi,
    pub pointer_override: Option<Vec2>,
    pub targeting: Option<TargetingView<'a>>,
    pub show_help: bool,
    pub show_battle_log: bool,
    pub battle_log_filter: crate::ui_action::BattleLogFilter,
    pub show_settings: bool,
    pub tactical_panel_open: bool,
}

pub struct TitleDrawContext<'a> {
    pub(crate) data: &'a GameData,
    pub(crate) save_exists: bool,
    pub(crate) assets: &'a AssetManager,
    pub(crate) visuals: &'a VisualCatalog,
    pub(crate) ui: &'a VirtualUi,
    pub(crate) controller_focus_continue: Option<bool>,
    pub(crate) hover_preview: bool,
    pub(crate) new_campaign_armed: bool,
}

pub(crate) fn draw_title(context: TitleDrawContext<'_>) -> Vec<UiAction> {
    crate::title_scene_ui::draw(context)
}

pub(crate) fn briefing_modifier_bounds() -> Rect {
    Rect::new(
        BRIEFING_MODIFIER_X,
        BRIEFING_MODIFIER_Y,
        BRIEFING_MODIFIER_WIDTH,
        BRIEFING_MODIFIER_HEIGHT,
    )
}

fn briefing_modifier_copy(mission: &MissionDef) -> String {
    format!(
        "{} // {} // {}",
        if mission.operation_modifier.is_engine_effect() {
            "ENGINE EFFECT"
        } else {
            "PRESSURE MODIFIER"
        },
        mission.operation_modifier.label(),
        mission.operation_modifier.description()
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
    let mouse = pointer_position(ui);
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
    if campaign.operations_completed == 0 {
        draw_ui_text_ex(
            "MARA VENN // Get the refuge lights back. Bring my people home.",
            200.0,
            314.0,
            TextStyle::new(13.0, dark::ACCENT).params(),
        );
    }
    draw_mission_vignette(data, mission, assets, visuals);
    if mission.operation_modifier != crate::data::OperationModifier::None {
        let modifier = briefing_modifier_bounds();
        draw_text_block(
            &briefing_modifier_copy(mission),
            modifier.x,
            modifier.y,
            modifier.w,
            modifier.h,
            13.0,
            2.0,
            dark::NEGATIVE,
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
    crate::briefing_intel_ui::draw(campaign, data, mission, vec2(880.0, 328.0));
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
    crate::first_hour_colony_ui::draw_briefing_focus(campaign, data);
    draw_ui_text_ex(
        "TOUCH // TAP COLONIST · FORMATION · DEPLOY · COLONY  //  PAD // D-PAD · A · X · START · B",
        200.0,
        646.0,
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
    let mouse = ctx
        .pointer_override
        .unwrap_or_else(|| pointer_position(ctx.ui));
    let input_enabled = tactical_world_input_enabled(
        ctx.show_help,
        ctx.show_battle_log,
        ctx.phase_replay.is_active(),
        ctx.first_hour.help_open,
        ctx.show_settings,
    );
    let suppress_actions =
        crate::tactical_map_ui::draw(&ctx, camera, mouse, input_enabled, &mut actions);
    crate::tactical_hud::draw(&ctx, mouse, &mut actions);
    crate::first_hour_tactical_ui::draw_command_focus(&ctx);
    if ctx.show_help {
        actions.clear();
        crate::help_ui::draw(mouse, &mut actions);
    } else if ctx.show_battle_log {
        actions.clear();
        crate::battle_log_ui::draw(ctx.session, ctx.battle_log_filter, mouse, &mut actions);
    }
    if ctx.phase_replay.is_active() {
        actions.clear();
        ctx.phase_replay.draw(mouse, &mut actions);
        crate::first_hour_tactical_ui::draw_replay_focus(&ctx);
    }
    suppress_map_release_actions(&mut actions, suppress_actions);
    actions
}

fn tactical_world_input_enabled(
    show_help: bool,
    show_log: bool,
    replay_active: bool,
    first_hour_help_open: bool,
    settings_open: bool,
) -> bool {
    !show_help && !show_log && !replay_active && !first_hour_help_open && !settings_open
}

pub(crate) fn suppress_map_release_actions(actions: &mut Vec<UiAction>, suppress: bool) {
    if suppress {
        actions.clear();
    }
}

pub(crate) fn mutation_button_label(
    selected: Option<&crate::state::UnitState>,
    enabled: bool,
) -> &'static str {
    let Some(unit) = selected else {
        return "MUTATION GIFT";
    };
    let Some(label) = mutation_action_label(&unit.mutation) else {
        return "NO MUTATION";
    };
    if enabled {
        return label;
    }
    if unit.incapacitated {
        return "INCAPACITATED";
    }
    if unit.mutation_gift_used {
        return "SPENT";
    }
    if unit.mutation == "Regenerative Tissue" && unit.health == unit.max_health {
        return "FULL HEALTH";
    }
    if unit.action_points == 0 {
        return "NO AP";
    }
    "UNAVAILABLE"
}

fn mutation_action_label(mutation: &str) -> Option<&'static str> {
    match mutation {
        "Neural Bloom" => Some("NEURAL FOCUS"),
        "Chitinous Growth" => Some("HARDEN CARAPACE"),
        "Regenerative Tissue" => Some("ACCELERATE TISSUE"),
        "Elastic Musculature" => Some("COIL MUSCLE"),
        "Symbiotic Organism" => Some("FEEDING FRENZY"),
        _ => None,
    }
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

#[cfg(test)]
mod tests;
