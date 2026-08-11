use super::*;

#[test]
fn logical_clip_rect_converts_to_letterboxed_physical_pixels() {
    let ui = VirtualUi {
        logical_width: 1280.0,
        logical_height: 720.0,
        scale: 0.8,
        offset: vec2(40.0, 12.0),
    };
    assert_eq!(
        ui_clip_pixels(&ui, Rect::new(18.0, 106.0, 884.0, 568.0), 1.5),
        (82, 145, 1061, 682)
    );
}
