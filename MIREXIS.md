# MIREXIS  
## Game Design Document

## 1. High Concept

**Mirexis** is a turn-based grid tactics game set on a remote human colony trapped between three vastly more powerful factions.

The player commands a growing roster of named colonists, defectors, scientists, soldiers, scavengers, and mutants while attempting to keep their settlement alive.

Every recruit has:

- unique strengths and weaknesses
- preferred class paths
- a defining mutation
- personal relationships and history
- freedom to train into unconventional roles

Characters have natural aptitudes, but the player is never forced to follow them.

A heavily mutated researcher can become a frontline tank.  
A naturally gifted heavy soldier can be trained as a psionic specialist.  
A medic with unusual sensory mutations might become the colony's best sniper.

Between tactical missions, the player expands and protects a persistent colony that provides training, research, equipment, medical care, and defensive infrastructure.

The player is not building an empire.

They are trying to keep one settlement alive while much larger powers tear the planet apart around them.

---

# 2. Genre

- Turn-based grid tactics
- Character recruitment and progression
- Light colony management
- Tactical RPG
- Science-fiction survival

### Primary Inspirations

- Fire Emblem
- XCOM
- Final Fantasy Tactics
- Battle Brothers
- StarCraft, primarily for faction contrast rather than gameplay

---

# 3. Design Pillars

## 3.1 Characters Before Units

Every combatant in the player's core roster is a named character.

Characters should become memorable because of:

- personality
- relationships
- mutation
- combat role
- unusual builds
- scars and injuries
- story events

The player should think:

> "This is Kira."

rather than:

> "This is my level 14 sniper."

---

## 3.2 Flexible Character Builds

Characters have natural aptitudes rather than hard class restrictions.

A character might naturally excel at:

- heavy weapons
- medicine
- biotechnology
- psionics
- mobility
- melee combat
- engineering

Following those strengths creates an efficient build.

Ignoring them remains completely valid.

Unusual character builds should sometimes produce surprising combinations that are stronger or stranger than the obvious choice.

---

## 3.3 Mutations Change How Characters Play

Every recruit possesses one major mutation.

Mutations are not primarily statistical bonuses.

They should alter:

- positioning
- movement
- survivability
- resource management
- ability interactions
- targeting decisions

Each mutation should generally contain:

**Gift**

A clear advantage.

**Complication**

A drawback, limitation, or unusual interaction.

**Evolution**

Later choices that allow the mutation to develop in different directions.

---

## 3.4 The Colony Matters

The colony is the player's home between missions.

Buildings unlock mechanics, improve characters, provide resources, and prepare the settlement for future attacks.

Major colony structures physically appear during defensive missions.

The player therefore gradually constructs the battlefield they may later have to defend.

---

## 3.5 The Player Is the Underdog

The colony is not a fourth galactic superpower.

Enemy factions possess:

- larger armies
- superior logistics
- advanced technology
- specialised forces
- orbital assets

The player survives through:

- scavenging
- adaptation
- diplomacy
- unconventional tactics
- stolen technology
- mutation
- exploiting conflicts between enemies

---

# 4. Setting

## Mirexis

Mirexis is a remote colony world originally settled for resource extraction and scientific research.

For years, it was considered strategically insignificant.

That changed when large-scale biological mutations began appearing among native organisms and colonists.

At roughly the same time, buried structures and technologies of unknown origin were discovered beneath the surface.

Multiple powers now consider Mirexis valuable.

The colony is caught directly between them.

The player controls one isolated settlement that has largely been abandoned by the institutions that founded it.

---

# 5. The Three Major Factions

The exact names are placeholders and should receive their own naming pass.

## 5.1 The Directorate

A technologically advanced human military and corporate power.

### Identity

- disciplined
- industrial
- heavily armed
- technologically sophisticated
- willing to sacrifice colonies for strategic objectives

### Combat Style

- ranged firepower
- drones
- suppression
- powered armour
- artillery
- battlefield electronics

Directorate units now open a viable activation with **Suppression Lock**, spending one
action point to Disrupt the most vulnerable visible colonist for their next player phase.

