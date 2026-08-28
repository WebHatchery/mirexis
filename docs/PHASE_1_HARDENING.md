# Phase 1 Hardening Record

Status: automated and internal verification complete; external playtests pending

## Reliability paths

| Path | Evidence | Result |
|---|---|---|
| New game | Touch-visible **NEW OPERATION**, two-step replacement confirmation when a save exists, immediate autosave | pass |
| Continue | Browser refresh followed by **CONTINUE** restored the serialized city goal | pass |
| Tactical save/load | Existing persistence tests and visible battle **SAVE** / **LOAD** controls | pass |
| Victory and defeat | Deterministic outcome tests plus first-hour tests for both outcome values | pass |
| Recovery | Failure advances to debrief and city; emergency stores guarantee one 24-material preparation | pass |
| Tutorial restart/skip | Visible field-guide controls; both preserve the essential campaign goal | pass |
| Destructive save actions | **NEW OPERATION** becomes **CONFIRM NEW COLONY** and **DELETE SAVE** becomes **CONFIRM DELETE** before mutation | pass |
| Browser refresh | WebGL campaign autosave survived a page reload on 29 August 2026 | pass |
| Resize | Live WebGL city inspected at 1024×768, 1280×720, and 1440×900 with no console warning/error | pass |

## Accessibility and presentation audit

| Concern | Implemented response | Remaining observation target |
|---|---|---|
| Touch/click completion | Every required action has a labelled button, map target, speech marker, or direct tap gesture; no keyboard-only first-hour action | Confirm in two human touch-primary sessions |
| Text scale | Fixed 1280×720 virtual UI scales uniformly and preserves layout at audited desktop sizes | Ask testers to assess smallest 1024×768 labels |
| Contrast | Bright text, outlined panels, dark surfaces, and high-luminance goal accents remain legible in captures | Observe low-quality displays and glare |
| Colour-independent meaning | Buttons and status text name state; target rings, icons, forecasts, and event prose do not rely on hue alone | Include colour-vision feedback in external sessions |
| Motion | Persistent **REDUCED MOTION** completes unit presentation immediately and suppresses hostile replay animation | Confirm the faster state changes remain comprehensible |
| Audio | Persistent 0–100% volume and mute; critical events retain text, markers, meters, and debrief state | Listen for fatigue and overlap in observed sessions |
| Overlay input safety | Help, settings, battle log, and hostile replay clear underlying release actions | Repeat on touch-primary browsers |
| Playtest evidence | Campaign-persisted elapsed time, first-action milestones, operation durations/rounds, invalid commands, and field-guide opens are visible in the field guide | Observer still records exact input method and qualitative findings |

## Automated verification baseline

- 239 Mirexis unit tests pass.
- External asset-registry and Rust source-size gates pass.
- `publish.ps1` passes without parameters for Windows and WebGL and deploys to preview.
- Canonical title and colony captures were refreshed after the new controls.

## Honest boundary

This record does not substitute for the five required uncoached first-time playtests. It
records only deterministic, visual, and operator-run evidence. Human session results belong
in `PHASE_1_PLAYTEST_LOG.md`, including any assistance and the exact wording that caused it.
