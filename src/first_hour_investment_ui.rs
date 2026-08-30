//! Three legible and equally priced preparations for the second operation.

use crate::campaign::CampaignState;
use crate::first_hour::{FirstHourProgress, FirstHourStage};
use crate::ui::UiAction;
use crate::ui_widgets::button;
use macroquad::prelude::*;
use macroquad_toolkit::prelude::{dark, TextStyle};

pub(crate) const INVESTMENT_COST: i32 = 24;

pub(crate) fn label(id: &str) -> &'static str {
    match id {
        "bastion_mesh" => "Bastion Mesh",
        "survey_uplink" => "Survey Uplink",
        "rapid_injectors" => "Rapid Injectors",
        _ => "Unknown preparation",
    }
}

pub(crate) fn effect(id: &str) -> &'static str {
    match id {
        "bastion_mesh" => "+1 ARMOUR",
        "survey_uplink" => "+8 ACCURACY",
        "rapid_injectors" => "+1 MOVEMENT",
        _ => "",
    }
}

pub(crate) fn active_summary(id: &str) -> Option<&'static str> {
    match id {
        "bastion_mesh" => Some("PREP // BASTION MESH // SQUAD +1 ARM"),
        "survey_uplink" => Some("PREP // SURVEY UPLINK // SQUAD +8 ACC"),
        "rapid_injectors" => Some("PREP // RAPID INJECTORS // SQUAD +1 MOV"),
        _ => None,
    }
}

pub(crate) fn tactical_summary(progress: &FirstHourProgress) -> Option<&'static str> {
    (progress.stage == FirstHourStage::SecondOperationTactical)
        .then(|| active_summary(&progress.investment_name))
        .flatten()
}

pub(crate) fn draw_tactical_summary(progress: &FirstHourProgress, origin: Vec2) {
    let Some(summary) = tactical_summary(progress) else {
        return;
    };
    draw_text_ex(
        summary,
        origin.x,
        origin.y,
        TextStyle::new(10.0, dark::POSITIVE).params(),
    );
}

pub(crate) fn draw_active_summary(campaign: &CampaignState, origin: Vec2) {
    if campaign.operations_completed != 1 {
        return;
    }
    let Some(summary) = active_summary(&campaign.first_hour.investment_name) else {
        return;
    };
    draw_text_ex(
        summary,
        origin.x,
        origin.y,
        TextStyle::new(10.0, dark::POSITIVE).params(),
    );
}

pub(crate) fn draw(campaign: &CampaignState, mouse: Vec2, actions: &mut Vec<UiAction>) {
    text(
        "CHOOSE ONE FIELD PREPARATION",
        878.0,
        302.0,
        17.0,
        dark::ACCENT,
    );
    text(
        "Each costs 24 materials and modifies the entire second-operation squad.",
        878.0,
        326.0,
        11.0,
        dark::TEXT_DIM,
    );
    let affordable = campaign.colony.resources.materials >= INVESTMENT_COST;
    for (index, (id, name, effect)) in [
        ("bastion_mesh", "BASTION MESH", "+1 ARMOUR"),
        ("survey_uplink", "SURVEY UPLINK", "+8 ACCURACY"),
        ("rapid_injectors", "RAPID INJECTORS", "+1 MOVEMENT"),
    ]
    .iter()
    .enumerate()
    {
        let y = 352.0 + index as f32 * 74.0;
        if button(
            Rect::new(878.0, y, 362.0, 38.0),
            &format!("{name} // {INVESTMENT_COST} MAT"),
            affordable,
            mouse,
        ) {
            actions.push(UiAction::ChooseFirstHourInvestment((*id).to_owned()));
        }
        text(effect, 892.0, y + 56.0, 13.0, dark::POSITIVE);
    }
    if !affordable {
        text(
            "LOCKED // Recover 24 materials before committing a preparation.",
            878.0,
            590.0,
            12.0,
            dark::NEGATIVE,
        );
    }
}

fn text(value: &str, x: f32, y: f32, size: f32, color: Color) {
    draw_text_ex(value, x, y, TextStyle::new(size, color).params());
}

#[cfg(test)]
mod tests;
