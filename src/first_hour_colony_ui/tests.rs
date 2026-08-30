use super::*;
use crate::first_hour::FirstHourStage;

fn progress(stage: FirstHourStage) -> FirstHourProgress {
    FirstHourProgress {
        stage,
        ..FirstHourProgress::default()
    }
}

#[test]
fn mission_handoff_focus_moves_from_header_to_briefing_button() {
    let progress = progress(FirstHourStage::PrepareFirstOperation);

    let closed = focus_target(&progress, false).unwrap();
    assert_eq!(closed.0, OPERATIONS_BUTTON);
    assert_eq!(closed.1, "OPERATIONS");

    let open = focus_target(&progress, true).unwrap();
    assert_eq!(open.0, BRIEFING_BUTTON);
    assert_eq!(open.1, "BRIEF SELECTED MISSION");
}

#[test]
fn second_operation_focus_keeps_the_handoff_actionable() {
    let progress = progress(FirstHourStage::SecondOperation);
    let target = focus_target(&progress, true).unwrap();

    assert_eq!(target.0, BRIEFING_BUTTON);
    assert_eq!(target.1, "BRIEF SELECTED MISSION");
}

#[test]
fn investment_focus_covers_all_equal_preparation_choices() {
    let progress = progress(FirstHourStage::MakeInvestment);
    let target = focus_target(&progress, true).unwrap();

    assert_eq!(target.0, INVESTMENT_CHOICES);
    assert_eq!(target.1, "CHOOSE PREPARATION");
    assert!(target.0.contains(Vec2::new(900.0, 520.0)));
}

#[test]
fn colony_focus_is_hidden_when_guidance_is_disabled_or_help_is_open() {
    let mut progress = progress(FirstHourStage::SecondOperation);
    progress.guidance_enabled = false;
    assert!(focus_target(&progress, false).is_none());

    progress.guidance_enabled = true;
    progress.help_open = true;
    assert!(focus_target(&progress, false).is_none());
}
