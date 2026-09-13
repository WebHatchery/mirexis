//! Each migration case represents a separate historical save shape and is
//! intentionally retained instead of being collapsed into parameterized smoke tests.

use super::*;
use mirexis::state::GameSession;

mod late_migrations;

mod facility_upgrade;
mod identity_buildings;
mod watchtower;

mod first_hour;

mod save_migrations;
