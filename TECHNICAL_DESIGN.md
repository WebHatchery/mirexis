# Mirexis Technical Design

Status: Phase 0 through Phase 4 roadmap complete
Current campaign slice: Phase One — Isolation
Save/content version: 0.5.0
Target platforms: Windows and browser/WASM
Runtime: Rust 2021, Macroquad, macroquad-toolkit

## 1. Purpose

This is the implementation contract for the game design in `MIREXIS.md`. It now
describes the working Phase One vertical slice rather than a future roadmap. Keep
it accurate when schemas, ownership, simulation order, or save behavior changes.

The implemented slice proves all roadmap systems together:

1. Start or continue a persistent colony.
2. Inspect resources, faction attention, an assault countdown, research, events,
   recruits, facilities, and mission offers.
3. Place construction, train a colonist, treat injuries, and craft equipment.
4. Select an authored or seeded generated mission and deploy available recruits.
5. Resolve deterministic movement, attacks, an interactive objective, enemy AI,
   victory, or failure.
6. Apply rewards, XP, injury, recovery, pressure, threat, and new mission offers.
7. Return through debrief to the same saved colony.

## 2. Architectural Principles

- Simulation owns rules; presentation emits `UiAction` intents only.
- Content and balance are embedded JSON; Rust owns schemas and invariants.
- A tactical outcome is reproducible from battle state plus ordered commands.
- Persistent `CharacterRecord` values are separate from deployed `UnitState` values.
- Tactical outcomes cross into strategy through `MissionOutcome`; tactical code does
  not directly edit resources, injuries, construction, or faction pressure.
- Stable snake_case IDs are save contracts. Display names may change independently.
- Derived values such as deployment stats and colony-defense maps are recomputed.
- Reusable runtime behavior belongs in macroquad-toolkit. Project-local behavior is
  limited to Mirexis rules and presentation.
- Every `.rs` file stays below 800 non-test lines. Add named sibling modules rather
  than `mod.rs` files as ownership grows.

## 3. Runtime State and Transitions

The implemented application state machine is:

```text
Boot -> Title -> Colony -> MissionBriefing -> Tactical -> Debrief -> Colony
          |         |             |              |
          |         +-- strategic actions       +-- manual tactical save/load
          +-- continue saved colony/battle
```

Important transition payloads:

- `StrategyState::selected_mission()` returns an immutable `MissionInstance`.
- `materialize_selected()` derives a `MissionDef` and map recipe for deployment.
- `CampaignState::deployment_roster()` derives tactical `UnitDef` values.
- `GameSession::mission_outcome()` returns a `MissionOutcome` for debrief.
- `CampaignState::apply_mission_outcome()` is the single strategic application path.

`game.rs` coordinates these services. It must not absorb their calculations.

## 4. Module Ownership

| Module | Owns | Must not own |
|---|---|---|
| `main.rs` | Window configuration, frame loop, capture entry | Game rules |
| `game.rs` | App state, intent dispatch, toolkit save integration | Tactical/strategic calculations |
| `data.rs` | Embedded JSON schemas, loading, registry validation | Mutable campaign state |
| `state.rs` | Tactical commands, validation, execution, AI, events, outcomes | Drawing, colony mutation |
| `campaign.rs` | Persistent recruits, progression, deployment, debrief application | Raw input or drawing |
| `colony.rs` | Resources, facilities, placement, queue, defense-map derivation | Mission rendering |
| `strategy.rs` | Attention, threats, research, events, mission generation | Tactical mutation |
| `persistence.rs` | Project schema migrations inside toolkit slots | Platform storage paths |
| `ui.rs` | Title, briefing, tactical, debrief rendering and intents | Direct state mutation |
| `colony_ui.rs` | Colony rendering and strategic intents | Direct state mutation |

Split `state.rs` by cohesive responsibility before adding abilities, statuses, or
multi-objective logic that would push it toward the source limit.

## 5. Tactical Simulation Contract

### 5.1 State and commands

`TacticalState` owns the grid, occupancy, terrain costs, edge cover, units, phase,
round, objective, seeded RNG, and serializable `BattleEvent` log. The supported
commands are move, attack, and interact. Ending a phase is a session operation that
runs hostile commands and advances the round.

The authority boundary is:

```text
validate(&Command) -> Result<CommandCost, RuleError>
execute(Command)  -> Result<Vec<BattleEvent>, RuleError>
```

UI highlighting and hostile candidate generation both call the same validator.
Only `execute` mutates tactical state or consumes RNG.

### 5.2 Movement

- `macroquad_toolkit::pathfinding::find_path_with` supplies weighted A*.
- Orthogonal movement accounts for bounds, blocked tiles, living occupancy, terrain
  cost, action points, and each unit's derived move range.
- Paths and their total cost are emitted in `BattleEvent::UnitMoved`.
- Colony-defense blocked tiles are derived from saved building coordinates.

### 5.3 Attacks and outcomes

