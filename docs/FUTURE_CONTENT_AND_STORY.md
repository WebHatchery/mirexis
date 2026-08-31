# Mirexis Future Content and Story Plan

Status: pre-production direction; not an implementation commitment  
Created: 30 August 2026  
Planning horizon: after Playable Game Phase 1 human acceptance

## 1. Purpose

Mirexis already proves its five-phase campaign, seven base classes, eight advanced
classes, colony construction, mutation evolution, and three ending paths. Future work
should deepen those systems into a memorable campaign rather than merely increasing
their counts.

The first route-exclusive outsider slice now covers both ends of the early campaign:
Directorate Requisition can build a Waystation, recruit Veya Orn, and carry her origin,
equipment action, relationships, disagreements, and closing choice through saves, while
Adaptation can recruit Sedge, a Mireborn courier, for biomass. Sedge brings a distinct
MAP HAZARD action, Gene Lab presence, Brood attention consequence, three-beat Waystation arc,
and ending voice. Each Contact protocol now also carries a three-stage witness, contradiction,
and aftermath thread
through the starting colonists. The remaining origins and equivalent faction routes stay
future work.

An unchosen matching outsider now remains present as a Waystation guest. The guest is
visible on the colony map, can be approached through touch-first walking, and presents the
authored arrival note plus the normal recruitment handoff without becoming a roster member
until recruitment succeeds.

Adaptation and Escalation now carry that continuity through multiple colony voices: defence,
clinic, engineering, xenobiology, and any route outsider present at the Waystation each leave
an authored phase note before the finale.

The Commons is also now a working home-space slice: a powered settlement can host one
shared meal per operation, spend food, and turn the active squad's table into persistent
relationship progress.

The Relay Mast now gives that settlement a risky communication choice: one powered scan per
operation refreshes mission routes and exposes the colony to whichever faction is watching
hardest.

This document plans the next content horizon: playable peoples and origins, classes,
skills, buildings, upgrades, recruits, and the story that connects them. It is subordinate
to `PLAYABLE_GAME_PHASE_1.md` until the required uncoached first-hour playtests are
complete. All fourteen base-class techniques are now implemented as a deliberately
bounded tactical slice; mastery forks, new peoples, and later story work
  remain planning direction beyond the implemented Directorate Exile slice.

The first new defence structure is now playable: a powered Watchtower can be placed in
the colony and becomes stronger directional cover during a colony assault. Its coverage
fails when the tower is damaged or the settlement grid cannot supply it, keeping the
structure useful without making it a free permanent advantage.

The first facility branch is now playable as well. An operational Power Plant can spend
55 materials to queue one of two mutually exclusive level-2 choices, completed after the
next operation: Redundant Grid preserves two power through plant damage, while an
operational Hot Core adds three supply and makes the most visible faction more attentive
after each operation.
The choice is touch-visible and save-backed; the facility branches now carry tactical,
social, and narrative contracts. The Command Centre now also offers
Signal Cartography, which reveals a third mission route, or Counterintelligence Cell, which
reduces mission pressure while the centre remains online. The Barracks now offers Simulation
Hall, which makes retraining and technique trials cheaper, or Doctrine Yard, which adds a Rally
formation and training barricades to colony defence while the barracks remains online.
The Infirmary now offers Trauma Ward, which shortens recovery, softens persistent scar
tradeoffs, and protects one adjacent stabilization position in colony defence, or Adaptation
Clinic, which makes mutation-delayed healing safer and lowers direct treatment to three biomass.
After either branch completes, Ilya can be approached in the colony for a one-time,
save-backed field note that names the medical choice rather than leaving it as an anonymous
stat modifier.

## 2. Expansion rules

Every future feature should satisfy at least three of these tests:

1. It changes a tactical decision rather than only adding a passive number.
2. It creates a visible use or consequence in the colony.
3. A named character cares about it.
4. A faction reacts to it.
5. It changes an operation, event, or ending acknowledgement.
6. It creates a viable build without making an existing build obsolete.

The following limits keep the game recognisably Mirexis:

- **Named people before generic units.** New playable peoples enter through authored
  recruits, not a detached race-selection screen.
- **Classes remain training.** Biology, origin, and mutation may change how a class is
  expressed, but they do not prohibit a person from learning it.
- **One strong choice beats five small bonuses.** Upgrades should branch or add a new
  action instead of creating long stacks of `+5%` improvements.
