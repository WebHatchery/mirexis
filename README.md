# Mirexis

Mirexis is a Rust + Macroquad turn-based tactics game about a persistent frontier
colony caught between the Directorate, the Brood, and the Ascendants. The complete
game concept is in [MIREXIS.md](MIREXIS.md); current architecture and continuation
contracts are in [TECHNICAL_DESIGN.md](TECHNICAL_DESIGN.md).

## Current Slice: Complete Identity-Branched Campaign

The completed roadmap slice includes:

- a title → colony → briefing → tactical → debrief campaign loop
- deterministic weighted movement, attacks, cover, armour, objectives, and enemy AI
- prepaid overwatch that reacts once when hostile movement enters weapon range and line of fire
- obstacle-blocked firing lanes and once-per-round tactical mutation gifts
- secure-and-clear, elimination, holdout, extraction, activate-then-defend signal trace,
  and destructible-asset defense objectives with live progress
- faction-specific battlefields and Brood, Directorate, or Ascendant enemy squads
- eighteen generated operation templates, including a Brood sporefield extraction, an
  Ascendant fractured-vault purge, three protocol-gated Contact signal traces, and a
  phase-gated Adaptation scavenging operation, plus Thin Shelter's vulnerable transmitter
- save-stable authored and vertically mirrored variants for every mission map family
- eleven class actions, explicit ally/enemy targeting, and combat statuses
- role-aware hostile activations and deterministic holdout reinforcements
- once-per-phase faction abilities: Brood Predatory Surge, Directorate Suppression Lock,
  and Ascendant Phase Ward, each surfaced through status markers and the battle log
- visible faction terrain hazards across nine battlefields: damaging Directorate fire
  lanes, hindering Brood spore blooms, and accuracy-disrupting Ascendant static rifts
- click-to-inspect hostile intent previews that expose enemy vitals, weapon profiles,
  faction abilities, likely targets, and the predicted first move or attack on the grid
- player action previews for validated movement AP, landing-hazard consequences, attack
  hit chance, normal/critical damage range, and weapon AP before commands are committed
- cyan movement-route previews that trace the validated detour around blockers and label
  every entered tile with its individual terrain AP cost before relocation
- red invalid-command previews that explain obstructed shots, missing paths, occupied
  destinations, range failures, wrong phases, and insufficient AP on the focused tile
- colony-green valid-shot tracers plus an explicit directional-cover penalty beside the
  final hit chance, distinct from orange hostile-intent forecasting
- translucent red weapon-range projection for an inspected hostile, clipped by authored
  weapon range and current line-of-fire blockers to reveal safe tiles and cover shadows
- pink corner threat marks for tiles an inspected hostile can reach with one legal move
  and still attack, with an in-panel legend separating immediate from next-step danger
- a command-blocking tactical field manual available from the footer, `H`, or Escape,
  covering the turn loop, selection, previews, hostile intent, hazards, and objectives
- an expandable recent battle-history panel available from `LOG` or `B`, preserving the
  ordered movement, attacks, damage, statuses, abilities, hazards, and phase transitions
- materialized mission-briefing intelligence for contract/deadline, hostile role
  composition, faction ability, and known hazard counts before squad deployment
- briefing-side selected-colonist loadout inspection for derived mutation/weapon stats,
  armour, movement, action economy, and scars, with unobstructed five-person roster rows
- deterministic operation danger ratings on mission offers and threat briefings, derived
  from hostiles, mixed powers, reinforcement contracts, deadlines, hazards, and modifiers
- reinforcement forecasts that disclose arrival rounds, likely roles, and east-edge entry
  pressure in both deployment intelligence and the live tactical objective panel
- orange on-grid insertion markers during the round before a queued hostile wave arrives,
  pairing exact preferred tiles with the live round warning
- short-lived on-unit combat callouts for damage, healing, and newly applied statuses,
  derived from ordered battle events without entering tactical saves
- a bounded, skippable hostile-activity replay that pauses tactical input and presents
  important enemy movements, attacks, abilities, reinforcements, and phase handoff in beats
- briefing-selectable wedge, line, and column deployment formations with deterministic
  fallback around blocked, hazardous, occupied, objective, or out-of-bounds entry cells
- a two-step end-phase readiness guard when active colonists retain action points, cancelled
  by any intervening tactical command and bypassed when nobody can act
- next-ready-colonist cycling from the tactical footer or `Tab`, skipping incapacitated or
  fully spent squad members and synchronizing both selected unit and grid focus
- on-grid action-point pips for every active colonist, with an explicit `SPENT` marker
  when that unit can no longer act during the current colony phase
- compact on-grid vitality bars for every combatant plus `A#` effective-armour labels,
  exposing damage and defensive strength without changing the current selection
- directional cyan cover edges with visible protection strength, making defended facings
  and open flanks readable from the tactical grid before attacks are exchanged
- persistent three-colonist squad selection with a visible reserve
- persistent pair relationships that grow through shared victories and character
  events, surface in roster and briefing screens, and grant bounded trusted-squad bonuses
- roster-wide class training, four level/phase/mastery-gated advanced disciplines,
  and slot-replacing workshop equipment
- inspectable equipment effects plus Breach Scattergun and Needle Carbine profiles that
  alter tactical weapon range, damage, accuracy, and attack-point economy
- player-targeted medkit, toolkit, and survey-harness actions in tactical missions
- destructible battlefield cover that opens movement routes and firing lanes
- five persistent recruits with aptitudes, classes, mutations, equipment, XP, injury,
  and operation-based recovery
