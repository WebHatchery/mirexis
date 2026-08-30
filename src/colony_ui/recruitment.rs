use super::{colony_button, draw_ui_text_ex};
use crate::campaign::CampaignState;
use crate::data::GameData;
use crate::ui::UiAction;
use macroquad::prelude::*;
use macroquad_toolkit::prelude::{dark, TextStyle};

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
        &format!(
            "RECRUIT {} // {} {}",
            outsider.name.to_uppercase(),
            outsider.recruitment_cost,
            resource.to_uppercase()
        ),
        campaign.recruitment_resource_amount(resource) >= outsider.recruitment_cost,
        mouse,
    ) {
        actions.push(UiAction::RecruitOutsider);
    }
}
