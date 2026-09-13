//! Integration tests for the audio module.

use mirexis::audio::*;
#[path = "support/test_prelude.rs"]
pub mod test_prelude;

use crate::test_prelude::*;

#[path = "audio/tests.rs"]
mod tests;