- **The colony is a home, not a spreadsheet.** A new facility needs an inhabitant,
  physical presence, peacetime use, and defence-mission consequence.
- **The player remains an underdog.** Hybrid technology makes the settlement ingenious,
  not a fourth galactic empire.
- **Mystery narrows over time.** Later revelations should reinterpret early events rather
  than replace them with unrelated lore.

## 3. Story foundation

### 3.1 The hidden truth

The three implemented ending revelations are different views of one truth:

> Mirexis is a world-scale refuge engine: part organism, part machine, and part memory.
> It was made to preserve life through a catastrophe by sensing danger, adapting local
> biology, and opening routes through a phase network.

The engine has three layers:

- Its **body** is the planetary ecology. The Brood is an injured, decentralised immune
  response that has continued adapting without a coherent instruction.
- Its **nervous system** is the buried lattice. Ascendant relics are interfaces and
  maintenance tools left by the civilisation that once tended it.
- Its **new voice** is emerging through mutated colonists. Their changes are not a simple
  infection; Mirexis is testing whether these stranded people can become new stewards.

This preserves all three finale truths. The Human Redoubt learns that an engine remade
the world to preserve life under siege. The Living Commonwealth learns that Mirexis is a
dormant symbiote sharing roots with the Brood. The Open Threshold learns that Ascendant
relics interface with the mind below. No ending is the route that alone discovers the
"real" answer.

### 3.2 What each power wants

- **The Directorate** contains competing human interests. Asset Command wants the refuge
  engine contained and weaponised; some civilian officers believe the colony should be
  evacuated or allowed to govern itself. The blackout began as a Directorate quarantine,
  but not every Directorate character supports it.
- **The Brood** is a family of choruses rather than one evil hive. Feral broods read the
  colony as a wound. Older nodes are trying to reconnect with Mirexis, while severed
  organisms can become individuals away from the command noise.
- **The Ascendants** are custodians and inheritors, not omniscient creators. They know how
  to operate pieces of the lattice but have mythologised much of its purpose. They fear
  that humanity will wake the engine incorrectly and fear even more that it may prefer
  humanity to them.
- **The colony** wants survival without surrender. Its central question is not which
  power has the best technology, but what forms of dependence can coexist with freedom.

### 3.3 Campaign story pass

The implemented campaign phases remain the spine. Future narrative work should fill
them with recurring character and colony beats.

| Phase | Story job | Existing anchor | Future reveal and choice | Lasting change |
|---|---|---|---|---|
| Isolation | Make the settlement worth protecting and turn the communications failure into a mystery | First-hour arrival, Dead Freight, Root Burn, Cold Signal | Evidence shows the blackout is an intentional quarantine. Decide whether to broadcast, hide, or investigate before answering anyone. | The first investment, one new scar or bond, and a public place in the colony change. |
| Contact | Give each power a credible voice and a dangerous offer | Black Channel, Living Chorus, or Open Circuit | The chosen signal is not merely intercepted; something below Mirexis answers through it. Shelter a route-specific outsider despite a cost. | A contact recruit, prototype family, and faction-specific colony problem appear. |
| Adaptation | Make mutation personal and prove the planet is reacting to the squad | Glass Nerve, Gene Lab, first evolution | Kira's signal and the evolved colonists form a partial map of the refuge engine. Decide who may be changed deliberately and who controls that knowledge. | Two character arcs branch, the colony gains an identity mark, and advanced training becomes a social issue. |
| Escalation | Force every compromise into public view | Three Knives and the chosen escalation response | The three powers attack the same buried heart for incompatible reasons. The colony cannot stay invisible and must declare how it will resist. | A path facility is begun, an outsider chooses loyalty, and one faction relationship closes while another changes shape. |
| Mirexis | Pay off people and place before resolving the cosmic mystery | Last Wall, Root Choir, or Door of Light | The player chooses the engine's relationship with the colony, not a colour-coded ending. | The city, surviving recruits, bonds, scars, facilities, and earlier mercy are acknowledged in the ending. |

Post-campaign operations should be an epilogue sandbox, not a sixth escalation phase.
The first repeatable path-specific loop is implemented: putting down rogue weapons in the
Redoubt, answering a new chorus in the Commonwealth, or rescuing travellers from unstable
routes beyond the Threshold. Generic pressure operations remain available alongside the
path operation, so completion expands the world instead of closing the campaign hub.

