# Mirexis

Mirexis is a Rust + Macroquad turn-based tactics game about a persistent frontier
colony caught between the Directorate, the Brood, and the Ascendants. The complete
game concept is in [MIREXIS.md](MIREXIS.md); current architecture and continuation
contracts are in [TECHNICAL_DESIGN.md](TECHNICAL_DESIGN.md).

## Current Slice: Phase One — Isolation

The completed roadmap slice includes:

- a title → colony → briefing → tactical → debrief campaign loop
- deterministic weighted movement, attacks, cover, armour, objectives, and enemy AI
- obstacle-blocked firing lanes and once-per-round tactical mutation gifts
- secure-and-clear, elimination, holdout, and extraction objectives with live progress
- faction-specific battlefields and Brood, Directorate, or Ascendant enemy squads
- save-stable authored and vertically mirrored variants for every mission map family
- seven class actions, explicit Medic/Engineer/Psionic targeting, and combat statuses
- role-aware hostile activations and deterministic holdout reinforcements
- persistent three-colonist squad selection with a visible reserve
- roster-wide class training and slot-replacing workshop equipment
- player-targeted medkit, toolkit, and survey-harness actions in tactical missions
- destructible battlefield cover that opens movement routes and firing lanes
- four persistent recruits with aptitudes, classes, mutations, equipment, XP, injury,
  and operation-based recovery
- colony resources, barracks, infirmary, workshop, physical construction placement,
  and a defense map generated from the saved layout
- Directorate, Brood, and Ascendant attention; a telegraphed assault; persistent
  research doctrines; character events; and seeded mission generation
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

- Mouse: use colony actions, select missions, select colonists, move, attack,
  complete tactical objectives, or activate mutation and class actions
- Arrow keys: move the tactical cursor
- Enter: confirm title/briefing/debrief or end the colony tactical phase
- S / L: save / load an in-progress battle
- Escape: step back from briefing/tactical or return to the title

## Data

Embedded content under `assets/data/` covers configuration, the authored mission,
tactical baselines, persistent characters, classes, mutations, equipment, Phase One
campaign content, and the texture manifest. `GameData::load()` validates IDs and
cross-references before the game starts.
