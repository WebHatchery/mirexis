//! Colony hub presentation and strategic intent production.
pub mod operations;

use crate::campaign::{Availability, CampaignState};
use crate::colony::{BuildingKind, SIGNAL_CARTOGRAPHY_UPGRADE};
use crate::data::GameData;
use crate::ui::UiAction;
use crate::visual_assets::VisualCatalog;
use macroquad::prelude::*;
use macroquad_toolkit::assets::AssetManager;
use macroquad_toolkit::prelude::*;

pub mod commons;
pub mod context;
pub mod decision_affordance;
pub mod decisions;
pub mod event_affordance;
pub mod medical;
pub mod recruitment;
pub mod relay;
pub mod research;
pub mod research_affordance;
pub mod salvage;
pub mod scene;
pub mod upgrades;
pub use context::{ColonyDrawContext, ColonyDrawResult};
pub use scene::{colony_map_input_enabled, draw_colony};

// Dense late-campaign hubs can exhaust Macroquad's per-font-size glyph atlas
// when every label shares the toolkit font. The hub's buttons and map labels
// already use the built-in font, so keep all colony text on that stable atlas.
pub fn draw_ui_text_ex<'a>(
    text: &str,
    x: f32,
    y: f32,
    mut params: TextParams<'a>,
) -> TextDimensions {
    params.font = None;
    draw_text_ex(text, x, y, params)
}

// Operations panels combine several read-only presentation sources and a
// single returned action sink; the explicit arguments preserve that boundary.
#[allow(clippy::too_many_arguments)]
pub fn should_draw_colony_plan(campaign: &CampaignState) -> bool {
    !campaign.strategy.campaign_complete
}

pub fn mission_briefing_bounds() -> Rect {
    Rect::new(878.0, 478.0, 362.0, 30.0)
}

pub fn character_event_card_bounds() -> Rect {
    Rect::new(878.0, 514.0, 362.0, 162.0)
}

pub fn colony_plan_baseline(campaign: &CampaignState, research_surface_visible: bool) -> f32 {
    if research_surface_visible {
        676.0
    } else if campaign.strategy.available_event().is_some() {
        668.0
    } else {
        650.0
    }
}

