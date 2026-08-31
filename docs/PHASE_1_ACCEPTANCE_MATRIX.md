# Phase 1 Acceptance Matrix

Status: implementation evidence audited; observed-session gates pending
Audit baseline: save/content 1.209.0, validated and committed with this matrix

This matrix maps every Phase 1 workstream and definition-of-done requirement to
authoritative evidence. `VERIFIED` means code, tests, captures, or an operator-run path
proves the implementation requirement. `HUMAN PENDING` means the requirement asks what
an unfamiliar player understands, recalls, prefers, or can complete; automation and the
developer are not substitutes for that evidence.

## Definition of done

| ID | Requirement | Evidence | Status |
|---|---|---|---|
| D01 | Golden path reaches the second-operation promise beat | `FirstHourStage`, `first_hour/tests.rs`, `ui_briefing.png`, `ui_debrief.png`, and `ui_colony.png` | VERIFIED |
| D02 | Five first-time players complete without external instructions | Five empty required rows in `PHASE_1_PLAYTEST_LOG.md` | HUMAN PENDING |
| D03 | At least two end-to-end runs use visible touch/click controls only | Touch-primary rows 2 and 4 await observed runs | HUMAN PENDING |
| D04 | Operation one teaches the core loop and operation two demonstrates transfer | Ordered `TacticalLesson` transitions, a cover-tile-gated first move, reachable opening firing lane and objective route, guided forecast confirmation, contextual tactical focus, a persisted `SecondOperationTactical` focus, colony-side handoff focus, usable ability recovery, and unscripted second-operation play; player transfer must still be observed | IMPLEMENTED / HUMAN PENDING |
| D05 | City supplies direction, preparation, character connection, and visible consequences | `first_hour_ui.rs`, `first_hour_investment_ui.rs`, colony conversations plus prerequisite-aware Operations treatment, research, and character-event controls, separated first-hour briefing and character-event lanes, outcome-sensitive `first_hour_consequences_ui.rs`, clear debrief goal/result layering, fielded-versus-reserve squad disclosure including recruited fielded colonists, recruited-roster list coverage, deployed-squad XP disclosure, and colony/debrief captures | VERIFIED |
| D06 | First investment has a clear observable effect | Three 24-material preparations, persistent Operations/tactical breadcrumbs in `ui_first_hour_operations.png` and `ui_second_operation_tactical.png`, and `first_investment_changes_only_the_second_deployment` | VERIFIED |
| D07 | Victory, defeat, save, continue, refresh, and resize paths are verified | `PHASE_1_HARDENING.md`, persistence/outcome tests including finished-save debrief restoration, and the recorded WebGL operator pass | VERIFIED |
| D08 | Audio/settings exist and critical information remains visual | `audio.rs`, audio tests, settings UI, settings-modal input isolation, event prose/markers, and browser persistence pass; fatigue remains an observation target | VERIFIED / HUMAN PENDING |
| D09 | No severity 1 or 2 first-hour issue remains | No implementation-known severity 1/2 issue; five-session issue discovery and triage have not occurred | HUMAN PENDING |
| D10 | Documentation and verification captures match the build | 100-scene manifest, scene-local capture reset, Contact event Operations reference, acceptance matrix test, `capture_audit.json`, recruited-roster briefing/roster/Gene Lab captures, `ui_first_hour_guide.png`, `ui_first_hour_return.png`, `ui_first_hour_dialogue.png`, `ui_first_hour_promise.png`, `ui_first_hour_ability.png`, `ui_second_operation_tactical.png`, `ui_first_hour_operations.png`, `ui_first_hour_tactical.png`, `ui_first_hour_attack.png`, `ui_first_hour_enemy_phase.png`, `ui_first_hour_objective.png`, and `ui_first_hour_replay.png` | VERIFIED |
| D11 | Parameterless publisher passes and changes use project commit conventions | Recorded in `PHASE_1_HARDENING.md`; rerun and commit required after each audit fix | VERIFIED AT LAST CHECKPOINT |

## Workstream audit

### 5.1 First-session direction and onboarding

