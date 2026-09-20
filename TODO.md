# Mirexis TODO

## UI_STYLE review — 20 September 2026

Audit and planning only; no UI implementation is included. The previous TODO contained
“No outstanding AI-agent tasks.” There were no task checkboxes or completion records to
merge. Existing Phase 1 implementation/acceptance history remains in its linked documents;
this review does not reopen completed implementation indiscriminately.

### Evidence and scope

Read `AGENTS.md`, `UI_STYLE.md`, `CODE_STANDARDS.md`, `GAME_DEVELOPMENT_GUIDE.md`,
`README.md`, and `MIREXIS.md`. No project-local `PROJECT_AGENTS.md` was found.
Inspected screen renderers, application/navigation flow, pointer conversion, camera
framing, controls, and existing verification documentation. Visually reviewed these
existing 1280×720 images in `docs/verification/`:

- `ui_colony.png`, `ui_first_hour_operations.png`, `ui_mirexis.png`.
- `ui_gameplay.png`, `ui_action_preview.png`, `ui_briefing.png`.
- `ui_roster.png`, `ui_advanced_roster.png`, `ui_gene_lab.png`, `ui_debrief.png`.

These are repository captures, not fresh runs or proof of the current browser build.
No game launch, new capture, live touch interaction, or publish was performed for this
TODO-only audit. `docs/PHASE_1_HARDENING.md` records earlier browser city checks at
1024×768, 1280×720, and 1440×900; those historical checks do not establish current
small-screen readability or completion of the human playtest gate.

Preserve the existing strengths: dominant colony/battlefield world views, contextual
construction, objective forecasts and exact costs, hostile replay and battle history,
first-hour action cues, and Gene Lab's focused comparison with irreversible-choice warnings.
Mirexis has game-specific art and flow; no claim is made that its UI is an unchanged
copied template. Its persistent utility rail, repeated chrome, and help footers still
need the subtraction/adaptation pass required by UI_STYLE §8.

### Verified findings — ordered implementation tasks

The ordering starts with the screen contract, then composition and interaction access,
then feedback and detail. Dependencies are explicit. “Verified” below means supported
by the cited code or reviewed capture; it does not mean live interaction was tested.

- [ ] **UI-01 — Define decision-led screen briefs and the supported viewport contract.**
  **Scope:** `README.md` or `MIREXIS.md`; `src/ui.rs` logical dimensions;
  `src/game/types.rs::AppState`, `game_page.json`, and the existing hardening record.
  **Problem:** README/GDD do not declare normal/minimum canvas sizes or the seven-part
  UI_STYLE §1 screen briefs. A fixed 1280×720 virtual layout and historical resize passes
  leave no actionable small-screen usability contract.
  **Change:** Record briefs for colony exploration, Operations/strategic choice, roster,
  Gene Lab, briefing, tactical selection/targeting, and debrief, with quiet utility routes.
  Use 1280×720 as the normal reference and evaluate 1024×768 as a provisional minimum
  desktop canvas; explicitly decide the supported touch/embedded canvas minimum from
  measurements. Define at most 2–3 strong attention regions per normal state.
  **Accept/verify:** Each brief names the current decision, dominant area, advancing action,
  essential costs/risks, deferred facts, camera/reflow, and visible touch path. Measure
  actual canvas dimensions inside native, standalone browser, and embedded play; do not
  label a viewport supported merely because it scales without clipping. Establish this
  contract before accepting UI-02 through UI-10.

- [ ] **UI-02 — Separate tactical decisions from utilities and expose the next combat action.**
  **Scope:** `src/tactical_hud.rs::draw_edge_controls`, `draw_command_panel`,
  `draw_phase_actions`; `src/colony_header_ui.rs::draw`; `src/ui_action.rs` and navigation handlers.
  **Evidence/problem:** `ui_gameplay.png` and `ui_action_preview.png` show COMMAND, LOG,
  GUIDE, AUDIO, NEXT, SAVE, LOAD, TITLE, and DELETE as one equal-weight rail. END PHASE
  and objective/ability actions require the drawer while save management is always visible.
  The colony similarly sandwiches OPERATIONS between SETTINGS and TITLE.
  **Change:** Put save/load/delete/title/settings in a separately opened, clearly labelled
  utility menu with visible return and existing destructive confirmations. Place NEXT READY,
  contextual commands/objective interaction, and END PHASE in one gameplay action area;
  distinguish the current advancing action without implying END PHASE is always preferable
  to acting. Separate the colony Operations entry from utility navigation.
  **Accept/verify:** Battlefield, gameplay controls, and compact mission state are the only
  strong attention regions; utilities remain discoverable. At UI-01 sizes, touch-select a
  unit, act, cycle ready units, secure an objective, and confirm an early phase end; then
  open/close utilities, save/load, and cancel deletion using a disposable save. No menu tap
  issues a world command. Preserve readiness warnings and targeting cancellation.

