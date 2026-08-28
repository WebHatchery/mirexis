# Phase 1 Acceptance Matrix

Status: implementation evidence audited; observed-session gates pending
Audit baseline: save/content 1.65.0, validated and committed with this matrix

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
| D04 | Operation one teaches the core loop and operation two demonstrates transfer | Ordered `TacticalLesson` transitions and unscripted `SecondOperation`; player transfer must still be observed | IMPLEMENTED / HUMAN PENDING |
| D05 | City supplies direction, preparation, character connection, and visible consequences | `first_hour_ui.rs`, `first_hour_investment_ui.rs`, colony conversations, `first_hour_consequences_ui.rs`, and colony/debrief captures | VERIFIED |
| D06 | First investment has a clear observable effect | Three 24-material preparations and `first_investment_changes_only_the_second_deployment` | VERIFIED |
| D07 | Victory, defeat, save, continue, refresh, and resize paths are verified | `PHASE_1_HARDENING.md`, persistence/outcome tests, and the recorded WebGL operator pass | VERIFIED |
| D08 | Audio/settings exist and critical information remains visual | `audio.rs`, audio tests, settings UI, event prose/markers, and browser persistence pass; fatigue remains an observation target | VERIFIED / HUMAN PENDING |
| D09 | No severity 1 or 2 first-hour issue remains | No implementation-known severity 1/2 issue; five-session issue discovery and triage have not occurred | HUMAN PENDING |
| D10 | Documentation and verification captures match the build | 86-scene manifest, acceptance matrix test, `capture_audit.json`, and `ui_first_hour_guide.png` | VERIFIED |
| D11 | Parameterless publisher passes and changes use project commit conventions | Recorded in `PHASE_1_HARDENING.md`; rerun and commit required after each audit fix | VERIFIED AT LAST CHECKPOINT |

## Workstream audit

### 5.1 First-session direction and onboarding

| Required work | Evidence | Status |
|---|---|---|
| Opening situation, immediate goal, and destination | Arrival goal and Mara marker/landmark | VERIFIED |
| Introduce only the next meaningful control | Stage-owned `primary_goal` and ordered tactical lesson | VERIFIED |
| Exact visible-control or gesture language | Prompts consistently name Tap, Drag, CONTINUE, OPERATIONS, DEPLOY, ATTACK, END PHASE, HELP, and visible targets | VERIFIED |
| Highlight relevant target without blocking inspection | Non-blocking goal banner, Mara marker, operation control, and tactical overlays | VERIFIED |
| Visible revisitable help | FIRST-HOUR FIELD GUIDE and tactical HELP | VERIFIED |
| Restartable and skippable prompts without removing goals | `restart`, `guidance_enabled`, and first-hour tests | VERIFIED |
| Persist progress and migrate saves with no tutorial fields | Save schema 1.65 and migration fixtures for 1.63 and 1.64 | VERIFIED |
| Defer non-actionable panels | Deferred-system list and first-hour gating in colony controls | VERIFIED |

Acceptance that an unfamiliar player completes without help or keyboard and resumes with
the right understood context remains part of H01–H04 below.

### 5.2 City as home and preparation space

| Required work | Evidence | Status |
|---|---|---|
| Strong first-NPC landmark, readable approach, short interaction | Mara landmark, approach path, interaction distance, and exact city goal | VERIFIED |
| Concise introductions for coordinator and starting squad | Authored speaker table and colony dialogue | VERIFIED |
| Environmental and HUD primary goal | Goal banner plus highlighted world/control targets | VERIFIED |
| Readable colonists and facilities without permanent labels | Speech markers, proximity dialogue, building art/state | VERIFIED |
| Outcome-sensitive post-operation conversation | Ilya debrief line and first-return conversation state | VERIFIED |
| Physical/visual consequence after each operation | Refuge and outer-route consequence drawing | VERIFIED |
| Travel supports atmosphere without delay | Direct tap-to-walk, short Mara route, visible OPERATIONS shortcut | IMPLEMENTED / HUMAN PENDING |
| Timely choices foregrounded | Three dedicated preparations; advanced systems deferred | VERIFIED |

Ten-second goal recognition, city attachment, and traversal friction require observation.

### 5.3 Tactical flow and combat feel

| Required work | Evidence | Status |
|---|---|---|
| Authored teaching operation | `operation_glassroot` and first-operation stage | VERIFIED |
| Core-rule order | `TacticalLesson`: select, cover move, attack, enemy phase, objective, ability | VERIFIED |
| Less-scripted second operation | Generated Isolation offer with `ApplyLearning` guidance only | VERIFIED |
| Reduce low-value clicks | Direct tile/hostile intents, target forecast, visible actions, guarded end phase | VERIFIED |
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
| XP, injury, relationships, equipment, and resources visible then inspectable | Debrief plus roster/loadout/colony surfaces | VERIFIED |
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
| Overlay/focus/resize input safety | Modal input clearing and map-release tests | VERIFIED |
| Explicit save migrations | Schema table and immediate-previous-version fixture | VERIFIED |
| Canonical captures replaced rather than duplicated | Flat 86-scene capture inventory and audit | VERIFIED |

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
