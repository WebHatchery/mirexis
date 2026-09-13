//! Integration tests for the campaign module.

use mirexis::campaign::*;
#[path = "support/test_prelude.rs"]
pub mod test_prelude;

use crate::test_prelude::*;

#[path = "campaign/tests.rs"]
mod tests;
