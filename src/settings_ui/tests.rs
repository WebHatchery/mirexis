use super::*;

#[test]
fn volume_meter_tracks_supported_range() {
    let bounds = volume_meter_bounds();

    assert_eq!(volume_meter_fill_width(0), 0.0);
    assert_eq!(volume_meter_fill_width(50), bounds.w / 2.0);
    assert_eq!(volume_meter_fill_width(100), bounds.w);
}

#[test]
fn volume_meter_clamps_corrupt_values() {
    assert_eq!(volume_meter_fill_width(u8::MAX), volume_meter_bounds().w);
}
