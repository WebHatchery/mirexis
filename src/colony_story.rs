//! Persistent character-led colony beats for the first campaign horizon.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ColonyStoryState {
    #[serde(default)]
    heard_beats: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ColonyBeat {
    pub(crate) id: &'static str,
    pub(crate) title: &'static str,
    pub(crate) text: &'static str,
}

impl ColonyStoryState {
    pub(crate) fn has_heard(&self, beat_id: &str) -> bool {
        self.heard_beats.iter().any(|heard| heard == beat_id)
    }

    pub(crate) fn acknowledge(&mut self, beat_id: &str) {
        if !self.has_heard(beat_id) {
            self.heard_beats.push(beat_id.to_owned());
        }
    }
}

pub(crate) fn current_beat(
    character_id: &str,
    operations_completed: u32,
    first_outcome_won: Option<bool>,
    second_outcome_won: Option<bool>,
) -> Option<ColonyBeat> {
    let outcome = match operations_completed {
        0 => None,
        1 => first_outcome_won,
        _ => second_outcome_won,
    };
    match (character_id, operations_completed, outcome) {
        ("mara_venn", 0, _) => Some(ColonyBeat {
            id: "mara_arrival",
            title: "THE WEST REFUGE",
            text: "Kira. The west refuge is failing and the mire is moving around it. Ilya and Sol are ready. Tap OPERATIONS, read GLASSROOT, and choose who we risk.",
        }),
        ("mara_venn", 1, Some(true)) => Some(ColonyBeat {
            id: "mara_first_victory",
            title: "A LIGHT LEFT ON",
            text: "The refuge lights are back on. Good. Spend the recovery where it changes the next fight; the Directorate clock did not stop for us.",
        }),
        ("mara_venn", 1, Some(false)) => Some(ColonyBeat {
            id: "mara_first_defeat",
            title: "NOT THE COLONY",
            text: "We lost the refuge, not the colony. Treat the wounded or strengthen the next squad, then we move before the Directorate closes the road.",
        }),
        ("mara_venn", 1, _) => Some(ColonyBeat {
            id: "mara_first_aftermath",
            title: "WHAT THE FIELD LEFT",
            text: "The field took something from us and left work behind. Spend the recovery where it changes the next fight; the road will still be there.",
        }),
        ("mara_venn", _, Some(true)) => Some(ColonyBeat {
            id: "mara_second_victory",
            title: "TWO ROADS HOLD",
            text: "Two field routes are holding. That is enough for the Directorate to start measuring them, and enough for us to choose where the next stand begins.",
        }),
        ("mara_venn", _, Some(false)) => Some(ColonyBeat {
            id: "mara_second_defeat",
            title: "THE LINE BENDS",
            text: "The second route broke, but it did not take the refuge with it. We repair what we can, keep the people together, and decide what must never be abandoned.",
        }),
        ("mara_venn", _, _) => Some(ColonyBeat {
            id: "mara_second_aftermath",
            title: "THE ROAD AHEAD",
            text: "The refuge has changed and the road ahead is still unwritten. Keep the people together while we decide where the next stand begins.",
        }),
        ("ilya_reed", 0, _) => Some(ColonyBeat {
            id: "ilya_arrival",
            title: "A CLINIC WITHOUT TEXTBOOKS",
            text: "The clinic is stocked for ordinary injuries and Mirexis has stopped giving us ordinary injuries. Bring people back breathing; I will learn the rest from what their bodies tell me.",
        }),
        ("ilya_reed", 1, Some(true)) => Some(ColonyBeat {
            id: "ilya_first_victory",
            title: "ACCOUNTED FOR",
            text: "Everyone who returned is accounted for. Mara calls that readiness. I call it a chance to avoid spending people like spare parts.",
        }),
        ("ilya_reed", 1, Some(false)) => Some(ColonyBeat {
            id: "ilya_first_defeat",
            title: "FAILURE IS A CONDITION",
            text: "Failure is a condition, not a verdict. Tap REQUEST TREATMENT if someone is recovering; I can get a viable squad back into the field.",
        }),
        ("ilya_reed", 1, _) => Some(ColonyBeat {
            id: "ilya_first_aftermath",
            title: "ACCOUNTED FOR",
            text: "Everyone who returned is accounted for. Before we call the result, let the clinic finish its work and tell us what the field cost.",
        }),
        ("ilya_reed", _, Some(true)) => Some(ColonyBeat {
            id: "ilya_second_victory",
            title: "THE COST OF RETURNING",
            text: "A victory still leaves a body to mend. Let the scars tell us what the field is asking, before the colony turns survival into a habit of looking away.",
        }),
        ("ilya_reed", _, Some(false)) => Some(ColonyBeat {
            id: "ilya_second_defeat",
            title: "NO ONE IS DISPOSABLE",
            text: "We can recover from this. The first thing the colony must not recover from is believing that a lost operation makes a lost person acceptable.",
        }),
        ("ilya_reed", _, _) => Some(ColonyBeat {
            id: "ilya_second_aftermath",
            title: "THE CLINIC KEEPS SCORE",
            text: "Two operations leave a pattern in every body that returns. I will read it carefully before the colony turns survival into a habit of looking away.",
        }),
        ("sol_cairn", 0, _) => Some(ColonyBeat {
            id: "sol_arrival",
            title: "BROKEN MACHINES, USEFUL CLUES",
            text: "Every dead system leaves a pattern behind. I can keep the power plant breathing, but I need the field teams to bring me the pieces that still remember how to work.",
        }),
        ("sol_cairn", 1, Some(true)) => Some(ColonyBeat {
            id: "sol_first_victory",
            title: "THE GRID REMEMBERS",
            text: "The west line is drawing power again. It is not a repair so much as a conversation with a machine that has been waiting for someone to ask the right question.",
        }),
        ("sol_cairn", 1, Some(false)) => Some(ColonyBeat {
            id: "sol_first_defeat",
            title: "SALVAGE THE LESSON",
            text: "The field hardware is damaged, not useless. Give me one working route and I can turn the wreckage into enough time for the next squad.",
        }),
        ("sol_cairn", 1, _) => Some(ColonyBeat {
            id: "sol_first_aftermath",
            title: "SALVAGE THE SIGNAL",
            text: "The field hardware is damaged, not useless. Bring me what survived and I will tell you what the route was trying to become.",
        }),
        ("sol_cairn", _, Some(true)) => Some(ColonyBeat {
            id: "sol_second_victory",
            title: "THE NEXT BROKEN MACHINE",
            text: "Two field routes are holding, which means the Directorate has started measuring them. The assault clock is our next broken machine.",
        }),
        ("sol_cairn", _, Some(false)) => Some(ColonyBeat {
            id: "sol_second_defeat",
            title: "KEEP THE LIGHTS ON",
            text: "The routes failed before the grid did. That distinction matters. Give me the damaged parts and I will make the refuge harder to surprise next time.",
        }),
        ("sol_cairn", _, _) => Some(ColonyBeat {
            id: "sol_second_aftermath",
            title: "THE CLOCK IS MOVING",
            text: "The assault clock is moving even while I keep the lights on. Give me one clear route and I can turn the next broken machine into a warning.",
        }),
        ("nadi_vale", 0, _) => Some(ColonyBeat {
            id: "nadi_arrival",
            title: "THE THING THAT ANSWERS",
            text: "My symbiote reacts to the Brood before I can name the threat. I am still deciding whether that makes it a patient, a partner, or a warning with a pulse.",
        }),
        ("nadi_vale", 1, Some(true)) => Some(ColonyBeat {
            id: "nadi_first_victory",
            title: "A QUIETER SIGNAL",
            text: "The sample went quiet when the refuge came back online. Quiet is not the same as safe, but it gives us a moment to listen without mistaking fear for evidence.",
        }),
        ("nadi_vale", 1, Some(false)) => Some(ColonyBeat {
            id: "nadi_first_defeat",
            title: "THE BODY KEEPS SCORE",
            text: "The Brood learned something from the failed route, and so did we. I can help the wounded recover, but we should not pretend adaptation has no price.",
        }),
        ("nadi_vale", 1, _) => Some(ColonyBeat {
            id: "nadi_first_aftermath",
            title: "A SIGNAL UNRESOLVED",
            text: "The signal changed when we came back. I need one quiet hour before I call it a warning, a summons, or something that wants to be understood.",
        }),
        ("nadi_vale", _, Some(true)) => Some(ColonyBeat {
            id: "nadi_second_victory",
            title: "SOMETHING BELOW HEARS US",
            text: "The signal is no longer only coming from the field. Something below Mirexis is answering the colony, and I do not think it cares which faction gets the credit.",
        }),
        ("nadi_vale", _, Some(false)) => Some(ColonyBeat {
            id: "nadi_second_defeat",
            title: "LISTEN BEFORE CUTTING",
            text: "The signal changed when the route failed. Before we cut it away, we need to know whether it is calling the Brood, warning us about them, or trying to call us home.",
        }),
        ("nadi_vale", _, _) => Some(ColonyBeat {
            id: "nadi_second_aftermath",
            title: "THE ANSWER IS NOT READY",
            text: "The signal is stronger, but the answer is not ready. We need the colony to listen together before one frightened voice decides what Mirexis means.",
        }),
        ("veya_orn", 0, _) => Some(ColonyBeat {
            id: "veya_arrival",
            title: "THE PERSON BEHIND THE CIPHER",
            text: "The Waystation received a Directorate exile before it received a clean signal. Veya says the codes are stolen; Mara says the danger is still theirs. Tap OPERATIONS to review the first terms.",
        }),
        ("veya_orn", 1, Some(true)) => Some(ColonyBeat {
            id: "veya_first_victory",
            title: "A RANGE THAT MOVES",
            text: "Veya came back from the first operation with the target map intact and their story less rehearsed. The colony can use what they know without pretending trust arrived with them.",
        }),
        ("veya_orn", 1, Some(false)) => Some(ColonyBeat {
            id: "veya_first_defeat",
            title: "NO CLEAN ALIBI",
            text: "The failed route gave the Directorate a better line on us, and Veya did not look away from the cost. If they stay, the Waystation has to be a place where blame can be spoken aloud.",
        }),
        ("veya_orn", 1, _) => Some(ColonyBeat {
            id: "veya_first_aftermath",
            title: "THE CIPHER IS NOT A CONFESSION",
            text: "Veya has a cipher, a uniform, and no clean way to separate the two. The colony will decide what the codes are worth before it decides what their bearer is worth.",
        }),
        ("veya_orn", _, _) => Some(ColonyBeat {
            id: "veya_second_aftermath",
            title: "A HOME WITH WITNESSES",
            text: "The Waystation is no longer a stop between Directorate lines. Veya chose to remain where every argument has a witness, and the colony remembers that choice as a person rather than a resource.",
        }),
        _ => None,
    }
}

