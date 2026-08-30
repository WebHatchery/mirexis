//! Operations-panel affordance for the Relay Mast's risky intelligence pulse.

use crate::campaign::{CampaignState, RELAY_SCAN_POWER_COST, RELAY_SIGNAL_ATTENTION};
use crate::colony::BuildingKind;
use crate::ui::UiAction;
use crate::ui_widgets::button;
use macroquad::prelude::{Rect, Vec2};

pub(super) fn draw(
    campaign: &CampaignState,
    routine_operations_visible: bool,
    mouse: Vec2,
    actions: &mut Vec<UiAction>,
) {
    if !routine_operations_visible || !campaign.colony.has_facility(BuildingKind::RelayMast) {
        return;
    }
    let already_scanned = campaign.relay_scan_operation == Some(campaign.operations_completed);
    let label = if already_scanned {
        "RELAY // SCANNED".to_owned()
    } else {
        format!(
            "RELAY // {} PWR +{} ATT",
            RELAY_SCAN_POWER_COST, RELAY_SIGNAL_ATTENTION
        )
    };
    if button(
        Rect::new(1064.0, 344.0, 176.0, 30.0),
        &label,
        campaign.relay_scan_available(),
        mouse,
    ) {
        actions.push(UiAction::RunRelayScan);
    }
}
