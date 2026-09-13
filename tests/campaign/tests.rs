//! Campaign regression cases stay separate because each covers a distinct
//! unlock, resource, or persistence-facing rule.

use super::*;
use mirexis::state::CharacterConsequence;

fn consequence(id: &str, name: &str) -> CharacterConsequence {
    CharacterConsequence {
        id: id.to_owned(),
        name: name.to_owned(),
    }
}

mod commons;
mod evolution;
mod facility_upgrade;
mod identity;
mod recruitment;
mod relay;
mod research;
mod salvage;

mod authored_campaign_rules;
