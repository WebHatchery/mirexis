//! Integration tests for the persistence module.

use mirexis::persistence::*;
#[path = "support/test_prelude.rs"]
pub mod test_prelude;

use crate::test_prelude::*;

#[path = "persistence/tests.rs"]
mod tests;
