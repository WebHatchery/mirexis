# Mirexis Technical Design

Status: systems-rich tech demo; playable-game refinement in progress
Current production target: cohesive first-hour playable build
Save/content version: 1.75.0
Target platforms: Windows and browser/WASM
Runtime: Rust 2021, Macroquad, macroquad-toolkit

## 1. Purpose

This is the implementation contract for the game design in `MIREXIS.md`. It describes
the working systems prototype rather than claiming product readiness. Keep it accurate
when schemas, ownership, simulation order, or save behavior changes. The active product
roadmap lives in `docs/PLAYABLE_GAME_PHASE_1.md`.

The implemented slice proves all roadmap systems together:

1. Start or continue a persistent colony.
2. Inspect resources, faction attention, an assault countdown, research, events,
   recruitable outsiders, facilities, and mission offers.
3. Place construction, host a Commons meal, train a colonist, treat injuries, and craft
   equipment.
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
| `action_preview.rs` | Read-only validated movement/attack consequence and rejection projection | State mutation or drawing |
| `action_preview_ui.rs` | Command banner, movement route, valid-shot tracer, and rejection styling | Command execution |
| `battle_log_ui.rs` | Command-blocking recent ordered-event history panel | Simulation mutation |
| `combat_feedback.rs` | Bounded transient damage, healing, and status callouts | Simulation mutation or persistence |
| `briefing_intel_ui.rs` | Materialized contract, hostile, ability, and hazard briefing summary | Mission generation |
| `briefing_loadout_ui.rs` | Selected colonist derived combat/loadout summary in briefing | Campaign mutation |
| `briefing_deployment_ui.rs` | Squad rows, formation selector, and deploy/stand-down controls | Session creation |
| `danger_rating.rs` | Deterministic shared offer/briefing danger score and bands | Save state or rendering |
| `game.rs` | App state, intent dispatch, toolkit save integration | Tactical/strategic calculations |
| `game/input.rs` | State-aware keyboard translation and replay input lock | Simulation mutation |
| `game/capture_scenes.rs` | Deterministic visual-reference state construction | Runtime input handling |
| `game/capture_tactical.rs` | Tactical mechanic showcase capture construction | Runtime input handling |
| `ui_debrief.rs` | Operation results and campaign-finale presentation | Outcome or campaign mutation |
| `gene_lab_ui.rs` | Mutation inspection and evolution intents | Campaign mutation |
| `ui_action.rs` | Shared screen-to-state action vocabulary | Rendering and action execution |
| `data.rs` | Embedded JSON schemas and registry validation | Mutable campaign state |
| `data/loader.rs` | Embedded JSON loading and source-labelled content assembly | Schema definitions or mutable campaign state |
| `defense_objective.rs` | Vulnerable-asset attacks, integrity, deadline result, and hostile targeting helpers | Rendering or campaign rewards |
| `state.rs` | Tactical commands, validation, execution, events, outcomes | Drawing, colony mutation |
| `state/creation.rs` | Fresh tactical-session construction and mission-derived defaults | Runtime command execution |
| `tactical.rs` | Serializable tactical types and geometry helpers | Campaign or drawing |
| `tactical_ai.rs` | Deterministic hostile targeting and movement | Presentation or strategy |
| `reinforcements.rs` | Holdout wave construction, placement, deployment, and read-only forecasts | Rendering |
| `reinforcement_ui.rs` | One-round on-grid preferred insertion-tile warnings | Wave scheduling or mutation |
| `class_actions.rs` | Class actions, targeting, damage, healing, and status application | UI state |
| `class_training.rs` | Training costs, advanced-class gates, switching, and class-definition validation | UI state |
| `skill_training.rs` | XP-gated technique learning, active loadout slots, and loadout normalization | Tactical execution or drawing |
| `skills.rs` | Deterministic technique validation, targeting, effects, and technique events | Campaign progression or drawing |
| `class_action_ui.rs` | Immediate or targeted class-action intent | Simulation mutation |
| `skill_ui.rs` | Touch-first active-technique buttons and targeting intents | Simulation mutation |
| `equipment_actions.rs` | Field-item validation, targeting rules, and deterministic effects | UI state |
| `enemy_abilities.rs` | Faction ability validation, target choice, effects, and AI activation | General hostile movement or rendering |
| `enemy_intent.rs` | Read-only hostile first-action forecasts using live AI selectors | State mutation or drawing |
| `enemy_intent_ui.rs` | Hostile inspector, immediate/one-move threat envelopes, and first-action forecast | AI policy or state mutation |
| `hazards.rs` | Landing-triggered faction hazard damage, statuses, immunity, and events | Pathfinding or drawing |
| `hazard_ui.rs` | Grid hazard symbols and compact tactical legend | Simulation mutation |
| `help_ui.rs` | Command-blocking tactical quick-reference overlay | Game rules or persistence |
| `equipment_catalog.rs` | Equipment-definition invariants and weapon-profile validation | Runtime state |
| `cover_actions.rs` | Cover attack validation, integrity damage, and terrain removal | UI state |
| `campaign.rs` | Persistent recruits, progression, deployment, debrief application | Raw input or drawing |
| `campaign/derivation.rs` | Derived tactical profiles from class, origin, mutation, gear, legacy, and trauma | Persistent mutation or rendering |
| `campaign/commons.rs` | Once-per-operation Commons meals and relationship progression | Colony rendering or tactical mutation |
| `campaign/relay.rs` | Once-per-operation route scans, mission refresh, and attention exposure | Colony rendering or tactical mutation |
| `campaign/outsider.rs` | Waystation recruitment gates and Contact-era outsider conversations | Rendering or raw input |
| `colony_story.rs` | Persistent character-led colony beats and save-safe acknowledgement state | Rendering or tactical mutation |
| `relationships.rs` | Pair-bond progression, summaries, validation, and derived deployment bonuses | Rendering or save migration |
| `overwatch.rs` | Prepaid reaction validation, trigger ordering, and reaction damage | Enemy movement policy or rendering |
| `objective_ui.rs` | Objective progress, description, and live wave forecast panel | Objective mutation |
| `phase_refresh.rs` | Team AP, regeneration, temporary-stat, and per-phase-use reset | Command validation or drawing |
| `phase_readiness.rs` | Active-colonist count and deterministic next-ready selection | Action execution or persistence |
| `phase_replay.rs` | Bounded timed hostile-event playback and skip affordance | Simulation ordering or persistence |
| `trauma.rs` | Bounded persistent scar selection and deployment tradeoff modifiers | Temporary recovery or rendering |
| `colony.rs` | Resources, facilities, placement, queue, defense-map derivation | Mission rendering |
| `strategy.rs` | Attention, threats, research, events, mission generation | Tactical mutation |
| `strategy_choices.rs` | Irreversible Contact-independent strategic choice transactions | Mission generation or rendering |
| `strategy_events.rs` | Campaign-event projection and availability rules | Event consequences or UI |
| `strategy_rewards.rs` | Contact and Escalation mission-recovery bonuses | Mission generation or colony mutation |
| `map_variants.rs` | Seed-driven safe transforms of authored battlefield geometry | Mission selection |
| `persistence.rs` | Project schema migrations inside toolkit slots | Platform storage paths |
| `ui.rs` | Title, briefing, tactical, debrief rendering and intents | Direct state mutation |
| `grid_ui.rs` | Tactical viewport geometry and pointer hit-testing | Simulation rules |
| `ui_widgets.rs` | Shared tactical buttons, status labels, and event summaries | State mutation |
| `colony_ui.rs` | Colony rendering and strategic intents | Direct state mutation |
| `roster_ui.rs` | Colonist selection, training, and equipment intents | Campaign mutation |
| `equipment_ui.rs` | Tactical item button and targeting intent | Simulation mutation |
| `formation.rs` | Safe deterministic wedge, line, and column colony entry placement | Tactical persistence |
| `cover_rules.rs` | Directional facing and cover-penalty calculation shared by combat and preview | Drawing or state mutation |
| `cover_ui.rs` | Directional cover strength plus integrity and targetability presentation | Simulation mutation |
| `tactical_unit_ui.rs` | Unit tokens, selection/target outlines, AP readiness, vitality, and effective armour | Simulation mutation |

