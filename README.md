# Mirexis

Mirexis is a Rust and Macroquad turn-based tactics game about a persistent frontier
colony caught between the Directorate, the Brood, and the Ascendants.

## Project status

Mirexis now has a Phase 1 first-hour candidate: an authored arrival, contextual city and
tactical teaching, two-operation progression, a meaningful preparation choice, visible
colony consequences, procedural audio, persistent settings, and recovery hardening. The
complete campaign foundation still extends through three late-game identity branches.

Phase 1 implementation and automated verification are complete, but the phase is not yet
accepted: five uncoached first-time playtests, including two touch-primary sessions, still
have to satisfy its exit criteria. See [docs/PLAYABLE_GAME_PHASE_1.md](docs/PLAYABLE_GAME_PHASE_1.md)
and [docs/PHASE_1_PLAYTEST_LOG.md](docs/PHASE_1_PLAYTEST_LOG.md).

## What works now

- A title-to-city-to-operation-to-debrief loop with native and browser saves.
- A navigable 2.5D city with NPC interaction, construction, repairs, facilities,
  resources, research, recruitment, and faction pressure.
- Deterministic grid combat with action points, weighted movement, line of fire, cover,
  armour, hazards, statuses, objectives, enemy roles, reinforcements, and overwatch.
- Persistent named colonists with classes, mutations, equipment, relationships, scars,
  experience, injury, and recovery.
- XP-gated Soldier, Defender, Scout, Medic, and Engineer techniques with one-slot loadouts,
  self/unit/tile targeting, deterministic effects, and save-safe phase use tracking.
- Authored and generated operations against three mechanically distinct factions.
- A complete prototype campaign-state path through Isolation, Contact, Adaptation,
  Escalation, and three Mirexis identity endings.
- Touch-visible controls for required actions, supplemented by mouse and keyboard.
- Persistent volume, mute, and reduced-motion settings with procedural event audio and ambience.
- Campaign-owned first-hour guidance with visible help, restart, skip, save migration, and defeat-forward recovery.
- Deterministic capture scenes and tests for simulation, content, saves, and migrations.

## Documentation

| Document | Purpose |
|---|---|
| [MIREXIS.md](MIREXIS.md) | Creative vision and game design direction |
| [TECHNICAL_DESIGN.md](TECHNICAL_DESIGN.md) | Current architecture, ownership, and implementation contracts |
| [docs/PLAYABLE_GAME_PHASE_1.md](docs/PLAYABLE_GAME_PHASE_1.md) | Active roadmap from tech demo to playable first-hour build |
| [docs/FUTURE_CONTENT_AND_STORY.md](docs/FUTURE_CONTENT_AND_STORY.md) | Post-Phase-1 plans for story, peoples, classes, skills, buildings, and upgrades |
| [docs/PHASE_1_BEAT_SHEET.md](docs/PHASE_1_BEAT_SHEET.md) | Authored first-hour sequence, economy, speakers, and deferred systems |
| [docs/PHASE_1_HARDENING.md](docs/PHASE_1_HARDENING.md) | Automated reliability, accessibility, resize, and recovery evidence |
| [docs/PHASE_1_PLAYTEST_LOG.md](docs/PHASE_1_PLAYTEST_LOG.md) | Required human session evidence and issue triage |
| [docs/PHASE_1_ACCEPTANCE_MATRIX.md](docs/PHASE_1_ACCEPTANCE_MATRIX.md) | Requirement-by-requirement implementation evidence and remaining human gates |
| [docs/AUDIO_SOURCES.md](docs/AUDIO_SOURCES.md) | Audio palette provenance and future attribution requirements |
| [docs/UI_GRAPHICS_VISUAL_SPEC.md](docs/UI_GRAPHICS_VISUAL_SPEC.md) | Visual language and UI acceptance principles |
| [art_sources/README.md](art_sources/README.md) | Source-art and atlas preparation notes |
| [AGENTS.md](AGENTS.md) | Project workflow and validation rules |

`docs/verification/` contains generated visual-regression references and their capture
manifest. It is test evidence, not product-planning documentation.

## Build and validation

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

Refresh deterministic visual references with:

```powershell
.\scripts\capture_ui.ps1
```

## Controls

All required actions have visible mouse/touch targets. Keyboard controls are optional
shortcuts.

- City: tap terrain to walk, hold the movement pad to move freely, tap a colonist to
  approach and talk, and use Build mode for construction and repairs.
- Tactical missions: select a colonist, then use visible grid and command controls to
  move, attack, use techniques or equipment, complete objectives, and end the phase.
- `W` / `A` / `S` / `D`: city movement.
- `E`: talk to an adjacent city NPC.
- Arrow keys: tactical cursor.
- `Enter`: advance supported screens or end the player phase.
- `S` / `L`: save or load an in-progress battle.
- `Escape`: close the current overlay or step back.

## Data

Embedded content under `assets/data/` owns configuration, missions, tactical baselines,
characters, classes, mutations, equipment, campaign content, and the texture manifest.
`GameData::load()` validates identifiers and cross-references before play begins.