pub(crate) fn commons_meal_beat(character_id: &str) -> Option<ColonyBeat> {
    let (id, title, text) = match character_id {
        "kira_voss" => (
            "commons_meal_kira",
            "THE TABLE IS A FORMATION",
            "The meal was not a briefing, but everyone left it knowing where the others would stand. The field does not get to decide what holds us together.",
        ),
        "mara_venn" => (
            "commons_meal_mara",
            "A PLACE TO RETURN",
            "The meal gave the squad somewhere to return to before the operation gave us another reason to leave. That is a kind of preparation the Directorate cannot measure.",
        ),
        "ilya_reed" => (
            "commons_meal_ilya",
            "NO ONE EATS ALONE",
            "Three people sharing food is not a medical protocol. It is still evidence that the colony knows the difference between a body and a person.",
        ),
        "sol_cairn" => (
            "commons_meal_sol",
            "A CIRCUIT OF HANDS",
            "The Commons ran on one small circuit and a lot of hands passing bowls. That is how the grid survives too: not because it is strong, but because someone keeps closing the loop.",
        ),
        "nadi_vale" => (
            "commons_meal_nadi",
            "THE QUIET BETWEEN BITES",
            "The symbiote listened while the squad ate. No alarm, no hunger, no signal from below—just a quiet long enough for trust to become something the body could recognise.",
        ),
        "veya_orn" => (
            "commons_meal_veya",
            "WITNESSES AT THE TABLE",
            "Veya expected an interrogation and got a meal with too many witnesses to rewrite afterward. The colony is learning that belonging can be recorded without becoming surveillance.",
        ),
        _ => return None,
    };
    Some(ColonyBeat { id, title, text })
}

pub(crate) fn phase_beat(phase_id: &str, character_id: &str) -> Option<ColonyBeat> {
    let (expected_character, id, title, text) = match phase_id {
        "contact" => (
            "kira_voss",
            "phase_contact_kira",
            "THE ANSWER UNDER THE SIGNAL",
            "The Black Channel answered from below the colony, and the Living Chorus answered from within it. The signal is not a faction's property anymore; it is a question we have to carry together.",
        ),
        "adaptation" => (
            "nadi_vale",
            "phase_adaptation_nadi",
            "THE BODY MAPS THE ROOM",
            "The Gene Lab keeps asking what a body can become, but the answer cannot belong to the instrument alone. Neural Bloom gives us new senses only if the person inside them remains the author.",
        ),
        "escalation" => (
            "mara_venn",
            "phase_escalation_mara",
            "THEY CAN SEE THE LIGHTS",
            "Three powers are converging on the same patch of dark, and now they can see our lights. The colony cannot survive as an invisible camp; it needs a defence that the people behind it can name and direct.",
        ),
        _ => return None,
    };
    (expected_character == character_id).then_some(ColonyBeat { id, title, text })
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

#[cfg(test)]
mod tests;