Split `state.rs` by cohesive responsibility before adding abilities, statuses, or
multi-objective logic that would push it toward the source limit.

## 5. Tactical Simulation Contract

### 5.1 State and commands

`TacticalState` owns the grid, occupancy, terrain costs, faction hazard tiles, edge cover,
obscuring fields, units, phase, round, objective, seeded RNG, and serializable `BattleEvent` log.
The supported
commands are move, attack, interact, activate mutation, activate class action, and
activate technique. Carried field equipment also grants validated target commands whose
per-mission use is serialized with the unit. Techniques are learned by deployed
colonists at XP thresholds, equip into a class-defined technique slot, and reset their
once-per-player-phase use record during phase refresh.
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

Player action preview also calls `validate(&Command)` rather than duplicating reachability
or attack rules. It reports movement AP and landing hazard effects, or exact hit chance,
normal/critical damage, and weapon AP for an attackable unit. Hover and keyboard-focused
tiles share the same presentation; previewing never mutates state or consumes RNG.

### 5.2 Movement

- `macroquad_toolkit::pathfinding::find_path_with` supplies weighted A*.
- Orthogonal movement accounts for bounds, blocked tiles, living occupancy, terrain
  cost, action points, and each unit's derived move range.
- Paths and their total cost are emitted in `BattleEvent::UnitMoved`.
- Landing on a hazard resolves after movement and before overwatch. Directorate Fire
  Lanes deal two damage, Brood Spore Blooms deal one damage and Hinder, and Ascendant
  Static Rifts Disrupt. Units are immune to hazards authored by their own faction.
- Hazard geometry is serialized, mirrored with its map recipe, visibly marked, and named
  in both the grid legend and ordered battle events.