| Required work | Evidence | Status |
|---|---|---|
| Opening situation, immediate goal, and destination | Arrival goal, Mara marker/landmark, first-return Ilya marker, and route traces | VERIFIED |
| Introduce only the next meaningful control | Stage-owned `primary_goal` and ordered tactical lesson | VERIFIED |
| Exact visible-control or gesture language | Prompts consistently name Tap, Drag, CONTINUE, OPERATIONS, DEPLOY, ATTACK, END PHASE, HELP, NEXT READY, and visible targets | VERIFIED |
| Highlight relevant target without blocking inspection | Non-blocking goal banner, Mara/Ilya markers and route traces, colony Operations/briefing/preparation focus, tactical overlays, and lesson-specific map/command focus | VERIFIED |
| Visible revisitable help | FIRST-HOUR FIELD GUIDE and tactical HELP | VERIFIED |
| Restartable and skippable prompts without removing goals | Stage-preserving `restart`, current-lesson reset for tactical stages, `guidance_enabled`, and first-hour tests | VERIFIED |
| Persist progress and migrate saves with no tutorial fields | Save schema 1.65 and migration fixtures for 1.63 and 1.64 | VERIFIED |
| Defer non-actionable panels | Deferred-system list and first-hour gating in colony controls | VERIFIED |

Acceptance that an unfamiliar player completes without help or keyboard and resumes with
the right understood context remains part of H01–H04 below.

### 5.2 City as home and preparation space

| Required work | Evidence | Status |
|---|---|---|
| Strong first-NPC landmark, readable approach, short interaction | Mara landmark, Ilya recovery marker, approach paths, interaction distance, and exact city goals | VERIFIED |
| Concise introductions for coordinator and starting squad | Authored speaker table and colony dialogue | VERIFIED |
| Environmental and HUD primary goal | Goal banner plus highlighted world/control targets | VERIFIED |
| Readable colonists and facilities without permanent labels | Speech markers, proximity dialogue, building art/state | VERIFIED |
| Outcome-sensitive post-operation conversation | Ilya debrief line and first-return conversation state | VERIFIED |
| Physical/visual consequence after each operation | Outcome-sensitive refuge and outer-route consequence drawing | VERIFIED |
| Travel supports atmosphere without delay | Direct tap-to-walk, short Mara route, destination trace, visible OPERATIONS shortcut | IMPLEMENTED / HUMAN PENDING |
| Timely choices foregrounded | Three dedicated preparations; advanced systems deferred | VERIFIED |

Ten-second goal recognition, city attachment, and traversal friction require observation.

### 5.3 Tactical flow and combat feel

| Required work | Evidence | Status |
|---|---|---|
| Authored teaching operation | `operation_glassroot` and first-operation stage | VERIFIED |
| Core-rule order | `TacticalLesson`: select, cover move gated by entering a cover tile, attack through the reachable opening firing lane, enemy phase, objective within the guided travel window, ability | VERIFIED |
| Less-scripted second operation | Generated Isolation offer with `ApplyLearning` guidance only | VERIFIED |
| Reduce low-value clicks | Direct tile/hostile intents outside the lesson; guided attack forecast confirmation, visible actions, and guarded end phase | VERIFIED |
| Camera, focus, targeting, end phase, and replay review | Camera/input tests and tactical capture set | VERIFIED |
| Distinct visual/audio results | Event feedback, procedural palette, battle log, and tactical captures | VERIFIED |
| Active effects and duration explicit | Status rail now displays each remaining phase count; dedicated unit test | VERIFIED |
| Forecast agrees with deterministic resolution | Action-preview and deterministic attack tests | VERIFIED |
| Duration and inactive-wait targets measured | Persisted operation duration/round metrics are ready; three internal and five external samples absent | HUMAN PENDING |

Player explanations of attacks, objective recognition, hostile pacing, and touch completion
remain observed-session requirements.

### 5.4 Progression, economy, and consequences

| Required work | Evidence | Status |
|---|---|---|
| Starting resources, rewards, recovery, and valid investments defined | `PHASE_1_BEAT_SHEET.md` and embedded data | VERIFIED |
| Investment changes operation two | Armour, accuracy, or movement bonus plus unit test | VERIFIED |
| Unavailable choices explain prerequisites | Warning text and lock reasons across first-hour surfaces | VERIFIED |
| Defeat/injury cannot deadlock economy | Emergency 24-material reserve and recovery tests | VERIFIED |
| XP, automatic technique learning, injury, relationships, equipment, and resources visible then inspectable | Debrief notification plus roster/loadout/colony surfaces | VERIFIED |
| Advanced systems hidden until promise | Deferred list and stage-aware controls | VERIFIED |

Recall of gains, spending, and benefit—and perceived balance among the choices—requires
the observed sessions.

### 5.5 Character and narrative pass

