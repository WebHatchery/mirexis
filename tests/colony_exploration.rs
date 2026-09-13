//! Integration tests for the colony_exploration module.

use mirexis::colony_exploration::*;
#[path = "support/test_prelude.rs"]
mod test_prelude;

use crate::test_prelude::*;

#[path = "colony_exploration/tests.rs"]
mod tests;

#[path = "colony_exploration/dialogue.rs"]
mod dialogue;