Directorate battlefields may contain marked **Fire Lanes**. A non-Directorate unit that
ends movement in one takes two direct damage; Directorate troops know the firing schedule
and cross safely.

### Technology

- rail weapons
- combat drones
- cybernetics
- shield generators
- automated turrets
- powered armour

### Relationship With The Colony

The Directorate may technically claim ownership of the colony.

Its commanders view colonists primarily as assets, liabilities, or witnesses.

---

## 5.2 The Brood

An aggressively adaptive biological ecosystem.

It is unclear whether the Brood is:

- a species
- a collective intelligence
- a biological weapon
- an emergent planetary organism

Different Brood creatures evolve for specialised roles.

### Combat Style

- overwhelming numbers
- rapid movement
- ambushes
- biological artillery
- regeneration
- adaptive mutations

Brood units now open an activation with **Predatory Surge**, spending one action point to
become Quickened while they close on the squad or a vulnerable mission asset.

### Battlefield Characteristics

Brood forces may alter terrain by spreading:

- biomass
- spores
- corrosive growth
- spawning structures

Authored Brood maps now place visible **Spore Blooms**. A non-Brood unit that ends
movement in one takes one damage and becomes Hindered through its next activation. Brood
organisms are adapted to the bloom and ignore it.

### Relationship With The Colony

The mutations appearing among colonists appear to be connected to Brood biology.

Whether this connection is infection, adaptation, communication, or something else remains unclear.

---

## 5.3 The Ascendants

An ancient or highly advanced alien civilisation.

Their technology operates according to principles barely understood by human science.

### Combat Style

- teleportation
- shields
- energy weapons
- gravity manipulation
- psionic abilities
- battlefield control

Ascendant units now open an activation with **Phase Ward**, spending one action point for
Guarded armour that persists through the colony's response.

Ascendant battlefields may fracture into marked **Static Rifts**. A non-Ascendant unit
that ends movement in one becomes Disrupted through its next activation, while Ascendant
constructs remain phase-aligned and unaffected.

### Technology

- phase technology
- energy constructs
- dimensional manipulation
- autonomous relics
- psionic amplification

### Relationship With The Colony

The Ascendants appear interested in something beneath Mirexis.

The colony may unknowingly have been built directly above an important site.

---

# 6. Player Faction

The player's faction is simply the colony itself.

It begins with:

- civilians
- a small security force
- improvised equipment
- limited medical facilities
- basic industrial capability

Over time the colony develops into a hybrid society using technology scavenged from all three factions.

Late-game colonies may contain combinations such as:

- Directorate drones
- Brood-grown defensive walls
- Ascendant shield technology
- heavily mutated colonists
- improvised human machinery

The visual identity of the colony should reflect the player's technological choices.

---

# 7. Core Game Loop

The primary campaign loop is:

1. Return to colony.
2. Talk to characters and resolve events.
3. Manage construction, research, equipment, and recovery.
4. Select a mission.
5. Choose squad.
6. Deploy into tactical battle.
7. Complete objectives.
8. Recover resources, equipment, recruits, or information.
9. Deal with injuries and consequences.
10. Advance enemy activity.
11. Prepare for the next crisis.

Occasionally the normal loop is interrupted by:

- enemy attacks
- colony emergencies
- mutation outbreaks
- political events
- character conflicts
- faction ultimatums

---

# 8. Tactical Combat

## Battlefield

Combat takes place on a square grid.

Maps may contain:

- elevation
- buildings
- destructible cover
- hazards
- environmental effects
- alien structures
- biological growth
- civilians
- interactive machinery

---

## Turn Structure

Combat is turn-based.

The preferred initial model is alternating player and enemy phases.

During the colony phase, selecting a hostile reveals its current vitals, weapon profile,
faction ability, and deterministic first-action forecast. An orange line marks the
predicted attack target or next movement step without changing the active colonist, so
enemy pressure can be read before committing to end the phase.

