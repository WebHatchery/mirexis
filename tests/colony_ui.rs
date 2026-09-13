//! Integration tests for the colony_ui module.

use mirexis::colony_ui::*;
#[path = "support/test_prelude.rs"]
mod test_prelude;

use crate::test_prelude::*;

#[path = "colony_ui/tests.rs"]
mod tests;

#[path = "colony_ui/commons.rs"]
mod commons;
#[path = "colony_ui/decision_affordance.rs"]
mod decision_affordance;
#[path = "colony_ui/event_affordance.rs"]
mod event_affordance;
#[path = "colony_ui/medical.rs"]
mod medical;
#[path = "colony_ui/recruitment.rs"]
mod recruitment;
#[path = "colony_ui/relay.rs"]
mod relay;
#[path = "colony_ui/research.rs"]
mod research;
#[path = "colony_ui/research_affordance.rs"]
mod research_affordance;
#[path = "colony_ui/scene.rs"]
mod scene;
#[path = "colony_ui/upgrades.rs"]
mod upgrades;
