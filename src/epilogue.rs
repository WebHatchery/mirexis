//! Derived endgame acknowledgement for the colony that survived Mirexis.

use crate::campaign::{Availability, CampaignState};
use crate::colony::BuildingKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct EpilogueDossier {
    pub(crate) institution: String,
    pub(crate) institution_status: &'static str,
    people: String,
    trusted_bonds: usize,
    bond_detail: String,
    scars: String,
    evolutions: String,
    character_voice: String,
}

impl EpilogueDossier {
    pub(crate) fn lines(&self) -> [String; 6] {
        [
            format!(
                "CIVIC // {} // {}",
                self.institution.to_uppercase(),
                self.institution_status
            ),
            format!("PEOPLE // {}", self.people),
            format!(
                "BONDS // {} TRUSTED+ // {}",
                self.trusted_bonds, self.bond_detail
            ),
            format!("SCARS // {}", self.scars),
            format!("EVOLUTION // {}", self.evolutions),
            format!("VOICE // {}", self.character_voice),
        ]
    }

    pub(crate) fn debrief_line(&self) -> String {
        format!(
            "COLONY LEGACY // {} {} // {} // {} TRUSTED+ BONDS // {} // {}",
            self.institution.to_uppercase(),
            self.institution_status,
            self.people,
            self.trusted_bonds,
            self.scars,
            self.evolutions
        )
    }
}

pub(crate) fn derive(campaign: &CampaignState) -> Option<EpilogueDossier> {
    if !campaign.strategy.campaign_complete {
        return None;
    }
    let identity = campaign
        .colony
        .buildings
        .iter()
        .find(|building| building.kind.is_identity());
    let kind = identity
        .map(|building| building.kind)
        .or_else(|| BuildingKind::identity_for_path(&campaign.strategy.mirexis_path_id));
    let institution = kind.map_or_else(
        || "Unrecorded Identity".to_owned(),
        |kind| kind.name().to_owned(),
    );
    let institution_status = identity.map_or("NOT BUILT", |building| {
        if building.damaged {
            "DAMAGED"
        } else if campaign.colony.building_is_powered(&building.id) {
            "ONLINE"
        } else {
            "OFFLINE"
        }
    });

    let ready = campaign
        .roster
        .iter()
        .filter(|character| character.availability == Availability::Ready)
        .count();
    let recovering = campaign.roster.len().saturating_sub(ready);
    let people = format!(
        "{} COLONISTS // {} READY / {} RECOVERING",
        campaign.roster.len(),
        ready,
        recovering
    );

    let mut trusted = campaign
        .relationships
        .iter()
        .filter(|relationship| relationship.bond >= 3)
        .collect::<Vec<_>>();
    trusted.sort_by(|left, right| {
        right
            .bond
            .cmp(&left.bond)
            .then_with(|| left.first_id.cmp(&right.first_id))
            .then_with(|| left.second_id.cmp(&right.second_id))
    });
    let bond_detail = trusted.first().map_or_else(
        || "NO TRUSTED PAIR RECORDED".to_owned(),
        |relationship| {
            format!(
                "{} + {}",
                character_name(campaign, &relationship.first_id),
                character_name(campaign, &relationship.second_id)
            )
        },
    );

    let scarred = campaign
        .roster
        .iter()
        .filter(|character| !character.traumas.is_empty())
        .map(|character| character.name.clone())
        .collect::<Vec<_>>();
    let scar_count = campaign
        .roster
        .iter()
        .map(|character| character.traumas.len())
        .sum::<usize>();
    let scars = if scar_count == 0 {
        "NONE RECORDED".to_owned()
    } else {
        format!("{} CARRIED // {}", scar_count, scarred.join(", "))
    };

    let evolved = campaign
        .roster
        .iter()
        .filter(|character| !character.mutation_evolution_id.is_empty())
        .map(|character| character.name.clone())
        .collect::<Vec<_>>();
    let evolutions = if evolved.is_empty() {
        "NONE // THE OLD BODY HOLDS".to_owned()
    } else {
        format!("{} COLONISTS // {}", evolved.len(), evolved.join(", "))
    };
    let character_voice = character_voice(campaign);

    Some(EpilogueDossier {
        institution,
        institution_status,
        people,
        trusted_bonds: trusted.len(),
        bond_detail,
        scars,
        evolutions,
        character_voice,
    })
}

fn character_voice(campaign: &CampaignState) -> String {
    let (character_id, ready_line, recovering_line) =
        match campaign.strategy.mirexis_path_id.as_str() {
            "human_redoubt" => (
                "mara_venn",
                "THE ARSENAL OPENS FROM THE INSIDE.",
                "THE ARSENAL KEEPS THE INJURED WITHIN ITS LIGHT.",
            ),
            "living_commonwealth" => (
                "nadi_vale",
                "WE DID NOT BECOME ONE BODY; WE MADE ROOM FOR DIFFERENCE.",
                "THE GARDEN HAS ROOM FOR THE BODY THAT NEEDS TIME.",
            ),
            "open_threshold" => (
                "sol_cairn",
                "EVERY OPEN ROUTE NEEDS A WAY HOME.",
                "THE THRESHOLD WAITS; RETURN IS PART OF THE JOURNEY.",
            ),
            _ => return "NO PATH CONTACT // THE COLONY'S ACCOUNT REMAINS OPEN.".to_owned(),
        };
    let line = campaign
        .roster
        .iter()
        .find(|character| character.id == character_id)
        .map_or(recovering_line, |character| {
            if character.availability == Availability::Ready {
                ready_line
            } else {
                recovering_line
            }
        });
    format!("{} // {}", character_name(campaign, character_id), line)
}

fn character_name(campaign: &CampaignState, character_id: &str) -> String {
    campaign
        .roster
        .iter()
        .find(|character| character.id == character_id)
        .map_or_else(
            || character_id.to_owned(),
            |character| character.name.clone(),
        )
}

#[cfg(test)]
mod tests;
