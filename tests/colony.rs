//! Integration tests for the colony module.

use mirexis::colony::*;
#[path = "support/test_prelude.rs"]
pub mod test_prelude;

use crate::test_prelude::*;

#[path = "colony/tests.rs"]
mod tests;
