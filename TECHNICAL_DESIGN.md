# Mirexis Technical Design

Status: Phase 0 foundation  
Target platforms: Windows and browser/WASM  
Runtime: Rust 2021, Macroquad, macroquad-toolkit

## 1. Purpose

This document turns the game design in `MIREXIS.md` into an implementation plan.
It defines ownership boundaries, deterministic simulation rules, data contracts,
save policy, testing expectations, and an incremental roadmap. The design favors
small modules and replaceable placeholder systems so the game can grow without a
large rewrite.

## 2. Phase 0 Baseline

The current executable proves the full platform path and a narrow tactical loop:

1. Load embedded configuration, mission, roster, and texture-manifest JSON.
2. Enter a title screen.
3. Start a deterministic tactical sandbox.
4. Select named colonists and move across a validated grid using action points.
5. End the colony phase and resolve deterministic placeholder hostile movement.
6. Save, load, delete, and migrate versioned state through macroquad-toolkit.
7. Render through a 1280×720 virtual UI on native and WASM targets.

Phase 0 is complete when native tests, linting, WASM compilation, and the project
publisher pass. It is not intended to prove final combat balance or campaign UX.

## 3. Architectural Principles

- Simulation owns rules; presentation emits intents and never mutates state.
- Content and balance are JSON-backed; Rust supplies schemas and invariants.
- A tactical outcome must be reproducible from initial state plus ordered commands.
- Runtime-only types do not leak into long-lived campaign data without a reason.
- IDs are stable snake_case strings. Display names may change without breaking saves.
- Shared rendering, input, persistence, camera, asset, and pathfinding needs should
  use macroquad-toolkit before project-local equivalents are introduced.
- Source files stay under the shared 800 non-test-line limit and should normally
  remain in the 200–400 line range.

## 4. Current Module Ownership

| Module | Owns | Must not own |
|---|---|---|
| `main.rs` | Window configuration, frame loop, capture harness | Gameplay rules |
| `game.rs` | App state, UI-intent dispatch, persistence integration | Tactical calculations |
| `data.rs` | JSON schemas and embedded content loading | Mutable runtime state |
| `state.rs` | Tactical state, movement validation, turns, save schema | Drawing and raw input |
| `ui.rs` | Title/tactical drawing and `UiAction` production | Direct state mutation |

As systems grow, use named parent files (`combat.rs`, `combat/resolution.rs`) and
do not create `mod.rs` files.

## 5. Target Runtime State Machine

The Phase 0 `Title -> Tactical` flow should evolve into explicit application states:

```text
Boot -> Title -> Colony -> MissionBriefing -> Tactical -> Debrief -> Colony
                   |                           |
                   +---- Campaign events <-----+
```

Transitions carry typed payloads. For example, tactical deployment receives a
`MissionInstance` and `SquadLoadout`; debrief returns a `MissionOutcome`. Tactical
code must not directly edit colony resources, injuries, or faction pressure.

## 6. Tactical Simulation

### 6.1 Core Types

Introduce these in the first tactical milestone:

- `BattleState`: map, units, round, phase, objective state, deterministic RNG.
- `MapState`: terrain grid, elevation, cover edges, hazards, interactables.
- `CombatantState`: stable character/enemy ID, stats, resources, statuses, position.
- `Command`: move, attack, ability, item, interact, defend, end activation/phase.
- `CommandResult`: ordered events describing movement, damage, statuses, and deaths.
- `BattleEvent`: serializable facts consumed by animation, audio, logs, and tests.

The simulation API should follow `validate(command) -> Result<cost, RuleError>` and
`execute(command) -> Vec<BattleEvent>`. UI previews use validation; only execution
mutates battle state.

### 6.2 Turn Economy

Retain alternating player and enemy phases initially. Each combatant receives a
small action-point budget at phase start. Movement costs one point per orthogonal
tile before modifiers. Attacks and abilities declare point costs in data. Avoid
special-case “move plus action” booleans because they make hybrid classes harder.

### 6.3 Grid and Pathfinding

Use `macroquad_toolkit::grid` types. Replace Phase 0 Manhattan-only movement with
weighted flood-fill/pathfinding that accounts for occupancy, elevation, hazards,
movement traits, and mutation exceptions. Store cover on tile edges rather than on
the occupying tile so attacks from different directions resolve correctly.

### 6.4 Combat Resolution

Centralize hit, damage, armour, critical, and status calculations in a stateless
resolver. Suggested initial order:

1. Validate range, line of sight, target, resources, and phase.
2. Build accuracy from weapon, user stats, range band, cover, elevation, statuses.
3. Draw from the battle-owned deterministic RNG.
4. Apply armour/shield mitigation and damage channels.
5. Emit injury, incapacitation, objective, and reaction events.

Numbers belong in data, while the order of operations and invariants belong in Rust.

### 6.5 Enemy AI

