//! Waystation conversations and route-exclusive outsider choices.

use crate::campaign::{CampaignState, OutsiderChoice};
use crate::ui::UiAction;
use crate::ui_widgets::button;
use crate::visual_assets::VisualCatalog;
use macroquad::prelude::*;
use macroquad_toolkit::assets::AssetManager;
use macroquad_toolkit::prelude::{dark, draw_ui_text_ex, TextStyle};

pub(crate) fn draw(
    campaign: &CampaignState,
    assets: &AssetManager,
    visuals: &VisualCatalog,
    mouse: Vec2,
    actions: &mut Vec<UiAction>,
) {
    let Some(beat) = campaign.outsider_arc_beat() else {
        return;
    };
    draw_ui_text_ex(
        &format!(
            "WAYSTATION // OUTSIDER ARC // BEAT {}/3 // DISAGREEMENTS {}/2",
            beat.stage + 1,
            campaign.outsider_arc_disagreements().min(2)
        ),
        878.0,
        510.0,
        TextStyle::new(12.0, dark::WARNING).params(),
    );
    visuals.draw_portrait(
        assets,
        beat.outsider_id,
        beat.outsider_name,
        Rect::new(878.0, 518.0, 48.0, 56.0),
        dark::WARNING,
    );
    draw_ui_text_ex(
        beat.title,
        938.0,
        528.0,
        TextStyle::new(13.0, dark::TEXT_BRIGHT).params(),
    );
    let description = wrap_words(beat.description, 42);
    for (index, line) in description.into_iter().take(3).enumerate() {
        draw_ui_text_ex(
            &line,
            938.0,
            545.0 + index as f32 * 12.0,
            TextStyle::new(9.5, dark::TEXT_DIM).params(),
        );
    }
    for (index, choice) in beat.choices.iter().enumerate() {
        let rect = Rect::new(878.0 + index as f32 * 184.0, 580.0, 178.0, 64.0);
        let enabled = affordable(campaign, choice);
        if button(rect, "", enabled, mouse) {
            actions.push(UiAction::ResolveOutsiderBeat(
                beat.stage,
                choice.id.to_owned(),
            ));
        }
        draw_ui_text_ex(
            choice.label,
            rect.x + 8.0,
            rect.y + 18.0,
            TextStyle::new(10.0, if enabled { dark::TEXT } else { dark::TEXT_DIM }).params(),
        );
        draw_ui_text_ex(
            &cost_label(choice),
            rect.x + 8.0,
            rect.y + 32.0,
            TextStyle::new(9.0, dark::ACCENT).params(),
        );
        for (line_index, line) in wrap_words(choice.description, 28)
            .into_iter()
            .take(2)
            .enumerate()
        {
            draw_ui_text_ex(
                &line,
                rect.x + 8.0,
                rect.y + 45.0 + line_index as f32 * 9.0,
                TextStyle::new(8.0, dark::TEXT_DIM).params(),
            );
        }
    }
}

fn affordable(campaign: &CampaignState, choice: &OutsiderChoice) -> bool {
    campaign.colony.resources.materials >= choice.materials_cost
        && campaign.colony.resources.food >= choice.food_cost
        && campaign.colony.resources.power >= choice.power_cost
        && campaign.colony.resources.biomass >= choice.biomass_cost
}

fn cost_label(choice: &OutsiderChoice) -> String {
    let mut costs = Vec::new();
    if choice.materials_cost > 0 {
        costs.push(format!("{} MAT", choice.materials_cost));
    }
    if choice.food_cost > 0 {
        costs.push(format!("{} FOOD", choice.food_cost));
    }
    if choice.power_cost > 0 {
        costs.push(format!("{} POWER", choice.power_cost));
    }
    if choice.biomass_cost > 0 {
        costs.push(format!("{} BIOMASS", choice.biomass_cost));
    }
    if costs.is_empty() {
        "NO RESOURCE COST".to_owned()
    } else {
        costs.join(" // ")
    }
}

fn wrap_words(value: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut line = String::new();
    for word in value.split_whitespace() {
        if !line.is_empty() && line.len() + word.len() + 1 > width {
            lines.push(std::mem::take(&mut line));
        }
        if !line.is_empty() {
            line.push(' ');
        }
        line.push_str(word);
    }
    if !line.is_empty() {
        lines.push(line);
    }
    lines
}
