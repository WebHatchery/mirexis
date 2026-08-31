use super::*;

#[test]
fn launcher_stays_inside_the_operations_panel() {
    let panel = Rect::new(862.0, 74.0, 408.0, 608.0);
    let launcher = launcher_bounds();

    assert!(launcher.x >= panel.x);
    assert!(launcher.right() <= panel.right());
    assert!(launcher.y >= panel.y);
    assert!(launcher.bottom() <= panel.bottom());
}

#[test]
fn note_window_keeps_the_selected_note_visible() {
    assert_eq!(note_window_start(0, 20), 0);
    assert_eq!(note_window_start(7, 20), 0);
    assert_eq!(note_window_start(8, 20), 1);
    assert_eq!(note_window_start(19, 20), 12);
    assert_eq!(note_window_start(3, 4), 0);
}