Hovering a reachable tile—or focusing it with keyboard navigation—previews its validated
movement AP and any landing hazard consequence. An attackable hostile instead previews
the exact hit chance, normal-to-critical damage range, and weapon AP before the attack is
committed.

A reachable movement destination also traces the exact validated route in cyan. Each
entered tile labels its own terrain cost, making blocked-cell detours and expensive ground
visible alongside the existing total AP preview before the move is committed.

Directional cover is drawn as a reinforced cyan edge on the protected side of its tile.
The nearby number names the accuracy penalty imposed from that facing, so stronger cover
and exposed flanks can be read directly from battlefield geometry before units exchange fire.

The tactical footer also opens a field manual that pauses command input and summarizes
selection, action points, previews, enemy intent, hazard symbols, objective reading, and
keyboard controls. It can be closed through its return button, `H`, or Escape.

The tactical footer's battle log opens a command-pausing history of the fifteen most
recent ordered simulation events. It lets players reconstruct dense hostile phases—moves,
attacks, damage, statuses, faction abilities, hazards, and phase transitions—rather than
relying only on the latest-event line.

Mission briefing now includes threat intelligence derived from the materialized operation:
objective contract and deadline, deployed hostile count and roles, the primary faction
ability, and known hazard types/counts. This information sits beside recovery rewards so
squad selection can answer the mission actually being deployed.

Every mission offer also carries a deterministic danger band, while its briefing names
the band and exact score. Hostile strength, multiple enemy powers, reinforcement-bearing
contracts, short deadlines, battlefield hazards, and escalation modifiers all raise the
same shared rating, so risk can be compared before committing to deployment.

Holdout and signal-trace briefings forecast reinforcement rounds, likely roles, and their
east-edge approach. Once deployed, the objective panel reads the serialized wave queue and
keeps the next arrival visible until it enters, so surviving a timed contract involves
planning for the second line rather than discovering it after ending a phase.

During the round immediately before arrival, each queued unit's preferred insertion tile
is outlined in orange and names the arrival round. These markers sit beneath battlefield
units, remain read-only, and disappear when the wave deploys or its warning window passes.

New damage, healing, and status events also produce short-lived callouts over the affected
battlefield unit. Red loss, green recovery, and amber status labels remain visible long
enough to read after a fast hostile phase, while a bounded presentation queue prevents old
events from accumulating or changing deterministic combat and save state.

Ending the colony phase still resolves hostile simulation atomically, but important enemy
events then play through a short presentation queue. Movement, attacks, reactions, faction
abilities, reinforcements, incapacitations, and the phase handoff each receive a readable
beat while commands are paused; Space or Escape skips the remaining playback immediately.

Before deployment, the squad can cycle among wedge, line, and column entry formations.
The chosen shape is applied only when a new tactical session is created. Any preferred cell
that is blocked, hazardous, occupied, reserved for the objective, or outside the map falls
back deterministically to a safe western entry cell; loaded battles retain realized positions.

Ending a colony phase while active colonists retain action points now arms a readiness
warning instead of immediately yielding to hostiles. The button reports how many colonists
are still ready and requires a second End Phase command; any intervening tactical action
cancels the warning, while a fully spent squad hands off without an unnecessary confirmation.

The tactical footer and `Tab` key select the next non-incapacitated colonist with unspent
action points, wrapping through deployment order and moving grid focus to their current tile.
Fully spent squad members are skipped, and invoking the control with nobody ready leaves the
current selection unchanged rather than inventing a command target.

Every active colonist token also carries a row of action-point pips. Filled pips show the
actions that remain without requiring selection, while a colonist at zero action points is
labelled `SPENT`; incapacitated units omit the readiness treatment because their existing
state already explains why they cannot act.

Every colony and hostile token carries a compact vitality bar whose colour shifts from
green through amber to red as health falls. Combatants with armour also show an `A#` label
for their current effective protection, including temporary and status-derived bonuses, so
attrition and hard targets can be read across the grid without changing selection.

Each briefing roster row also selects that colonist for a derived loadout summary beside
the threat report. Mutation, primary weapon, health, accuracy, armour, movement, damage,
range, weapon AP, and lasting scars can be compared while toggling the three-person squad.