- Soldier, Defender, Scout, Medic, Engineer, Psionic, and Biotech Specialist each have two
  data-defined techniques.
  Controlled Burst fires twice for the weapon cost, Armour Drill halves armour for the
  next attack, Interpose and Anchor Point apply one-phase Guarded protection, Slipstep
  crosses a highlighted hazard without landing resolution, and Spotter's Mark grants the
  next allied attack a cover-breaking accuracy bonus. Stabilise revives an incapacitated
  ally at one vitality without actions, Combat Stimulant trades two immediate ally AP for
  two Hindered phases, Portable Cover places adjacent destructible directional cover, and
  Overcharge strengthens the next carried equipment action. Kinetic Draw pulls a hostile
  onto a validated clear tile, Premonition applies one-phase Disrupted intent, Adaptive
  Secretion gives an ally resistance to the nearest visible hazard, and Spore Veil creates
  a one-phase radius-one obscuring field with an accuracy penalty.
- Eight hybrid advanced classes now complete the first hybrid tier. Make an Entrance
  crosses a short validated route to breach cover or strike and mark a hostile; Raise
  Bastion places stronger destructible directional cover on a valid map tile. Carry
  Through moves a nearby injured or incapacitated ally and the specialist toward a
  deterministic safe tile before stabilising both behind Guarded protection. Borrowed
  Weather converts a nearby faction hazard into a one-phase neutral field, shares
  regeneration with allies in its radius, and leaves the Warden Hindered. All four
  advanced actions use the same authoritative unit/tile targeting path as techniques;
  their temporary battlefield geometry remains battle-local.
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

`ObjectiveKind` selects one of five victory contracts. `SecureAndClear` requires an
adjacent interaction followed by neutralizing all hostiles. `EliminateAll` resolves as
soon as the last hostile falls. `Holdout` resolves when the squad survives beyond the
round limit, or earlier if it neutralizes every attacker. Every contract still fails if
all colonists are incapacitated; non-holdout missions also fail at their deadline.

`Extraction` is the fourth contract: any active colonist adjacent to the evac tile may
spend one action point to end the operation immediately, even while hostiles remain.
The shorter Directorate courier template uses this rule to turn movement, cover
breaching, and defensive abilities into an alternate route to victory.

`SignalTrace` is the first two-stage contract. The squad must reach and activate a
relay, then survive through the round limit while deterministic waves reinforce the
defenders. Reaching the deadline before activation fails; activating and outlasting it
wins even with enemies present, while clearing every queued wave can finish early.

### 5.4 Enemy AI

Hostiles are ordered by stable ID. They attack the lowest-health valid colonist,
otherwise move toward a role-specific engagement distance using validated commands,
then continue spending their action economy. Hunters close to melee while artillery,
line infantry, and drones seek ranged spacing. Ties use stable health/ID or
distance/coordinate ordering. Sporecasters and rift wardens hinder movement on a hit;
suppression drones disrupt accuracy. Mission faction filters deployment to Brood organisms,
Directorate riflemen and a suppression drone, or Ascendant sentinels and a rift
warden; factions no longer borrow the authored Glassroot enemy squad.

During the player phase, selecting a hostile derives a read-only forecast from a cloned
enemy-phase view of the battle. It reuses the live attack and movement selectors, exposes
the faction ability and likely target, and draws the predicted attack line or first step.
Forecast state is never serialized and never consumes tactical RNG.

Holdout and SignalTrace missions prebuild deterministic reinforcement waves for rounds
three and five.
Queued units are part of `TacticalState`, so saves preserve future pressure exactly.
Arrival chooses the nearest valid edge tile, emits a battle event, and prevents an early
elimination victory while waves remain. The sidebar reports pending waves.

Each authored map recipe has two deterministic layouts selected by the mission seed:
the authored geometry and a vertical mirror. Mirroring preserves left/right deployment
pressure, rotates north/south cover facings, and transforms terrain, hazards, and objective tiles
together. A safety gate rejects any candidate that blocks an objective, hazard, or authored
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
aptitude-priced material costs. Workshop choices cover starter equipment plus three
Contact prototypes. Crafting replaces any item in the same slot while preserving other
slots, so primary weapons, armour, tools, and modules remain mutually coherent rather
than accumulating blindly.

Seven initial class families are loaded from `classes.json`. Aptitude changes the
material cost of training but never class eligibility. Switching classes retains
learned fundamentals while `active_skills` is truncated to the active class's slot
limit.

Every base class has one once-per-round tactical action. Soldier focuses its shots;
Defender braces through the hostile phase; Scout converts Surge into action points and
movement; Medic dresses a chosen nearby wound; Engineer launches an armour-ignoring
shock drone at a chosen hostile; Psionic disrupts a chosen hostile; and Biotech grants
short-lived squad regeneration. Vanguard, Pathfinder, Lifewright, and Null Adept provide
the original hybrid actions, while Breacher and Fortifier now add map-targeted breach and
bastion actions. Targeted actions use the same highlighted targeting mode and execution
validator as field equipment. Class identity is carried into `UnitState` rather than
inferred from text.

The Field Medkit restores five vitality to a chosen wounded colonist within three
tiles. The Field Toolkit grants Guarded to a chosen nearby colonist, and the Survey
Harness applies Disrupted to a chosen hostile within six tiles. Each costs one action
point and may be used once per mission. Arming an item highlights only targets accepted
by the same command validator used at execution time.

