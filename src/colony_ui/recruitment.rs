use super::{colony_button, draw_ui_text_ex};
use crate::campaign::CampaignState;
use crate::data::GameData;
use crate::ui::UiAction;
use macroquad::prelude::*;
use macroquad_toolkit::prelude::{dark, TextStyle};

fn recruit_button_label(name: &str, cost: i32, resource: &str, available: i32) -> String {
    let name = name.to_uppercase();
    let resource = resource.to_uppercase();
    if available < cost {
        format!("RECRUIT {name} // NEED {cost} {resource}")
    } else {
        format!("RECRUIT {name} // {cost} {resource}")
    }
}

pub(super) fn draw(
    campaign: &CampaignState,
    data: &GameData,
    mouse: Vec2,
    choosing_contact: bool,
    choosing_escalation: bool,
    choosing_mirexis: bool,
    actions: &mut Vec<UiAction>,
) {
    if !campaign.outsider_recruit_available(data)
        || choosing_contact
        || choosing_escalation
        || choosing_mirexis
        || campaign.strategy.available_event().is_some()
    {
        return;
    }
    let outsider = campaign
        .available_outsider(data)
        .expect("recruitment availability should provide an outsider");
    let resource = if outsider.recruitment_resource.is_empty() {
        "materials"
    } else {
        outsider.recruitment_resource.as_str()
    };
    let resource_available = campaign.recruitment_resource_amount(resource);
    draw_ui_text_ex(
        &format!(
            "WAYSTATION // {} AWAITING A DECISION",
            outsider.origin.to_uppercase()
        ),
        878.0,
        600.0,
        TextStyle::new(12.0, dark::WARNING).params(),
    );
    if colony_button(
        Rect::new(878.0, 606.0, 362.0, 36.0),
        &recruit_button_label(
            &outsider.name,
            outsider.recruitment_cost,
            resource,
            resource_available,
        ),
        resource_available >= outsider.recruitment_cost,
        mouse,
    ) {
        actions.push(UiAction::RecruitOutsider);
    }
}

#[cfg(test)]
mod tests;
