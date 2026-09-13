//! Operations-panel affordance for field-doctrine choices.

pub fn choice_button_label(materials: i32, cost: i32) -> String {
    if materials < cost {
        format!("NEED {} MAT", cost)
    } else {
        format!("COMPLETE // {} MAT", cost)
    }
}
