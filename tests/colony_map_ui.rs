//! Integration tests for the colony_map_ui module.

use mirexis::colony_map_ui::*;
#[path = "support/test_prelude.rs"]
pub mod test_prelude;

use crate::test_prelude::*;

#[path = "colony_map_ui/tests.rs"]
mod tests;

#[path = "colony_map_ui/controls.rs"]
mod controls;
