//! Operations-panel affordance for the next field doctrine.

pub(super) fn button_label(name: &str, materials: i32, cost: i32) -> String {
    if materials < cost {
        format!("RESEARCH {} // NEED {} MAT", name, cost)
    } else {
        format!("RESEARCH {} // {} MAT", name, cost)
    }
}

#[cfg(test)]
mod tests;
