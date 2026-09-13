//! Integration tests for the tactical_map_ui module.

use mirexis::tactical_map_ui::*;
#[path = "support/test_prelude.rs"]
mod test_prelude;

use crate::test_prelude::*;

#[path = "tactical_map_ui/tests.rs"]
mod tests;

#[path = "tactical_map_ui/targeting_card.rs"]
mod targeting_card;
