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

- A title-to-city-to-operation-to-debrief loop with native and browser saves, including a
  visible first-hour route trace to the coordinator's speech marker.
- A first-operation tactical lesson that marks the next map target or command-rail control,
  requires the squad to enter a real cover tile and then presents a reachable opening firing
  lane and objective route, without blocking battlefield inspection. Guided attacks select a
  hostile first and expose the forecast card's visible ATTACK confirmation.
- The first-operation briefing focuses DEPLOY SQUAD when the selected squad is viable and
  recovers to the first selectable colonist row when squad selection is empty.
- The ability lesson recovers onto a colonist with a valid visible action and focuses that
  specific mutation, class, or field-item control.
- First-hour colony handoffs focus the visible Operations, briefing, equal-choice preparation,
  and required conversation CONTINUE controls so the campaign remains touch-completable between
  operations.
- A navigable 2.5D city with NPC interaction, construction, repairs, facilities,
  resources, research, recruitment, route-gated Waystation contacts, and faction pressure.
- A powered Commons with one shared meal per operation, turning squad preparation into
  persistent relationship progression with visible debrief and colony-story feedback.
- A powered Relay Mast with one risky route scan per operation: mission offers refresh,
  but the faction already watching hardest gains attention.
- A placeable powered Watchtower that becomes strong directional cover in colony-defense
  operations while remaining a physical obstacle when its grid goes dark.
- Level-2 Command Centre branches: Signal Cartography reveals a third mission route with
  clearer reinforcement intelligence, or Counterintelligence Cell reduces mission pressure.
- Level-2 Barracks branches: Simulation Hall lowers retraining and technique-trial thresholds,
  or Doctrine Yard unlocks Rally formation and training barricades in colony defence.
- Level-2 Infirmary branches: Trauma Ward shortens recovery, softens persistent scars, and adds
  a stabilization barricade, or Adaptation Clinic makes mutation complications safer and treatment cheaper.
- A level-2 Power Plant branch choice: Redundant Grid preserves damaged-plant supply, or
  Hot Core raises output while increasing the most visible faction's attention per operation.
- Level-2 Hydroponics branches: Community Kitchen improves food and Commons efficiency, or
  Culture Beds grows extra biomass for adaptation.
- Level-2 Workshop branches: Precision Bench lowers weapon and armour fabrication costs, or
  Drone Bay makes facility repairs cheaper.
- Level-2 Gene Lab branches: Stabilisation Wing suppresses evolution complications while
  online, or Evolution Chamber reduces mutation evolution biomass costs.
- Optional Research Annex: a powered physical evidence archive reduces doctrine research by
  five materials and becomes a visible colony-defence objective.
- Optional Salvage Yard: each successful operation leaves one recoverable object; sort it once
  per operation into materials, a one-use doctrine insight, or a free standard equipment prototype.
- Phase Five path choices establish one physical identity building: Redoubt Arsenal, Choir
  Garden, or Threshold Spire, each changing colony defence and preparation.
- Completed campaigns record a derived colony legacy dossier covering the chosen institution,
  people, trusted bonds, scars, mutation evolutions, engine relationship, faction pressure,
  and witnessed mercy.
- Completed campaigns prioritize a repeatable path-specific epilogue operation: clear inherited
  Directorate fire in the Redoubt, answer a new Brood chorus in the Commonwealth, or bring a
  stranded traveller home through the Threshold.
- Finale and epilogue operations carry the chosen engine relationship into combat: hardened
  Redoubt armour, Commonwealth regeneration, or Threshold movement guidance.
- Each finale has an engine-facing objective contract: defend the Redoubt Arsenal core,
  secure the Commonwealth chorus, or bind and clear the Threshold route.
- Victories in the continuing epilogue are recorded in the ending register and unlock a
  path-specific follow-up reflection from the identity contact.
- After campaign completion, each identity building offers one stewardship action per operation:
  fortify the Redoubt, answer the Commonwealth, or guide a Threshold return to reduce its linked
  faction's attention for a disclosed resource cost.
- Identity-building conversations react to damage and power failure, then remember the repair or
  restoration that brings the civic project back online.
- Before the finale, each identity building offers one preparation action per operation:
  drill the Redoubt, tune the Commonwealth, or calibrate the Threshold to reduce linked faction
  attention for a disclosed resource cost and unlock a named contact reflection.
- The identity contact returns a second, path-specific reflection after the final operation, then
  unlocks a third post-ending scene in the continuing colony.
- Contact, Adaptation, and Escalation each surface phase-specific colony field notes through
  named colonist voices and persistent acknowledgement.
- Adaptation and Escalation now carry recurring field notes through the defence, clinic,
  engineering, xenobiology, and active outsider voices.
- Each Contact protocol now carries a three-stage witness, contradiction, and aftermath thread
  through distinct colonist voices before Adaptation begins.
- Completed ending dossiers add an authored identity-contact voice that reflects the named
  survivor's readiness or recovery.
- Completed campaigns expose path-specific finale field notes for every starting colonist and
  the recruited route outsider, with Kira's navigator voice preserved in the ending register.
- Deterministic grid combat with action points, weighted movement, line of fire, cover,
  armour, hazards, statuses, objectives, enemy roles, reinforcements, and overwatch.
- Eight hybrid advanced classes with deterministic class actions, including Breacher lunge/
  breach pressure, Fortifier map-deployed bastions, Rescue Specialist extraction, and
  Chorus Warden hazard conversion.
- Persistent named colonists with classes, mutations, equipment, relationships, scars,
  origins, experience, injury, and recovery, including route-specific outsider arcs for
  Veya Orn's Directorate Exile and Sedge's Mireborn Adapted origin.
- The Waystation recruits Veya through Directorate Requisition or Sedge once Adaptation
  opens a route, and gives each outsider an origin-specific equipment action.
- An unchosen route contact remains visible as a Waystation guest, with a field note and
  a touch-safe recruitment handoff that leaves the contact outside the roster until chosen.
- Adaptation can recruit Sedge through the Waystation for biomass; their origin-specific
  MAP HAZARD action braces the courier while disrupting a hostile, and their three-beat
  arc records Brood attention, relationships, disagreements, and route legacies.
- XP-gated techniques for all seven base classes with one-slot loadouts,
  self/unit/tile targeting, deterministic effects, and save-safe phase use tracking.
- Authored and generated operations against three mechanically distinct factions.
- A complete prototype campaign-state path through Isolation, Contact, Adaptation,
  Escalation, and three Mirexis identity endings.
- Touch-visible controls for required actions, supplemented by mouse and keyboard.
- Persistent volume, mute, and reduced-motion settings with procedural event audio and ambience.
- Archived colony field notes remain available from Operations with speaker, title, and transcript context.
- The paginated Memorial Register keeps persistent scars, recovery records, lost objectives, and character legacies visible from Operations.
- Campaign-owned first-hour guidance with visible help, restart, skip, save migration, advance-control
  focus, and defeat-forward recovery.
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
  move, attack, use techniques or equipment, complete objectives, and end the phase. During
  guided attacks, tap a hostile to inspect its forecast, then tap ATTACK in the forecast card.
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
