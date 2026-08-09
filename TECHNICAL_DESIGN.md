# Mirexis Technical Design

Status: Phase 0 through Phase 4 roadmap complete
Current campaign slice: Phase One — Isolation
Save/content version: 1.4.0
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
4. Select an authored or seeded generated mission and choose three ready recruits.
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
          |         |  +-> Roster ---------+
          |         |  +-> strategic actions
          |         |                 Tactical: manual save/load
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
| `game/capture_scenes.rs` | Deterministic visual-reference state construction | Runtime input handling |
| `data.rs` | Embedded JSON schemas, loading, registry validation | Mutable campaign state |
| `state.rs` | Tactical commands, validation, execution, events, outcomes | Drawing, colony mutation |
| `tactical.rs` | Serializable tactical types and geometry helpers | Campaign or drawing |
| `tactical_ai.rs` | Deterministic hostile targeting and movement | Presentation or strategy |
| `reinforcements.rs` | Holdout wave construction, placement, and deployment | Rendering |
| `class_actions.rs` | Class actions, targeting, damage, healing, and status application | UI state |
| `class_action_ui.rs` | Immediate or targeted class-action intent | Simulation mutation |
| `equipment_actions.rs` | Field-item validation, targeting rules, and deterministic effects | UI state |
| `cover_actions.rs` | Cover attack validation, integrity damage, and terrain removal | UI state |
| `campaign.rs` | Persistent recruits, progression, deployment, debrief application | Raw input or drawing |
| `colony.rs` | Resources, facilities, placement, queue, defense-map derivation | Mission rendering |
| `strategy.rs` | Attention, threats, research, events, mission generation | Tactical mutation |
| `map_variants.rs` | Seed-driven safe transforms of authored battlefield geometry | Mission selection |
| `persistence.rs` | Project schema migrations inside toolkit slots | Platform storage paths |
| `ui.rs` | Title, briefing, tactical, debrief rendering and intents | Direct state mutation |
| `grid_ui.rs` | Tactical viewport geometry and pointer hit-testing | Simulation rules |
| `ui_widgets.rs` | Shared tactical buttons, status labels, and event summaries | State mutation |
| `colony_ui.rs` | Colony rendering and strategic intents | Direct state mutation |
| `roster_ui.rs` | Colonist selection, training, and equipment intents | Campaign mutation |
| `equipment_ui.rs` | Tactical item button and targeting intent | Simulation mutation |
| `cover_ui.rs` | Cover integrity bars and attackable-tile outlines | Simulation mutation |
| `tactical_unit_ui.rs` | Unit tokens, selection rings, and target outlines | Simulation mutation |

Split `state.rs` by cohesive responsibility before adding abilities, statuses, or
multi-objective logic that would push it toward the source limit.

## 5. Tactical Simulation Contract

### 5.1 State and commands

`TacticalState` owns the grid, occupancy, terrain costs, edge cover, units, phase,
round, objective, seeded RNG, and serializable `BattleEvent` log. The supported
commands are move, attack, interact, activate mutation, and activate class action.
Carried field equipment also grants validated target commands whose per-mission use is
serialized with the unit.
Blocked cover may be attacked with the active weapon through the same command boundary.
Ending a phase is a session operation that resolves timed statuses, runs hostile
commands, and advances the round.

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
- Destroyed cover is removed from the authoritative blocked set, immediately opening
  that tile to pathfinding and any firing line that crosses it.

### 5.3 Attacks and outcomes

Resolution order is fixed:

1. Validate phase, teams, incapacitation, range, and action points.
2. Reject shots whose firing line crosses a solid blocked tile.
3. Calculate accuracy from the attacker, range falloff, and the target-facing cover edge.
4. Draw 1–100 from the battle-owned toolkit `SeededRng`.
5. Apply critical bonus, armour mitigation, damage, and incapacitation.
6. Emit roll, damage, incapacitation, objective, and battle-end events in order.