## 4. Character arcs

The five starting colonists should carry the main mystery emotionally. Each arc needs an
early colony scene, a pressure scene after a relevant operation, and a late culmination
that reacts to the player's choices.

| Character | Want | Pressure | Possible culmination |
|---|---|---|---|
| Kira Voss | Prove the patterns in the static are useful and that she is not losing herself | The refuge engine begins anticipating her decisions, making her wonder where instinct ends and instruction begins | She becomes its navigator, silences the signal, or teaches the colony to hear it collectively rather than through her alone. |
| Mara Venn | Make herself strong enough that nobody has to hold another tunnel alone | Her carapace protects others but increasingly turns fear and pain into something she can ignore | She chooses armour, vulnerability, or a protector role shared with the squad rather than borne alone. |
| Ilya Reed | Keep medicine humane while bodies stop obeying human textbooks | Effective treatments begin requiring consent to irreversible change and resources that could save someone else | Ilya authors the colony's adaptation ethics, rejects deliberate alteration, or accepts a carefully bounded living medicine. |
| Sol Cairn | Keep every broken system understandable and repairable | Hybrid machines work, but some respond to mood, memory, or biological need rather than tools | Sol builds an open civic infrastructure, centralises control for safety, or admits that stewardship can replace complete understanding. |
| Nadi Vale | Learn whether her symbiote is a patient, organ, companion, or spy | It starts answering the Brood chorus and withholding information it believes would frighten her | Nadi negotiates the first person-to-chorus accord, severs the bond, or helps the symbiote become an independent citizen. |

No culmination should require romance, permanent death, or a single morality score.
Bonds, rivalries, scars, class history, and evolution choices provide the conditions.

## 5. Playable peoples and origins

“Race” is best treated as **people and origin** in Mirexis. It describes history, body,
community needs, and how the world responds to a recruit. It should never serve as a
fixed intelligence, morality, or class aptitude modifier.

### 5.1 Planned origins

| Origin | Campaign entry | Narrative question | Mechanical promise | Production note |
|---|---|---|---|---|
| Colony Founder | New game | What does an ordinary person owe a home that may no longer be ordinary? | Flexible class history and the richest network of colony relationships | Already represented by the starting roster; deepen before widening. |
| Directorate Exile | Contact | Can someone trained to treat colonies as assets learn to belong to one? | Smartlink, drone, suppression, and command interactions with a visible attention cost | Veya Orn is implemented through the Waystation; expand the route before adding another human rig. |
| Mireborn Adapted | Adaptation | Is deliberate adaptation inheritance, coercion, or a new culture? | Sedge's MAP HAZARD action braces the courier while disrupting a hostile; recruitment costs biomass and raises Brood attention | Implemented with bespoke full-body/portrait art, Gene Lab presence, three-beat outsider arc, story beats, and ending acknowledgement. |
| Severed Brood | Contact or late Adaptation | Does individuality begin when the chorus is lost, or when another person listens? | Spore-field interaction, biomass recovery, and symbiotic equipment behaviour | Highest art and animation cost; add as one authored recruit before considering a generic pool. |
| Echo-Bound | Contact or late Adaptation | Is a body carrying an Ascendant echo one person, two people, or a tool being mistaken for both? | Rift navigation, power-fed abilities, and lattice interaction | Begin with a human host or colony-built shell before a wholly new rig. |

### 5.2 First recruit concepts

Veya Orn and Sedge are the two implemented route-specific origins; the remaining names
are working names for a later naming pass.

- **Veya Orn — Directorate Exile / Soldier.** A former Directorate range officer who
  reaches the colony through Directorate Requisition. Her Waystation arc asks whether a
  stolen targeting cipher can shelter people without making the refuge another asset.
  BREAK TARGETING NET gives her origin a tactical verb, while the three conversations
  track disagreements, relationships, faction attention, and a closing choice.
- **Sedge — Mireborn Adapted / Scout.** A courier from an unregistered outlying family
  whose body changed long before the official first mutation. Their existence proves the
  colony's timeline is wrong and makes secrecy inside the settlement a concrete issue.
  Adaptation recruits Sedge through the Waystation for 8 biomass. Their MAP HAZARD action
  gives them Guarded protection while Disrupting a hostile. Their three-beat Waystation arc
  spends route resources, changes Brood attention, strengthens relationships, and records a
  personal route legacy before the colony beats carry the map from the Gene Lab to the ending record.
