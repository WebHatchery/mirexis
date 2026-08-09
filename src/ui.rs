//! Immediate-mode title and tactical presentation.

use crate::campaign::CampaignState;
use crate::data::{GameData, MissionDef, ObjectiveKind, Team};
use crate::grid_ui::GridView;
use crate::state::{GameSession, MissionOutcome, ObjectiveState, TacticalPhase};
use crate::ui_widgets::{action_status, button, event_summary};
use macroquad::prelude::*;
use macroquad_toolkit::prelude::*;
use macroquad_toolkit::ui::{draw_ui_text_ex, VirtualUi};

pub use crate::ui_action::UiAction;

pub const LOGICAL_WIDTH: f32 = 1280.0;
pub const LOGICAL_HEIGHT: f32 = 720.0;

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
    pub data: &'a GameData,
    pub mission: &'a MissionDef,
    pub session: &'a GameSession,
    pub save_exists: bool,
    pub loaded_assets: usize,
    pub ui: &'a VirtualUi,
    pub targeting: Option<TargetingView<'a>>,
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
        "PHASE ONE: ISOLATION  //  KEEP THE COLONY ALIVE",
        106.0,
        626.0,
        TextStyle::new(16.0, Color::new(0.38, 0.52, 0.55, 1.0)).params(),
    );
    actions
}

pub fn draw_mission_briefing(
    data: &GameData,
    campaign: &CampaignState,
    mission: &MissionDef,
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
        190.0,
        TextStyle::new(34.0, dark::TEXT_BRIGHT).params(),
    );
    draw_text_block(
        &mission.briefing,
        200.0,
        230.0,
        640.0,
        100.0,
        21.0,
        7.0,
        dark::TEXT_DIM,
    );
    if mission.operation_modifier != crate::data::OperationModifier::None {
        draw_ui_text_ex(
            &format!(
                "PRESSURE MODIFIER // {} // {}",
                mission.operation_modifier.label(),
                mission.operation_modifier.description()
            ),
            200.0,
            342.0,
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
    draw_ui_text_ex(
        &format!(
            "DEPLOYMENT // {}/{} SELECTED // SUPPLY {} FOOD",
            campaign.selected_squad_count(),
            crate::campaign::SQUAD_LIMIT,
            campaign.deployment_food_cost(data)
        ),
        200.0,
        370.0,
        TextStyle::new(15.0, dark::ACCENT).params(),
    );
    for (index, character) in campaign.roster.iter().enumerate() {
        let class_name = data
            .classes
            .iter()
            .find(|class| class.id == character.active_class)
            .map_or(character.active_class.as_str(), |class| class.name.as_str());
        let state = if character.availability != crate::campaign::Availability::Ready {
            "RECOVERING"
        } else if character.deployment_selected {
            "DEPLOY"
        } else {
            "RESERVE"
        };
        let label = format!(
            "[{}] {} · {} · LV{} · {} XP",
            state, character.name, class_name, character.level, character.experience
        );
        if button(
            Rect::new(200.0, 384.0 + index as f32 * 32.0, 650.0, 28.0),
            &label,
            character.availability == crate::campaign::Availability::Ready,
            mouse,
        ) {
            actions.push(UiAction::ToggleDeployment(character.id.clone()));
        }
    }
    if button(
        Rect::new(820.0, 520.0, 250.0, 48.0),
        &format!(
            "DEPLOY SQUAD · {} FOOD",
            campaign.deployment_food_cost(data)
        ),
        campaign.selected_squad_count() > 0
            && campaign.colony.resources.food >= campaign.deployment_food_cost(data),
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
        actions.push(UiAction::ReturnToColony);
    }
    actions
}

pub fn draw_debrief(
    mission: &MissionDef,
    outcome: &MissionOutcome,
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
            "OPERATION SUCCESS"
        } else {
            "OPERATION FAILED"
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
        format!("Operation: {}", mission.name),
        format!(
            "Squad returned: {}/{}",
            outcome.colonists_deployed - outcome.colonists_incapacitated.len(),
            outcome.colonists_deployed
        ),
        injuries,
        format!("Hostiles neutralised: {}", outcome.hostiles_neutralised),
        format!(
            "Recovered: {} materials · {} biomass · {} power",
            outcome.materials_awarded, outcome.biomass_awarded, outcome.power_awarded
        ),
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
        actions.push(UiAction::ReturnToColony);
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
        &ctx.mission.name,
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
    let view = GridView::new(
        ctx.session.tactical.fog.width,
        ctx.session.tactical.fog.height,
        grid_rect,
    );
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
        if matches!(
            ctx.mission.objective_kind,
            ObjectiveKind::SecureAndClear | ObjectiveKind::Extraction | ObjectiveKind::SignalTrace
        ) && position == ctx.session.tactical.objective_tile
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
        if let Some(cover) = ctx
            .session
            .tactical
            .destructible_cover
            .iter()
            .find(|cover| cover.position == position)
        {
            crate::cover_ui::draw_cover(
                view,
                cover,
                ctx.session.can_attack_selected_cover(position),
            );
        }
    }
    for unit in &ctx.session.tactical.units {
        let targetable = match ctx.targeting {
            Some(TargetingView::Equipment {
                unit_id,
                equipment_id,
            }) => ctx
                .session
                .can_use_equipment(unit_id, equipment_id, &unit.id),
            Some(TargetingView::ClassAction { unit_id }) => {
                ctx.session.can_target_class_action(unit_id, &unit.id)
            }
            None => false,
        };
        crate::tactical_unit_ui::draw_unit(
            view,
            unit,
            ctx.session.tactical.selected_unit.as_deref() == Some(&unit.id),
            targetable,
        );
    }
    if is_mouse_button_released(MouseButton::Left) {
        if let Some(tile) = view.tile_at(mouse) {
            if let Some(targeting) = ctx.targeting {
                let target = ctx
                    .session
                    .tactical
                    .units
                    .iter()
                    .find(|unit| unit.position == tile);
                let action = target.and_then(|target| match targeting {
                    TargetingView::Equipment {
                        unit_id,
                        equipment_id,
                    } if ctx
                        .session
                        .can_use_equipment(unit_id, equipment_id, &target.id) =>
                    {
                        Some(UiAction::UseEquipmentOn(target.id.clone()))
                    }
                    TargetingView::ClassAction { unit_id }
                        if ctx.session.can_target_class_action(unit_id, &target.id) =>
                    {
                        Some(UiAction::UseClassActionOn(target.id.clone()))
                    }
                    _ => None,
                });
                if let Some(action) = action {
                    actions.push(action);
                } else {
                    actions.push(UiAction::CancelTargeting);
                }
                return;
            }
            let hostile = ctx
                .session
                .tactical
                .units
                .iter()
                .find(|unit| unit.position == tile && unit.team == Team::Hostile);
            if let Some(hostile) = hostile.filter(|unit| ctx.session.can_attack_selected(&unit.id))
            {
                actions.push(UiAction::AttackSelected(hostile.id.clone()));
            } else if ctx.session.can_attack_selected_cover(tile) {
                actions.push(UiAction::AttackCover(tile));
            } else if ctx.session.can_move_selected_to(tile) {
                actions.push(UiAction::MoveSelected(tile));
            } else {
                actions.push(UiAction::SelectTile(tile));
            }
        }
    }
}