- [ ] **UI-03 — Recompose the roster around one preparation decision and safe inspection.**
  **Scope:** `src/roster_ui.rs`, `src/roster_ui/detail.rs::{draw_selected_character,
  draw_training_controls,draw_techniques,draw_equipment_controls}`; roster UI actions/state.
  **Evidence/problem:** Both roster captures crowd all training, techniques, and equipment
  into the lower half. Fifteen class definitions produce five rows starting at y=334 with
  36-pixel steps: the final row starts at y=478, at the technique heading, and extends into
  the y=486 technique controls. Class/technique descriptions use pointer containment;
  enabled taps immediately train/learn/equip, unlike equipment's persistent info action.
  Players cannot comfortably compare unfamiliar choices before spending resources.
  **Change:** Keep selected character/current loadout as support; show one explicit
  Training, Techniques, or Equipment view as the dominant comparison. Give each a bounded
  scroll/pagination layout, persistent tap-to-inspect details, and a separate commit action.
  Group advanced/locked content behind a visible disclosure with prerequisites, revealing
  it naturally at the relevant phase. Replace the 28×24 equipment info affordance with a
  usable inspection target. Remove redundant headings and the shared cramped readout strip.
  **Accept/verify:** No row overlaps another section; inspect any available or locked option
  without changing class, equipment, XP, or materials. Costs, effects, slot constraints,
  replacement consequences, and lock reasons remain beside the commit action. Check both
  captured roster states plus recruited colonists, long class names, scars, and insufficient
  resources at all UI-01 sizes; complete inspect → cancel → inspect → commit by touch.

- [ ] **UI-04 — Make Operations a focused mission/strategic-choice surface.**
  **Scope:** `src/colony_ui/operations.rs::{draw_operations,draw_operations_summary,
  draw_decision_panel,draw_operations_footer}`, `src/colony_ui.rs::draw_character_event`,
  `src/colony_header_ui.rs`, and contextual facility launchers.
  **Evidence/problem:** `ui_first_hour_operations.png` packs archive buttons, repeated
  resources, faction/phase/defence summaries, roster/treatment/upgrades, mission selection,
  a second event decision, and the construction plan into a 408-pixel drawer. The event
  choices are 22 pixels tall; the phase-five capture retains unrelated preparation chrome
  above an important identity choice. Essential decisions lose emphasis among equal buttons.
  **Change:** Make mission comparison and BRIEF SELECTED MISSION the normal drawer focus.
  Open pending events and campaign branches as deliberate focused decisions, with a visible
  pending indicator and return path. Move archives to quiet history access, treatment and
  upgrades to preparation/facility details, and construction-plan prose to Build mode.
  Give resource totals one readable home instead of repeating the header inventory; retain
  relevant balances/costs beside spending choices and urgent assault/power warnings.
  **Accept/verify:** Players can identify the current decision immediately and revisit every
  deferred system. Equally valid branch choices have equal emphasis. Inspect first return,
  three mission offers, injury/power shortage, pending research/event, and Mirexis branches
  at UI-01 sizes. Touch-select a mission, open briefing, return, resolve an event, and find
  treatment/history without losing the active objective. Preserve the map as the dominant
  exploration area when the drawer is closed. Depends on UI-01/02.

- [ ] **UI-05 — Add tactical touch zoom and frame targets within the unobscured world.**
  **Scope:** `src/tactical_map_ui.rs::{draw,tactical_viewport}`;
  `src/grid_ui.rs::WorldCamera::{update,reveal_changed_tactical_selection,tactical_start}`;
  `src/camera_controls.rs`; colony camera integration as a reuse reference.
  **Evidence/problem:** Tactical zoom uses `mouse_wheel()` and advertises WHEEL TO ZOOM;
  visible plus/minus controls exist only for the colony. Selection reveal/clamping uses
  the full world rectangle, not the area left by the command drawer and forecast cards.
  Reviewed tactical images show substantial empty upper-left space and targets under/near
  right-hand overlays. Live obstruction frequency is not yet measured.
  **Change:** Add visible zoom out/in and recenter-selected controls, using toolkit camera
  transforms and the existing release guard. Calculate a safe focus region from open HUD
  surfaces; reframe selected units/objective routes there after relevant selection/layout
  changes without snapping the camera back during intentional panning. Tune entry framing
  around squad and near-term route, retaining enough terrain context.
  **Accept/verify:** Touch-only players can zoom, pan, regain the squad, and select targets
  at both zoom limits without an accidental move/attack. At UI-01 sizes check drawer open/
  closed, forecast visible, map edges, elevated tiles, resize, and display scaling. Target
  art and hit positions agree and an inspected target stays visible. Depends on UI-02.

