//! Operations-panel affordance for field-doctrine choices.

pub(super) fn choice_button_label(materials: i32, cost: i32) -> String {
    if materials < cost {
        format!("NEED {} MAT", cost)
    } else {
        format!("COMPLETE // {} MAT", cost)
    }
}

#[cfg(test)]
mod tests;
