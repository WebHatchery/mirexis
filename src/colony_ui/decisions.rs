//! Resource-gated Contact, Escalation, and Mirexis choice panels.

use super::{colony_button, decision_affordance, draw_ui_text_ex};
use crate::campaign::CampaignState;
use crate::colony::Resources;
use crate::colony_decision_ui::DecisionKind;
use crate::data::GameData;
use crate::ui::UiAction;
use macroquad::prelude::*;
use macroquad_toolkit::prelude::{dark, TextStyle};

pub(super) fn draw_choice_panel(
    kind: DecisionKind,
    campaign: &CampaignState,
    data: &GameData,
    mouse: Vec2,
    actions: &mut Vec<UiAction>,
) {
    match kind {
        DecisionKind::Contact => draw_contact(campaign, data, mouse, actions),
        DecisionKind::Escalation => draw_escalation(campaign, data, mouse, actions),
        DecisionKind::Mirexis => draw_mirexis(campaign, data, mouse, actions),
    }
}

fn draw_contact(
    campaign: &CampaignState,
    data: &GameData,
    mouse: Vec2,
    actions: &mut Vec<UiAction>,
) {
    draw_ui_text_ex(
        "FIRST CONTACT // SPEND 2 COMPONENTS // CHOOSE ONE",
        878.0,
        514.0,
        TextStyle::new(11.0, dark::ACCENT).params(),
    );
    for (index, protocol) in data.campaign.contact_protocols.iter().enumerate() {
        let bonus = if protocol.materials_bonus > 0 {
            format!("+{} MATERIALS", protocol.materials_bonus)
        } else if protocol.biomass_bonus > 0 {
            format!("+{} BIOMASS", protocol.biomass_bonus)
        } else {
            format!("+{} POWER", protocol.power_bonus)
        };
        let affordable =
            campaign.colony.resources.alien_components >= protocol.alien_components_cost;
        let label = decision_affordance::button_label(
            &protocol.name,
            &bonus,
            affordable,
            &format!("{} COMPONENTS", protocol.alien_components_cost),
        );
        if colony_button(
            Rect::new(878.0, 516.0 + index as f32 * 31.0, 362.0, 30.0),
            &label,
            affordable,
            mouse,
        ) {
            actions.push(UiAction::ChooseContactProtocol(protocol.id.clone()));
        }
    }
}

fn draw_escalation(
    campaign: &CampaignState,
    data: &GameData,
    mouse: Vec2,
    actions: &mut Vec<UiAction>,
) {
    draw_ui_text_ex(
        "CONVERGENCE CAPTURED // CHOOSE ONE RESPONSE",
        878.0,
        514.0,
        TextStyle::new(11.0, dark::ACCENT).params(),
    );
    for (index, response) in data.campaign.escalation_responses.iter().enumerate() {
        let effect = if response.threat_delay > 0 {
            format!(
                "{} MAT // DELAY ASSAULT +{}",
                response.materials_cost, response.threat_delay
            )
        } else if response.attention_change_all < 0 {
            format!(
                "{} BIOMASS // ATTENTION {}",
                response.biomass_cost, response.attention_change_all
            )
        } else {
            format!(
                "{} POWER // +{} MAT / ATTENTION +{}",
                response.power_cost, response.materials_bonus, response.attention_change_all
            )
        };
        let affordable = resources_can_pay(
            &campaign.colony.resources,
            response.materials_cost,
            response.biomass_cost,
            response.power_cost,
        );
        let shortfalls = decision_affordance::resource_shortfalls(
            &campaign.colony.resources,
            response.materials_cost,
            response.biomass_cost,
            response.power_cost,
        );
        let label =
            decision_affordance::button_label(&response.name, &effect, affordable, &shortfalls);
        if colony_button(
            Rect::new(878.0, 516.0 + index as f32 * 31.0, 362.0, 30.0),
            &label,
            affordable,
            mouse,
        ) {
            actions.push(UiAction::ChooseEscalationResponse(response.id.clone()));
        }
    }
}

fn draw_mirexis(
    campaign: &CampaignState,
    data: &GameData,
    mouse: Vec2,
    actions: &mut Vec<UiAction>,
) {
    draw_ui_text_ex(
        "MIREXIS REVEALED // CHOOSE WHAT THE COLONY BECOMES",
        878.0,
        514.0,
        TextStyle::new(11.0, dark::ACCENT).params(),
    );
    for (index, path) in data.campaign.mirexis_paths.iter().enumerate() {
        let effect = if path.defense_cover_bonus > 0 {
            format!(
                "{} MAT // DEFENCE COVER +{}",
                path.materials_cost, path.defense_cover_bonus
            )
        } else if path.deployment_food_discount > 0 {
            format!(
                "{} BIO // FOOD -{}",
                path.biomass_cost, path.deployment_food_discount
            )
        } else {
            format!(
                "{} POWER // VICTORY POWER +{}",
                path.power_cost, path.power_bonus
            )
        };
        let affordable = resources_can_pay(
            &campaign.colony.resources,
            path.materials_cost,
            path.biomass_cost,
            path.power_cost,
        );
        let shortfalls = decision_affordance::resource_shortfalls(
            &campaign.colony.resources,
            path.materials_cost,
            path.biomass_cost,
            path.power_cost,
        );
        let label = decision_affordance::button_label(&path.name, &effect, affordable, &shortfalls);
        if colony_button(
            Rect::new(878.0, 516.0 + index as f32 * 31.0, 362.0, 30.0),
            &label,
            affordable,
            mouse,
        ) {
            actions.push(UiAction::ChooseMirexisPath(path.id.clone()));
        }
    }
}

fn resources_can_pay(
    resources: &Resources,
    materials_cost: i32,
    biomass_cost: i32,
    power_cost: i32,
) -> bool {
    resources.materials >= materials_cost
        && resources.biomass >= biomass_cost
        && resources.power >= power_cost
}
