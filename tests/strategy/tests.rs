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

#[path = "tests/suite_a.rs"]
mod suite_a;
#[path = "tests/suite_b.rs"]
mod suite_b;
#[path = "tests/suite_c.rs"]
mod suite_c;