- [ ] **UI-06 — Integrate guidance into each screen and retire completed teaching chrome.**
  **Scope:** `src/first_hour_ui.rs::{draw_goal,goal_banner_layout,draw_help}`,
  `src/first_hour_colony_ui.rs`, `src/ui.rs::draw_mission_briefing`,
  `src/tactical_map_ui.rs::draw_tactical_controls`, roster/Gene Lab footer instructions.
  **Evidence/problem:** In `ui_briefing.png`, the generic goal panel at (20,76), 520×96,
  overlays the briefing header and mission-title area. The same large panel remains in
  the late-campaign colony capture. Tiny permanent control prose remains along multiple
  screen bottoms; the field guide also always includes internal session metrics.
  **Change:** Allocate guidance space in each screen's layout instead of drawing over
  content. Use a compact current objective after onboarding; dismiss completed instructions
  while preserving HELP, restart/skip, and necessary BEGIN ARRIVAL/CONTINUE CAMPAIGN actions.
  Move repeated gesture/controller instruction lists to contextual first-use help and the
  reopenable guide. Put observer metrics behind a separate playtest-details disclosure.
  **Accept/verify:** Mission name, squad rows, food cost, threats, and DEPLOY SQUAD remain
  visible while learning. Test arrival, briefing with empty/valid squad, return, tutorial
  skip/restart, and late campaign at UI-01 sizes. Touch paths name exact visible controls,
  survive save/load, and never hide a required handoff. Depends on the screen compositions.

- [ ] **UI-07 — Give current tactical state one home and remove stale event prose.**
  **Scope:** `src/tactical_hud.rs::{draw_world_chrome,draw_command_panel,draw_selected_unit}`,
  `src/objective_ui.rs`, `src/skill_ui.rs::draw_action_buttons`, `src/combat_feedback.rs`,
  `src/battle_log_ui.rs`, and `src/ui_widgets.rs::event_summary`.
  **Evidence/problem:** `ui_action_preview.png` repeats objective progress in the top strip
  and drawer; the drawer permanently prints `event_log.last()` below state and commands.
  An old action can read as current status. The drawer also reserves disabled teaching
  controls for LEARN IN BARRACKS/EMPTY SLOT while combat needs the space.
  **Change:** Consolidate persistent objective/round state and selected-unit AP/vitals into
  the areas defined in UI-02. Show expanded objective instructions only when needed.
  Remove the permanent last-event line; reuse bounded combat feedback and retrievable LOG.
  Omit unearned/empty technique placeholders during ordinary combat, retaining help access
  and honest disabled reasons for learned actions. Remove nested decorative borders and
  technical headings such as COMMAND DRAWER where placement already supplies meaning.
  **Accept/verify:** After move, hit/miss, healing, objective damage, and a phase transition,
  expired feedback leaves correct readable state; LOG recovers what happened. Check normal,
  inspected-hostile, targeting, no-AP, and urgent reinforcement states at UI-01 sizes,
  including reduced motion. Costs, threats, and phase-end readiness remain visible.
  Depends on UI-02/05; avoid redesigning the already useful replay/history systems.

- [ ] **UI-08 — Reflow small layouts before reducing text and hit targets.**
  **Scope:** `src/ui.rs`/`VirtualUi` use, `src/ui_widgets.rs::button_with_state`,
  `src/colony_header_ui.rs`, `src/roster_ui.rs`, `src/camera_controls.rs`, and revised layouts.
  **Evidence/problem:** The code uses fixed logical positions, 9.5-pixel equipment text,
  22–30-pixel-high decision/navigation controls, and uniform virtual scaling. At an actual
  1024-pixel-wide canvas, a 24-logical-pixel control is about 19 pixels high. This is a
  code-derived size, not a newly observed small-screen failure. Existing normal-size
  captures already show very small resource, equipment, and mission-detail text.
  **Change:** Implement compact layouts for the UI-01 contract with readable type and a
  proposed minimum 44×44 CSS-pixel interactive hit area after scaling. Reflow comparisons,
  scroll/paginate collections, and collapse secondary facts instead of fitting everything
  into smaller text. Use shared toolkit layout/pointer helpers; consider a toolkit upgrade
  for any missing generic capability. Keep touch bounds and visible layout aligned.
  **Accept/verify:** All key labels, shortages, and selected-state details remain readable
  at normal and declared minimum canvas sizes; adjacent hit targets do not overlap.
  Measure in native and browser/embedded canvases, including 1440×900 and display scaling.
  Touch-test the smallest close, zoom, info, event-choice, and deployment controls with
  long names/large balances. Depends on UI-03/04/06 so this does not preserve excess panels.

