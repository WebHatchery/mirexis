//! Touch-visible labels for persistent character-event choices.

pub(super) fn button_label(
    participant: &str,
    legacy_amount: i32,
    legacy_stat: &str,
    food_cost: i32,
    available_food: i32,
) -> String {
    let participant = participant.to_uppercase();
    if available_food < food_cost {
        format!("CHOOSE {participant} // NEED {food_cost} FOOD")
    } else {
        format!(
            "CHOOSE {participant} // {legacy_amount:+} {} IN FUTURE BATTLES",
            legacy_stat.to_uppercase()
        )
    }
}

#[cfg(test)]
mod tests;