Focused, Guarded, Quickened, Disrupted, Hindered, Regenerating, and Adapted are serialized
timed statuses. Obscuring fields are serialized tactical geometry with explicit remaining
phase counts and expire during phase refresh.
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

After Contact completes, Kira can irreversibly evolve Neural Bloom by spending eight
biomass. Expanded Cortex grants 10 deployment accuracy and adds one food to deployment
upkeep; Echo Mind grants two weapon damage and removes one movement. Evolution options
are data-backed gift/complication pairs, the chosen evolution ID persists on the
character, and the ordinary derived-stat path applies both halves. Further mutations
can gain evolution options without character-specific simulation branches.

Mara's Chitinous Growth has the second researched pair at ten biomass. Fortress
Carapace grants two more armour and removes another movement; Razor Plating grants two
weapon damage and adds one deployment-food upkeep. The same chamber, validation,
derived-stat, autosave, and irreversible-choice paths serve both colonists.

Ilya's Regenerative Tissue adds a support/recovery pair at ten biomass. Clean Marrow
cancels the base negative medical-healing trait, returning new injuries to the normal
two-operation recovery while removing one weapon damage. Feral Renewal adds two more
health regeneration per round and one deployment-food upkeep. These options exercise
both battle refresh and post-operation recovery consumers of mutation traits.

Sol's Elastic Musculature completes researched paths for the initial roster at ten
biomass. Lattice Tendons grants two movement and removes ten accuracy. Load-Bearing
Fascia cancels the base heavy-armour-efficiency penalty and removes one weapon damage.
Armour equipment now consumes that efficiency trait, so the base complication and its
stabilizing evolution both have observable deployment effects.

Nadi Vale expands the persistent roster to five as an initially unselected Biotech
Specialist reserve. Her Symbiotic Organism evolves at twelve biomass. Cooperative
Symbiote adds two armour and one round regeneration while removing one movement;
Predatory Symbiote adds three weapon damage while removing ten accuracy and adding a
second deployment-food upkeep. Migration adds Nadi unevolved and unselected to existing
saves without disturbing their three-colonist deployment choice.

Trait hooks whose owning combat or economy system does not yet exist remain derived
values; extend the owning system rather than adding mutation-specific switches.

Mission completion grants XP. Incapacitated colonists receive an operation-counted
injury and cannot deploy until recovery completes. The infirmary can accelerate the
first active recovery.

## 7. Colony Contract

`ColonyState` owns materials, base power, food, biomass, alien components, completed
buildings, the selected construction plan, construction reservations, and a monotonic
building serial.

The colony opens in exploration mode. `ColonyExplorer` owns a continuous world-space
avatar position, exact tap destination, held keyboard/touch direction, facing,
NPC approach/conversation state, and the explicit switch to Build mode. The avatar moves
freely between plot centres and resolves its two movement axes independently against
facility and construction footprints, allowing natural collision sliding without grid
snapping. Visible held-direction controls provide the same complete touch path without a
keyboard. Tapping an inhabitant approaches to a world-space interaction radius and opens
their contextual conversation; TALK provides the same interaction while nearby.
Inhabitants and the avatar render inside the isometric depth pass so foreground plots and
structures can occlude them.

Construction and repair plot taps are available only in Build mode. This keeps ordinary
exploration taps unambiguous while retaining the existing two-tap confirmation for
material-spending actions.

Initial facilities have stable coordinates:

- Command Centre: mission access and a critical defense objective.
- Barracks: aptitude-priced class training.
- Infirmary: injury treatment.
- Workshop: equipment crafting.
- Hydroponics: three food after each resolved operation while powered.
- Power Plant: four power supply and a critical defense objective.

The colony grid always offers a 20-material Barricade, a 35-material powered Watchtower,
or a 45-material Power Plant. A powered Watchtower contributes 45-strength directional
cover to colony defence; if it is damaged or offline, its tile remains blocked but it no
longer contributes cover.
Adaptation adds one unique 50-material Gene Lab. Every building and queued project owns
only its anchor plot. Planning also requires the complete 3x3 area centred on that anchor
to be inside the colony boundary and free: the anchor plus all eight directly or diagonally
adjacent plots. Those eight plots remain independently reportable rather than pretending to
belong to the building. A prospective building is checked with its own complete 3x3 zone;
that zone cannot share any plot with the 3x3 zone of a building or queued project. Thus two
anchors separated by only two plots are still invalid even though neither anchor lies in the
other's immediate clearance ring. Campaign time never depends on wall-clock time.

Power is a derived capacity model. Base recovered power and operational Power Plants
form supply; undamaged facilities form demand. A 7-demand/8-supply starting grid has
one unit of headroom. When supply falls short, stable building order determines which
later facilities display `NO POWER` and stop satisfying their gameplay gates.
The Gene Lab adds three demand, intentionally requiring recovered power or another
Power Plant before its evolution chamber operates.

Deployment commits one food per ready squad member plus positive mutation upkeep.
Powered Hydroponics returns three food after an operation, sustaining the standard
three-person squad. If Hydroponics is offline, a powered Command Centre recovers one
emergency ration so a smaller squad can keep the campaign moving.

