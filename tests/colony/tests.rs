//! Colony cases are grouped by construction, production, and spatial rules;
//! the higher count preserves distinct facility and boundary regressions.

use super::*;

#[path = "tests/construction_and_facilities.rs"]
mod construction_and_facilities;
#[path = "tests/layout_and_legacy_rules.rs"]
mod layout_and_legacy_rules;
#[path = "tests/resource_and_project_rules.rs"]
mod resource_and_project_rules;