- **Ninth-Voice-Apart — Severed Brood / Biotech Specialist.** A small command organism
  cut from a destroyed chorus. It can speak only by borrowing remembered phrases until
  repeated personal choices develop a voice of its own.
- **Aster-of-the-Gap — Echo-Bound / Psionic.** A damaged Ascendant maintenance echo using
  a colony-built frame. It remembers procedures but not the civilisation that made them,
  undercutting both Ascendant certainty and human assumptions about ownership.

Both currently implemented route-specific outsiders can join as their campaign phases
open, and each arc keeps independent progress. Unchosen future contacts may appear as
rivals, prisoners, correspondents, or post-campaign visitors so the world remains larger
than the roster.

### 5.3 Origin implementation contract

Each new origin requires:

- one named recruit with a want, fear, colony role, and three-beat arc;
- one innate interaction or action that cannot be reproduced by a flat stat bonus;
- one cost, complication, or social consequence;
- facility and equipment support that remains useful to other recruits;
- at least six colony or mission acknowledgements;
- one faction reaction and one ending acknowledgement;
- touch-readable tactical presentation and a complete portrait/token solution.

## 6. Class growth

### 6.1 Preserve the current foundation

The seven base classes remain the complete entry-level training set: Soldier, Defender,
Scout, Medic, Engineer, Psionic, and Biotech Specialist. Six implemented advanced
classes now form the first hybrid tier: Vanguard, Pathfinder, Lifewright, Null Adept,
Breacher, Fortifier, Rescue Specialist, and Chorus Warden.

The “possible paths” in the main design document are a pool of class fantasies, not a
commitment to build four subtrees for every base class. Skills should provide most
specialisation before more class IDs are added. The eight current hybrids are enough
to cover the demonstrated breach, fortification, rescue, and hazard-conversion verbs;
new class IDs should wait for play data to expose a real missing decision.

### 6.2 Hybrid signature actions

All eight current hybrid signatures are implemented through the validated unit/tile
targeting command path. Breacher's Make an Entrance and Fortifier's Raise Bastion open
or shape a firing lane; Rescue Specialist's Carry Through and Chorus Warden's Borrowed
Weather turn rescue and hazard conversion into explicit tactical choices. Do not fill
all 21 possible class pairings unless play data demonstrates a real missing build.
Later path-specific mastery should be a skill or doctrine, not another mandatory class
tier.

## 7. Skills and build expression

### 7.1 Skill vocabulary

Progression needs clear categories before it needs volume:

- **Fundamental:** learned when a class is trained and retained after switching.
- **Technique:** an equipped active or reaction learned through use of a base class.
- **Mastery:** a stronger class-defining choice earned after meaningful field experience.
- **Hybrid signature:** supplied by the active advanced class.
- **Origin trait:** innate and never consumes a class slot.
- **Mutation gift:** a separate biological action with its own complication.
- **Equipment action:** travels with an item and never pretends to be a learned skill.

A character should not eventually equip everything. The initial target is three active
skill slots for most base classes, four for support classes, and five for advanced
classes as already represented in data. Switching class retains learned fundamentals
but requires an explicit loadout choice; it should not silently unequip an important
technique.

### 7.2 First skill set

The first content pass now has two techniques for every base class: Soldier, Defender,
Scout, Medic, Engineer, Psionic, and Biotech Specialist. Each has one equipped technique
slot, XP-gated learning, deterministic target validation, save migration, and visible
touch controls. The next pass should balance the fourteen-technique set before a later
mastery choice evolves one of them horizontally.

