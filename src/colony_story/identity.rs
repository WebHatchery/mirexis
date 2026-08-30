//! Identity-building field notes that react to the colony's civic choices.

use super::{ColonyBeat, ColonyStoryState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct IdentityBuildingState {
    pub(crate) damaged: bool,
    pub(crate) powered: bool,
}

#[derive(Debug, Clone, Copy)]
struct IdentityCivicArc {
    damage: ColonyBeat,
    repair: ColonyBeat,
    power_failure: ColonyBeat,
    power_restored: ColonyBeat,
}

pub(crate) fn identity_npc(path_id: &str) -> Option<&'static str> {
    match path_id {
        "human_redoubt" => Some("mara_venn"),
        "living_commonwealth" => Some("nadi_vale"),
        "open_threshold" => Some("sol_cairn"),
        _ => None,
    }
}

pub(crate) fn identity_beat(
    path_id: &str,
    character_id: &str,
    campaign_complete: bool,
) -> Option<ColonyBeat> {
    if identity_npc(path_id) != Some(character_id) {
        return None;
    }
    let (id, title, text) = match (path_id, campaign_complete) {
        ("human_redoubt", false) => (
            "identity_redoubt_arsenal",
            "A WALL WITH A NAME",
            "The Arsenal is not a monument to fear. It is a promise that the people behind this wall get to decide what kind of emergency comes next.",
        ),
        ("human_redoubt", true) => (
            "identity_redoubt_ending",
            "THE WALL REMEMBERS",
            "The Arsenal did not make us safe. It made our fear answer to the people who live behind it. If the wall stands after us, let it remember who chose its shape.",
        ),
        ("living_commonwealth", false) => (
            "identity_choir_garden",
            "THE GARDEN LISTENS",
            "The Choir Garden has started answering before we ask. I want the colony to learn its language without letting the first loud voice call that consent.",
        ),
        ("living_commonwealth", true) => (
            "identity_choir_ending",
            "THE GARDEN ANSWERS",
            "The Garden answers in more than one voice now. We did not become Brood, and we did not stay untouched; we became a colony that can disagree without mistaking difference for danger.",
        ),
        ("open_threshold", false) => (
            "identity_threshold_spire",
            "A DOOR NEEDS A GRID",
            "The Threshold Spire is a door only while we can keep its current alive. If the lights fail, we do not lose a building—we lose the question we built it to ask.",
        ),
        ("open_threshold", true) => (
            "identity_threshold_ending",
            "THE DOOR HAS WITNESSES",
            "The Spire is open, but the door is not an escape from Mirexis. It is an invitation with witnesses: we will meet what answers as a colony, or not at all.",
        ),
        _ => return None,
    };
    Some(ColonyBeat { id, title, text })
}

pub(crate) fn identity_arc_beat(
    path_id: &str,
    character_id: &str,
    campaign_complete: bool,
    post_campaign_operations_completed: u32,
    identity_stewardship_completed: u32,
    story: &ColonyStoryState,
    identity_building_state: Option<IdentityBuildingState>,
) -> Option<ColonyBeat> {
    let identity = identity_beat(path_id, character_id, campaign_complete);
    let identity = identity.filter(|beat| !story.has_heard(beat.id));
    if !campaign_complete {
        return identity.or_else(|| {
            identity_condition_beat(path_id, character_id, identity_building_state, story)
        });
    }
    identity
        .or_else(|| identity_condition_beat(path_id, character_id, identity_building_state, story))
        .or_else(|| post_ending_beat(path_id, character_id, story))
        .or_else(|| stewardship_beat(path_id, character_id, identity_stewardship_completed, story))
        .or_else(|| {
            post_campaign_operation_beat(
                path_id,
                character_id,
                post_campaign_operations_completed,
                story,
            )
        })
}

fn identity_condition_beat(
    path_id: &str,
    character_id: &str,
    state: Option<IdentityBuildingState>,
    story: &ColonyStoryState,
) -> Option<ColonyBeat> {
    if identity_npc(path_id) != Some(character_id) {
        return None;
    }
    let state = state?;
    let arc = identity_civic_arc(path_id)?;
    let candidate = if state.damaged {
        arc.damage
    } else if !state.powered {
        arc.power_failure
    } else if story.has_heard(arc.damage.id) && !story.has_heard(arc.repair.id) {
        arc.repair
    } else if story.has_heard(arc.power_failure.id) && !story.has_heard(arc.power_restored.id) {
        arc.power_restored
    } else {
        return None;
    };
    (!story.has_heard(candidate.id)).then_some(candidate)
}