Every initially blocked battlefield tile also owns six battle-local cover integrity.
A cover attack costs the weapon's normal action points, obeys range and line of fire,
and deals the weapon's effective damage without a separate evasion roll. At zero
integrity the cover and any attached directional edge are removed, emitting an ordered
destruction event. Strategic colony structures are unchanged after the operation;
persistent building damage remains a separate campaign concern.

`ObjectiveKind` selects one of four victory contracts. `SecureAndClear` requires an
adjacent interaction followed by neutralizing all hostiles. `EliminateAll` resolves as
soon as the last hostile falls. `Holdout` resolves when the squad survives beyond the
round limit, or earlier if it neutralizes every attacker. Every contract still fails if
all colonists are incapacitated; non-holdout missions also fail at their deadline.

`Extraction` is the fourth contract: any active colonist adjacent to the evac tile may
spend one action point to end the operation immediately, even while hostiles remain.
The shorter Directorate courier template uses this rule to turn movement, cover
breaching, and defensive abilities into an alternate route to victory.

### 5.4 Enemy AI

Hostiles are ordered by stable ID. They attack the lowest-health valid colonist,
otherwise move toward a role-specific engagement distance using validated commands,
then continue spending their action economy. Hunters close to melee while artillery,
line infantry, and drones seek ranged spacing. Ties use stable health/ID or
distance/coordinate ordering. Sporecasters and rift wardens hinder movement on a hit;
suppression drones disrupt accuracy. Mission faction filters deployment to Brood organisms,
Directorate riflemen and a suppression drone, or Ascendant sentinels and a rift
warden; factions no longer borrow the authored Glassroot enemy squad.

Holdout missions prebuild deterministic reinforcement waves for rounds three and five.
Queued units are part of `TacticalState`, so saves preserve future pressure exactly.
Arrival chooses the nearest valid edge tile, emits a battle event, and prevents an early
elimination victory while waves remain. The sidebar reports pending waves.

Each authored map recipe has two deterministic layouts selected by the mission seed:
the authored geometry and a vertical mirror. Mirroring preserves left/right deployment
pressure, rotates north/south cover facings, and transforms terrain and objective tiles
together. A safety gate rejects any candidate that blocks an objective or any authored
unit spawn. Campaign saves preserve the seed; tactical saves preserve realized geometry.

## 6. Character and Progression Contract

`CharacterRecord` persists identity, biography, aptitude ratings, XP, level, active
class, class history, learned and active skills, mutation ID, injuries, availability,
equipment IDs, and deployment selection. `UnitState` is disposable battle state.

Briefing exposes all four current recruits as deploy or reserve rows. At least one and
at most three ready colonists may be selected; injuries disable their row. Selection is
autosaved, survives operations, and is applied before faction hostiles join the tactical
roster, so reserve choice affects class, mutation, and equipment access in the mission.

The colony roster screen persists a selected colonist and exposes every base class with
aptitude-priced material costs. Workshop choices cover all starter equipment. Crafting
replaces any item in the same slot while preserving other slots, so primary weapons,
armour, tools, and modules remain mutually coherent rather than accumulating blindly.

Seven initial class families are loaded from `classes.json`. Aptitude changes the
material cost of training but never class eligibility. Switching classes retains
learned fundamentals while `active_skills` is truncated to the active class's slot
limit.

Every base class has one once-per-round tactical action. Soldier focuses its shots;
Defender braces through the hostile phase; Scout converts Surge into action points and
movement; Medic dresses a chosen nearby wound; Engineer launches an armour-ignoring
shock drone at a chosen hostile; Psionic disrupts a chosen hostile; and Biotech grants
short-lived squad regeneration. The three targeted actions use the same highlighted
targeting mode and execution validator as field equipment. Class identity is carried
into `UnitState` rather than inferred from text.

The Field Medkit restores five vitality to a chosen wounded colonist within three
tiles. The Field Toolkit grants Guarded to a chosen nearby colonist, and the Survey
Harness applies Disrupted to a chosen hostile within six tiles. Each costs one action
point and may be used once per mission. Arming an item highlights only targets accepted
by the same command validator used at execution time.