`ColonyState::defense_map()` derives blocked tiles, cover tiles, powered Watchtower tiles,
and critical objectives from completed buildings. An expired assault materializes those
values into the tactical mission, so colony placement and defense geometry share one
source.

A failed colony-defense outcome deterministically damages one saved operational
facility, falling back to other structures only when every facility is already down.
Damaged Barracks, Infirmary, Workshop, and Gene Lab buildings stop satisfying their
gameplay gates. The layout marks damage and its material cost directly; clicking the
building repairs it and autosaves the restored facility.

## 8. Phase One and Contact Strategy Contract

`StrategyState` owns Phase One: Isolation:

- Directorate, Brood, and Ascendant attention values from 0–100.
- A telegraphed Directorate assault with operation countdown and strength.
- Data-backed research opportunities with persistent consequences: Field
  Fortifications raises colony-defense cover integrity from 6 to 10, Xeno-Triage
  reduces new injury recovery by one operation, and Salvage Doctrine adds 8
  materials to later mission rewards.
- Data-backed character events with participant validation, disclosed costs, and
  persistent character legacies. Survey Family Routes gives Sol +1 movement;
  Documented Carapace gives Mara +1 armour. Legacy modifiers are stored on the
  recipient's character record and folded into later deployment derivation.
- Seeded, immutable mission offers and the selected mission ID.

Mission resolution adjusts the responsible faction's attention, advances threats,
and generates two offers from `campaign.json` templates. Generation uses the
serialized toolkit RNG, so saving and loading preserves the future stream.

At 20 or more attention, generated operations gain a typed faction-pressure modifier.
Directorate fire-control gives hostile attacks +10 accuracy, Brood frenzy gives hostile
units +1 movement, and Ascendant interference gives colonist attacks -10 accuracy.
The modifier is fixed on the mission offer, disclosed in selection and briefing, and
realized into unit state when the tactical session is constructed.

Mission definitions and outcomes carry materials, biomass, and power separately.
Brood operations can recover biomass, Ascendant operations can recover power, and
Directorate operations concentrate on materials. The briefing discloses the full
recovery package; victory copies it into the outcome, and debrief application updates
all three saved colony resources through one path.

When an assault reaches zero, the only offer becomes a colony-defense mission.
Winning resets the countdown and reduces strength; failure returns it sooner and
stronger. This is the first escalation loop, not the full five-phase campaign.

Isolation now has a persistent completion contract: win three operations, complete
any research doctrine, and repel a colony-defense assault. Meeting all three gates
advances the campaign header to Phase Two: Contact and grants two Alien Components
once. The colony loop remains playable after the transition and exposes the Contact
protocol and operation layer described below.

The first Contact decision spends those two components on one mutually exclusive,
data-backed protocol. Directorate Requisition adds 10 materials, Brood Cultivation
adds 4 biomass, or Ascendant Capacitor adds 2 power to every later successful
operation. The chosen protocol is saved, cannot be replaced, and is applied while
mission rewards are materialized so briefing and debrief values remain honest.

Choosing a protocol regenerates the saved offer set with one guaranteed matching
Contact operation. Black Channel, Living Chorus, and Open Circuit each use a distinct
map family and the SignalTrace contract; non-matching Contact templates remain locked.
A victorious matching trace persists its aftermath and unlocks exactly one workshop
prototype: Directorate Smartlink, Brood Living Plate, or Ascendant Phase Lens. Other
protocol prototypes remain visible but locked, making the consequence auditable.
It also exposes one protocol-gated aftermath event. Resolving that event spends food,
reduces the contacted faction's attention, and gives a permanent accuracy, health, or
movement legacy to the named participant. Migration backfills newly authored event
states into old campaigns without replaying resolved events.

Contact completes after the signal trace is won, its aftermath event is resolved, and
the unlocked prototype is equipped on any colonist. These gates are visible in the
colony hub. Meeting all three advances the saved header and summary to Phase Three:
Adaptation once; the campaign sandbox remains available afterward.

Choosing Kira's first mutation evolution regenerates the saved offer set with
`ADAPTATION: GLASS NERVE` first. The phase-gated operation uses its own Ascendant
cradle recipe and a six-round SecureAndClear contract: reach the regulator, interact
with it, and eliminate all echoes. Its briefing makes Expanded Cortex's extra
deployment-food burden visible, while either evolution changes Kira's derived combat
stats. Migrating an already evolved 1.12 save regenerates its offers so the new
operation cannot remain hidden behind serialized Contact-era content.

Mutation evolution is accessed by clicking a completed, powered Gene Lab rather than
through the generic operations panel. Its dedicated chamber lists every colonist's
mutation state, exposes researched paths, discloses biomass costs and complications,
and preserves the existing selected-colonist state. The facility can be damaged,
repaired, or disabled by insufficient power. Migrating an evolved 1.13 save adds the
lab to an open colony plot without changing the established evolution.

Adaptation has a persistent three-gate completion contract: win Glass Nerve, evolve
at least two colonists, and keep the Gene Lab operational. The colony hub displays
each gate. Meeting all three advances the saved phase to Phase Four: Escalation,
regenerates mission offers without the Adaptation-only operation, and leaves unfinished
Gene Lab paths available. A transition can occur after either the operation or the
second evolution, so both orderings share one idempotent refresh path.