| Class | Technique | Tactical purpose | Possible mastery fork |
|---|---|---|---|
| Soldier | Controlled Burst | Trade mobility for a reliable two-shot attack sequence | Suppress the target or conserve one action point on a neutralisation |
| Soldier | Armour Drill | Prepare the next hit to ignore part of armour | Apply to the squad's next hit or keep it as a stronger personal shot |
| Defender | Interpose | Step into a line threatening a nearby ally and become Guarded | Protect a wider angle or counterattack once |
| Defender | Anchor Point | Resist forced movement and strengthen adjacent cover | Extend protection to an objective or retain limited mobility |
| Scout | Slipstep | Move to a valid hazardous tile without triggering its landing effect | End Guarded beside an ally or refund part of the cost when ending in cover |
| Scout | Spotter's Mark | Mark a hostile so the next allied attack ignores part of its cover | Extend the mark or expose adjacent cover edges |
| Medic | Stabilise | Return an incapacitated ally to minimal vitality without restoring their actions | Safer ranged use or a stronger adjacent rescue |
| Medic | Combat Stimulant | Grant short action economy now followed by a disclosed penalty later | Reduce the penalty or share the benefit |
| Engineer | Portable Cover | Place destructible directional cover on a valid empty edge | Recover and redeploy it or electrify it against one crossing |
| Engineer | Overcharge | Improve an equipment action or powered objective for one round | Stronger effect with burnout or smaller repeatable effect |
| Psionic | Kinetic Draw | Pull a unit one validated tile without passing through blockers | Affect an ally safely or disrupt a hostile after movement |
| Psionic | Premonition | Reveal and soften one hostile's next intent | Share the forecast with allies or punish the hostile for following it |
| Biotech Specialist | Adaptive Secretion | Give an ally temporary resistance matching a visible hazard or damage source | Strong single-target adaptation or weaker squad adaptation |
| Biotech Specialist | Spore Veil | Create a short-lived obscuring field with a clear boundary | Healing veil or hindering veil |

Every skill must work with deterministic preview, enemy intent, battle log, phase replay,
and touch controls before it joins the content pool. No skill should require remembering
an invisible exception.

### 7.3 Progression cadence

Use operations and demonstrated class use rather than passive time:

1. Training a base class grants its fundamental and class action.
2. Completing one operation while meaningfully using the class offers its first technique.
3. Completing a personal or facility challenge offers the second technique.
4. Level 3 plus both prerequisite class histories opens advanced training, as now.
5. A late character event evolves one known technique into one of two mastery forms.

The exact thresholds require campaign balance testing. The important rule is that a
build choice should follow a story or play event the player can remember.

## 8. Buildings and colony growth

### 8.1 Existing facilities first

The current fixed facilities already cover command, training, medicine, crafting, food,
power, and mutation evolution. Future work should give those places upgrades, inhabitants,
and events before adding many new structures.

Each core facility may reach level 2 and choose one visible branch. The Command Centre,
Barracks, Infirmary, Power Plant, Hydroponics, Workshop, and Gene Lab branch pairs are now implemented in this
system.
Level 3 should be reserved for the late-game colony identity, not a generic numeric improvement.

| Facility | Branch A | Branch B | Defence consequence |
|---|---|---|---|
| Command Centre | **Signal Cartography:** a third operation route and clearer reinforcement forecasts *(implemented)* | **Counterintelligence Cell:** reduced mission pressure and faction attention *(implemented)* | A damaged branch removes its strategic benefit until repaired. |
| Barracks | **Simulation Hall:** cheaper retraining and technique trials *(implemented)* | **Doctrine Yard:** Rally formation and training barricades *(implemented)* | Adds a rally zone and two destructible training barricades in colony defence. |
| Infirmary | **Trauma Ward:** shorter recovery, softened scars, and a stabilization barricade *(implemented)* | **Adaptation Clinic:** safer mutation complications and three-biomass treatment *(implemented)* | Adds a stabilization position near the infirmary; the infirmary remains an urgent casualty objective. |
| Workshop | **Precision Bench:** weapon and armour modification *(implemented)* | **Drone Bay:** reusable deployables and repairs *(implemented)* | Adds a friendly emplacement but stores volatile components. |
| Hydroponics | **Community Kitchen:** food efficiency and relationship events *(implemented)* | **Culture Beds:** biomass cultivation and biotech ingredients *(implemented)* | Creates soft cover and a fragile food objective. |
| Power Plant | **Redundant Grid:** damage tolerance and reliable supply *(implemented)* | **Hot Core:** higher supply with an attention cost *(implemented)* | Changes which systems fail first during an assault. |
| Gene Lab | **Stabilisation Wing:** suppress chosen evolution complications while online *(implemented)* | **Evolution Chamber:** reduce mutation evolution biomass costs *(implemented)* | Provides bio-cover but risks a hostile growth if breached. |

### 8.2 New general buildings

Build these in priority order:

1. **Commons.** Implemented as a powered, once-per-operation shared meal that spends food
   and advances bonds among the active squad. Future work can add arguments, celebrations,
   post-operation recovery, and a fuller civilian defence role.
2. **Relay Mast.** Implemented as a powered, once-per-operation route scan that refreshes
   mission offers for stored power while raising the dominant faction's attention. Future
   work can add authored communication choices without hiding the broadcast risk.
