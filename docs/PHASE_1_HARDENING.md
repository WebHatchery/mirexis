# Phase 1 Hardening Record

Status: automated and internal verification complete; external playtests pending

## Reliability paths

| Path | Evidence | Result |
|---|---|---|
| New game | Touch-visible **NEW OPERATION**, two-step replacement confirmation when a save exists, immediate autosave | pass |
| Continue | Browser refresh followed by **CONTINUE** restores the serialized city goal; finished tactical saves reopen the debrief with its reconstructed outcome and materialized mission context | pass |
| Tactical save/load | Existing persistence tests and visible battle **SAVE** / **LOAD** controls | pass |
| Victory and defeat | Deterministic outcome tests plus first-hour tests for both outcome values | pass |
| Recovery | Failure advances to debrief and city; Ilya's routed recovery conversation precedes preparation; contextual and Operations treatment actions share injury, biomass, and infirmary gates; emergency stores guarantee one 24-material preparation | pass |
| Tutorial restart/skip | Visible field-guide controls; both preserve the essential campaign goal | pass |
| Destructive save actions | **NEW OPERATION** becomes **CONFIRM NEW COLONY** and **DELETE SAVE** becomes **CONFIRM DELETE** before mutation | pass |
| Browser refresh | WebGL campaign autosave survived a page reload on 29 August 2026 | pass |
| Resize | Live WebGL city inspected at 1024×768, 1280×720, and 1440×900 with no console warning/error | pass |

## Accessibility and presentation audit

| Concern | Implemented response | Remaining observation target |
|---|---|---|
| Touch/click completion | Every required action has a labelled button, map target, speech marker, or direct tap gesture; first-hour advance goals focus BEGIN ARRIVAL and CONTINUE CAMPAIGN, briefing guidance focuses DEPLOY SQUAD or a selectable colonist row, guided attacks add a visible forecast-card ATTACK confirmation and a NEXT cue on the confirmation itself, objective guidance focuses SECURE OBJECTIVE once actionable, hostile replay guidance focuses SKIP REPLAY, the enemy-phase rail shows READY, SPENT, and INCAP counts beside the confirmation action, return goals focus RETURN TO COLONY, and required conversations focus CONTINUE after the correct speaker opens; skipped guidance suppresses focus cues; no keyboard-only first-hour action | Confirm in two human touch-primary sessions |
| Text scale | Fixed 1280×720 virtual UI scales uniformly and preserves layout at audited desktop sizes | Ask testers to assess smallest 1024×768 labels |
| Contrast | Bright text, outlined panels, dark surfaces, and high-luminance goal accents remain legible in captures | Observe low-quality displays and glare |
| Colour-independent meaning | Buttons and status text name state; target rings, icons, forecasts, and event prose do not rely on hue alone | Include colour-vision feedback in external sessions |
| Motion | Persistent **REDUCED MOTION** completes unit presentation immediately and suppresses hostile replay animation | Confirm the faster state changes remain comprehensible |
| Audio | Persistent 0–100% volume and mute; critical events retain text, markers, meters, and debrief state | Listen for fatigue and overlap in observed sessions |
| Overlay input safety | First-hour field guide, settings, Facility Upgrades, Salvage Yard, tactical help, battle log, and hostile replay clear underlying release actions; reference overlays are mutually exclusive, blocking colony overlays suppress colony/tactical map gestures and drawer changes, pause movement, and preserve visible close paths; NEXT READY cancels stale tactical targeting before changing unit focus; first-hour guide actions preserve active tactical/debrief save context; new colony/deployment sessions clear replay, combat feedback, targeting, confirmation, overlays, audio cursors, and battle-history filters; save restore clears the same transient tactical/audio state | Repeat on touch-primary browsers |
| Playtest evidence | Campaign-persisted elapsed time, first-action milestones, operation durations/rounds, invalid commands, and field-guide opens are visible in the field guide | Observer still records exact input method and qualitative findings |

## Automated verification baseline

- 500 Mirexis unit tests pass, including deployment-row affordance coverage for full,
  minimum, and recovering squads, construction availability coverage for unlocks,
  materials, and unique projects, effective repair-cost affordability coverage, and
  tactical equipment cancellation coverage, invalid-target persistence coverage, and
  roster prerequisite-label coverage, Gene Lab biomass-label coverage, and Waystation
  outsider shortfall-label coverage, tactical target-gate label coverage, and mutation
  availability-label coverage, objective interaction-label coverage, Overwatch
  availability-label coverage, class-action availability-label coverage, technique
  availability-label coverage, field-item availability-label coverage, and deployment
  food-gate label coverage, Commons meal-gate label coverage, and Relay scan-gate label
  coverage, treatment-gate label coverage, research material-gate label coverage, and
  Waystation recruitment resource-gate label coverage, Waystation dialogue recruitment
  shortfall-label coverage, contextual treatment and Gene Lab blocker-label coverage,
  targeted class-action, field-item, and technique team/range blocker coverage, and
  targeting-card rule-reason coverage, guided attack-confirmation focus coverage,
  objective-interaction, replay-skip, end-phase geometry, readiness-summary, hit-impact,
  incapacitation, objective-completion, extraction-completion, objective-damage, and
  cover-outcome feedback,
  attack-outcome event-summary, callout-lane geometry, recovery-audio, battle-log
  event-grouping, filter-count, hostile-replay-consequence, and display-name event-summary
  coverage,
  debrief-return, dialogue-continue, advance-control, and briefing-deployment focus
  coverage, and campaign decision resource-gate
  label coverage, facility-upgrade blocker-label coverage, first-hour investment
  material-gate label coverage, character-event food-gate label coverage, Build Mode
  construction-blocker label coverage, roster equipment-row geometry coverage, finale
  debrief summary coverage, completed-campaign debrief voice coverage, bounded
  ending-register coverage, briefing pressure geometry coverage, bounded
  briefing-intelligence coverage, and workshop equipment-panel geometry coverage.
- External asset-registry and Rust source-size gates pass.
- `publish.ps1` passes without parameters for Windows and WebGL and deploys to preview.
- The canonical set contains 100 audited 1280×720 scenes, including the first-hour
  field guide, guided tactical lesson, second-operation tactical lesson, actionable
  ability lesson, first-return recovery handoff, promise consequence, and colony handoff;
  debrief goal/result layering, fielded-versus-reserve squad disclosure including recruited
  fielded colonists, recruited-roster list coverage, XP and automatic technique-learning
  disclosure,
  enemy-phase readiness and replay-skip transitions, forecast/first-hour layout changes,
  the wrapped escalation pressure modifier, and bounded threat-intelligence rows
  replace their affected images.
  The three ending captures also open the Operations drawer so each path-specific identity
  card and bounded colony legacy register is visible in the canonical evidence.

## Honest boundary

This record does not substitute for the five required uncoached first-time playtests. It
records only deterministic, visual, and operator-run evidence. Human session results belong
in `PHASE_1_PLAYTEST_LOG.md`, including any assistance and the exact wording that caused it.
