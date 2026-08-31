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

#[test]
fn debrief_return_button_stays_inside_the_result_panel() {
    let button = debrief_return_button_bounds();
    let panel = Rect::new(140.0, 76.0, 1000.0, 568.0);

    assert_eq!(button, Rect::new(860.0, 574.0, 220.0, 48.0));
    assert!(button.x >= panel.x);
    assert!(button.y >= panel.y);
    assert!(button.right() <= panel.right());
    assert!(button.bottom() <= panel.bottom());
}

#[test]
fn debrief_focus_covers_both_operation_returns_only() {
    let button = Some(debrief_return_button_bounds());

    assert_eq!(debrief_focus_target(FirstHourStage::FirstReturn), button);
    assert_eq!(debrief_focus_target(FirstHourStage::SecondReturn), button);
    assert_eq!(
        debrief_focus_target(FirstHourStage::FirstReturnColony),
        None
    );
}
