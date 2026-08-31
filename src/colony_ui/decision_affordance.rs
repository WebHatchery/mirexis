//! Shared labels for irreversible campaign resource decisions.

use crate::colony::Resources;

pub(super) fn button_label(
    name: &str,
    effect: &str,
    affordable: bool,
    requirement: &str,
) -> String {
    let name = name.to_uppercase();
    if affordable {
        format!("{name} // {effect}")
    } else {
        format!("{name} // NEED {requirement}")
    }
}

pub(super) fn resource_shortfalls(
    resources: &Resources,
    materials_cost: i32,
    biomass_cost: i32,
    power_cost: i32,
) -> String {
    let mut shortfalls = Vec::new();
    if resources.materials < materials_cost {
        shortfalls.push(format!("{} MAT", materials_cost));
    }
    if resources.biomass < biomass_cost {
        shortfalls.push(format!("{} BIO", biomass_cost));
    }
    if resources.power < power_cost {
        shortfalls.push(format!("{} PWR", power_cost));
    }
    shortfalls.join(" // ")
}

#[cfg(test)]
mod tests;
