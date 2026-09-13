//! Authored colony story beat selection.

use super::*;

pub fn current_beat(
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
    match character_id {
        "mara_venn" => mara_venn_beat(operations_completed, outcome),
        "ilya_reed" => ilya_reed_beat(operations_completed, outcome),
        "sol_cairn" => sol_cairn_beat(operations_completed, outcome),
        "nadi_vale" => nadi_vale_beat(operations_completed, outcome),
        "sedge" => sedge_beat(operations_completed, outcome),
        "ninth_voice_apart" => ninth_voice_apart_beat(operations_completed, outcome),
        "veya_orn" => veya_orn_beat(operations_completed, outcome),
        _ => None,
    }
}

fn mara_venn_beat(operations_completed: u32, outcome: Option<bool>) -> Option<ColonyBeat> {
    match (operations_completed, outcome) {
        (0, _) => Some(ColonyBeat {
                            id: "mara_arrival",
                            title: "THE WEST REFUGE",
                            text: "Kira. The west refuge is failing and the mire is moving around it. Ilya and Sol are ready. Tap OPERATIONS, read GLASSROOT, and choose who we risk.",
                        }),
        (1, Some(true)) => Some(ColonyBeat {
                            id: "mara_first_victory",
                            title: "A LIGHT LEFT ON",
                            text: "The refuge lights are back on. Good. Spend the recovery where it changes the next fight; the Directorate clock did not stop for us.",
                        }),
        (1, Some(false)) => Some(ColonyBeat {
                            id: "mara_first_defeat",
                            title: "NOT THE COLONY",
                            text: "We lost the refuge, not the colony. Treat the wounded or strengthen the next squad, then we move before the Directorate closes the road.",
                        }),
        (1, _) => Some(ColonyBeat {
                            id: "mara_first_aftermath",
                            title: "WHAT THE FIELD LEFT",
                            text: "The field took something from us and left work behind. Spend the recovery where it changes the next fight; the road will still be there.",
                        }),
        (_, Some(true)) => Some(ColonyBeat {
                            id: "mara_second_victory",
                            title: "TWO ROADS HOLD",
                            text: "Two field routes are holding. That is enough for the Directorate to start measuring them, and enough for us to choose where the next stand begins.",
                        }),
        (_, Some(false)) => Some(ColonyBeat {
                            id: "mara_second_defeat",
                            title: "THE LINE BENDS",
                            text: "The second route broke, but it did not take the refuge with it. We repair what we can, keep the people together, and decide what must never be abandoned.",
                        }),
        (_, _) => Some(ColonyBeat {
                            id: "mara_second_aftermath",
                            title: "THE ROAD AHEAD",
                            text: "The refuge has changed and the road ahead is still unwritten. Keep the people together while we decide where the next stand begins.",
                        }),
    }
}

fn ilya_reed_beat(operations_completed: u32, outcome: Option<bool>) -> Option<ColonyBeat> {
    match (operations_completed, outcome) {
        (0, _) => Some(ColonyBeat {
                            id: "ilya_arrival",
                            title: "A CLINIC WITHOUT TEXTBOOKS",
                            text: "The clinic is stocked for ordinary injuries and Mirexis has stopped giving us ordinary injuries. Bring people back breathing; I will learn the rest from what their bodies tell me.",
                        }),
        (1, Some(true)) => Some(ColonyBeat {
                            id: "ilya_first_victory",
                            title: "ACCOUNTED FOR",
                            text: "Everyone who returned is accounted for. Mara calls that readiness. I call it a chance to avoid spending people like spare parts.",
                        }),
        (1, Some(false)) => Some(ColonyBeat {
                            id: "ilya_first_defeat",
                            title: "FAILURE IS A CONDITION",
                            text: "Failure is a condition, not a verdict. Tap REQUEST TREATMENT if someone is recovering; I can get a viable squad back into the field.",
                        }),
        (1, _) => Some(ColonyBeat {
                            id: "ilya_first_aftermath",
                            title: "ACCOUNTED FOR",
                            text: "Everyone who returned is accounted for. Before we call the result, let the clinic finish its work and tell us what the field cost.",
                        }),
        (_, Some(true)) => Some(ColonyBeat {
                            id: "ilya_second_victory",
                            title: "THE COST OF RETURNING",
                            text: "A victory still leaves a body to mend. Let the scars tell us what the field is asking, before the colony turns survival into a habit of looking away.",
                        }),
        (_, Some(false)) => Some(ColonyBeat {
                            id: "ilya_second_defeat",
                            title: "NO ONE IS DISPOSABLE",
                            text: "We can recover from this. The first thing the colony must not recover from is believing that a lost operation makes a lost person acceptable.",
                        }),
        (_, _) => Some(ColonyBeat {
                            id: "ilya_second_aftermath",
                            title: "THE CLINIC KEEPS SCORE",
                            text: "Two operations leave a pattern in every body that returns. I will read it carefully before the colony turns survival into a habit of looking away.",
                        }),
    }
}

