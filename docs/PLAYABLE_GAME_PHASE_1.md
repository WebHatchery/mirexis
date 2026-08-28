# Playable Game Roadmap — Phase 1

Status: planned  
Created: 28 August 2026  
Product target: a cohesive, enjoyable, and testable first hour

> This is production Phase 1 of moving Mirexis from tech demo to playable game. It is
> separate from the in-fiction campaign phase named Isolation and from the completed
> technical milestones recorded in `TECHNICAL_DESIGN.md`.

## 1. Purpose

Mirexis already contains an unusually broad set of connected systems. The risk is no
longer whether city exploration, combat, progression, pressure, and campaign branching
can be implemented. The risk is that a new player meets all of them as prototype parts
instead of experiencing a clear game.

Phase 1 turns the existing foundation into a deliberately authored first-hour build.
It should establish the fantasy of protecting a strange frontier home, teach the player
without requiring outside instructions, deliver two satisfying operations, and make the
player want to continue.

This phase favours depth, pacing, feedback, and reliability over additional systems.

## 2. Player outcome

By the end of the first session, a new player should be able to say:

- I know who these colonists are and why their city matters.
- I understand what I can do in the city and why I should return there.
- I can select, move, attack, use cover, read danger, and complete an objective.
- I made at least one meaningful squad choice and one meaningful city choice.
- I saw those choices affect a later scene or operation.
- I know the immediate campaign goal and what threatens the colony.
- I trust save, continue, defeat, and recovery to preserve my progress.

The target session is 45–75 minutes for a first-time player, ending after the second
operation and its city consequence. Exact timing must be measured in playtests rather
than treated as a fixed content quota.

## 3. Phase boundaries

### In scope

- A fully authored new-game opening and first-hour sequence.
- Contextual, touch-first onboarding for city and tactical play.
- First-hour pacing, encounter tuning, economy tuning, and recovery tuning.
- Clear city purpose, character introductions, and visible post-operation consequences.
- Combat responsiveness, feedback, legibility, and reduction of unnecessary input.
- Audio foundations and a minimum coherent sound pass.
- Save/continue, defeat, recovery, and browser-resize reliability.
- Structured internal and external playtesting with issue triage.

### Out of scope

- New campaign phases or ending branches.
- A large increase in recruit, class, mutation, item, enemy, or mission counts.
- Elevation, permanent death, romance, or other major unproved systems.
- Final campaign-wide balance or final-release content volume.
- Final art replacement for every prototype asset.
- Localisation beyond ensuring text systems can accommodate later localisation.

Existing systems may be simplified, delayed, or hidden during the first hour when they
compete with comprehension. Implemented breadth is not a reason to expose everything at
once.

## 4. The first-hour golden path

The build should deliberately support this sequence while still allowing exploration:

1. **Arrival:** Start a new campaign, understand the immediate crisis, and gain control
   without reading a manual.
2. **Meet the colony:** Walk to one clearly signposted person, learn the city interaction
   model, and meet the first deployable colonists.
3. **Prepare:** Inspect one operation, choose a small squad, and understand the objective
   and danger at a glance.
4. **First operation:** Learn selection, movement, cover, attack, enemy phase, an ability,
   and the objective through staged situations.
5. **Return:** See injuries, rewards, relationships, and the operation's visible effect
   in the city before being asked to manage resources.
6. **Choose:** Make one city or squad investment with a clear opportunity cost.
7. **Second operation:** Apply learned rules in a less scripted encounter with one new
   enemy behaviour or objective pressure.
8. **Promise:** Return to a changed city, reveal the next threat, autosave, and present a
   clear reason to continue.

Failure during either operation must lead to an understandable recovery route. It must
never strand the campaign, silently erase progress, or require a keyboard.

## 5. Workstreams

### 5.1 First-session direction and onboarding

Create a small onboarding state owned by campaign progress, not a brittle sequence of UI
screens. Each step should respond to the player's current context and complete from the
same validated action used in normal play.

Required work:

- Establish the opening situation, immediate goal, and first destination in under one
  minute of normal play.
- Introduce only the control needed for the player's next meaningful action.
- Use exact visible-control or gesture language for every prompt.
- Highlight the relevant world target or control without blocking unrelated inspection.
- Allow prompts to be revisited from a visible help surface.
- Make the tutorial restartable and skippable, with skip never removing essential goals.
- Persist tutorial progress and test migrated saves with no tutorial fields.
- Remove or defer panels whose information is not actionable in the first session.

Acceptance:

- A player unfamiliar with Mirexis completes the golden path without developer help,
  external documentation, or keyboard input.