Escalation begins with the phase-gated `ESCALATION: THREE KNIVES` holdout. Its
contested-rift map supports safe mirrored variants and keeps a convergence beacon under
pressure for six rounds. The operation always applies Three-Power Crossfire: hostiles
gain 5 accuracy and 1 movement while colonists lose 5 accuracy. This compound modifier
is disclosed before deployment and applied through the same tactical pressure path as
the three faction-specific modifiers. Its recovery includes materials, biomass, and
power. Migrating an already transitioned 1.18 save regenerates offers so Phase Four is
immediately playable.

Three Knives overrides the usual single-faction roster with explicit hostile unit IDs:
one Directorate rifleman, one Brood sporecaster, and one Ascendant rift warden. The
selection persists on generated mission offers and materializes into the tactical
mission, while ordinary operations continue selecting all hostiles from their primary
faction. Mixed units share the hostile AI and crossfire modifier but retain their own
roles, stats, names, positions, and specialist effects. Older Phase Four offers are
regenerated so an unstarted Three Knives battle gains the mixed deployment.

Winning Three Knives persists `escalation_operation_completed` and exposes one
irreversible, data-backed convergence response in the colony hub. Bastion Beacon spends
30 materials and adds two operations to active assault countdowns. Living Decoy spends
8 biomass and removes 8 attention from every faction. Weaponized Lattice spends 4 power,
adds 5 attention to every faction, and adds 8 materials to all later victorious mission
recovery. The selected response ID is saved, the resource and pressure effects apply
once, and the hub keeps the committed response visible afterward.

Committing a response regenerates the serialized offer set with one matching follow-up
operation prioritized. Bastion Beacon opens `BREAKWATER`, a Directorate elimination
siege on the beacon-bastion map. Living Decoy opens `FALSE HEART`, a Brood extraction
through the decoy warrens. Weaponized Lattice opens `LIVE WIRE`, an Ascendant signal
trace inside a lattice storm. All three retain the Phase Four crossfire modifier while
changing faction, objective, recovery, and map family. Response-locked templates remain
unavailable before the choice and mutually exclusive afterward.

Escalation has a persistent three-gate completion contract: win Three Knives, commit
one convergence response, and win the response-locked follow-up operation. The colony
hub exposes the operation, response, and branch state. Meeting all three gates advances
the saved campaign to `PHASE FIVE // MIREXIS`, updates its summary, and regenerates a
playable offer pool while dedicated Phase Five content is developed.

Phase Five immediately exposes one irreversible, data-backed identity choice. Human
Redoubt spends 45 materials and adds 4 integrity to colony-defense cover. Living
Commonwealth spends 14 biomass and removes 1 food from each deployment cost. Open
Threshold spends 6 power and adds 3 power to every later victorious mission recovery.
Costs, effects, and affordability are visible in the colony hub; the chosen path ID is
saved and continues to modify the existing defense, supply, or recovery calculation.

Committing that choice immediately regenerates mission offers around one path-locked
Phase Five operation. Human Redoubt opens `LAST WALL`, a Directorate elimination battle
through a breached settlement perimeter. Living Commonwealth opens `ROOT CHOIR`, a
Brood signal trace in a living garden. Open Threshold opens `DOOR OF LIGHT`, an
Ascendant secure-and-clear operation at the colony's new gate. These templates remain
mutually exclusive, carry distinct objectives and map families, and are prioritized
for both new campaigns and migrated saves that already committed an identity.

Winning the operation belonging to the committed path sets the persistent Mirexis
operation and campaign completion gates. The selected path supplies a data-backed ending
title, revelation, and colony legacy: a sovereign redoubt, a planetary symbiosis, or an
open link to the intelligence beneath Mirexis. The finale operation retires from the
offer pool, the debrief and colony hub expose the ending, and generic post-campaign
operations remain available so a completed save is still playable.

## 9. Content Registry

Current embedded files under `assets/data/` are:

