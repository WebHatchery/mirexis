//! Integration tests for the phase_replay module.

use mirexis::phase_replay::*;
#[path = "support/test_prelude.rs"]
pub mod test_prelude;

use crate::test_prelude::*;

#[path = "phase_replay/tests.rs"]
mod tests;