- No prompt asks for an action that lacks a visible tap/click route.
- Returning after a save resumes at the correct goal with the necessary context.

### 5.2 City as home and preparation space

The city should connect emotional attachment to tactical preparation. Walking is not the
goal; meeting people, seeing consequences, and making choices are.

Required work:

- Give the first required NPC a strong landmark, readable approach, and short interaction
  distance.
- Author concise introductory conversations for the starting squad and one coordinator.
- Surface the next primary goal in the environment and city HUD.
- Make interactable colonists and facilities readable without turning the city into a
  field of permanent labels.
- Stage at least one post-operation conversation that changes according to outcome.
- Show one physical or visual city consequence after each of the first two operations.
- Ensure travel time between first-hour destinations supports atmosphere without delay;
  add a visible fast route if repeated traversal becomes friction.
- Audit construction and management options so only timely, understandable choices are
  foregrounded.

Acceptance:

- Testers can identify the current city goal and its location within ten seconds.
- Each city visit contains a character beat, a preparation decision, and a visible change.
- No required interaction depends on discovering a keyboard shortcut or a precise pixel.

### 5.3 Tactical flow and combat feel

The tactical rules are established; Phase 1 makes their use fast, readable, and
satisfying.

Required work:

- Author the first operation as a teaching encounter rather than a normal generated map.
- Introduce core rules in this order: select, move, cover, attack, enemy phase, objective,
  then one class or mutation action.
- Tune the second operation to test learning without tutorial scripting.
- Reduce low-value clicks between intent and command execution.
- Review camera framing, cursor focus, target priority, end-phase flow, and hostile replay
  speed on mouse and touch.
- Give hits, misses, armour, cover destruction, injury, objective progress, and victory
  distinct visual and audio responses.
- Expand inspected status information so active effects and duration are explicit.
- Keep combat forecasts accurate; never use animation or presentation that contradicts
  deterministic results.
- Establish mission-duration and inactive-wait targets, then measure them in playtests.

Initial tuning targets:

| Measure | First operation | Second operation |
|---|---:|---:|
| Expected rounds | 4–6 | 6–9 |
| First-time duration | 12–20 min | 18–28 min |
| New mechanics introduced | Core commands only | One faction behaviour and one pressure |
| Restart required for ordinary mistakes | No | No |

These are starting hypotheses. Update them from recorded sessions.

Acceptance:

- Players can explain why an attack hit or missed and why damage changed.
- Players recognise the objective before the end of round one.
- Hostile activity remains readable but does not dominate mission time.
- Touch play can complete both operations, including tutorial and recovery actions.

### 5.4 Progression, economy, and consequences

The first hour needs a small, legible decision space. It should demonstrate long-term
promise without presenting the entire campaign economy.

Required work:

- Define expected starting resources, first-operation rewards, recovery costs, and the
  two or three valid first investments.
- Ensure the first meaningful investment changes the second operation or its preparation.
- Explain unavailable choices through clear prerequisites, not unexplained disabled UI.
- Prevent an early defeat or unlucky injury from creating an unrecoverable economy.
- Make XP, injury, relationship, equipment, and resource changes visible once, then
  discoverable for later inspection.
- Decide which advanced systems remain hidden until after the first-hour promise beat.

Acceptance:

- Testers can state what they gained, what they spent, and how the choice helped.
- Every offered first-hour choice is viable; one option is not mathematically dominant.
- Victory and defeat both move the campaign forward, with different costs and tone.

### 5.5 Character and narrative pass

Phase 1 needs a few memorable people rather than more lore volume.

Required work:

- Define a one-sentence role, want, tension, and speaking style for each starting speaker.
- Keep first conversations brief and responsive to player action.
- Establish one relationship contrast inside the initial squad.
- Connect mission briefings and debriefs to named colonists rather than generic system
  narration wherever practical.
- Record continuity flags for the decisions that first-hour dialogue acknowledges.
- Edit all first-hour text for clarity, voice, mobile line length, and repetition.

Acceptance:

- Playtesters can recall at least two colonists by name and describe the difference
  between them.
- Dialogue reinforces goals and consequences without restating UI text.

### 5.6 Audio and presentation foundation

Audio is currently an explicit product-readiness gap. Phase 1 does not need a final
score, but it needs a coherent response language.

Required work:

- Add volume controls and persistent mute settings before relying on audio cues.
- Establish a small sound palette for focus/click, invalid action, movement, weapon hit,
  weapon miss, damage, ability, objective update, victory, and defeat.