Replace direct placeholder movement with a scoring AI. Candidate actions are
generated by the same command validator used by the player. A faction behavior
profile weights safety, objective pressure, focus fire, range, biomass spread,
teleport control, or suppression. Deterministic tie-breaking is mandatory.

## 7. Character and Progression Model

Separate persistent `CharacterRecord` from deployed `CombatantState`.
`CharacterRecord` should hold identity, biography keys, aptitudes, learned skills,
class history, mutation path, injuries, relationships, equipment IDs, and campaign
availability. Deployment derives tactical stats from that record and content data.

Aptitudes modify training cost or rate, never eligibility. Class switching should
use skill-slot and prerequisite constraints so a character cannot eventually equip
every learned ability. Mutation gift, complication, and evolution should be modeled
as composable rule hooks, not one large mutation switch statement.

## 8. Colony and Campaign

The strategic layer should own:

- resources and population
- buildings, construction queue, damage, and physical footprint
- roster availability, training, recovery, and injuries
- research opportunities and completed projects
- faction threat/attention and telegraphed crises
- mission offers, deadlines, and campaign flags

Colony buildings need stable placement coordinates from their first implementation,
even if the initial UI uses fixed slots. The same placement data later generates
colony-defense tactical maps. Campaign time advances through explicit operations or
events, never wall-clock time.

Mission generation consumes campaign state and emits an immutable `MissionInstance`
containing seed, map recipe, participants, objectives, rewards, and consequences.
Debrief applies the result through one campaign service so rewards and failures are
auditable and testable.

## 9. Data Layout Target

Grow `assets/data/` by domain:

```text
assets/data/
  config/
  characters/
  classes/
  mutations/
  abilities/
  equipment/
  enemies/
  factions/
  missions/
  colony/
  research/
  localization/
```

Each definition has an `id`, optional schema version, and references other content
by ID. Loading should validate duplicate IDs, missing references, invalid numeric
ranges, impossible placements, and cyclic prerequisites. CI tests should load and
validate the complete registry.

## 10. Save and Compatibility Policy

Use a single campaign save envelope containing:

- `schema_version`
- build/content version
- campaign seed
- persistent campaign state
- optional in-progress battle state

Never serialize textures, UI layout, caches, or derived reachability. Recompute
derived data after loading. Every schema change adds a migration from the immediately
previous version and fixtures for older representative saves. Unknown content IDs
must produce a useful error rather than a panic or silent deletion.

Autosave at colony entry, mission deployment, and debrief completion. Manual saves
are safe in the colony. Mid-battle saves can remain a later decision, though the
Phase 0 state demonstrates technical feasibility.

## 11. Rendering and UX

Keep world rendering and screen-space UI separate. Layout uses the toolkit virtual
resolution; camera viewports must use toolkit logical-to-physical conversion for
high-DPI displays. Input maps to semantic actions before simulation sees it.

The first art pass should emphasize faction readability through shape and palette:
industrial Directorate, organic Brood, luminous geometric Ascendants, and visibly
improvised colony hybrids. Gameplay cannot rely on color alone. Add icons, silhouettes,
and text labels for accessibility, plus rebindable inputs and scalable text later.

## 12. Testing Strategy

- Unit tests: costs, pathing, line of sight, cover, damage, mutations, status order.
- Data tests: all JSON loads, IDs are unique, references resolve, values are valid.
- Scenario tests: execute command sequences and compare final state/event streams.
- Save tests: round trips plus migration fixtures.
- UI capture: deterministic title, colony, tactical, and debrief scenes.
- Build gates: fmt, clippy with warnings denied, tests, native release, WASM release.

Do not base simulation tests on frame timing or rendering output.

## 13. Milestone Roadmap

### Phase 1A — Tactical Rules

Add weighted pathfinding, activation/action economy, attacks, accuracy, cover,
armour, incapacitation, objective completion, and a battle event stream.

### Phase 1B — Vertical Mission

Ship one authored rescue mission with four colonists, two enemy archetypes, one
interactive objective, deployment, victory/failure, and debrief consequences.

### Phase 2 — Character Identity

Add persistent recruits, aptitudes, initial class families, five mutations with
gift/complication behavior, equipment, experience, injury, and recovery.

### Phase 3 — Colony Loop

Add colony hub, resources, barracks, infirmary, workshop, construction placement,
mission selection, and a colony-defense map generated from placement data.

### Phase 4 — Campaign Pressure

Add faction attention, telegraphed attacks, research opportunities, character events,
mission generation, and Phase One: Isolation content.

## 14. Immediate Next Slice

The next development task should implement Phase 1A as a narrow end-to-end slice:
one weapon, one attack command, cover-aware accuracy, damage/incapacitation events,
an objective tile, a deterministic enemy action, and a debrief transition. This
tests the intended command/event boundary before class, mutation, or colony breadth
adds pressure to the architecture.
