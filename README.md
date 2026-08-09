# Mirexis

Mirexis is a Rust + Macroquad turn-based tactics game about a persistent frontier
colony caught between the Directorate, the Brood, and the Ascendants. The complete
game concept is in [MIREXIS.md](MIREXIS.md); current architecture and continuation
contracts are in [TECHNICAL_DESIGN.md](TECHNICAL_DESIGN.md).

## Current Slice: Phase Five — Identity-Branched Operations

The completed roadmap slice includes:

- a title → colony → briefing → tactical → debrief campaign loop
- deterministic weighted movement, attacks, cover, armour, objectives, and enemy AI
- obstacle-blocked firing lanes and once-per-round tactical mutation gifts
- secure-and-clear, elimination, holdout, extraction, and activate-then-defend signal
  trace objectives with live progress
- faction-specific battlefields and Brood, Directorate, or Ascendant enemy squads
- seventeen generated operation templates, including a Brood sporefield extraction, an
  Ascendant fractured-vault purge, three protocol-gated Contact signal traces, and a
  phase-gated Adaptation scavenging operation
- save-stable authored and vertically mirrored variants for every mission map family
- seven class actions, explicit Medic/Engineer/Psionic targeting, and combat statuses
- role-aware hostile activations and deterministic holdout reinforcements
- persistent three-colonist squad selection with a visible reserve
- roster-wide class training and slot-replacing workshop equipment
- player-targeted medkit, toolkit, and survey-harness actions in tactical missions
- destructible battlefield cover that opens movement routes and firing lanes
- four persistent recruits with aptitudes, classes, mutations, equipment, XP, injury,
  and operation-based recovery
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

- Mouse: choose and place colony construction, repair facilities, select missions and
  colonists, move, attack,
  complete tactical objectives, or activate mutation and class actions
- Arrow keys: move the tactical cursor
- Enter: confirm title/briefing/debrief or end the colony tactical phase
- S / L: save / load an in-progress battle
- Escape: step back from briefing/tactical or return to the title

## Data

Embedded content under `assets/data/` covers configuration, the authored mission,
tactical baselines, persistent characters, classes, mutations, equipment, campaign
content through the identity-branched Phase Five operations, and the texture manifest. `GameData::load()`
validates IDs and cross-references before the game starts.