- Add restrained city and tactical ambience with clean state transitions.
- Define attribution and source records for every added asset.
- Verify missing audio fails clearly without blocking the game.
- Audit animation timing, screen transitions, and text cadence alongside sound rather
  than as independent polish.

Acceptance:

- Critical events remain understandable while muted.
- Repeated UI and combat sounds do not clip, stack excessively, or fatigue testers.
- Settings persist across native and browser sessions.

### 5.7 Reliability, accessibility, and recovery

Required work:

- Test new game, autosave, manual tactical save, continue, defeat, restart, and browser
  refresh across the golden path.
- Add clear confirmation or recovery for destructive campaign actions.
- Verify common desktop browser sizes and touch interaction at every required step.
- Audit text scale, contrast, colour-independent meaning, and motion/readability settings.
- Ensure input focus, overlays, and resizing cannot issue unintended tactical commands.
- Keep save migrations explicit for every schema change made during the phase.
- Replace current verification captures when the same scene changes; do not create
  duplicate evidence folders.

Acceptance:

- No known progression blocker, save corruption, inaccessible required action, or crash
  remains in the first-hour path.
- The standard publisher passes from a clean checkout.

## 6. Delivery order

Work should proceed in vertical slices so each checkpoint is independently playable and
committable.

### Checkpoint A — Lock the experience

- Write the first-hour beat sheet and choose the two operations.
- Record the systems and UI deferred until later.
- Establish baseline timings from at least three complete internal runs.
- Turn observed friction into a prioritised issue list.

### Checkpoint B — Playable opening

- Implement arrival, first NPC direction, contextual city prompts, and squad introduction.
- Persist onboarding state and verify touch-only completion to briefing.
- Validate and commit as one coherent opening slice.

### Checkpoint C — First operation and return

- Build and tune the teaching operation.
- Add combat-feedback and minimum audio responses required by that operation.
- Implement debrief, visible city consequence, and recovery behaviour.
- Playtest the complete arrival-to-return sequence before expanding it.

### Checkpoint D — Choice and second operation

- Implement the first meaningful investment and its tactical consequence.
- Tune the less-scripted second operation.
- Add the second city consequence and next-threat promise beat.

### Checkpoint E — Hardening

- Complete settings, accessibility, save, resize, touch, and failure-path audits.
- Run external first-time playtests without coaching.
- Fix all blockers and high-severity comprehension or pacing failures.
- Refresh verification references and run the required publisher.

Major checkpoints should be validated and committed separately. Avoid accumulating
onboarding, balance, narrative, audio, and unrelated refactors in one change.

## 7. Playtest method

Every observed session should record:

- device, window size, and input method;
- total session time and operation round counts;
- time to first movement, first city interaction, and first tactical attack;
- prompts reread, ignored, or misunderstood;
- moments the player asks what to do or why something happened;
- accidental commands, invalid-command frequency, and idle hostile time;
- defeat, restart, save, load, and recovery behaviour;
- two favourite moments and two points of friction;
- which characters, goals, and choices the player recalls afterward.

Do not explain controls during an observed first-time test unless the session is blocked.
If help is required, record the exact point and wording. Fix recurring causes rather than
adding a generic instruction panel.

Severity order:

1. Crash, corrupt save, progression blocker, or inaccessible required action.
2. Player cannot identify the goal or understand a consequential result.
3. Player can proceed but encounters repeated friction, delay, or accidental input.
4. Presentation inconsistency or low-impact polish issue.

Phase 1 requires at least five uncoached first-time sessions after Checkpoint D, including
at least two touch-primary sessions. Small samples guide iteration; they are not evidence
of market fit.

## 8. Definition of done

Phase 1 is complete only when all of the following are true:

- The golden path is implemented from new game through the second-operation promise beat.
- Five first-time players can complete it without external instructions; any assistance
  required is documented and resolved or explicitly accepted.
- At least two successful end-to-end runs use only visible touch/click controls.
- The first operation teaches the core tactical loop and the second demonstrates transfer.
- The city provides direction, preparation, character connection, and visible consequence.
- The first investment has a clear and observable effect.
- Victory, defeat, save, continue, refresh, and resize paths are verified.
- Required audio/settings foundations exist, while all critical information remains visual.
- No open severity 1 or severity 2 issue remains in the first-hour path.
- Documentation reflects the shipped behaviour and all replaced verification captures are
  current.
- `./publish.ps1` passes with no parameters and the result is committed using project
  conventions.

Completing Phase 1 does not make Mirexis content-complete. It produces the reliable,
measurable player-experience baseline required to plan Phase 2: campaign depth, content
production, and longer-term balance.
