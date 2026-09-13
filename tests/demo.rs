//! Integration tests for the demo module.

use mirexis::demo::*;
#[path = "support/test_prelude.rs"]
pub mod test_prelude;

use crate::test_prelude::*;

#[path = "demo/tests.rs"]
mod tests;