3. **Research Annex.** Implemented as a 60-material, one-power physical evidence archive
   and critical defence objective. While online it reduces doctrine research by five
   materials, keeping research tied to the existing currency rather than adding a second one.
4. **Waystation.** Implemented for Directorate Requisition and Mireborn Adaptation as
   adaptable quarters, quarantine, and cultural space for Veya Orn and Sedge. Unchosen
   matching outsiders remain visible as guests before recruitment, while future route variants
   can extend it to Severed Brood and Echo-Bound visitors without framing every outsider as a
   prisoner.
5. **Salvage Yard.** Implemented as an optional 55-material, one-power recovery workshop and
   critical defence objective. Each victorious operation stores one recovered object; the
   yard sorts one object per operation into 24 materials, a 12-material next-doctrine insight,
   or a free standard equipment prototype. The Workshop remains responsible for finished
   equipment and consumes the prototype when the player chooses the item to craft.
6. **Memorial Archive.** The first Memorial Register slice records existing scars, recovery
   records, and campaign legacies in a touch-visible colony overlay. Future work should extend
   the register to retired colonists and lost objectives once those systems have enough content
   to honour.

### 8.3 Identity buildings

Escalation should unlock exactly one major identity project. These are inhabited civic
institutions, not superweapons.

- **Redoubt Arsenal** *(implemented)*: human engineering, resilient cover, militia preparation, and
  controlled Directorate salvage. Its risk is hierarchy becoming permanent emergency law.
- **Choir Garden** *(implemented)*: living walls, communal healing, Brood dialogue, and biomass cycling.
  Its risk is the colony's needs being interpreted by a growing nonhuman consensus.
- **Threshold Spire** *(implemented)*: phase routes, shield fields, Echo-Bound visitors, and remote
  exploration. Its risk is power dependence and attention from beyond Mirexis.

The late-game city art, defence map, epilogue, associated NPC placement, ambient effects, and
persistent identity-building field notes now acknowledge the chosen project. A completed campaign
also exposes one touch-accessible stewardship action per operation at the identity building;
multi-scene identity arcs now react to building damage, repair, power failure, and restoration;
before completion, each identity building also exposes one path-specific preparation action per
operation that spends its associated resource, relieves linked faction attention, and unlocks a
contact reflection.

### 8.4 Defence structures

Barricades remain the basic option. The powered **Watchtower** is now implemented: its
placed tile becomes 45-strength directional cover in colony defence, while an unpowered or
damaged tower remains an obstacle without contributing cover. Later construction may add Shield Nodes,
Mine Strips, Living Walls, and Field Clinics. Each needs a visible tactical rule and a
counterplay risk: towers can be isolated, shields depend on power, mines constrain friendly
routes, living walls may spread, and clinics attract attackers. Placement should matter
more than total structure count.

## 9. Upgrade economy

Use four distinct upgrade layers so one screen does not own every form of progression:

| Layer | Paid with | Changes | Primary location |
|---|---|---|---|
| Training | Materials and class experience | Fundamentals, techniques, advanced classes | Barracks |
| Equipment modification | Materials plus recovered faction components | One weapon, armour, tool, or module action | Workshop |
| Adaptation | Biomass, consent, and character events | Mutation gift and complication | Gene Lab |
| Civic doctrine | Facility project plus campaign decision | Colony-wide rule and visible identity | Command Centre or identity building |

Equipment should generally choose between two behaviours rather than climb five rarity
tiers. A rifle may become a stable precision weapon or an armour-breaking overload weapon;
living plate may become reactive armour or regenerative tissue. The player should be able
to explain what changed without comparing a page of small percentages.

Research unlocks these choices but does not purchase them a second time. Alien Components
remain rare keys for cross-faction prototypes and late identity work rather than becoming
a universal premium currency.

## 10. Content delivery roadmap

### Gate 0 — Accept the first hour

Complete the outstanding five uncoached sessions, including two touch-primary sessions.
Resolve severity 1 and 2 findings before widening the game. Use observed comprehension,
mission length, and investment behaviour as the balance baseline for every later slice.

### Slice 1 — Home and histories

Goal: deepen the existing people and city without new tactical systems.

- Add three-beat arcs for Kira, Mara, Ilya, Sol, and Nadi.
- Implemented: build the Commons and connect one shared meal to persistent squad bonds,
  the next debrief, and authored post-operation field notes.