### Player Phase

The player activates characters in any order.

Characters can generally:

- move
- attack
- use abilities
- interact
- use items
- defend
- perform class actions

The implemented Overwatch action spends the equipped weapon's attack-point cost during
the player phase. It then fires once at the first hostile movement that enters range and
line of fire during the enemy phase, with a 15-point reaction accuracy penalty. The shot
is prepaid, deterministic, visibly armed on the battlefield, and expires if it is not
triggered before the next player phase. This makes weapon profiles and firing lanes matter
even when the player chooses not to attack immediately.

Some abilities may consume both movement and action.

---

# 9. Tactical Objectives

Battles should regularly involve objectives beyond eliminating every enemy.

Examples include:

- rescue civilians
- hold a defensive position
- destroy a biological nest
- protect a vulnerable structure
- steal technology
- capture an enemy officer
- escort researchers
- activate machinery
- escape before reinforcements arrive
- survive for a fixed number of turns

- defend a convoy
- sabotage artillery
- retrieve mutation samples
- protect colony infrastructure
- assassinate a commander
- recover a stranded colonist
- seal a breach

The implemented **Thin Shelter** operation combines structure protection with a survival
deadline. A refuge transmitter begins with 12 integrity and must survive five rounds.
Hostile AI paths toward the transmitter and attacks it whenever it has range and line of
fire; destruction fails the operation immediately. Its integrity, deadline, and map marker
remain visible, while overwatch lets the squad turn the attackers' objective-focused
movement against them.

---

# 10. Characters

Every major recruit has:

- name
- visual design
- personality
- background
- mutation
- combat aptitudes
- relationships
- preferred classes
- personal events

---

# 11. Character Aptitudes

Characters possess aptitude ratings for different disciplines.

Possible disciplines include:

- Ballistics
- Heavy Weapons
- Melee
- Mobility
- Engineering
- Medicine
- Biology
- Psionics
- Defence
- Leadership

Aptitudes influence progression efficiency rather than preventing access.

Example:

## Mara Venn

Former mining supervisor.

### Aptitudes

Heavy Weapons: ★★★★★  
Defence: ★★★★  
Engineering: ★★★  
Medicine: ★★  
Psionics: ★

Recommended development:

Heavy Gunner → Bulwark

The player could instead train Mara as:

Medic → Xenobiologist → Psionic Surgeon

She will require more investment and may develop weaker core statistics, but the build remains possible.

---

# 12. Class System

Classes represent training rather than biology.

The playable class tree begins with seven broad disciplines. After reaching level 3,
entering Adaptation, and mastering both prerequisite disciplines, a colonist can train
one of four hybrid advanced classes:

- Soldier + Defender → Vanguard: projects Guarded status across a nearby formation.
- Scout + Engineer → Pathfinder: combines action economy, mobility, and focused fire.
- Medic + Biotech Specialist → Lifewright: heals an ally and seeds regeneration.
- Psionic + Engineer → Null Adept: damages and disrupts a hostile at extended range.

Advanced training costs more materials than base retraining. Class history persists,
so experimentation builds toward promotion instead of being discarded.

Initial broad families could include:

## Soldier

General firearms specialist.

Possible paths:

- Rifleman
- Marksman
- Assault
- Heavy Gunner

---

## Defender

Durability and battlefield control.

Possible paths:

- Bulwark
- Shield Specialist
- Vanguard
- Siege Walker

---

## Scout

Mobility and reconnaissance.

Possible paths:

- Infiltrator
- Ranger
- Saboteur
- Pathfinder

---

## Medic

Healing and support.

Possible paths:

- Field Medic
- Surgeon
- Combat Medic
- Xenobiologist

---

## Engineer

Technology and battlefield devices.

Possible paths:

- Drone Controller
- Mechanic
- Demolitionist
- Systems Specialist

---

## Psionic

Mental and anomalous abilities.

Possible paths:

- Telepath
- Kinetic
- Warden
- Rift Specialist

---

## Biotech Specialist

Mutation-focused combat and support.

