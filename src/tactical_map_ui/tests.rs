use super::*;

#[test]
fn visible_tactical_camera_controls_stay_inside_the_header_and_outside_the_map() {
    let panel = tactical_panel();
    let viewport = tactical_viewport(panel);
    let origin = camera_controls_origin(panel);
    let bounds = Rect::new(
        origin.x,
        origin.y,
        crate::camera_controls::STRIP_WIDTH,
        crate::camera_controls::CONTROL_HEIGHT,
    );

    assert!(bounds.x >= panel.x);
    assert!(bounds.right() <= panel.right());
    assert!(bounds.y >= panel.y);
    assert!(bounds.bottom() <= viewport.y);
}