- Implemented: Contact, Adaptation, and Escalation each expose a phase-specific colony field note
  through a named voice and the persistent story ledger.
- Establish the unified refuge-engine truth in internal narrative references.
- Implemented: ending acknowledgement now derives a colony legacy dossier from bonds, scars,
  evolution, people, and the chosen civic institution.
- Implemented: each identity contact now has an establishment note and a completed-path
  reflection after the final Mirexis operation.
- Implemented: acknowledging either identity note archives it in the campaign story ledger,
  including the completed-path reflection.
- Implemented: the ending dossier includes an authored, readiness-aware voice variation from the
  identity contact for each path.
- Implemented: the ending dossier adds an engine register assembled from the chosen relationship,
  Escalation response, highest faction pressure, and resolved protective events.
- Implemented: each identity path unlocks a third, character-specific post-ending scene after its
  final reflection is acknowledged.
- Implemented: path-specific finale field notes now pay off every starting colonist and the
  recruited route outsider, while Kira's player-avatar culmination appears in the ending register.
- Implemented: acknowledged field notes are retained as chronological transcripts in a
  touch-visible Operations archive, so the colony's story history can be revisited after dialogue.

Exit test: players recall three colonists, can describe one changing relationship, and
notice the colony responding to an operation without opening a ledger.

### Slice 2 — Training has teeth

Goal: turn retained skill fields and class slots into meaningful builds.

- Implement and balance the first two techniques for all seven base classes as a vertical slice.
- Add skill preview, loadout, learning, save migration, and touch controls.
- Tune the fourteen-technique interactions before adding mastery branches.
- Balance the eight hybrid class actions against the fourteen base techniques before
  adding mastery branches or other class IDs.

Exit test: two characters in the same class can play differently, and retraining creates
a useful hybrid without creating an obviously optimal skill pile.

### Slice 3 — Strangers at the gate

Goal: make Contact change who can belong to the colony.

- Implemented: add the Waystation as a route-gated, power-aware colony facility.
- Implemented: recruit Veya Orn as a route-exclusive Directorate Exile with a three-beat
  arc, persistent origin, and the Exile Cipher action.
- Implemented: give Sedge an independent three-beat Adaptation arc with resource choices,
  Brood attention, relationship progression, disagreements, and a closing route legacy.
- Implemented: add Sedge during Adaptation through the Waystation, with a biomass cost,
  bespoke art, a combined tactical action, Brood attention reaction, and ending voice.
- Implemented: keep the matching unchosen outsider present at the Waystation as a non-roster
  guest with an authored arrival note and recruitment handoff.

Exit test: the outsider is remembered as a person, their origin changes play, and their
presence provokes at least two disagreements that do not reduce to approval points.

### Slice 4 — A colony chooses its shape

Goal: make building and upgrade choices visible in city life and defence.

- Add level-2 branch upgrades to seven existing facilities *(implemented for Command Centre,
  Barracks, Infirmary, Power Plant, Hydroponics, Workshop, and Gene Lab)*.
- Implemented: build a Relay Mast and use its once-per-operation route scan.
- Implemented: build a powered Watchtower whose placement adds stronger directional cover
  to colony defence and loses that benefit when the grid fails.
- Implemented: queue the first level-2 Power Plant branch, complete it after an operation,
  and carry its supply or faction-attention consequence through saves and mission outcomes.
- Implemented: queue a level-2 Hydroponics branch, with Community Kitchen improving food
  and Commons efficiency or Culture Beds growing biomass while the facility is online.
- Implemented: queue a level-2 Workshop branch, with Precision Bench reducing weapon and armour
  fabrication costs or Drone Bay reducing facility repair costs while the Workshop is online.
- Implemented: queue a level-2 Gene Lab branch, with Stabilisation Wing suppressing chosen
  evolution complications or Evolution Chamber reducing mutation evolution biomass costs while
  the Gene Lab is online.
- Implemented: queue a level-2 Command Centre branch, with Signal Cartography revealing a third
  mission route or Counterintelligence Cell reducing mission pressure while the centre is online.
- Implemented: queue a level-2 Barracks branch, with Simulation Hall reducing retraining and
  technique-trial thresholds or Doctrine Yard unlocking Rally formation and training barricades
  while the Barracks is online.