fn draw_sidebar(ctx: &UiContext<'_>, mouse: Vec2, actions: &mut Vec<UiAction>) {
    let panel = Rect::new(856.0, 96.0, 406.0, 530.0);
    draw_surface_with_title(
        panel,
        Some(ctx.mission.name.as_str()),
        &SurfaceStyle::new(Color::new(0.055, 0.07, 0.08, 0.98))
            .with_border(1.0, Color::new(0.25, 0.38, 0.40, 0.8))
            .with_header(42.0, Color::new(0.08, 0.105, 0.115, 1.0)),
        TextStyle::new(17.0, dark::TEXT),
    );
    let x = panel.x + 18.0;
    draw_ui_text_ex(
        &format!("OBJECTIVE // {}", objective_progress(ctx)),
        x,
        panel.y + 78.0,
        TextStyle::new(15.0, Color::new(0.43, 0.83, 0.69, 1.0)).params(),
    );
    draw_text_block(
        &ctx.mission.objective,
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
        draw_ui_text_ex(
            &action_status(unit),
            x,
            panel.y + 332.0,
            TextStyle::new(14.0, dark::TEXT_DIM).params(),
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
    if button(
        Rect::new(x, panel.bottom() - 100.0, panel.w - 36.0, 44.0),
        "END COLONY PHASE",
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

fn objective_progress(ctx: &UiContext<'_>) -> String {
    let hostiles = ctx
        .session
        .tactical
        .units
        .iter()
        .filter(|unit| unit.team == Team::Hostile && !unit.incapacitated)
        .count();
    match ctx.mission.objective_kind {
        ObjectiveKind::SecureAndClear => match ctx.session.tactical.objective_state {
            ObjectiveState::Active => format!("UNSECURED · {} HOSTILES", hostiles),
            ObjectiveState::Secured => format!("SECURED · {} HOSTILES", hostiles),
            ObjectiveState::Victory => "SECURED · AREA CLEAR".to_owned(),
            ObjectiveState::Failed => "FAILED".to_owned(),
        },
        ObjectiveKind::EliminateAll => format!("{} HOSTILES", hostiles),
        ObjectiveKind::Holdout => {
            let remaining = ctx
                .mission
                .round_limit
                .saturating_sub(ctx.session.tactical.round)
                + 1;
            let waves = ctx.session.tactical.reinforcement_waves.len();
            format!(
                "{} ROUNDS · {} HOSTILES · {} WAVES",
                remaining, hostiles, waves
            )
        }
        ObjectiveKind::Extraction => match ctx.session.tactical.objective_state {
            ObjectiveState::Active => format!("REACH EVAC · {} HOSTILES", hostiles),
            ObjectiveState::Victory => "COLONIST EVACUATED".to_owned(),
            ObjectiveState::Failed => "EVACUATION FAILED".to_owned(),
            ObjectiveState::Secured => "EVACUATION CONFIRMED".to_owned(),
        },
        ObjectiveKind::SignalTrace => match ctx.session.tactical.objective_state {
            ObjectiveState::Active => format!("RELAY OFFLINE · {} HOSTILES", hostiles),
            ObjectiveState::Secured => format!(
                "TRACE {} ROUNDS · {} HOSTILES · {} WAVES",
                ctx.mission
                    .round_limit
                    .saturating_sub(ctx.session.tactical.round)
                    + 1,
                hostiles,
                ctx.session.tactical.reinforcement_waves.len()
            ),
            ObjectiveState::Victory => "CONTACT TRACE COMPLETE".to_owned(),
            ObjectiveState::Failed => "CONTACT TRACE LOST".to_owned(),
        },
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
            "{} campaign  //  {} assets  //  click a hostile to attack",
            if ctx.mission.name.starts_with("ADAPTATION:") {
                "Adaptation"
            } else if ctx.mission.name.starts_with("CONTACT:") {
                "Contact"
            } else {
                "Isolation"
            },
            ctx.loaded_assets
        ),
        620.0,
        y + 28.0,
        TextStyle::new(14.0, dark::TEXT_DIM).params(),
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
