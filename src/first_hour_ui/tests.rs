use super::*;
use crate::first_hour::FirstHourStage;

#[test]
fn return_goal_banner_sits_above_the_debrief_panel() {
    let layout = goal_banner_layout(FirstHourStage::FirstReturn);

    assert_eq!(layout.panel, Rect::new(580.0, 8.0, 680.0, 60.0));
    assert!(layout.panel.bottom() < 76.0);
    assert!(layout.help_button.x + layout.help_button.w <= layout.panel.x + layout.panel.w);
}

#[test]
fn ordinary_goal_banner_keeps_the_standard_left_layout() {
    let layout = goal_banner_layout(FirstHourStage::MakeInvestment);

    assert_eq!(layout.panel, Rect::new(20.0, 76.0, 520.0, 96.0));
    assert_eq!(layout.text_origin, vec2(38.0, 108.0));
}
