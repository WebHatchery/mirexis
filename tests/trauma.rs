//! Integration tests for the trauma module.

#[path = "support/test_prelude.rs"]
mod test_prelude;

use crate::test_prelude::*;
use mirexis::trauma::*;

#[path = "trauma/tests.rs"]
mod tests;