- [ ] **UI-09 — Consolidate debrief consequences around the returning squad.**
  **Scope:** `src/ui_debrief.rs`, `src/game/debrief_flow.rs`, notification setup in
  `src/game.rs`/`src/game/bootstrap.rs`, and `src/first_hour_consequences_ui.rs`.
  **Evidence/problem:** `ui_debrief.png` repeats recovery rewards in prose and a framed
  field-record card, while three separate learned-technique notifications demand attention
  alongside the result, squad row, and goal banner. The outcome has several competing homes.
  **Change:** Use one outcome/recovery summary and one squad-consequence area; remove the
  duplicate salvage readout and de-emphasize the decorative field record. Group simultaneous
  technique unlock feedback and make learned techniques inspectable from squad results or
  the roster after notifications expire. Retain returned/reserve distinction, injuries,
  losses, irreversible consequences, and a prominent RETURN TO COLONY.
  **Accept/verify:** At UI-01 sizes compare victory, defeat/incapacitation, multiple unlocks,
  recruited squad, and finale results. Before and after toast expiry, a touch player can
  identify who returned, what changed, what was lost, and how to continue; feedback does
  not cover the return control. Depends on UI-06/08. Confirm each dense state's current
  behavior before changing it; only the ordinary debrief capture was reviewed here.

### Further inspection and acceptance gates — not confirmed runtime defects

- [ ] **UI-10 — Verify touch preview, overlay exclusion, and camera recovery in live play.**
  **Scope:** `src/tactical_map_ui/rendering.rs::handle_click`, `src/action_preview_ui.rs`,
  `src/tactical_map_ui/targeting_card.rs`, `src/game/input.rs`, colony interaction controls,
  `scripts/capture_ui.ps1`, and `docs/verification/`.
  **Uncertainty/effect:** Static evidence does not prove whether ordinary (unguided) touch
  movement lets players read AP/hazard consequences before committing, whether rail/header
  releases reach the map underneath, or whether every dense overlay recovers correctly.
  These are inspection targets, not asserted failures.
  **Action:** Exercise tap-select/inspect/confirm/cancel on move, attack, cover, skill,
  equipment, and objective actions; drag across and release outside overlays; inspect
  damaged/powerless facilities and cancel construction. Fix only reproduced gaps, using
  explicit visible confirmation/inspection and shared consumed-input bounds as appropriate.
  **Accept/verify:** Run at 1280×720, provisional 1024×768, and UI-01's declared touch/embedded
  minimum. Include current selection near each edge, zoom limits, resize, long descriptions,
  and dense late-game inspectors. Record actual canvas sizes, input method, build/scene,
  observations, and remaining limitations. Refresh relevant captures directly in
  `docs/verification/`, replacing equivalent scenes; no screenshot alone proves interaction.

- [ ] **UI-11 — Complete the visual review and preserve the existing human acceptance gate.**
  **Scope:** All changed screens; `docs/PHASE_1_PLAYTEST_LOG.md`,
  `docs/PHASE_1_ACCEPTANCE_MATRIX.md`, `docs/PHASE_1_HARDENING.md`, verification inventory.
  **Uncertainty/effect:** Historical capture and resize passes do not establish the updated
  attention budget, practical touch usability, or five uncoached first-time playtests
  (including two touch-primary sessions), which the README still lists as pending.
  **Action:** After the cohesive changes above, review the ten UI_STYLE §9 criteria against
  normal/minimum, first-use, selected, dense, and urgent/failure states. Preserve prior
  completion history and add dated evidence rather than replacing it with blanket “pass”.
  Run `./publish.ps1` without parameters after meaningful UI implementation and report its
  result/blocker; retain useful regression tests in `tests/` without adding cosmetic tests.
  **Accept/verify:** Every changed screen has capture plus interaction evidence at the
  declared sizes; essential costs, warnings, navigation/recovery, and help remain usable.
  Record human session results honestly and keep that acceptance gate open until satisfied.
  Compile/publish success or this audit must not be used as a substitute for those sessions.
