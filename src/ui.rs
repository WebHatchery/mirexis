//! Immediate-mode title and tactical presentation.

use crate::campaign::CampaignState;
use crate::data::{GameData, Team};
use crate::state::{
    BattleEvent, GameSession, MissionOutcome, ObjectiveState, TacticalPhase, UnitState,
};
use macroquad::prelude::*;
use macroquad_toolkit::grid::TilePos;
use macroquad_toolkit::prelude::*;
use macroquad_toolkit::ui::{draw_ui_text_ex, RectExt, VirtualUi};

pub const LOGICAL_WIDTH: f32 = 1280.0;
pub const LOGICAL_HEIGHT: f32 = 720.0;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UiAction {
    StartMission,
    DeployMission,
    Continue,
    ReturnToTitle,
    SelectTile(TilePos),
    MoveSelected(TilePos),
    AttackSelected(String),
    InteractObjective,
    EndPhase,
    Save,
    Load,
    DeleteSave,
}

pub struct UiContext<'a> {
    pub data: &'a GameData,
    pub session: &'a GameSession,
    pub save_exists: bool,
    pub loaded_assets: usize,
    pub ui: &'a VirtualUi,
}

pub fn draw_title(data: &GameData, save_exists: bool, ui: &VirtualUi) -> Vec<UiAction> {
    let mut actions = Vec::new();
    let mouse = ui.mouse_position();
    draw_rectangle(
        0.0,
        0.0,
        LOGICAL_WIDTH,
        LOGICAL_HEIGHT,
        Color::new(0.025, 0.04, 0.055, 1.0),
    );
    for index in 0..18 {
        let x = index as f32 * 82.0 - 120.0;
        draw_line(
            x,
            540.0,
            x + 310.0,
            80.0,
            2.0,
            Color::new(0.08, 0.23, 0.23, 0.35),
        );
    }
    draw_ui_text_ex(
        "MIREXIS",
        100.0,
        205.0,
        TextStyle::new(88.0, Color::new(0.74, 0.96, 0.88, 1.0)).params(),
    );
    draw_ui_text_ex(
        "ONE COLONY. THREE POWERS. NO SAFE GROUND.",
        106.0,
        248.0,
        TextStyle::new(19.0, Color::new(0.37, 0.79, 0.72, 1.0)).params(),
    );
    draw_text_block(
        &data.mission.briefing,
        106.0,
        292.0,
        520.0,
        120.0,
        20.0,
        6.0,
        dark::TEXT_DIM,
    );

    if button(
        Rect::new(106.0, 450.0, 240.0, 48.0),
        "NEW OPERATION",
        true,
        mouse,
    ) {
        actions.push(UiAction::StartMission);
    }
    if button(
        Rect::new(362.0, 450.0, 240.0, 48.0),
        "CONTINUE",
        save_exists,
        mouse,
    ) {
        actions.push(UiAction::Continue);
    }
    draw_ui_text_ex(
        "PHASE 0  //  TACTICAL READINESS PROTOTYPE",
        106.0,
        626.0,
        TextStyle::new(16.0, Color::new(0.38, 0.52, 0.55, 1.0)).params(),
    );
    actions
}

