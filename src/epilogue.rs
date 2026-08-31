//! Derived endgame acknowledgement for the colony that survived Mirexis.

use crate::campaign::{Availability, CampaignState};
use crate::colony::BuildingKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct EpilogueDossier {
    pub(crate) institution: String,
    pub(crate) institution_status: &'static str,
    people: String,
    ready: usize,
    recovering: usize,
    trusted_bonds: usize,
    bond_detail: String,
    scars: String,
    scar_count: usize,
    evolutions: String,
    evolved_count: usize,
    character_voice: String,
    navigator_voice: String,
    engine_relationship: &'static str,
    engine_response: &'static str,
    faction_pressure: String,
    mercy_count: usize,
    post_campaign_operations_completed: u32,
    identity_stewardship_completed: u32,
}

impl EpilogueDossier {
    pub(crate) fn lines(&self) -> [String; 8] {
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
            format!("NAVIGATOR // {}", self.navigator_voice),
            format!(
                "ENGINE // {} / {} // {} // MERCY {} // EPILOGUE WORK {} // CIVIC WORK {}",
                self.engine_relationship,
                self.engine_response,
                self.faction_pressure,
                self.mercy_count,
                self.post_campaign_operations_completed,
                self.identity_stewardship_completed
            ),
        ]
    }

    pub(crate) fn debrief_summary_lines(&self) -> [String; 3] {
        [
            format!(
                "COLONY LEGACY // {} {} // {} COLONISTS",
                self.institution.to_uppercase(),
                self.institution_status,
                self.people.split_whitespace().next().unwrap_or("0")
            ),
            format!(
                "READY {} // RECOVERING {} // TRUSTED+ {} // SCARS {} // EVOLVED {}",
                self.ready,
                self.recovering,
                self.trusted_bonds,
                self.scar_count,
                self.evolved_count
            ),
            format!(
                "ENGINE {} / {} // MERCY {} // EPILOGUE {} // CIVIC {}",
                self.engine_relationship,
                self.engine_response,
                self.mercy_count,
                self.post_campaign_operations_completed,
                self.identity_stewardship_completed
            ),
        ]
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
    let navigator_voice = navigator_voice(campaign);
    let engine_relationship = engine_relationship(&campaign.strategy.mirexis_path_id);
    let engine_response = engine_response(&campaign.strategy.escalation_response_id);
    let faction_pressure = faction_pressure(campaign);
    let mercy_count = campaign
        .strategy
        .character_events
        .iter()
        .filter(|event| event.resolved && event.attention_change < 0)
        .count();

    Some(EpilogueDossier {
        institution,
        institution_status,
        people,
        ready,
        recovering,
        trusted_bonds: trusted.len(),
        bond_detail,
        scars,
        scar_count,
        evolutions,
        evolved_count: evolved.len(),
        character_voice,
        navigator_voice,
        engine_relationship,
        engine_response,
        faction_pressure,
        mercy_count,
        post_campaign_operations_completed: campaign.strategy.post_campaign_operations_completed,
        identity_stewardship_completed: campaign.identity_stewardship_completed,
    })
}

fn navigator_voice(campaign: &CampaignState) -> String {
    let line = match campaign.strategy.mirexis_path_id.as_str() {
        "human_redoubt" => "THE SIGNAL FALLS QUIET BEHIND THE WALL.",
        "living_commonwealth" => "THE MANY VOICES KEEP THEIR OWN NAMES.",
        "open_threshold" => "EVERY ROUTE CARRIES A WAY HOME.",
        _ => "THE MAP REMAINS OPEN TO THE COLONY.",
    };
    format!("{} // {}", character_name(campaign, "kira_voss"), line)
}

fn engine_relationship(path_id: &str) -> &'static str {
    match path_id {
        "human_redoubt" => "HUMAN BOUNDARY",
        "living_commonwealth" => "LIVING ACCORD",
        "open_threshold" => "OPEN RECIPROCITY",
        _ => "UNRECORDED RELATION",
    }
}

fn engine_response(response_id: &str) -> &'static str {
    match response_id {
        "bastion_beacon" => "ARMOURED",
        "living_decoy" => "SHELTERED",
        "weaponized_lattice" => "DIRECTED",
        _ => "UNANSWERED",
    }
}

fn faction_pressure(campaign: &CampaignState) -> String {
    campaign
        .strategy
        .factions
        .iter()
        .max_by_key(|faction| (faction.attention, faction.id.clone()))
        .map_or_else(
            || "NO FACTION PRESSURE".to_owned(),
            |faction| format!("{} {}", faction.name.to_uppercase(), faction.attention),
        )
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
    let voice = format!("{} // {}", character_name(campaign, character_id), line);
    if campaign
        .roster
        .iter()
        .any(|character| character.id == "sedge")
    {
        format!("{} // SEDGE: THE OLD BODY WAS HERE FIRST.", voice)
    } else {
        voice
    }
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
