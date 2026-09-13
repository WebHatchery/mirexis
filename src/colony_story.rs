//! Persistent character-led colony beats for the first campaign horizon.
pub mod beats;
pub use beats::*;

use serde::{Deserialize, Serialize};

pub mod identity;
pub use identity::{identity_arc_beat, identity_npc, IdentityArcProgress, IdentityBuildingState};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ColonyStoryState {
    #[serde(default)]
    pub heard_beats: Vec<String>,
    #[serde(default)]
    pub archived_notes: Vec<ArchivedFieldNote>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArchivedFieldNote {
    pub beat_id: String,
    pub speaker: String,
    pub title: String,
    pub text: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColonyBeat {
    pub id: &'static str,
    pub title: &'static str,
    pub text: &'static str,
}

impl ColonyStoryState {
    pub fn has_heard(&self, beat_id: &str) -> bool {
        self.heard_beats.iter().any(|heard| heard == beat_id)
    }

    pub fn acknowledge(&mut self, beat_id: &str) {
        if !self.has_heard(beat_id) {
            self.heard_beats.push(beat_id.to_owned());
        }
    }

    pub fn acknowledge_note(&mut self, speaker_id: &str, speaker: &str, beat: ColonyBeat) {
        self.acknowledge(beat.id);
        if self
            .archived_notes
            .iter()
            .any(|note| note.beat_id == beat.id)
        {
            return;
        }
        self.archived_notes.push(ArchivedFieldNote {
            beat_id: beat.id.to_owned(),
            speaker: if speaker.is_empty() {
                speaker_id.to_owned()
            } else {
                speaker.to_owned()
            },
            title: beat.title.to_owned(),
            text: beat.text.to_owned(),
        });
    }

    pub fn archived_notes(&self) -> &[ArchivedFieldNote] {
        &self.archived_notes
    }

    pub fn acknowledged_count(&self) -> usize {
        self.heard_beats.len()
    }
}

pub fn finale_beat(
    path_id: &str,
    character_id: &str,
    campaign_complete: bool,
    story: &ColonyStoryState,
) -> Option<ColonyBeat> {
    if !campaign_complete {
        return None;
    }
    let (id, title, text) = match (path_id, character_id) {
        ("human_redoubt", "kira_voss") => (
            "finale_redoubt_kira",
            "A MAP WITH AN EDGE",
            "Kira's maps stop at the Arsenal wall, not because the signal ends, but because the people behind it have chosen what may enter. She is learning to hear a boundary as an answer.",
        ),
        ("human_redoubt", "mara_venn") => (
            "finale_redoubt_mara",
            "A SHARED SHIELD",
            "Mara gives the Arsenal a shared drill instead of a single shield. The wall can protect the colony only when the people behind it can name what it is for.",
        ),
        ("human_redoubt", "ilya_reed") => (
            "finale_redoubt_ilya",
            "NO BODY IS A RESOURCE",
            "Ilya writes the Arsenal's first medical rule beside the armour racks: no body is a resource. Recovery becomes part of defence, not the price of admission.",
        ),
        ("human_redoubt", "sol_cairn") => (
            "finale_redoubt_sol",
            "POWER WITH WITNESSES",
            "Sol leaves the Arsenal's circuits open to inspection. A wall that cannot explain its power will eventually ask the people inside it to become obedient.",
        ),
        ("human_redoubt", "nadi_vale") => (
            "finale_redoubt_nadi",
            "A LIVING CHANNEL",
            "Nadi keeps one channel from the Garden open beneath the Arsenal. The colony can hold its ground without pretending the living signal is an enemy.",
        ),
        ("human_redoubt", "veya_orn") => (
            "finale_redoubt_veya",
            "THE CIPHER HAS WITNESSES",
            "Veya hangs the broken cipher outside the Arsenal's command room. It remains useful only when witnesses can refuse the order it would once have carried.",
        ),
        ("human_redoubt", "sedge") => (
            "finale_redoubt_sedge",
            "A HOME IN THE MARGINS",
            "Sedge marks the routes around the Arsenal in family names, not military coordinates. The wall protects a home when it remembers where home came from.",
        ),
        ("human_redoubt", "ninth_voice_apart") => (
            "finale_redoubt_ninth",
            "A WALL THAT LEAVES A GAP",
            "Ninth places a quiet resonance at the Arsenal's edge and leaves one channel unclaimed. The wall protects the colony best when it can shelter a voice without making it repeat an order.",
        ),
        ("living_commonwealth", "kira_voss") => (
            "finale_commonwealth_kira",
            "A MAP FOR MANY VOICES",
            "Kira stops searching for one voice in the Garden's chorus. Her map becomes a way for different answers to share a page without becoming one command.",
        ),
        ("living_commonwealth", "mara_venn") => (
            "finale_commonwealth_mara",
            "STRENGTH LEAVES ROOM",
            "Mara lets the Garden grow around the armour instead of asking the armour to lead. Strength is still protection when it leaves room for another body to choose its shape.",
        ),
        ("living_commonwealth", "ilya_reed") => (
            "finale_commonwealth_ilya",
            "CONSENT BESIDE THE ROOTS",
            "Ilya posts consent beside the Garden's living beds. The colony may accept a treatment that changes it, but no cure begins by hiding the question.",
        ),
        ("living_commonwealth", "sol_cairn") => (
            "finale_commonwealth_sol",
            "A READABLE LIVING CIRCUIT",
            "Sol builds labels into the Garden's roots so every living circuit has a readable return path. Stewardship begins where complete control gives way to care.",
        ),
        ("living_commonwealth", "nadi_vale") => (
            "finale_commonwealth_nadi",
            "THE FIRST COMMONWEALTH",
            "Nadi answers the Garden without speaking for every voice inside it. The first commonwealth is not one body; it is an agreement to keep making room.",
        ),
        ("living_commonwealth", "veya_orn") => (
            "finale_commonwealth_veya",
            "A PERSON BEFORE A DESIGNATION",
            "Veya teaches the Garden's gates to recognise a person before a designation. The old command can still find a route, but it no longer decides who belongs.",
        ),
        ("living_commonwealth", "sedge") => (
            "finale_commonwealth_sedge",
            "A CITIZEN'S HISTORY",
            "Sedge brings the marsh route into the Garden as a family record. Adaptation is no longer a specimen or a secret; it is a citizen's history.",
        ),
        ("living_commonwealth", "ninth_voice_apart") => (
            "finale_commonwealth_ninth",
            "THE CHORUS MAKES ROOM",
            "Ninth joins the Garden without dissolving into it. The Commonwealth learns that many voices do not need one origin, only enough room to answer in their own time.",
        ),
        ("open_threshold", "kira_voss") => (
            "finale_threshold_kira",
            "EVERY ROUTE HAS A HOME",
            "Kira gives every Threshold route a name and a way home. The signal can call beyond Mirexis now, but it no longer gets to call only her.",
        ),
        ("open_threshold", "mara_venn") => (
            "finale_threshold_mara",
            "THE GUARD AT THE OPEN DOOR",
            "Mara trains the Threshold's return crews to protect the crossing and the person who comes back changed. An open door needs a guard who knows when not to close it.",
        ),
        ("open_threshold", "ilya_reed") => (
            "finale_threshold_ilya",
            "HELP MUST ARRIVE AS AN OFFER",
            "Ilya sends a consent ledger through the Threshold with every medical team. Even beyond Mirexis, help must arrive as an offer before it becomes an intervention.",
        ),
        ("open_threshold", "sol_cairn") => (
            "finale_threshold_sol",
            "A PUBLIC HAND ON THE SWITCH",
            "Sol leaves the Threshold's controls legible to the people who use them. The network may be older than the colony, but its future still needs a public hand on the switch.",
        ),
        ("open_threshold", "nadi_vale") => (
            "finale_threshold_nadi",
            "THE RIGHT TO ANSWER NO",
            "Nadi listens through the Threshold and hears distance without mistaking it for absence. The signal is allowed to travel while the person receiving it keeps the right to answer no.",
        ),
        ("open_threshold", "veya_orn") => (
            "finale_threshold_veya",
            "A KEY THAT COMES HOME",
            "Veya turns the cipher into a return protocol instead of a targeting key. Every route is safer when the person who opens it can also close it.",
        ),
        ("open_threshold", "sedge") => (
            "finale_threshold_sedge",
            "THE ROUTE BELONGS TO ITS TRAVELLERS",
            "Sedge carries the first Threshold map back to the reeds and leaves the margins blank for the families who follow. A route belongs to its travellers, not the first colony to copy it.",
        ),
        ("open_threshold", "ninth_voice_apart") => (
            "finale_threshold_ninth",
            "A SIGNAL THAT CAN STOP",
            "Ninth sends one phrase through the Threshold and keeps the next one. The open route becomes safer when a voice can cross the distance and still choose silence on the other side.",
        ),
        _ => return None,
    };
    (!story.has_heard(id)).then_some(ColonyBeat { id, title, text })
}

pub fn commons_meal_beat(character_id: &str) -> Option<ColonyBeat> {
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
        "sedge" => (
            "commons_meal_sedge",
            "A PLACE IN THE RECORD",
            "Sedge brought reed-salt to the Commons and watched everyone decide whether it was food or evidence. By the second bowl, the colony had made the simpler choice: it was something their family knew how to share.",
        ),
        "ninth_voice_apart" => (
            "commons_meal_ninth",
            "THE FIRST UNBORROWED WORD",
            "Ninth repeated the name of the meal, then stopped and made a sound no instrument could classify. Nobody translated it. The Commons let the silence stand as a contribution.",
        ),
        _ => return None,
    };
    Some(ColonyBeat { id, title, text })
}

pub fn facility_upgrade_beat(
    upgrade_id: &str,
    character_id: &str,
    story: &ColonyStoryState,
) -> Option<ColonyBeat> {
    let (expected_character, id, title, text) = match upgrade_id {
        "trauma_ward" => (
            "ilya_reed",
            "facility_trauma_ward_ilya",
            "THE SCAR IS NOT THE SENTENCE",
            "The Trauma Ward does not erase what the field did to a body. It gives the person who carries the scar a better chance to decide what comes next, and gives the colony one less excuse to call damage destiny.",
        ),
        "adaptation_clinic" => (
            "ilya_reed",
            "facility_adaptation_clinic_ilya",
            "THE CLINIC LEARNS TO ASK",
            "The Adaptation Clinic makes treatment safer, but safety is not permission. I will keep the mutation in the room as evidence, not an owner, and ask the person what kind of recovery they will accept.",
        ),
        _ => return None,
    };
    (expected_character == character_id && !story.has_heard(id)).then_some(ColonyBeat {
        id,
        title,
        text,
    })
}

pub fn phase_beat(phase_id: &str, character_id: &str) -> Option<ColonyBeat> {
    let (id, title, text) = match (phase_id, character_id) {
        ("contact", "kira_voss") => (
            "phase_contact_kira",
            "THE ANSWER UNDER THE SIGNAL",
            "The Black Channel answered from below the colony, and the Living Chorus answered from within it. The signal is not a faction's property anymore; it is a question we have to carry together.",
        ),
        ("adaptation", "mara_venn") => (
            "phase_adaptation_mara",
            "THE COLONY MAKES ROOM",
            "The Gene Lab can change what enters the colony, but a door is not a verdict. If we ask the adapted to become useful before we ask what they want, we have built another Directorate with better lighting.",
        ),
        ("adaptation", "ilya_reed") => (
            "phase_adaptation_ilya",
            "CONSENT HAS A PULSE",
            "The clinic can record a changed body without claiming it. Every treatment in Adaptation begins with the same question: is the person asking for help, or are we asking the person to become easier to keep?",
        ),
        ("adaptation", "sol_cairn") => (
            "phase_adaptation_sol",
            "POWER FOR THE UNKNOWN",
            "The Gene Lab draws power from a grid built for pumps and doors. I can keep it online, but I will not pretend current makes an experiment ethical.",
        ),
        ("adaptation", "nadi_vale") => (
            "phase_adaptation_nadi",
            "THE BODY MAPS THE ROOM",
            "The Gene Lab keeps asking what a body can become, but the answer cannot belong to the instrument alone. Neural Bloom gives us new senses only if the person inside them remains the author.",
        ),
        ("adaptation", "sedge") => (
            "phase_adaptation_sedge",
            "THE OLD MAP IN THE LAB",
            "Sedge knows the Gene Lab's corridors by the pressure in the roots. Their family changed before our records began, and the first ethical question is simple: do we ask what they know before we ask what we can take?",
        ),
        ("adaptation", "ninth_voice_apart") => (
            "phase_adaptation_ninth",
            "THE ORGANISM IS LISTENING",
            "Ninth can carry a signal between the Gene Lab and the field, but transmission is not obedience. The colony has to build a language that leaves room for the organism to stop answering.",
        ),
        ("escalation", "mara_venn") => (
            "phase_escalation_mara",
            "THEY CAN SEE THE LIGHTS",
            "Three powers are converging on the same patch of dark, and now they can see our lights. The colony cannot survive as an invisible camp; it needs a defence that the people behind it can name and direct.",
        ),
        ("escalation", "ilya_reed") => (
            "phase_escalation_ilya",
            "THE WOUND IS A MESSAGE",
            "Three powers on one horizon turn every injury into a decision. I can close the wound, but the colony has to decide what kind of future it is willing to send people back into.",
        ),
        ("escalation", "sol_cairn") => (
            "phase_escalation_sol",
            "THE GRID UNDER PRESSURE",
            "The colony's power is visible from every approach now. I can reinforce the circuit, but a strong grid is not a command structure; someone still has to decide what it is protecting.",
        ),
        ("escalation", "nadi_vale") => (
            "phase_escalation_nadi",
            "THE SIGNAL HAS A BODY",
            "The thing below Mirexis is no longer content to be a pattern in the instruments. It is reaching through the pressure between factions, and I need the colony to hear the person inside the warning.",
        ),
        ("escalation", "veya_orn") => (
            "phase_escalation_veya",
            "A CIPHER IN THE CROSSFIRE",
            "The Directorate taught Veya to read a battlefield as a set of assets. Three powers have made that lesson useless; the colony must decide whether a map can protect people without turning them into positions.",
        ),
        ("escalation", "sedge") => (
            "phase_escalation_sedge",
            "THE ROUTE HAS WITNESSES",
            "Sedge's map was private when it was only a way home. With three powers converging, every route becomes public the moment someone survives it; the colony must learn how to witness without taking ownership.",
        ),
        ("escalation", "ninth_voice_apart") => (
            "phase_escalation_ninth",
            "A BORROWED COMMAND IN THE CROSSFIRE",
            "The three powers want Ninth's resonance because it can turn a line into a formation. The organism has to decide whether the colony is a shelter, a chorus, or another command that has learned to sound gentle.",
        ),
        _ => return None,
    };
    Some(ColonyBeat { id, title, text })
}

pub fn contact_route_beat(
    protocol_id: &str,
    character_id: &str,
    trace_completed: bool,
    contact_complete: bool,
) -> Option<ColonyBeat> {
    let stage = if contact_complete {
        2
    } else if trace_completed {
        1
    } else {
        0
    };
    let (expected_character, id, title, text) = match (protocol_id, stage) {
        ("directorate_requisition", 0) => (
            "kira_voss",
            "contact_directorate_witness",
            "THE BLACK CHANNEL HAS A WITNESS",
            "The Directorate calls this a requisition because a clean word can make a dirty demand sound lawful. Kira copied the transmission into the colony ledger so nobody has to trust the signal alone.",
        ),
        ("directorate_requisition", 1) => (
            "sol_cairn",
            "contact_directorate_contradiction",
            "THE CODE DISAGREES WITH THE MAP",
            "The stolen code opens a route, but its map still assumes the colony is an asset. Sol can make the machine obey; the harder question is whether we should let an old command decide where we stand.",
        ),
        ("directorate_requisition", 2) => (
            "mara_venn",
            "contact_directorate_aftermath",
            "THE GATE IS NOT A RETURN ADDRESS",
            "The Directorate can still find the gate, but it no longer owns what happens behind it. Mara marks the route as ours and leaves the old command language outside the wall.",
        ),
        ("brood_cultivation", 0) => (
            "nadi_vale",
            "contact_brood_witness",
            "THE CHORUS HAS A PULSE",
            "The contained culture answers when Nadi speaks, but an answer is not consent. She keeps the glass open long enough for the colony to hear a living thing without pretending it belongs to us.",
        ),
        ("brood_cultivation", 1) => (
            "ilya_reed",
            "contact_brood_contradiction",
            "A DOCILE THING STILL CHOOSES",
            "The Brood echo is calm under observation and still changes when the room changes. Ilya refuses to call that obedience; medicine begins where the subject is allowed to remain unpredictable.",
        ),
        ("brood_cultivation", 2) => (
            "mara_venn",
            "contact_brood_aftermath",
            "THE GLASS DOES NOT MAKE US SAFE",
            "The colony learned to keep the chorus alive without making it a tool. Mara records the danger beside the benefit, because a shelter that hides its cost is only another kind of cage.",
        ),
        ("ascendant_capacitor", 0) => (
            "sol_cairn",
            "contact_ascendant_witness",
            "A CIRCUIT LOOKS BACK",
            "The Ascendant lattice woke before Sol closed the circuit. It did not transmit a message so much as notice the colony, and now every repair has to account for being observed.",
        ),
        ("ascendant_capacitor", 1) => (
            "kira_voss",
            "contact_ascendant_contradiction",
            "THE ROUTE REMEMBERS US",
            "Kira's map shows a path the Ascendants insist was sealed. The route remembers our movement anyway, which means the lattice is not only a road; it is a witness with a history of its own.",
        ),
        ("ascendant_capacitor", 2) => (
            "nadi_vale",
            "contact_ascendant_aftermath",
            "THE LIGHT IS NOT AN ANSWER",
            "The capacitor gives the colony power and asks for a future it cannot explain. Nadi leaves the question written beside the switch: access is useful, but usefulness is not permission.",
        ),
        _ => return None,
    };
    (expected_character == character_id).then_some(ColonyBeat { id, title, text })
}