pub fn draw_character_event(
    campaign: &CampaignState,
    data: &GameData,
    assets: &AssetManager,
    visuals: &VisualCatalog,
    event: &crate::strategy::CharacterEventState,
    mouse: Vec2,
    actions: &mut Vec<UiAction>,
) {
    let definition = data
        .campaign
        .events
        .iter()
        .find(|definition| definition.id == event.id);
    let legacy_stat = if event.legacy_stat.is_empty() {
        definition.map_or("", |definition| definition.legacy_stat.as_str())
    } else {
        event.legacy_stat.as_str()
    };
    let legacy_amount = if event.legacy_amount == 0 {
        definition.map_or(0, |definition| definition.legacy_amount)
    } else {
        event.legacy_amount
    };
    let attention_faction = if event.attention_faction.is_empty() {
        "directorate"
    } else {
        event.attention_faction.as_str()
    };
    let attention_name = campaign
        .strategy
        .factions
        .iter()
        .find(|faction| faction.id == attention_faction)
        .map_or(attention_faction, |faction| faction.name.as_str());
    let card = character_event_card_bounds();
    draw_rectangle(
        card.x,
        card.y,
        card.w,
        card.h,
        Color::new(0.038, 0.058, 0.062, 0.98),
    );
    draw_rectangle_lines(card.x, card.y, card.w, card.h, 1.0, dark::WARNING);
    draw_ui_text_ex(
        "COLONY EVENT // DECISION",
        card.x,
        card.y + 14.0,
        TextStyle::new(12.0, dark::WARNING).params(),
    );
    if let Some(definition) = definition {
        for (index, participant) in definition.participants.iter().take(2).enumerate() {
            if let Some(character) = campaign
                .roster
                .iter()
                .find(|character| &character.id == participant)
            {
                crate::portrait_ui::draw_character_portrait(
                    assets,
                    visuals,
                    Rect::new(card.x + index as f32 * 56.0, card.y + 22.0, 50.0, 58.0),
                    &character.id,
                    &character.name,
                    dark::WARNING,
                );
            }
        }
        draw_ui_text_ex(
            &event.title.to_uppercase(),
            card.x + 116.0,
            card.y + 30.0,
            TextStyle::new(13.0, dark::TEXT_BRIGHT).params(),
        );
        for (index, line) in fit_text_to_box_ex(
            &definition.description,
            card.w - 124.0,
            38.0,
            TextStyle::new(10.0, dark::TEXT_DIM)
                .with_macroquad_font()
                .with_line_gap(4.0),
            10.0,
        )
        .lines
        .into_iter()
        .take(3)
        .enumerate()
        {
            draw_ui_text_ex(
                &line,
                card.x + 116.0,
                card.y + 49.0 + index as f32 * 14.0,
                TextStyle::new(10.0, dark::TEXT_DIM).params(),
            );
        }
    }
    draw_ui_text_ex(
        &format!(
            "CHOOSE CARRIER // {:+} {} // BOND +2 // {} FOOD // {} {:+}",
            legacy_amount,
            legacy_stat.to_uppercase(),
            event.food_cost,
            attention_name.to_uppercase(),
            event.attention_change
        ),
        card.x,
        card.y + 86.0,
        TextStyle::new(10.0, dark::TEXT_DIM).params(),
    );
    for (index, participant_id) in event.participants.iter().take(2).enumerate() {
        let Some(character) = campaign
            .roster
            .iter()
            .find(|character| &character.id == participant_id)
        else {
            continue;
        };
        let first_name = character
            .name
            .split_whitespace()
            .next()
            .unwrap_or(&character.name);
        let label = event_affordance::button_label(
            first_name,
            legacy_amount,
            legacy_stat,
            event.food_cost,
            campaign.colony.resources.food,
        );
        if colony_button(
            Rect::new(card.x, card.y + 92.0 + index as f32 * 24.0, card.w, 22.0),
            &label,
            campaign
                .strategy
                .can_resolve_first_event(campaign.colony.resources.food),
            mouse,
        ) {
            actions.push(UiAction::ResolveCharacterEvent(participant_id.clone()));
        }
    }
}

pub fn draw_ending_card(campaign: &CampaignState, assets: &AssetManager, visuals: &VisualCatalog) {
    let (cell, accent) = match campaign.strategy.mirexis_path_id.as_str() {
        "human_redoubt" => (10, Color::new(0.36, 0.70, 0.88, 1.0)),
        "living_commonwealth" => (9, Color::new(0.64, 0.92, 0.38, 1.0)),
        "open_threshold" => (11, Color::new(0.72, 0.52, 1.0, 1.0)),
        _ => (2, dark::ACCENT),
    };
    let rect = Rect::new(878.0, 512.0, 80.0, 72.0);
    visuals.draw_atlas_cell(assets, &visuals.terrain, cell, rect, WHITE);
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, accent);
    draw_line(
        rect.x + 8.0,
        rect.bottom() - 8.0,
        rect.right() - 8.0,
        rect.y + 8.0,
        2.0,
        accent,
    );
}

pub fn draw_epilogue_dossier(dossier: &crate::epilogue::EpilogueDossier) {
    draw_ui_text_ex(
        "COLONY LEGACY REGISTER",
        878.0,
        590.0,
        TextStyle::new(11.0, dark::ACCENT).params(),
    );
    for (index, line) in dossier.register_lines().iter().enumerate() {
        draw_ui_text_ex(
            line,
            878.0,
            602.0 + index as f32 * 11.0,
            TextStyle::new(7.5, dark::TEXT_DIM).params(),
        );
    }
}

pub fn colony_button(rect: Rect, label: &str, enabled: bool, mouse: Vec2) -> bool {
    crate::ui_widgets::button(rect, label, enabled, mouse)
}