fn identity_civic_arc(path_id: &str) -> Option<&'static IdentityCivicArc> {
    match path_id {
        "human_redoubt" => Some(&REDOUBT_CIVIC_ARC),
        "living_commonwealth" => Some(&COMMONWEALTH_CIVIC_ARC),
        "open_threshold" => Some(&THRESHOLD_CIVIC_ARC),
        _ => None,
    }
}

const REDOUBT_CIVIC_ARC: IdentityCivicArc = IdentityCivicArc {
    damage: ColonyBeat {
        id: "civic_redoubt_damage",
        title: "THE WALL TAKES A HIT",
        text: "The Arsenal took the hit the colony could not afford to call abstract. Its broken face is now a civic question: who gets protected first when the wall needs hands?",
    },
    repair: ColonyBeat {
        id: "civic_redoubt_repair",
        title: "REPAIR IS A VOTE",
        text: "We repaired the Arsenal with materials that could have made another room. The choice is not clean, but it is visible: defence means something different when the people can argue over its cost.",
    },
    power_failure: ColonyBeat {
        id: "civic_redoubt_power_failure",
        title: "A DARK WALL IS STILL A WALL",
        text: "The Arsenal can keep its shape in the dark, but its systems cannot keep the promise alone. Sol can restore the grid; Mara wants the colony to decide what the power is for before it returns.",
    },
    power_restored: ColonyBeat {
        id: "civic_redoubt_power_restored",
        title: "THE CIRCUIT COMES HOME",
        text: "The Arsenal's lights returned and the wall did not become a command. We restored a tool, then asked the people behind it to remain responsible for the boundary.",
    },
};

const COMMONWEALTH_CIVIC_ARC: IdentityCivicArc = IdentityCivicArc {
    damage: ColonyBeat {
        id: "civic_commonwealth_damage",
        title: "THE GARDEN CAN BLEED",
        text: "The Choir Garden is damaged, and the roots do not turn that wound into a lesson for us. We repair a living refuge by asking what it needs, not by making its pain useful.",
    },
    repair: ColonyBeat {
        id: "civic_commonwealth_repair",
        title: "CARE LEAVES A TRACE",
        text: "The Garden is growing around the repaired beds. Nadi records the scar beside the new shoots so the Commonwealth remembers care as work, not as a story about being untouched.",
    },
    power_failure: ColonyBeat {
        id: "civic_commonwealth_power_failure",
        title: "WHEN THE CHORUS GOES QUIET",
        text: "The Garden has gone quiet because the grid cannot carry its answer. We can restore the current, but no one gets to mistake silence for consent.",
    },
    power_restored: ColonyBeat {
        id: "civic_commonwealth_power_restored",
        title: "THE CHORUS RETURNS DIFFERENT",
        text: "Power returned and the Garden answered in a changed register. That is not a malfunction to erase; it is a reminder that a living refuge keeps a history of what we do to it.",
    },
};

const THRESHOLD_CIVIC_ARC: IdentityCivicArc = IdentityCivicArc {
    damage: ColonyBeat {
        id: "civic_threshold_damage",
        title: "THE DOOR BENDS",
        text: "The Threshold Spire is damaged, and every crack points both ways. Before we call it a cost of opening the route, we need to ask who bears the danger when the door cannot close.",
    },
    repair: ColonyBeat {
        id: "civic_threshold_repair",
        title: "A DOOR REPAIRED IN PUBLIC",
        text: "The Spire is repaired in the open, with every hand allowed to see what the route demands. A return system stays trustworthy only when its failures are part of the record.",
    },
    power_failure: ColonyBeat {
        id: "civic_threshold_power_failure",
        title: "THE DOOR HAS NO CURRENT",
        text: "The Spire's gate is dark. An open threshold without power is only a promise waiting to become a trap; the colony must restore the line before it invites anyone through.",
    },
    power_restored: ColonyBeat {
        id: "civic_threshold_power_restored",
        title: "THE WAY OPENS WITH A MEMORY",
        text: "The Spire is lit again, but the route remembers the blackout. We will guide the next return with that memory visible, not hide the failure behind a brighter switch.",
    },
};