fn sol_cairn_beat(operations_completed: u32, outcome: Option<bool>) -> Option<ColonyBeat> {
    match (operations_completed, outcome) {
        (0, _) => Some(ColonyBeat {
                            id: "sol_arrival",
                            title: "BROKEN MACHINES, USEFUL CLUES",
                            text: "Every dead system leaves a pattern behind. I can keep the power plant breathing, but I need the field teams to bring me the pieces that still remember how to work.",
                        }),
        (1, Some(true)) => Some(ColonyBeat {
                            id: "sol_first_victory",
                            title: "THE GRID REMEMBERS",
                            text: "The west line is drawing power again. It is not a repair so much as a conversation with a machine that has been waiting for someone to ask the right question.",
                        }),
        (1, Some(false)) => Some(ColonyBeat {
                            id: "sol_first_defeat",
                            title: "SALVAGE THE LESSON",
                            text: "The field hardware is damaged, not useless. Give me one working route and I can turn the wreckage into enough time for the next squad.",
                        }),
        (1, _) => Some(ColonyBeat {
                            id: "sol_first_aftermath",
                            title: "SALVAGE THE SIGNAL",
                            text: "The field hardware is damaged, not useless. Bring me what survived and I will tell you what the route was trying to become.",
                        }),
        (_, Some(true)) => Some(ColonyBeat {
                            id: "sol_second_victory",
                            title: "THE NEXT BROKEN MACHINE",
                            text: "Two field routes are holding, which means the Directorate has started measuring them. The assault clock is our next broken machine.",
                        }),
        (_, Some(false)) => Some(ColonyBeat {
                            id: "sol_second_defeat",
                            title: "KEEP THE LIGHTS ON",
                            text: "The routes failed before the grid did. That distinction matters. Give me the damaged parts and I will make the refuge harder to surprise next time.",
                        }),
        (_, _) => Some(ColonyBeat {
                            id: "sol_second_aftermath",
                            title: "THE CLOCK IS MOVING",
                            text: "The assault clock is moving even while I keep the lights on. Give me one clear route and I can turn the next broken machine into a warning.",
                        }),
    }
}

fn nadi_vale_beat(operations_completed: u32, outcome: Option<bool>) -> Option<ColonyBeat> {
    match (operations_completed, outcome) {
        (0, _) => Some(ColonyBeat {
                            id: "nadi_arrival",
                            title: "THE THING THAT ANSWERS",
                            text: "My symbiote reacts to the Brood before I can name the threat. I am still deciding whether that makes it a patient, a partner, or a warning with a pulse.",
                        }),
        (1, Some(true)) => Some(ColonyBeat {
                            id: "nadi_first_victory",
                            title: "A QUIETER SIGNAL",
                            text: "The sample went quiet when the refuge came back online. Quiet is not the same as safe, but it gives us a moment to listen without mistaking fear for evidence.",
                        }),
        (1, Some(false)) => Some(ColonyBeat {
                            id: "nadi_first_defeat",
                            title: "THE BODY KEEPS SCORE",
                            text: "The Brood learned something from the failed route, and so did we. I can help the wounded recover, but we should not pretend adaptation has no price.",
                        }),
        (1, _) => Some(ColonyBeat {
                            id: "nadi_first_aftermath",
                            title: "A SIGNAL UNRESOLVED",
                            text: "The signal changed when we came back. I need one quiet hour before I call it a warning, a summons, or something that wants to be understood.",
                        }),
        (_, Some(true)) => Some(ColonyBeat {
                            id: "nadi_second_victory",
                            title: "SOMETHING BELOW HEARS US",
                            text: "The signal is no longer only coming from the field. Something below Mirexis is answering the colony, and I do not think it cares which faction gets the credit.",
                        }),
        (_, Some(false)) => Some(ColonyBeat {
                            id: "nadi_second_defeat",
                            title: "LISTEN BEFORE CUTTING",
                            text: "The signal changed when the route failed. Before we cut it away, we need to know whether it is calling the Brood, warning us about them, or trying to call us home.",
                        }),
        (_, _) => Some(ColonyBeat {
                            id: "nadi_second_aftermath",
                            title: "THE ANSWER IS NOT READY",
                            text: "The signal is stronger, but the answer is not ready. We need the colony to listen together before one frightened voice decides what Mirexis means.",
                        }),
    }
}