Possible paths:

- Gene Weaver
- Spore Specialist
- Symbiote Handler
- Biomancer

---

# 13. Class Switching

Characters are not permanently locked into a single class.

Retraining requires colony infrastructure and time.

Skills learned from previous classes may sometimes remain available.

This allows hybrid builds such as:

- Defender / Psionic
- Scout / Medic
- Engineer / Heavy Gunner
- Biotech / Marksman

The system should reward experimentation without allowing every character to eventually acquire every skill.

---

# 14. Mutations

Every recruit possesses one major mutation.

Mutations should frequently create unusual interactions with classes.

---

## Example: Chitinous Growth

### Gift

Gain natural armour.

### Complication

Reduced base mobility.

### Possible Evolutions

**Reactive Carapace**

Gain additional defence after receiving damage.

**Living Fortress**

Large armour increase but further movement penalty.

---

## Example: Neural Bloom

### Gift

Character can learn limited psionic abilities regardless of class.

### Complication

More vulnerable to psychic disruption.

### Possible Evolutions

**Expanded Cortex**

Improved psionic range.

**Echo Mind**

Certain psionic abilities trigger twice at reduced strength.

---

## Example: Regenerative Tissue

### Gift

Recover health each turn.

### Complication

Conventional medical healing is less effective.

### Possible Evolutions

**Controlled Regeneration**

Reduces the healing penalty.

**Cancerous Bloom**

Regeneration dramatically increases when badly wounded but introduces additional risks.

---

## Example: Elastic Musculature

### Gift

Improved vaulting and movement.

### Complication

Reduced heavy armour efficiency.

### Possible Evolutions

**Coiled Muscle**

Gain bonus movement after melee attacks.

**Predatory Motion**

Move through enemy threat zones with reduced penalties.

---

## Example: Symbiotic Organism

### Gift

Gain an additional biological attack.

### Complication

The symbiote consumes additional food or biological resources.

### Possible Evolutions

**Cooperative Symbiote**

Provides defensive support.

**Predatory Symbiote**

Becomes a powerful offensive weapon.

---

## Example: Phase Mutation

### Gift

Can move through certain obstacles.

### Complication

Greater vulnerability to Ascendant technology.

### Possible Evolutions

**Phase Step**

Short-range teleportation.

**Unstable Existence**

Chance for enemy attacks to pass harmlessly through the character.

---

# 15. Colony

The colony functions as the player's strategic hub.

It should remain intentionally lighter than a full colony simulator.

The player makes meaningful decisions about:

- what to construct
- what to repair
- what to research
- who receives scarce resources
- how the settlement is defended

---

# 16. Colony Resources

Potential resources include:

## Materials

Used for:

- construction
- repair
- basic equipment

## Power

Required by advanced structures.

## Food

Supports population and certain biological technologies.

## Biomass

Collected from mutated creatures.

Used for:

- mutation research
- biological structures
- biotech equipment

## Alien Components

Rare Ascendant material used for advanced research.

---

# 17. Colony Buildings

## Command Centre

Provides:

- mission intelligence
- squad management
- strategic upgrades

---

## Barracks

Provides:

- class training
- retraining
- combat preparation

---

## Gene Lab

Provides:

- mutation research
- mutation stabilisation
- mutation evolution
- biological equipment

---

## Workshop

Provides:

- weapons
- armour
- cybernetics
- equipment modification

---

## Infirmary

Provides:

- wound recovery
- injury treatment
- medical upgrades

---

## Hydroponics

Provides:

- food
- population support
- biological resources

---

## Power Plant

Provides colony power.

Different power technologies may eventually become available.

---

## Defensive Structures

Examples:

- barricades
- automated turrets
- watchtowers
- shield generators
- minefields
- bunkers
- biological walls

---

## Research Facility

Allows investigation of:

- enemy technology
- mutations
- alien ruins
- captured equipment

---

# 18. Colony Layout

Buildings occupy physical positions within the settlement.

This layout matters during defence missions.

For example:

A watchtower constructed beside the western wall becomes usable high ground when enemies attack.

