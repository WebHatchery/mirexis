use super::*;

#[test]
fn an_audio_event_is_consumed_only_once() {
    let mut played_count = 0;

    assert_eq!(unplayed_event_start(&mut played_count, 1), Some(0));
    assert_eq!(unplayed_event_start(&mut played_count, 1), None);
}

#[test]
fn a_replaced_event_log_starts_a_fresh_audio_cursor() {
    let mut played_count = 9;

    assert_eq!(unplayed_event_start(&mut played_count, 2), Some(0));
    assert_eq!(played_count, 2);
}
