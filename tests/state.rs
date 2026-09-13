//! Integration tests for the state module.

use mirexis::state::*;
#[path = "support/test_prelude.rs"]
pub mod test_prelude;

use crate::test_prelude::*;

#[path = "state/tests.rs"]
mod tests;
