use super::*;

#[test]
fn title_recovery_control_is_visible_inside_the_colony_header() {
    let header = Rect::new(10.0, 10.0, LOGICAL_WIDTH - 20.0, 52.0);
    let title = title_bounds();
    assert!(title.x >= header.x);
    assert!(title.right() <= header.right());
    assert!(title.y >= header.y);
    assert!(title.bottom() <= header.bottom());
    assert!(
        title.x >= 1150.0,
        "title control overlaps campaign identity"
    );
}