A poorly protected gene laboratory may become a vulnerable mission objective.

A shield generator creates a protected battlefield zone.

Players therefore indirectly design future defence maps through colony construction.

---

# 19. Colony Attacks

Major enemy attacks are telegraphed.

Example:

**DIRECTORATE ASSAULT EXPECTED**

Arrival in:

**3 Operations**

The player chooses how to prepare.

Possible responses include:

- raid enemy supplies
- assassinate an officer
- gather building materials
- rescue reinforcements
- build defensive structures
- research weapons

Successful preparation missions weaken the attack.

Ignoring the threat allows the invading force to arrive at full strength.

---

# 20. Enemy Pressure

Each major faction has an activity or threat level.

Player actions change these values.

Examples:

Repeatedly stealing Directorate technology increases Directorate hostility.

Heavy use of biological experimentation attracts the Brood.

Activating alien ruins increases Ascendant attention.

Players cannot keep every faction passive.

---

# 21. Recruitment

New characters can be recruited through:

- story missions
- rescue operations
- colony growth
- faction defectors
- exploration
- prisoner recruitment
- random events

Recruitment should regularly force interesting choices.

Example:

A mission reveals two trapped groups.

One contains an experienced Directorate engineer.

The other contains three civilians.

The player may not have enough time to save both.

---

# 22. Character Events

Characters interact with each other around the colony.

Events may involve:

- friendships
- rivalries
- romances
- ideological disagreements
- mutation fears
- loyalty
- trauma
- faction history

The implemented pair-bond layer grows through shared victories and character events.
Roster inspection names each known bond, mission briefing identifies deployed trusted
partners, and debrief reports when a victory strengthens the squad's history.

At bond 3, a deployed pair becomes **Trusted** and each partner gains +2 accuracy. At
bond 5, the pair becomes **Bonded** and each partner also gains +1 armour. Only a
character's strongest deployed bond applies, so relationship bonuses remain small and
cannot stack across a full squad. Future authored events can turn this foundation into
rivalries, romances, ideological fractures, and reconciliation stories.

---

# 23. Injuries and Death

Character survival should matter.

Potential system:

Characters reduced to zero health become incapacitated rather than immediately dying.

Further damage, failed evacuation, certain enemies, or mission failure can cause permanent death.

Characters now receive a temporary recovery injury and, for their first two
incapacitations, a permanent tradeoff scar. Clouded Eye exchanges accuracy for damage,
Reinforced Ribs exchanges movement for armour, and Mire Reflex exchanges vitality for
movement. These scars remain after infirmary recovery, alter later deployments, and are
listed in roster history. The two-scar cap keeps a favourite colonist playable while still
letting battlefield failure rewrite that character.

Examples:

- damaged eye
- cybernetic replacement arm
- neurological damage
- mutation instability
- permanent scarring

The implemented scars deliberately include an unusual opportunity rather than acting as
pure penalties. Further authored injuries can build on this foundation.

---

# 24. Equipment

Characters can equip:

- primary weapon
- secondary weapon or tool
- armour
- accessory/module

Equipment families may include:

The current workshop exposes every item's exact tactical function before crafting.
Primary weapons can override handling rather than only modifying statistics: the Breach
Scattergun compresses range for armour-breaking damage, while the Needle Carbine trades
damage and accuracy for one-action-point repeated fire. These join the balanced Frontier
Rifle, heavy Mire LMG, and unmodified Service Pistol as the first complete weapon family.

## Human

Reliable and adaptable.

## Directorate

Advanced technological equipment.

## Brood

Living weapons and biological armour.

## Ascendant

Rare, powerful, difficult to understand.

Hybrid equipment should become possible later.

Example:

**Living Rail Rifle**

A Directorate weapon rebuilt using biological components that regenerates ammunition.

---

# 25. Research

Research is driven partly by what the player encounters.

Capturing or recovering enemy technology unlocks new research possibilities.

Research categories:

- Military
- Engineering
- Biotechnology
- Mutation
- Psionics
- Ascendant Technology
- Colony Infrastructure

