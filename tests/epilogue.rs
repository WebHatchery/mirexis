//! Integration tests for the epilogue module.

use mirexis::epilogue::*;
#[path = "support/test_prelude.rs"]
pub mod test_prelude;

use crate::test_prelude::*;

#[path = "epilogue/tests.rs"]
mod tests;