| File | Content |
|---|---|
| `game_config.json` | Identity, save version, grid, AP budget, root seed |
| `mission.json` | Authored Glassroot mission and tactical map data |
| `roster.json` | Tactical baselines and faction-tagged hostile archetypes |
| `recruitable_roster.json` | Tactical baselines for route-exclusive recruits |
| `characters.json` | Persistent recruits, outsider origins, and aptitude/loadout references |
| `classes.json` | Class families and deployment modifiers |
| `mutations.json` | Gift and complication hooks |
| `equipment.json` | Starter and protocol-gated Contact equipment modifiers |
| `campaign.json` | Isolation through Adaptation factions, protocols, research, events, missions, and maps |
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
training, treatment, crafting, repair, research, character-event resolution, deployment, and
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
| 1.4.0 | Hydroponics, Power Plant, construction selection, and power-model rebasing |
| 1.5.0 | Persistent Isolation victory, doctrine, and repelled-assault phase progress |
| 1.6.0 | Persistent mutually exclusive Contact protocol choice |
| 1.7.0 | Protocol-gated Contact offers and the SignalTrace objective contract |
| 1.8.0 | Persistent trace aftermath and protocol-gated workshop prototypes |
| 1.9.0 | Protocol-gated aftermath events with faction and character consequences |
| 1.10.0 | Persistent Contact completion and Adaptation transition |
| 1.11.0 | Persistent mutation evolution choice and derived gift/complication fields |
| 1.12.0 | Phase-gated Adaptation operation offers for already evolved campaigns |
| 1.13.0 | Gene Lab infrastructure for already evolved Adaptation campaigns |
| 1.14.0 | Data-backed Chitinous Growth evolution paths for unevolved Mara saves |
| 1.15.0 | Data-backed Regenerative Tissue evolution paths for unevolved Ilya saves |
| 1.16.0 | Data-backed Elastic Musculature evolution paths for unevolved Sol saves |
| 1.17.0 | Persistent Glass Nerve and Adaptation completion gates |
| 1.18.0 | Phase-gated Escalation operation offers for transitioned campaigns |
| 1.19.0 | Persistent Three Knives victory and convergence-response choice state |
| 1.20.0 | Response-locked Escalation offers for already committed campaigns |
| 1.21.0 | Explicit mixed-power hostile deployment for saved Escalation offers |
| 1.22.0 | Persistent response-branch victory and Escalation completion gates |
| 1.23.0 | Persistent Phase Five Mirexis identity path selection |
| 1.24.0 | Path-locked Phase Five offers for already committed identities |
| 1.25.0 | Persistent final-operation and campaign-completion gates |
| 1.26.0 | Nadi Vale reserve recruitment and Symbiotic Organism evolution paths |
| 1.27.0 | Data-backed advanced classes; existing class history and level remain valid |
| 1.28.0 | Inspectable equipment definitions and weapon-profile overrides |
| 1.29.0 | Persistent pair bonds, shared-victory history, and derived deployment bonuses |
| 1.30.0 | Save-stable per-unit overwatch state, defaulting older tactical saves to disarmed |
| 1.31.0 | Persistent per-character trauma histories, defaulting older rosters to unscarred |
| 1.32.0 | Defense-asset integrity fields, inactive by default for older tactical saves |
| 1.33.0 | Tactical faction identity and once-per-phase hostile ability use state |
| 1.34.0 | Save-stable realized hazard tiles, defaulting older active battles to none |
| 1.35.0 | No new fields; enemy intent remains derived from existing tactical state |
| 1.36.0 | No new fields; player action previews remain derived from tactical state |
| 1.37.0 | No save fields; tactical help visibility is battle-screen UI state only |
| 1.38.0 | No save fields; battle-log visibility is battle-screen UI state only |
| 1.39.0 | No save fields; briefing threat intel derives from materialized mission data |
| 1.40.0 | No new fields; briefing loadouts derive from persistent colonist records |
| 1.41.0 | No new fields; danger ratings derive from mission definitions and realized geometry |
| 1.42.0 | No new fields; forecasts derive from mission rules and serialized reinforcement queues |
| 1.43.0 | No new fields; entry markers read the existing queued unit positions one round early |
| 1.44.0 | No new fields; transient combat feedback derives from newly appended battle events |
| 1.45.0 | No new fields; hostile-phase playback is bounded presentation state only |
| 1.46.0 | No new fields; formation choice is applied only while creating a new session |
| 1.47.0 | No new fields; end-phase confirmation is transient application UI state |
| 1.48.0 | No new fields; next-ready selection changes existing selected unit and tile only |
| 1.49.0 | No new fields; readiness markers derive from existing unit AP and incapacitation state |
| 1.50.0 | No new fields; vitality and effective armour derive from existing unit combat state |
| 1.51.0 | No new fields; movement routes reuse deterministic tactical pathfinding state |
| 1.52.0 | No new fields; cover rendering reads existing directional edge definitions |
| 1.53.0 | No new fields; invalid previews expose existing command-validation errors |
| 1.54.0 | No new fields; shot tracers and cover breakdown derive from validated attacks |
| 1.55.0 | No new fields; threat envelopes derive from current hostile range and line of fire |
| 1.56.0 | No new fields; danger reach derives from one validated move and remaining attack AP |
| 1.64.0 | Persistent first-hour stage, tactical lesson, outcomes, investment, and guide settings; 1.63 saves infer arrival, investment, or completion from resolved operations |
| 1.65.0 | Persistent first-hour elapsed time, milestone timings, operation duration/round counts, invalid commands, and guide opens; 1.64 onboarding progress remains exact while metrics default safely |
| 1.67.0 | Soldier, Defender, and Scout technique pairs, active loadouts, tactical targeting, and phase-use migration |
| 1.68.0 | Medic and Engineer technique pairs, one-round equipment overcharge state, and legacy tactical default migration |
| 1.69.0 | Psionic and Biotech technique pairs, hazard adaptation, obscuring fields, and tactical runtime migration defaults |
| 1.70.0 | Breacher and Fortifier hybrid class actions, tile-targeted class-action input, and advanced-class migration defaults |
| 1.71.0 | Rescue Specialist and Chorus Warden hybrid class actions, hazard conversion events, and advanced-class migration defaults |
| 1.72.0 | Waystation construction, Directorate Exile recruitment, origin-derived profiles, Exile Cipher action, and outsider-arc migration defaults |
| 1.73.0 | Commons construction, once-per-operation shared meals, relationship progression, and Commons migration defaults |
| 1.74.0 | Relay Mast construction, once-per-operation route scans, mission refresh, attention exposure, and relay migration defaults |
| 1.75.0 | Watchtower construction, power-dependent directional cover in colony defence, and Watchtower migration compatibility |

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