| Required work | Evidence | Status |
|---|---|---|
| Role, want, tension, and voice for each speaker | Authored-speaker table in `PHASE_1_BEAT_SHEET.md` | VERIFIED |
| Brief action-responsive first conversations | Mara arrival and outcome-aware return dialogue | VERIFIED |
| Initial relationship contrast | Mara readiness versus Ilya recovery | VERIFIED |
| Named briefing/debrief voices | Mara briefing and Ilya debrief copy | VERIFIED |
| Continuity flags | First/second outcome and investment fields in `FirstHourProgress` | VERIFIED |
| First-hour text clarity and mobile length | Touch-language audit and canonical captures | VERIFIED / HUMAN PENDING |

Name recall, differentiation, and repetition are human comprehension gates.

### 5.6 Audio and presentation foundation

| Required work | Evidence | Status |
|---|---|---|
| Persistent volume and mute before cue reliance | Independent `audio_preferences` slot and settings controls | VERIFIED |
| Required response palette | `SoundCue` variants and palette coverage test | VERIFIED |
| Restrained city/tactical ambience and transitions | `AudioScene`, low ambience multiplier, and scene synchronization | VERIFIED |
| Attribution/source record | `AUDIO_SOURCES.md` documents generated sources and future policy | VERIFIED |
| Missing audio reports clearly without blocking play | `load_failures` notification path; critical presentation is not audio-owned | VERIFIED |
| Timing, transitions, and text cadence audited with sound | Reduced-motion/replay handling and one-cue-per-update policy | IMPLEMENTED / HUMAN PENDING |

Clipping, stacking, fatigue, and muted comprehension require listening sessions.

### 5.7 Reliability, accessibility, and recovery

| Required work | Evidence | Status |
|---|---|---|
| New game, autosave, tactical save, continue, defeat, restart, refresh | Hardening matrix and persistence/outcome tests | VERIFIED |
| Destructive confirmation/recovery | Two-step new-colony and delete-save guards | VERIFIED |
| Desktop sizes and touch routes | 1024×768, 1280×720, and 1440×900 browser audit; visible controls | VERIFIED / HUMAN PENDING |
| Text, contrast, colour redundancy, and motion/readability | Graphics acceptance matrix and reduced-motion setting | VERIFIED / HUMAN PENDING |
| Overlay/focus/resize input safety | First-hour, settings, Facility Upgrades, and Salvage Yard overlays consume underlying physical input and suppress colony/tactical map gestures, walk requests, drawer toggles, and stale camera state; first-hour, tactical help, and battle-history references are mutually exclusive; NEXT READY clears stale tactical targeting before changing unit focus; first-hour guide actions preserve active tactical/debrief save context; new colony/deployment sessions clear replay, combat feedback, targeting, confirmation, overlays, audio cursors, and battle-history filters; save restore clears the same transient tactical/audio state; modal input clearing, camera cleanup, map-release, session-transition, and restore tests | VERIFIED |
| Explicit save migrations | Schema table and immediate-previous-version fixture | VERIFIED |
| Canonical captures replaced rather than duplicated | Flat 97-scene capture inventory and audit | VERIFIED |

## Delivery-checkpoint audit

| Checkpoint | Implemented evidence | Remaining gate |
|---|---|---|
| A — Lock experience | Beat sheet, two-operation path, deferrals, playtest ledger | Three complete internal timing runs and their friction list |
| B — Playable opening | Campaign-owned arrival through briefing, migration, touch-visible controls | Human touch-only completion evidence |
| C — First operation and return | Teaching operation, feedback/audio, debrief, consequences, recovery | Observed arrival-to-return comprehension and pacing |
| D — Choice and second operation | Three investments, tactical effect, second consequence, promise | Observed choice comprehension and transfer |
| E — Hardening | Settings, recovery, resize, save/refresh, captures, publisher | Five uncoached sessions, two touch-primary; fix resulting severity 1/2 issues |

## Remaining observed-session gates

| ID | Required evidence | Completion condition |
|---|---|---|
| H01 | Internal timing baseline | Three complete internal runs recorded with total and operation timings, rounds, friction, and issue triage |
| H02 | External first-time sample | Five complete uncoached first-time sessions after Checkpoint D |
| H03 | Touch-primary coverage | At least two of H02 complete end to end through visible touch/click controls |
| H04 | Comprehension and recall | Session notes cover goals, attack results, objective, rewards, spending, benefit, and at least two characters |
| H05 | Subjective pacing/presentation | Notes cover travel, hostile idle time, text cadence, audio fatigue/overlap, motion, and favourite/friction moments |
| H06 | Issue closure | Every observed severity 1/2 issue is fixed and reverified or explicitly accepted; recurring lower-severity causes are triaged |

The game records the quantitative portion in the FIRST-HOUR FIELD GUIDE. A human observer
must supply the exact input method and qualitative evidence in `PHASE_1_PLAYTEST_LOG.md`.