- Implemented: queue a level-2 Infirmary branch, with Trauma Ward shortening recovery, softening
  persistent scar tradeoffs, and adding a stabilization barricade, or Adaptation Clinic making
  mutation complications safer and direct treatment cheaper while the Infirmary is online.
- Implemented: build a Research Annex as a physical evidence archive whose powered operation
  reduces doctrine research costs without introducing a second research currency.
- Implemented: build a Salvage Yard that turns victorious-operation recovery into a disclosed
  materials, research-insight, or standard-equipment-prototype choice.
- Implemented: establish one physical identity building for the chosen Mirexis path, with
  path-specific map cover, objective priority, power, and biomass consequences.
- Implemented: move the associated colony voice to each identity building, add a persistent
  path-specific field note, and expose powered/offline/damaged inspection copy and ambient cues.
- Implemented: expand the identity field notes into persistent multi-scene civic arcs that connect
  power failure, damage, repair, and restoration to the associated NPC scene.
- Implemented: add touch-accessible pre-finale identity preparation actions that trade the path
  resource for linked faction-pressure relief and a path-specific contact reflection.

Exit test: screenshots of two campaigns communicate different civic choices, and those
differences alter both preparation and a colony assault.

### Slice 5 — The heart below

Goal: make the implemented five-phase foundation narratively complete.

- Implemented: expand each Contact route with a persistent witness, contradiction, and
  aftermath field note keyed to the selected protocol and campaign completion flags.
- Implemented: add recurring evidence across Adaptation and Escalation through multiple active
  colony voices instead of a finale-only lore dump.
- Implemented: pay off all five starting arcs and the recruited route-exclusive outsider through
  path-specific finale field notes and the ending register.
- Implemented: add one repeatable, path-gated post-campaign operation for each engine future,
  using dedicated map variants and established objective verbs as an epilogue sandbox.
- Implemented: carry each engine relationship into those finale and epilogue battles as a
  distinct tactical effect—armour, regeneration, or movement guidance—without treating it as
  another faction-pressure penalty.
- Implemented: record successful epilogue work in the campaign save, surface the count in the
  ending register, and unlock a path-specific identity reflection after the first continuing
  operation.
- Implemented: add one post-campaign stewardship action per identity building, with a saved
  per-operation marker, disclosed resource cost, linked faction-pressure reduction, and a
  path-specific contact reflection.
- Implemented: stage each finale around a relationship with the refuge engine through distinct
  objective contracts—the Redoubt defends the Arsenal core, the Commonwealth secures and
  holds its answering chorus, and the Threshold binds and clears the open route.
- Implemented: add a concise finale dossier assembled from character, facility, faction, and
  witnessed-mercy flags.

Exit test: players can explain what Mirexis is, why each power wanted it, what their
colony chose, and what happened to the people they cared about.

## 11. Target content envelope

For a content-complete campaign, aim for a controlled set rather than an open-ended pool:

- 8–10 authored named characters across the full pool, with 7–8 recruitable in one run;
- 5 playable origins, with some mutually exclusive by campaign route;
- 7 base classes and 8 advanced classes;
- 14 base techniques, 7 later mastery choices, and one signature per advanced class;
- 7 upgraded core facilities, 4–6 general buildings, 3 identity buildings, and 5 defence
  structures;
- 3–4 authored operations per campaign phase plus generated pressure operations;
- 3 major endings with character- and colony-specific epilogue variations.

These are production ceilings for the first content-complete release, not quotas. Cut a
thin class, recruit, or building rather than ship content without story reaction,
tactical purpose, and colony consequence.

## 12. Immediate decisions before implementation

Resolve these questions in small prototypes or playtests:

1. Can a tactical skill loadout remain readable with mutation and equipment actions on
   the same touch-first command surface?
2. Should technique learning use class-specific experience, authored challenges, or a
   simpler operation-count gate?
3. Can temporary player-created cover fit deterministic pathing, AI, saves, previews,
   and replay without excessive exception handling?
4. Does the next Severed Brood recruit justify a unique rig, or should that recruit use
   a smaller support body with deliberately limited equipment presentation?
5. How many facility branches can the colony UI expose before the city begins feeling
   like a management menu?
6. Which earlier campaign flags are meaningful enough to acknowledge in the finale
   without producing brittle combinatorial dialogue?

Until those questions are answered, narrative writing, skill data, building schemas,
and final art should remain scoped to the next vertical slice rather than built in bulk.