Resolution order is fixed:

1. Validate phase, teams, incapacitation, range, and action points.
2. Calculate accuracy from the attacker, range falloff, and the target-facing cover edge.
3. Draw 1–100 from the battle-owned toolkit `SeededRng`.
4. Apply critical bonus, armour mitigation, damage, and incapacitation.
5. Emit roll, damage, incapacitation, objective, and battle-end events in order.

Victory currently requires completing the interactive objective and neutralizing all
hostiles. Failure occurs when every colonist is incapacitated or the round limit is
passed. Generated missions reuse this common objective contract.

### 5.4 Enemy AI

Hostiles are ordered by stable ID. They attack the lowest-health valid colonist,
otherwise move toward the nearest colonist using validated commands, then try to
attack again. Ties use stable health/ID or distance/coordinate ordering. Brood hunters
and Sporecasters differ through data-backed range, movement, armour, accuracy, and
damage profiles.

## 6. Character and Progression Contract

`CharacterRecord` persists identity, biography, aptitude ratings, XP, level, active
class, class history, learned and active skills, mutation ID, injuries, availability,
and equipment IDs. `UnitState` is disposable battle state.

Seven initial class families are loaded from `classes.json`. Aptitude changes the
material cost of training but never class eligibility. Switching classes retains
learned fundamentals while `active_skills` is truncated to the active class's slot
limit.

Mutation definitions contain composable gift and complication stat modifiers. The
five implemented mutations exercise these hooks:

| Mutation | Current gift behavior | Current complication hook |
|---|---|---|
| Chitinous Growth | Deployment armour | Reduced movement |
| Neural Bloom | Psionic access trait | Psychic-resistance trait |
| Regenerative Tissue | Health at phase refresh | Longer injury recovery |
| Elastic Musculature | Increased movement | Heavy-armour-efficiency trait |
| Symbiotic Organism | Biological-damage trait | Food-upkeep trait |

Trait hooks whose owning combat or economy system does not yet exist remain derived
values; extend the owning system rather than adding mutation-specific switches.

Mission completion grants XP. Incapacitated colonists receive an operation-counted
injury and cannot deploy until recovery completes. The infirmary can accelerate the
first active recovery.

## 7. Colony Contract

`ColonyState` owns materials, power, food, biomass, alien components, completed
buildings, construction reservations, and a monotonic building serial.

Initial facilities have stable coordinates:

- Command Centre: mission access and a critical defense objective.
- Barracks: aptitude-priced class training.
- Infirmary: injury treatment.
- Workshop: equipment crafting.

Clicking an empty colony plot reserves a barricade, deducts 20 materials, and queues
one operation of construction. Reserved plots cannot be reused. Campaign time only
advances when an operation resolves; it never depends on wall-clock time.

`ColonyState::defense_map()` derives blocked tiles, cover tiles, and critical
objectives from completed buildings. An expired assault materializes those values
into the tactical mission, so colony placement and defense geometry share one source.

## 8. Phase One Strategy Contract

`StrategyState` owns Phase One: Isolation:

- Directorate, Brood, and Ascendant attention values from 0–100.
- A telegraphed Directorate assault with operation countdown and strength.
- Data-backed research opportunities and resource effects.
- Data-backed character events with participant validation and choice costs.
- Seeded, immutable mission offers and the selected mission ID.

Mission resolution adjusts the responsible faction's attention, advances threats,
and generates two offers from `campaign.json` templates. Generation uses the
serialized toolkit RNG, so saving and loading preserves the future stream.

When an assault reaches zero, the only offer becomes a colony-defense mission.
Winning resets the countdown and reduces strength; failure returns it sooner and
stronger. This is the first escalation loop, not the full five-phase campaign.

## 9. Content Registry

Current embedded files under `assets/data/` are:

| File | Content |
|---|---|
| `game_config.json` | Identity, save version, grid, AP budget, root seed |
| `mission.json` | Authored Glassroot mission and tactical map data |
| `roster.json` | Tactical baselines and hostile archetypes |
| `characters.json` | Persistent recruits and aptitude/loadout references |
| `classes.json` | Class families and deployment modifiers |
| `mutations.json` | Gift and complication hooks |
| `equipment.json` | Starter equipment modifiers |
| `campaign.json` | Isolation factions, research, events, mission templates |
| `texture_manifest.json` | Runtime texture declarations |

`GameData::load()` rejects duplicate IDs, missing character/class/mutation/equipment
references, invalid aptitude ranges, missing event participants, and mission templates
that reference unknown factions. Add validation in the same change as every new
cross-reference or numeric invariant.

The files can move into domain subdirectories when volume warrants it; `include_str!`
paths and publisher asset packaging must be updated together.

## 10. Save and Compatibility Policy

Platform storage is entirely owned by macroquad-toolkit:

- `save_to_slot_with_version` writes atomic native files and WASM storage.
- `load_from_slot_with_migration` dispatches project schema migrations.
- `slot_exists` and `delete_slot` support title/management UI.
- `AutoSaveManager::force` coordinates event autosaves.

`SaveData` contains a version, persistent `CampaignState`, and optional
`TacticalState`. Colony saves omit tactical state; deployment and manual tactical
saves include it. Textures, UI layout, derived deployment stats, path caches, and
derived defense maps are never serialized.

Autosaves occur at new-colony creation, colony entry, mission selection, construction,
training, treatment, crafting, research, character-event resolution, deployment, and
debrief completion. Manual tactical save/load remains available.

Migration coverage:

| From | Adds |
|---|---|
| 0.1.0 | Tactical combat fields, deterministic RNG, event log |
| 0.2.0 | Persistent character campaign and mutation runtime defaults |
| 0.3.0 | Colony state while preserving character XP |
| 0.4.0 | Isolation strategy, pressure, and seeded offers |

Every future schema bump must migrate the immediately previous version and add a
fixture test. Validate saved content IDs before adding content removal or renaming.

## 11. Toolkit Utilization Gate

Before introducing project-local infrastructure, check macroquad-toolkit and extend
it when the behavior is reusable. Mirexis currently uses:

| Concern | Toolkit facility |
|---|---|
| Native/WASM saves and migrations | `persistence` slots and `AutoSaveManager` |
| Deterministic simulation/generation | `rng::SeededRng` |
| Weighted movement | `pathfinding::find_path_with` |
| Grid storage/positions | `grid::FlatGrid`, `TilePos` |
| Virtual resolution and semantic input | `VirtualUi`, `InputState` |
| Assets and embedded JSON | `AssetManager`, `data_loader` |
| Intent queue and feedback | `EventBus`, `NotificationManager` |
| Deterministic screenshots | `capture` harness |
| Source-size enforcement | `source_gate` |

Mirexis-specific command validation, campaign pressure, colony placement, content
schemas, and immediate-mode styling remain local because they encode this game's rules.

## 12. Rendering, Input, and Capture

Rendering uses a fixed 1280×720 toolkit virtual UI. Raw keyboard/mouse input is
translated into `UiAction` or tactical commands before simulation mutation. Tactical
units use labels as well as faction color, and colony buildings use text labels.

`scripts/capture_ui.ps1` captures `title`, `colony`, `briefing`, `gameplay`, and
`debrief` by default. `Game::begin_capture_scene()` seeds each scene deterministically.
Committed captures under `docs/verification/` are the visual regression references.

## 13. Verification

The completion baseline is:

- `cargo fmt -- --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test` (21 domain/migration tests plus the shared source-size gate)
- deterministic five-scene capture with visual inspection
- `.\publish.ps1` with no parameters (Windows release, WebGL release, packaging,
  preview deployment, and catalog update)

Simulation tests never depend on frame timing or rendered pixels.

## 14. Completed Milestones

| Milestone | Implemented evidence | Commit |
|---|---|---|
| Phase 0 — Foundation | Native/WASM runtime, toolkit persistence, grid sandbox | `1d7dad2` |
| Phase 1A — Tactical Rules | Weighted pathing, AP, attacks, cover, armour, events, AI | `64dafdf` |
| Phase 1B — Vertical Mission | Four-person rescue, two archetypes, briefing/debrief | `7e0ab44` |
| Phase 2 — Character Identity | Records, aptitudes, classes, mutations, gear, XP/injury | `6114654` |
| Phase 3 — Colony Loop | Resources, facilities, placement, construction, defense map | `d8065ef` |
| Phase 4 — Campaign Pressure | Attention, assaults, research, events, generated Isolation missions | `50e80ae` |

All milestones in the original roadmap are complete and validated through the
project-standard publisher.

## 15. Current Boundaries and Next Development

The roadmap is complete, but Mirexis is not content-complete. Preserve these explicit
boundaries when continuing:

- Tactical combat currently has one shared interactive-objective contract. Add typed
  objective variants before authoring escort, holdout, or multi-stage missions.
- True line of sight, elevation, destructible cover, abilities, items, statuses,
  reactions, and animation/audio consumers are not yet implemented.
- Generated outer-mire missions reuse Glassroot geometry with different seeds,
  objectives, rewards, and limits. Add data-backed map recipes before adding volume.
- The colony has fixed initial facilities and placeable barricades; population,
  building damage/repair, power demand, and free placement for every building remain.
- Mutation evolution, advanced classes, relationships, permanent death, and richer
  equipment slots need content and UI beyond the existing rule hooks.
- Isolation is a repeatable Phase One loop. Story gates and Phases Two–Five remain
  future campaign content.
- Saved content references need explicit validation before definitions can be removed
  or renamed safely.

The recommended next vertical slice is typed tactical objectives plus line of sight
and status resolution, followed by data-backed map recipes. Those additions exercise
the existing command/event boundary without requiring a strategic rewrite.
