//! Integration tests for the strategy module.

use mirexis::strategy::*;
#[path = "support/test_prelude.rs"]
pub mod test_prelude;

use crate::test_prelude::*;

#[path = "strategy/tests.rs"]
mod tests;