fn sedge_beat(operations_completed: u32, outcome: Option<bool>) -> Option<ColonyBeat> {
    match (operations_completed, outcome) {
        (0, _) => Some(ColonyBeat {
                            id: "sedge_arrival",
                            title: "THE COURIER IN THE REEDS",
                            text: "Sedge came from a family the colony's records never named. Their body learned the marsh before our instruments learned to call it a mutation; the Waystation is the first place they have been asked to stay.",
                        }),
        (1, Some(true)) => Some(ColonyBeat {
                            id: "sedge_first_victory",
                            title: "A ROUTE THAT BREATHES",
                            text: "Sedge returned with the route still alive in their head. They call it inheritance; the Gene Lab calls it evidence. The colony has to learn the difference between studying a gift and claiming it.",
                        }),
        (1, Some(false)) => Some(ColonyBeat {
                            id: "sedge_first_defeat",
                            title: "THE MARSH REMEMBERS",
                            text: "The failed route followed Sedge home. They did not ask us to call the adaptation a cure, only to stop pretending the danger began when the colony noticed it.",
                        }),
        (1, _) => Some(ColonyBeat {
                            id: "sedge_first_aftermath",
                            title: "A BODY WITHOUT A FILE",
                            text: "Sedge's route-map survived the operation, but the colony's records still have no honest category for them. We can make room before we make a label.",
                        }),
        (_, _) => Some(ColonyBeat {
                            id: "sedge_second_aftermath",
                            title: "THE TIMELINE MOVES",
                            text: "Sedge's family changed before the official first mutation, which means our history is missing a living chapter. The courier keeps bringing back routes; now the colony must decide who gets to write them down.",
                        }),
    }
}

fn ninth_voice_apart_beat(operations_completed: u32, outcome: Option<bool>) -> Option<ColonyBeat> {
    match (operations_completed, outcome) {
        (0, _) => Some(ColonyBeat {
                            id: "ninth_arrival",
                            title: "THE NINTH VOICE",
                            text: "Ninth arrived from a chorus that no longer exists, carrying commands in a voice that is not quite its own. The Waystation can offer shelter, but the colony must leave room for the first word it chooses.",
                        }),
        (1, Some(true)) => Some(ColonyBeat {
                            id: "ninth_first_victory",
                            title: "A LINE THAT ANSWERS",
                            text: "Ninth anchored the hostile line and came back with one borrowed phrase missing. The Brood heard the silence; the colony has to decide whether that absence is loss or the beginning of a self.",
                        }),
        (1, Some(false)) => Some(ColonyBeat {
                            id: "ninth_first_defeat",
                            title: "THE CHORUS STILL REACHES",
                            text: "The failed route gave the old chorus a path back through Ninth. The organism is not a broken tool, but the colony must make the next shelter strong enough for refusal to be possible.",
                        }),
        (1, _) => Some(ColonyBeat {
                            id: "ninth_first_aftermath",
                            title: "A PHRASE LEFT OPEN",
                            text: "Ninth returned with a sentence that stops before its command. We can record the gap, build around it, and let a voice become more than the thing that first carried it.",
                        }),
        (_, _) => Some(ColonyBeat {
                            id: "ninth_second_aftermath",
                            title: "THE SPACE BETWEEN WORDS",
                            text: "Ninth now speaks in phrases the destroyed chorus never gave it. The colony remembers that a borrowed beginning does not decide who gets to finish the sentence.",
                        }),
    }
}

fn veya_orn_beat(operations_completed: u32, outcome: Option<bool>) -> Option<ColonyBeat> {
    match (operations_completed, outcome) {
        (0, _) => Some(ColonyBeat {
                            id: "veya_arrival",
                            title: "THE PERSON BEHIND THE CIPHER",
                            text: "The Waystation received a Directorate exile before it received a clean signal. Veya says the codes are stolen; Mara says the danger is still theirs. Tap OPERATIONS to review the first terms.",
                        }),
        (1, Some(true)) => Some(ColonyBeat {
                            id: "veya_first_victory",
                            title: "A RANGE THAT MOVES",
                            text: "Veya came back from the first operation with the target map intact and their story less rehearsed. The colony can use what they know without pretending trust arrived with them.",
                        }),
        (1, Some(false)) => Some(ColonyBeat {
                            id: "veya_first_defeat",
                            title: "NO CLEAN ALIBI",
                            text: "The failed route gave the Directorate a better line on us, and Veya did not look away from the cost. If they stay, the Waystation has to be a place where blame can be spoken aloud.",
                        }),
        (1, _) => Some(ColonyBeat {
                            id: "veya_first_aftermath",
                            title: "THE CIPHER IS NOT A CONFESSION",
                            text: "Veya has a cipher, a uniform, and no clean way to separate the two. The colony will decide what the codes are worth before it decides what their bearer is worth.",
                        }),
        (_, _) => Some(ColonyBeat {
                            id: "veya_second_aftermath",
                            title: "A HOME WITH WITNESSES",
                            text: "The Waystation is no longer a stop between Directorate lines. Veya chose to remain where every argument has a witness, and the colony remembers that choice as a person rather than a resource.",
                        }),
    }
}
