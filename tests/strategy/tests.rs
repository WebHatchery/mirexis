//! Strategy cases remain one scenario per phase, faction, and deterministic
//! materialization rule so regressions retain their diagnostic names.

use super::*;

fn positions_are_connected(
    start: [i32; 2],
    goal: [i32; 2],
    blocked: &std::collections::HashSet<[i32; 2]>,
    width: i32,
    height: i32,
) -> bool {
    use std::collections::{HashSet, VecDeque};

    let mut frontier = VecDeque::from([start]);
    let mut visited = HashSet::from([start]);
    while let Some(position) = frontier.pop_front() {
        if position == goal {
            return true;
        }
        for next in [
            [position[0] - 1, position[1]],
            [position[0] + 1, position[1]],
            [position[0], position[1] - 1],
            [position[0], position[1] + 1],
        ] {
            if next[0] >= 0
                && next[1] >= 0
                && next[0] < width
                && next[1] < height
                && !blocked.contains(&next)
                && visited.insert(next)
            {
                frontier.push_back(next);
            }
        }
    }
    false
}

#[path = "tests/campaign_progression.rs"]
mod campaign_progression;
#[path = "tests/escalation_and_variants.rs"]
mod escalation_and_variants;
#[path = "tests/mission_materialization.rs"]
mod mission_materialization;