The player should not progress through a single linear research tree.

Campaign choices determine what technology becomes available.

---

# 26. Campaign Structure

The campaign is divided into escalating phases.

## Phase One: Isolation

The colony loses outside support.

The player establishes basic defences and builds the first squad.

---

## Phase Two: Contact

The major factions begin appearing.

The player learns that Mirexis is far more important than expected.

---

## Phase Three: Adaptation

Mutations become increasingly significant.

Enemy technology becomes available for scavenging.

The colony begins transforming.

---

## Phase Four: Escalation

The three powers openly contest the region.

The colony becomes strategically important.

Enemy assaults increase.

---

## Phase Five: Mirexis

The truth behind:

- the mutations
- the Brood
- the Ascendants
- the planet

is revealed.

The player must decide what the colony will become.

---

# 27. Campaign Decisions

The campaign should contain meaningful decisions rather than a simple good/evil system.

Examples:

- Accept protection from the Directorate?
- Experiment on dangerous mutations?
- Allow Brood tissue inside the colony?
- Activate unknown Ascendant machinery?
- Give scarce medicine to soldiers or civilians?
- Shelter enemy defectors?
- Destroy dangerous technology or use it?

Choices should change:

- recruits
- missions
- resources
- colony appearance
- relationships
- faction hostility
- available technologies

---

# 28. Late-Game Colony Identity

By the end of the campaign, different colonies may look radically different.

## Technology-Oriented Colony

Uses:

- drones
- automated defence
- cybernetics
- powered armour

---

## Biological Colony

Uses:

- living walls
- mutation enhancement
- regenerative structures
- biological weapons

---

## Ascendant Colony

Uses:

- shields
- teleportation
- gravity technology
- psionic systems

---

## Hybrid Colony

Combines all three.

Potentially extremely powerful.

Potentially extremely unstable.

---

# 29. Art Direction

Mirexis should visually combine:

- frontier science fiction
- industrial colony infrastructure
- advanced military technology
- disturbing biotechnology
- strange alien structures

The colony begins grounded and recognisably human.

As the campaign progresses, its appearance becomes increasingly unusual.

A late-game settlement might contain:

- rusting prefabricated housing
- glowing alien machinery
- biological walls
- hovering drones
- mutated trees
- improvised barricades

The aesthetic should communicate that everything has been rebuilt using whatever the colonists could steal, grow, repair, or barely understand.

---

# 30. Tone

The setting is dangerous but not relentlessly miserable.

Character interactions should create warmth, humour, rivalry, and everyday life inside an increasingly bizarre colony.

The contrast is important.

A character may spend one mission fighting genetically engineered horrors and return home to argue with someone about who keeps stealing food from hydroponics.

The colony should feel worth protecting.

---

# 31. Player Fantasy

The central fantasy is not:

> Become the greatest commander in the galaxy.

It is:

> Take a group of ordinary and increasingly extraordinary people, turn them into a bizarre tactical squad, and somehow keep their home alive against forces that should completely overwhelm them.

---

# 32. Current Core Feature Set

The current design is built around five interconnected systems:

1. **Turn-based grid combat**
2. **Named recruitable characters**
3. **Flexible class progression**
4. **Character-defining mutations**
5. **Persistent colony development and defence**

Everything added to Mirexis should strengthen at least one of these systems or create meaningful interactions between them.

---

# 33. Immediate Design Work

The next systems that require detailed design are:

### Class Framework
Define the base classes, advanced classes, hybridisation rules, and class progression.

### Mutation Framework
Create a larger mutation library and determine mutation evolution rules.

### Tactical Rules
Define:

- movement
- attack ranges
- cover
- elevation
- turn economy
- accuracy
- critical hits
- armour
- status effects

### Squad Structure
Determine:

- deployment size
- reserve characters
- experience distribution
- character death rules

### Colony Progression
Define:

- building slots or free placement
- construction costs
- resource generation
- attack frequency
- damage and rebuilding

### Campaign Structure
Determine how missions, faction pressure, story events, and colony attacks are generated and connected.