Focused, Guarded, Quickened, Disrupted, Hindered, and Regenerating are serialized timed statuses.
Their modifiers feed the same effective-stat methods used by command validation,
attacks, and hostile AI. Phase ownership controls expiry so defensive and hostile
debuffs survive long enough to affect the opposing phase.

Mutation definitions contain composable gift and complication stat modifiers. The
five implemented mutations exercise these hooks:

| Mutation | Current gift behavior | Current complication hook |
|---|---|---|
| Chitinous Growth | Deployment armour | Reduced movement |
| Neural Bloom | Psionic access trait | Psychic-resistance trait |
| Regenerative Tissue | Health at phase refresh | Longer injury recovery |
| Elastic Musculature | Increased movement | Heavy-armour-efficiency trait |
| Symbiotic Organism | Biological-damage trait | Food-upkeep trait |

Each mutation also exposes a one-action-point, once-per-round tactical gift. Neural
Bloom focuses accuracy, Chitinous Growth hardens armour, Regenerative Tissue restores
vitality, Elastic Musculature adds immediate action economy and movement range, and
Symbiotic Organism temporarily increases weapon damage. Round refresh clears temporary
bonuses and makes the gift available again.

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
- Data-backed research opportunities with persistent consequences: Field
  Fortifications raises colony-defense cover integrity from 6 to 10, Xeno-Triage
  reduces new injury recovery by one operation, and Salvage Doctrine adds 8
  materials to later mission rewards.
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
| `roster.json` | Tactical baselines and faction-tagged hostile archetypes |
| `characters.json` | Persistent recruits and aptitude/loadout references |
| `classes.json` | Class families and deployment modifiers |
| `mutations.json` | Gift and complication hooks |
| `equipment.json` | Starter equipment modifiers |
| `campaign.json` | Isolation factions, research, events, mission and map recipes |
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
| 0.5.0 | Tactical mutation-use and temporary combat fields |
| 0.6.0 | Typed objective fields for tactical state and mission offers |
| 0.8.0 | Class identity, action-use flag, and timed tactical statuses |
| 0.9.0 | Serialized holdout reinforcement queue |
| 1.0.0 | Persistent deployment selection, normalized to the three-colonist limit |
| 1.1.0 | Persistent roster-screen character selection |
| 1.2.0 | Tactical equipment identity and once-per-mission item usage |
| 1.3.0 | Battle-local destructible-cover integrity |

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

`scripts/capture_ui.ps1` captures `title`, `colony`, `research`, `roster`, `briefing`,
`gameplay`, `extraction`, `variant`, `equipment`, `class_target`, `breach`, and
`debrief` by default. Capture setup seeds each scene deterministically, including
objective, doctrine, and targeting states.
Committed captures under `docs/verification/` are the visual regression references.

## 13. Verification

The completion baseline is:

- `cargo fmt -- --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test` (47 domain/migration tests plus the shared source-size gate)
- deterministic twelve-scene capture with visual inspection
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

- Escort, defense-target integrity, and multi-stage mission contracts remain beyond the
  four implemented objective types.
- Elevation, long-lived injuries as tactical statuses, reactions, and animation/audio
  consumers are not yet implemented.
- The four current map recipes support authored and safe mirrored layouts. Additional
  transforms, elevation, spawn recipes, and battlefield families remain future work.
- The colony has fixed initial facilities and placeable barricades; population,
  building damage/repair, power demand, and free placement for every building remain.
- Mutation evolution, advanced classes, relationships, permanent death, and additional
  equipment families need content beyond the starter roster screen.
- Isolation is a repeatable Phase One loop. Story gates and Phases Two–Five remain
  future campaign content.
- Saved content references need explicit validation before definitions can be removed
  or renamed safely.

The recommended next vertical slice is another layer of Isolation campaign content:
character-event choices with persistent consequences, or additional mission templates
that test the squad and completed doctrines in new combinations.
