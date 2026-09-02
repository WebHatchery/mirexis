use super::*;
use crate::first_hour::FirstHourStage;

#[test]
fn goal_banner_avoids_the_debrief_and_keeps_controls_inside_each_layout() {
    let return_layout = goal_banner_layout(FirstHourStage::FirstReturn);
    let ordinary_layout = goal_banner_layout(FirstHourStage::MakeInvestment);

    assert!(return_layout.panel.bottom() < ordinary_layout.panel.y);
    for layout in [return_layout, ordinary_layout] {
        assert!(layout.panel.contains(layout.text_origin));
        assert!(layout.panel.contains(vec2(
            layout.help_button.x + layout.help_button.w,
            layout.help_button.y + layout.help_button.h,
        )));
    }
}

#[test]
fn debrief_return_button_stays_inside_the_result_panel() {
    let button = debrief_return_button_bounds();
    let panel = Rect::new(140.0, 76.0, 1000.0, 568.0);

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

#[test]
fn advance_focus_points_only_to_the_required_start_and_promise_controls() {
    assert_eq!(
        advance_focus_target(FirstHourStage::Arrival),
        Some(Rect::new(342.0, 114.0, 182.0, 26.0))
    );
    assert_eq!(
        advance_focus_target(FirstHourStage::Promise),
        Some(Rect::new(330.0, 114.0, 194.0, 26.0))
    );
    assert_eq!(advance_focus_target(FirstHourStage::MeetCoordinator), None);
}

#[test]
fn advance_controls_stay_inside_the_standard_goal_banner() {
    let panel = Rect::new(20.0, 76.0, 520.0, 96.0);

    for button in [
        begin_arrival_button_bounds(),
        continue_campaign_button_bounds(),
    ] {
        assert!(button.x >= panel.x);
        assert!(button.y >= panel.y);
        assert!(button.right() <= panel.right());
        assert!(button.bottom() <= panel.bottom());
    }
}

#[test]
fn goal_focus_disappears_when_guidance_is_skipped() {
    let mut progress = FirstHourProgress {
        stage: FirstHourStage::Promise,
        ..FirstHourProgress::default()
    };
    progress.guidance_enabled = false;

    assert_eq!(focus_target(&progress), None);
}