fn stewardship_beat(
    path_id: &str,
    character_id: &str,
    completed_actions: u32,
    story: &ColonyStoryState,
) -> Option<ColonyBeat> {
    if completed_actions == 0 || identity_npc(path_id) != Some(character_id) {
        return None;
    }
    let (id, title, text) = match path_id {
        "human_redoubt" => (
            "stewardship_redoubt_mara",
            "THE WALL NEEDS HANDS",
            "The Arsenal is not a threat that happens to belong to us. Every watchline we fortify is a choice to keep the colony's defence answerable to the people who pay for it.",
        ),
        "living_commonwealth" => (
            "stewardship_commonwealth_nadi",
            "A GARDEN IS A VERB",
            "The Garden answered because someone made room for the answer. We will keep tending it, not as a voice that speaks for everyone, but as a place where more than one voice can remain alive.",
        ),
        "open_threshold" => (
            "stewardship_threshold_sol",
            "RETURN IS A SYSTEM",
            "A route is not open because it can be crossed once. The Spire needs people to guide the return, record what came through, and keep the door from becoming somebody else's command.",
        ),
        _ => return None,
    };
    (!story.has_heard(id)).then_some(ColonyBeat { id, title, text })
}

fn post_ending_beat(
    path_id: &str,
    character_id: &str,
    story: &ColonyStoryState,
) -> Option<ColonyBeat> {
    let ending = identity_beat(path_id, character_id, true)?;
    if !story.has_heard(ending.id) {
        return None;
    }
    let (id, title, text) = match path_id {
        "human_redoubt" if character_id == "mara_venn" => (
            "post_ending_redoubt_mara",
            "THE GATE IS A CHOICE",
            "Mara has started the Arsenal's first open-gate drill. The point is not to make every visitor safe; it is to make sure safety remains a decision the people behind the wall can see and change.",
        ),
        "living_commonwealth" if character_id == "nadi_vale" => (
            "post_ending_commonwealth_nadi",
            "ROOM TO DISAGREE",
            "Nadi leaves one bed in the Garden unclaimed and one path through it unpruned. A living refuge is not finished when it grows; it is finished only when it can make room for a voice that says no.",
        ),
        "open_threshold" if character_id == "sol_cairn" => (
            "post_ending_threshold_sol",
            "A RETURN CURRENT",
            "Sol has marked the Threshold's first route with a light that points both ways. Opening a door is easy compared with proving that the people who cross it still have a way home.",
        ),
        _ => return None,
    };
    (!story.has_heard(id)).then_some(ColonyBeat { id, title, text })
}

fn post_campaign_operation_beat(
    path_id: &str,
    character_id: &str,
    completed_operations: u32,
    story: &ColonyStoryState,
) -> Option<ColonyBeat> {
    if completed_operations == 0 || identity_npc(path_id) != Some(character_id) {
        return None;
    }
    let post_ending_id = match path_id {
        "human_redoubt" => "post_ending_redoubt_mara",
        "living_commonwealth" => "post_ending_commonwealth_nadi",
        "open_threshold" => "post_ending_threshold_sol",
        _ => return None,
    };
    if !story.has_heard(post_ending_id) {
        return None;
    }
    let (id, title, text) = match path_id {
        "human_redoubt" => (
            "epilogue_operation_redoubt_mara",
            "THE WALL HAS WORK LEFT",
            "The old fire is quiet, but the Arsenal is not finished. Mara keeps the watchline open as a place where defence can be repaired, questioned, and handed to the next pair of hands.",
        ),
        "living_commonwealth" => (
            "epilogue_operation_commonwealth_nadi",
            "A CHORUS CAN ANSWER AGAIN",
            "The new roots did not ask Nadi to speak for them. They asked her to return, listen, and leave room for the next voice to make the garden larger without making it one thing.",
        ),
        "open_threshold" => (
            "epilogue_operation_threshold_sol",
            "THE WAY BACK IS WORK",
            "Sol marks another traveller's route beside the first. The Threshold is not a door the colony owns; it is a promise that has to be maintained by everyone who crosses it.",
        ),
        _ => return None,
    };
    (!story.has_heard(id)).then_some(ColonyBeat { id, title, text })
}