pub fn draw_mission_briefing(
    data: &GameData,
    campaign: &CampaignState,
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
        &data.mission.name,
        200.0,
        190.0,
        TextStyle::new(34.0, dark::TEXT_BRIGHT).params(),
    );
    draw_text_block(
        &data.mission.briefing,
        200.0,
        230.0,
        640.0,
        100.0,
        21.0,
        7.0,
        dark::TEXT_DIM,
    );
    draw_ui_text_ex(
        "DEPLOYMENT",
        200.0,
        370.0,
        TextStyle::new(15.0, dark::ACCENT).params(),
    );
    let squad = campaign
        .roster
        .iter()
        .filter(|character| character.availability == crate::campaign::Availability::Ready)
        .map(|character| {
            let class_name = data
                .classes
                .iter()
                .find(|class| class.id == character.active_class)
                .map_or(character.active_class.as_str(), |class| class.name.as_str());
            let mutation_name = data
                .mutations
                .iter()
                .find(|mutation| mutation.id == character.mutation_id)
                .map_or(character.mutation_id.as_str(), |mutation| {
                    mutation.name.as_str()
                });
            format!(
                "{} · {} · {} · {} XP",
                character.name, class_name, mutation_name, character.experience
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    draw_text_block(&squad, 200.0, 394.0, 650.0, 110.0, 18.0, 6.0, dark::TEXT);
    if button(
        Rect::new(820.0, 520.0, 250.0, 48.0),
        "DEPLOY SQUAD",
        true,
        mouse,
    ) {
        actions.push(UiAction::DeployMission);
    }
    if button(
        Rect::new(200.0, 520.0, 180.0, 48.0),
        "STAND DOWN",
        true,
        mouse,
    ) {
        actions.push(UiAction::ReturnToTitle);
    }
    actions
}

pub fn draw_debrief(data: &GameData, outcome: &MissionOutcome, ui: &VirtualUi) -> Vec<UiAction> {
    let mut actions = Vec::new();
    let mouse = ui.mouse_position();
    draw_rectangle(
        0.0,
        0.0,
        LOGICAL_WIDTH,
        LOGICAL_HEIGHT,
        Color::new(0.025, 0.04, 0.055, 1.0),
    );
    let won = outcome.result == ObjectiveState::Victory;
    let panel = Rect::new(220.0, 100.0, 840.0, 500.0);
    draw_surface(
        panel,
        &SurfaceStyle::new(Color::new(0.045, 0.065, 0.075, 0.98))
            .with_border(1.0, if won { dark::POSITIVE } else { dark::NEGATIVE }),
    );
    draw_rectangle(
        panel.x,
        panel.y,
        panel.w,
        48.0,
        Color::new(0.07, 0.12, 0.13, 1.0),
    );
    draw_text("OPERATION DEBRIEF", 535.0, 132.0, 18.0, dark::TEXT);
    draw_text(
        if won {
            "SURVEY TEAM RECOVERED"
        } else {
            "REFUGE LOST"
        },
        280.0,
        220.0,
        36.0,
        if won { dark::POSITIVE } else { dark::NEGATIVE },
    );
    let injuries = if outcome.colonists_incapacitated.is_empty() {
        "No squad incapacitations".to_owned()
    } else {
        format!(
            "Incapacitated: {}",
            outcome
                .colonists_incapacitated
                .iter()
                .map(|entry| entry.name.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        )
    };
    let report = [
        format!("Operation: {}", data.mission.name),
        format!(
            "Squad returned: {}/{}",
            outcome.colonists_deployed - outcome.colonists_incapacitated.len(),
            outcome.colonists_deployed
        ),
        injuries,
        format!("Hostiles neutralised: {}", outcome.hostiles_neutralised),
        format!("Materials recovered: {}", outcome.materials_awarded),
    ];
    for (index, line) in report.iter().enumerate() {
        draw_text(line, 280.0, 305.0 + index as f32 * 30.0, 21.0, dark::TEXT);
    }
    if button(
        Rect::new(780.0, 510.0, 220.0, 48.0),
        "RETURN TO COLONY",
        true,
        mouse,
    ) {
        actions.push(UiAction::ReturnToTitle);
    }
    actions
}

pub fn draw_tactical(ctx: UiContext<'_>) -> Vec<UiAction> {
    let mut actions = Vec::new();
    let mouse = ctx.ui.mouse_position();
    draw_header(&ctx);
    draw_map(&ctx, mouse, &mut actions);
    draw_sidebar(&ctx, mouse, &mut actions);
    draw_footer(&ctx, mouse, &mut actions);
    actions
}

fn draw_header(ctx: &UiContext<'_>) {
    let rect = Rect::new(18.0, 16.0, LOGICAL_WIDTH - 36.0, 66.0);
    draw_surface(
        rect,
        &SurfaceStyle::new(Color::new(0.045, 0.075, 0.085, 0.98))
            .with_border(1.0, Color::new(0.22, 0.62, 0.56, 0.8))
            .with_left_accent(5.0, Color::new(0.34, 0.86, 0.68, 1.0)),
    );
    draw_ui_text_ex(
        "MIREXIS",
        rect.x + 22.0,
        rect.y + 39.0,
        TextStyle::new(30.0, dark::TEXT_BRIGHT).params(),
    );
    draw_ui_text_ex(
        &ctx.data.mission.name,
        rect.x + 190.0,
        rect.y + 38.0,
        TextStyle::new(18.0, dark::TEXT_DIM).params(),
    );
    draw_badge(
        Rect::new(rect.right() - 300.0, rect.y + 18.0, 126.0, 30.0),
        &format!("ROUND {}", ctx.session.tactical.round),
        Color::new(0.12, 0.22, 0.24, 1.0),
        dark::TEXT,
    );
    let phase = match ctx.session.tactical.phase {
        TacticalPhase::Player => "COLONY PHASE",
        TacticalPhase::Enemy => "HOSTILE PHASE",
    };
    draw_badge(
        Rect::new(rect.right() - 160.0, rect.y + 18.0, 140.0, 30.0),
        phase,
        Color::new(0.13, 0.29, 0.24, 1.0),
        dark::TEXT,
    );
}

fn draw_map(ctx: &UiContext<'_>, mouse: Vec2, actions: &mut Vec<UiAction>) {
    let panel = Rect::new(18.0, 96.0, 820.0, 530.0);
    draw_surface_with_title(
        panel,
        Some("TACTICAL GRID // OUTER SETTLEMENT"),
        &SurfaceStyle::new(Color::new(0.035, 0.052, 0.062, 0.98))
            .with_border(1.0, Color::new(0.19, 0.40, 0.40, 0.9))
            .with_header(42.0, Color::new(0.06, 0.10, 0.11, 1.0)),
        TextStyle::new(17.0, dark::TEXT),
    );
    let grid_rect = Rect::new(
        panel.x + 24.0,
        panel.y + 60.0,
        panel.w - 48.0,
        panel.h - 82.0,
    );
    let view = GridView::new(ctx, grid_rect);
    for (position, _) in ctx.session.tactical.fog.iter_with_pos() {
        let rect = view.tile_rect(position);
        let mut color = if (position.x + position.y) % 2 == 0 {
            Color::new(0.105, 0.15, 0.145, 1.0)
        } else {
            Color::new(0.085, 0.13, 0.13, 1.0)
        };
        if ctx.session.tactical.blocked.contains(&position) {
            color = Color::new(0.22, 0.18, 0.13, 1.0);
        } else if ctx.session.can_move_selected_to(position) {
            color = Color::new(0.12, 0.27, 0.21, 1.0);
        }
        draw_rectangle(rect.x, rect.y, rect.w, rect.h, color);
        draw_rectangle_lines(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            1.0,
            Color::new(0.16, 0.26, 0.25, 0.65),
        );
        if position == ctx.session.tactical.selected_tile {
            draw_rectangle_lines(
                rect.x + 2.0,
                rect.y + 2.0,
                rect.w - 4.0,
                rect.h - 4.0,
                3.0,
                Color::new(0.78, 0.94, 0.63, 1.0),
            );
        }
        if position == ctx.session.tactical.objective_tile
            && ctx.session.tactical.objective_state == ObjectiveState::Active
        {
            draw_circle_lines(
                rect.x + rect.w * 0.5,
                rect.y + rect.h * 0.5,
                rect.w * 0.30,
                3.0,
                Color::new(0.95, 0.74, 0.24, 1.0),
            );
        }
    }
    for unit in &ctx.session.tactical.units {
        draw_unit(
            view,
            unit,
            ctx.session.tactical.selected_unit.as_deref() == Some(&unit.id),
        );
    }
    if is_mouse_button_released(MouseButton::Left) {
        if let Some(tile) = view.tile_at(mouse) {
            let hostile = ctx
                .session
                .tactical
                .units
                .iter()
                .find(|unit| unit.position == tile && unit.team == Team::Hostile);
            if let Some(hostile) = hostile.filter(|unit| ctx.session.can_attack_selected(&unit.id))
            {
                actions.push(UiAction::AttackSelected(hostile.id.clone()));
            } else if ctx.session.can_move_selected_to(tile) {
                actions.push(UiAction::MoveSelected(tile));
            } else {
                actions.push(UiAction::SelectTile(tile));
            }
        }
    }
}

fn draw_unit(view: GridView, unit: &UnitState, selected: bool) {
    let rect = view.tile_rect(unit.position);
    let color = match unit.team {
        Team::Colony => Color::new(0.22, 0.75, 0.63, 1.0),
        Team::Hostile => Color::new(0.86, 0.27, 0.25, 1.0),
    };
    let center = vec2(rect.x + rect.w * 0.5, rect.y + rect.h * 0.48);
    let color = if unit.incapacitated {
        Color::new(color.r * 0.35, color.g * 0.35, color.b * 0.35, 1.0)
    } else {
        color
    };
    draw_circle(center.x, center.y, rect.w * 0.28, color);
    draw_circle_lines(
        center.x,
        center.y,
        rect.w * 0.28,
        if selected { 4.0 } else { 2.0 },
        if selected {
            WHITE
        } else {
            Color::new(0.04, 0.08, 0.08, 1.0)
        },
    );
    let initials = unit
        .name
        .split_whitespace()
        .filter_map(|word| word.chars().next())
        .take(2)
        .collect::<String>();
    draw_text_centered_in_box(
        &initials,
        rect.x,
        rect.y,
        rect.w,
        rect.h - 2.0,
        15.0,
        Color::new(0.03, 0.07, 0.07, 1.0),
    );
}

fn draw_sidebar(ctx: &UiContext<'_>, mouse: Vec2, actions: &mut Vec<UiAction>) {
    let panel = Rect::new(856.0, 96.0, 406.0, 530.0);
    draw_surface_with_title(
        panel,
        Some("OPERATION GLASSROOT"),
        &SurfaceStyle::new(Color::new(0.055, 0.07, 0.08, 0.98))
            .with_border(1.0, Color::new(0.25, 0.38, 0.40, 0.8))
            .with_header(42.0, Color::new(0.08, 0.105, 0.115, 1.0)),
        TextStyle::new(17.0, dark::TEXT),
    );
    let x = panel.x + 18.0;
    draw_ui_text_ex(
        "OBJECTIVE",
        x,
        panel.y + 78.0,
        TextStyle::new(15.0, Color::new(0.43, 0.83, 0.69, 1.0)).params(),
    );
    draw_text_block(
        &ctx.data.mission.objective,
        x,
        panel.y + 92.0,
        panel.w - 36.0,
        60.0,
        17.0,
        4.0,
        dark::TEXT_DIM,
    );
    let selected = ctx.session.selected_unit();
    draw_ui_text_ex(
        "SELECTED COLONIST",
        x,
        panel.y + 178.0,
        TextStyle::new(15.0, Color::new(0.43, 0.83, 0.69, 1.0)).params(),
    );
    if let Some(unit) = selected {
        draw_ui_text_ex(
            &unit.name,
            x,
            panel.y + 212.0,
            TextStyle::new(25.0, dark::TEXT_BRIGHT).params(),
        );
        draw_ui_text_ex(
            &format!("{}  //  {}", unit.role, unit.mutation),
            x,
            panel.y + 239.0,
            TextStyle::new(16.0, dark::TEXT_DIM).params(),
        );
        meter(
            Rect::new(x, panel.y + 260.0, panel.w - 36.0, 22.0),
            unit.health as f32,
            unit.max_health as f32,
            dark::POSITIVE,
            Some(&format!("VITALS {}/{}", unit.health, unit.max_health)),
        );
        meter(
            Rect::new(x, panel.y + 292.0, panel.w - 36.0, 22.0),
            unit.action_points as f32,
            ctx.data.config.max_action_points as f32,
            Color::new(0.33, 0.65, 0.92, 1.0),
            Some(&format!("ACTION POINTS {}", unit.action_points)),
        );
    } else {
        draw_ui_text_ex(
            "Select a colony unit",
            x,
            panel.y + 216.0,
            TextStyle::new(18.0, dark::TEXT_DIM).params(),
        );
    }
    if button(
        Rect::new(x, panel.bottom() - 166.0, panel.w - 36.0, 38.0),
        "STABILISE REFUGE",
        ctx.session.can_interact_selected(),
        mouse,
    ) {
        actions.push(UiAction::InteractObjective);
    }
    if button(
        Rect::new(x, panel.bottom() - 120.0, panel.w - 36.0, 44.0),
        "END COLONY PHASE",
        true,
        mouse,
    ) {
        actions.push(UiAction::EndPhase);
    }
    draw_ui_text_ex(
        &format!(
            "Materials: {}   Round limit: {}",
            ctx.session.tactical.materials, ctx.data.mission.round_limit
        ),
        x,
        panel.bottom() - 48.0,
        TextStyle::new(15.0, dark::TEXT_DIM).params(),
    );
    if let Some(event) = ctx.session.tactical.event_log.last() {
        draw_ui_text_ex(
            &event_summary(event),
            x,
            panel.bottom() - 22.0,
            TextStyle::new(13.0, dark::TEXT_DIM).params(),
        );
    }
}

fn draw_footer(ctx: &UiContext<'_>, mouse: Vec2, actions: &mut Vec<UiAction>) {
    let y = 644.0;
    if button(Rect::new(18.0, y, 130.0, 44.0), "TITLE", true, mouse) {
        actions.push(UiAction::ReturnToTitle);
    }
    if button(Rect::new(160.0, y, 130.0, 44.0), "SAVE", true, mouse) {
        actions.push(UiAction::Save);
    }
    if button(
        Rect::new(302.0, y, 130.0, 44.0),
        "LOAD",
        ctx.save_exists,
        mouse,
    ) {
        actions.push(UiAction::Load);
    }
    if button(
        Rect::new(444.0, y, 150.0, 44.0),
        "DELETE SAVE",
        ctx.save_exists,
        mouse,
    ) {
        actions.push(UiAction::DeleteSave);
    }
    draw_ui_text_ex(
        &format!(
            "Phase 1A tactical rules  //  {} assets  //  click a hostile to attack",
            ctx.loaded_assets
        ),
        620.0,
        y + 28.0,
        TextStyle::new(14.0, dark::TEXT_DIM).params(),
    );
}

fn event_summary(event: &BattleEvent) -> String {
    match event {
        BattleEvent::UnitMoved { unit_id, cost, .. } => format!("{} moved · {} AP", unit_id, cost),
        BattleEvent::AttackRolled {
            roll, hit_chance, ..
        } => {
            format!("Attack roll {} · {}% target", roll, hit_chance)
        }
        BattleEvent::DamageApplied {
            amount, remaining, ..
        } => {
            format!("{} damage · {} vitality remains", amount, remaining)
        }
        BattleEvent::UnitIncapacitated { unit_id } => format!("{} incapacitated", unit_id),
        BattleEvent::ObjectiveSecured { .. } => "Survey refuge stabilised".to_owned(),
        BattleEvent::PhaseStarted { phase, round } => {
            format!("{:?} phase · round {}", phase, round)
        }
        BattleEvent::BattleEnded { outcome } => format!("Operation {:?}", outcome),
    }
}

fn button(rect: Rect, label: &str, enabled: bool, mouse: Vec2) -> bool {
    let hovered = enabled && rect.contains_point(mouse);
    let fill = if !enabled {
        Color::new(0.09, 0.11, 0.12, 1.0)
    } else if hovered {
        Color::new(0.18, 0.43, 0.37, 1.0)
    } else {
        Color::new(0.10, 0.28, 0.25, 1.0)
    };
    draw_surface(
        rect,
        &SurfaceStyle::new(fill).with_border(
            1.0,
            if enabled {
                Color::new(0.33, 0.72, 0.60, 1.0)
            } else {
                Color::new(0.20, 0.24, 0.25, 1.0)
            },
        ),
    );
    draw_text_centered_in_box(
        label,
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        16.0,
        if enabled { dark::TEXT } else { dark::TEXT_DIM },
    );
    hovered && is_mouse_button_released(MouseButton::Left)
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

#[derive(Debug, Clone, Copy)]
struct GridView {
    origin: Vec2,
    tile_size: f32,
    width: usize,
    height: usize,
}

impl GridView {
    fn new(ctx: &UiContext<'_>, rect: Rect) -> Self {
        let width = ctx.session.tactical.fog.width;
        let height = ctx.session.tactical.fog.height;
        let tile_size = (rect.w / width as f32).min(rect.h / height as f32).floor();
        Self {
            origin: vec2(
                rect.x + (rect.w - width as f32 * tile_size) * 0.5,
                rect.y + (rect.h - height as f32 * tile_size) * 0.5,
            ),
            tile_size,
            width,
            height,
        }
    }

    fn tile_rect(self, tile: TilePos) -> Rect {
        Rect::new(
            self.origin.x + tile.x as f32 * self.tile_size,
            self.origin.y + tile.y as f32 * self.tile_size,
            self.tile_size,
            self.tile_size,
        )
    }

    fn tile_at(self, point: Vec2) -> Option<TilePos> {
        let tile = TilePos::new(
            ((point.x - self.origin.x) / self.tile_size).floor() as i32,
            ((point.y - self.origin.y) / self.tile_size).floor() as i32,
        );
        (tile.x >= 0 && tile.y >= 0 && tile.x < self.width as i32 && tile.y < self.height as i32)
            .then_some(tile)
    }
}
