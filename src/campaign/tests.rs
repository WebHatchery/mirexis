use super::*;
use crate::state::CharacterConsequence;

fn consequence(id: &str, name: &str) -> CharacterConsequence {
    CharacterConsequence {
        id: id.to_owned(),
        name: name.to_owned(),
    }
}

mod evolution;
mod recruitment;

mod test_part_1;