- up to two lasting incapacitation scars per colonist, each pairing a permanent benefit
  with a permanent cost and remaining visible in roster history
- a reserve Biotech Specialist, Nadi Vale, whose Symbiotic Organism can irreversibly
  become a regenerative Cooperative Symbiote or a volatile Predatory Symbiote
- colony resources, powered facilities, operation rations, Hydroponics production,
  selectable Barricade/Power Plant/Gene Lab construction, layout-derived defenses,
  and repairs
- Directorate, Brood, and Ascendant attention; a telegraphed assault; persistent
  research doctrines and character-event legacies; seeded mission generation; and
  faction-pressure battlefield modifiers with materials, biomass, and power recovery
- a visible Isolation completion contract—three victories, one doctrine, and one
  repelled colony assault—that advances the persistent campaign into Contact
- a mutually exclusive Contact protocol that spends recovered Alien Components on
  Directorate material supply, Brood biomass cultivation, or Ascendant power recovery,
  then unlocks that protocol's own Contact battlefield
- protocol-specific Smartlink, Living Plate, or Phase Lens workshop prototypes that
  unlock permanently after the matching signal trace is won
- protocol-gated Contact aftermath events that alter the contacted faction's attention
  and leave a permanent character legacy on Kira, Mara, or Sol
- a visible Contact completion contract—trace, aftermath, and fielded prototype—that
  advances the persistent campaign into Phase Three: Adaptation
- an irreversible, biomass-funded Neural Bloom evolution for Kira: Expanded Cortex
  trades higher accuracy for food upkeep, while Echo Mind trades weapon damage for
  movement; the powered, damageable Gene Lab owns the choice and the resulting gift
  and complication both affect later deployments
- a second Gene Lab decision for Mara: Fortress Carapace trades mobility for armour,
  while Razor Plating trades deployment food for weapon damage
- a support-focused Regenerative Tissue decision for Ilya: Clean Marrow normalizes
  injury recovery at a damage cost, while Feral Renewal trades food for stronger
  round-by-round regeneration
- a mobility/equipment decision for Sol: Lattice Tendons trades accuracy for movement,
  while Load-Bearing Fascia restores armour efficiency at a weapon-damage cost
- an Adaptation-only Glass Nerve operation generated immediately after evolution,
  with a distinct Ascendant cradle battlefield, a six-round secure-and-clear contract,
  and visible materials, biomass, and power recovery
- a visible Adaptation completion contract—Glass Nerve victory, two evolved colonists,
  and an operational Gene Lab—that advances the persisted campaign into Phase Four:
  Escalation
- an Escalation-only Three Knives holdout on a contested-rift battlefield, where a
  Directorate rifleman, Brood sporecaster, and Ascendant warden deploy together under
  a forced crossfire that makes hostiles faster and more accurate while disrupting
  colonist accuracy
- a post-victory Escalation response: spend materials to delay the next assault, spend
  biomass to misdirect every faction, or spend power to trade higher attention for
  recurring material recovery
- three response-locked follow-up operations: a Directorate elimination siege for
  Bastion Beacon, a Brood extraction corridor for Living Decoy, or an Ascendant signal
  trace for Weaponized Lattice, each on its own battlefield family
- a visible Escalation completion contract—Three Knives victory, one committed
  response, and victory in its matching branch—that advances the persisted campaign
  into Phase Five: Mirexis while keeping the operation loop playable
- an irreversible Phase Five identity choice: Human Redoubt strengthens colony-defense
  cover, Living Commonwealth reduces deployment food, or Open Threshold increases
  power recovered from every later victory
- three identity-locked Phase Five operations: defend the Human Redoubt's Last Wall
  from a Directorate breach, awaken the Living Commonwealth's Root Choir under Brood
  attack, or secure the Open Threshold's Door of Light against Ascendant claimants
- three persistent campaign endings revealed by victory in the matching final operation:
  a sovereign Human Redoubt, an awakened Living Commonwealth, or an Open Threshold
  linked to the mind beneath Mirexis, followed by optional post-campaign operations
- versioned native/WASM saves and event autosaves through macroquad-toolkit
- deterministic visual captures and schema migrations from every prior project version

## Development

```powershell
cargo run
cargo test
cargo fmt -- --check
cargo clippy --all-targets --all-features -- -D warnings
```

The required end-to-end validation and packaging path is:

```powershell
.\publish.ps1
```

Capture all application scenes with:

```powershell
.\scripts\capture_ui.ps1
```

## Controls

- Mouse/touch in the colony: tap terrain to walk, tap a colonist to approach and talk,
  use the visible movement pad, or enter Build mode to construct and repair facilities
- Mouse in tactical missions: select colonists, move, attack, complete objectives, or
  activate mutation and class actions
- W / A / S / D: supplement the colony's visible movement pad
- E: talk to an adjacent colony NPC
- Arrow keys: move the tactical cursor
- Enter: confirm title/briefing/debrief or end the colony tactical phase
- S / L: save / load an in-progress battle
- Escape: step back from briefing/tactical or return to the title

## Data

Embedded content under `assets/data/` covers configuration, the authored mission,
tactical baselines, persistent characters, classes, mutations, equipment, campaign
content through the three identity-branched campaign endings, and the texture manifest. `GameData::load()`
validates IDs and cross-references before the game starts.
