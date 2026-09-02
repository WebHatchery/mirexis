use super::*;

#[test]
fn volume_meter_scales_supported_values_and_clamps_corrupt_input() {
    let bounds = volume_meter_bounds();

    for (volume, expected) in [
        (0, 0.0),
        (50, bounds.w / 2.0),
        (100, bounds.w),
        (u8::MAX, bounds.w),
    ] {
        assert_eq!(volume_meter_fill_width(volume), expected, "volume={volume}");
    }
}