`scripts/capture_ui.ps1` captures `title`, `colony`, `contact`, `damage`, `power`,
`construction`, `research`, `roster`, `advanced_roster`, `relationships`, `trauma`, `bonded_briefing`, `contact_gear`, `contact_event`, `adaptation`, `gene_lab`, `evolution`,
`mara_evolution`, `ilya_evolution`, `sol_evolution`, `nadi_evolution`, `escalation`, `escalation_operation`, `escalation_response`, `mirexis`, `mirexis_path`,
`redoubt_end`, `commonwealth_end`, `threshold_end`,
`finale_debrief`,
`adaptation_operation`, `glass_nerve`, `thin_shelter`, `breakwater`, `false_heart`, `live_wire`, `last_wall`,
`root_choir`, `door_of_light`, `legacy`,
`briefing`, `threat_briefing`, `loadout_briefing`, `pressure`, `gameplay`, `brood_ability`, `directorate_ability`, `ascendant_ability`, `hazard`, `intent`, `action_preview`, `movement_route`, `cover_edges`, `invalid_command`, `valid_shot`, `threat_range`, `danger_reach`, `help`, `battle_log`, `combat_feedback`, `phase_replay`, `end_phase_guard`,
`extraction`, `variant`, `sporefield`, `vault`, `three_knives`, `reinforcement_warning`, `line_formation`, `black_channel`, `living_chorus`,
`open_circuit`, `trace_active`, `readiness_markers`, `vitality_markers`,
`equipment`, `weapon_profile`, `overwatch`, `class_target`, `breach`, `debrief`, and `trauma_debrief` by default. Capture setup seeds
each scene deterministically, including objective, doctrine, legacy, pressure-modifier,
new-operation battlefield, and targeting states.
Committed captures under `docs/verification/` are the visual regression references.

## 13. Verification

The completion baseline is:

- `cargo fmt -- --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test` (290 Mirexis unit tests plus asset-registry and source-size gates)
- deterministic eighty-two-scene capture with visual inspection
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

These milestones record implementation breadth from the original technical roadmap.
Production Phase 1 now adds campaign-owned first-hour onboarding and a procedural audio
foundation. Balance, accessibility hardening, and player-experience validation remain in
progress and are tracked by the playable-game roadmap rather than these old milestones.

## 15. Current Boundaries and Next Development

The original systems roadmap is complete, but Mirexis remains a tech demo. Preserve
these explicit boundaries when continuing:

- Escort and additional multi-stage contracts remain beyond the six implemented objective
  types. Thin Shelter now provides a destructible defense-target contract.
- Prepaid single-shot overwatch now reacts to hostile movement in range and line of fire.
  Incapacitation now adds one of three bounded persistent tradeoff scars after battle.
  Elevation, in-battle injury statuses, additional reaction types, permanent death, and
  presentation animation consumers remain bounded and procedural event audio now covers
  movement, attacks, damage, abilities, objectives, outcomes, and scene ambience.
- The eighteen current map recipes support authored and safe mirrored layouts; nine
  early and Contact battlefields now carry faction hazards. Additional transforms,
  elevation, spawn recipes, and battlefield families remain future work.
- The colony has fixed core facilities and placeable Barricades, Power Plants, one
  Adaptation-gated Gene Lab, one Contact-gated Waystation, a Commons, and a Relay Mast;
  population and free placement for every building remain.
- Pair relationships now grow from shared victories and character events, and trusted
  deployed partners grant bounded, non-stacking accuracy and armour bonuses. Rivalries,
  romances, bespoke relationship scenes, permanent death, and broader armour/tool
  equipment families remain future content.
- Isolation has a visible completion gate and advances into a persistent Contact
  state with one faction-flavoured economic protocol and matching signal-trace
  operation whose victory unlocks a matching equipment prototype and aftermath event.
  Completing all three Contact gates advances into Adaptation. Neural Bloom has the
  first irreversible gift/complication evolution and its own scavenging operation;
  two evolved colonists and an operational Gene Lab complete Adaptation and enter
  Escalation. Three Knives fields all three enemy powers under a compound pressure
  modifier, and its convergence response creates the first
  persistent Phase Four strategic tradeoff with a matching follow-up operation. Winning
  that branch completes Escalation and enters Phase Five, where one identity path changes
  an existing colony economy and opens a dedicated operation against one of the three
  powers. Winning it reveals a path-specific truth and persists one of three campaign
  endings while leaving optional post-campaign operations available.
- Saved content references need explicit validation before definitions can be removed
  or renamed safely.

The next coordinated body of work is Phase 1 of the playable-game roadmap in
`docs/PLAYABLE_GAME_PHASE_1.md`. Small isolated feature additions should not displace
its first-hour flow, onboarding, pacing, combat-feel, and validation priorities.
