# Mirexis UI and Graphics Final Requirement Audit

Audit date: 2026-08-12
Completion rule: a requirement is complete only when current implementation and direct evidence both support it.

## Requirement-by-requirement record

| # | Required outcome | Authoritative implementation evidence | Direct verification evidence | Status |
|---:|---|---|---|---|
| 1 | Audit every rendered screen, overlay, ending, and capture scene | `scripts/capture_ui.ps1`; explicit builders in `src/game/capture_scenes.rs`, `capture_colony.rs`, `capture_debrief.rs`, and `capture_tactical.rs` | Section C of `UI_GRAPHICS_ACCEPTANCE_MATRIX.md` has 85 PASS rows; tests enforce an exact manifest/matrix/inventory set match | PASS |
| 2 | Coherent asset pipeline with atlases, definitions, channels, animation, scaling, loading, and obvious missing-art handling | `src/visual_assets.rs`; `texture_manifest.json`; `sprite_definitions.json`; 17 runtime texture records and four manifest-defined faction channels | `ASSET_PIPELINE_AUDIT.md`; atlas decode/divisibility/alpha/uniqueness/channel tests; 27-file published package audit; no magenta in 85 captures | PASS |
| 3 | Genuine three-quarter battlefield with matching hit testing, modular terrain, height, walls/cover, foreground, projected overlays, and desktop scaling | `src/grid_ui.rs`, `src/tactical_map_ui.rs`, `src/cover_ui.rs`, `src/hazard_ui.rs` | Centre/interior/elevation-transition inverse-hit tests; tactical captures; five supported-size responsive sets; grayscale audit | PASS |
| 4 | Production 2D assets for all recruits and enemy families, facings/states, allegiance, equipment/mutation cues, and portraits | Eleven 4 x 6 unit atlases, recruit/enemy portrait atlases, `src/tactical_unit_ui.rs`, manifest definitions | `asset_atlas_audit/unit_atlases.jpg`; tests resolve every roster record to authored unit and portrait art and reject duplicate/empty atlas cells | PASS |
| 5 | Inhabited colony with all building, construction, power, damage/repair, evolution, path, dressing, and interaction states | `src/colony_map_ui.rs`, `src/colony_ui.rs`, colony 4 x 4 atlas, authored recruit inhabitants | `colony`, `construction`, `power`, `damage`, `repair`, campaign-phase and three ending captures; responsive control audit | PASS |
| 6 | Rebuilt battle presentation: comparison, staging, recoil, impacts, criticals, faction effects, transitions, objectives, hazards, reinforcements, post-state | `src/action_preview_ui.rs`, `combat_feedback.rs`, `phase_replay.rs`, `enemy_intent_ui.rs`, `reinforcement_ui.rs`, `objective_ui.rs` | Forecast, valid/invalid shot, feedback, replay, abilities, hazard, reinforcement, objective, readiness, vitality, and debrief captures | PASS |
| 7 | One UI system across screens including typography, surfaces, controls, states, portraits, resources, warnings, tooltips, and input cues | `src/ui_widgets.rs`, shared toolkit dark theme/surfaces, portrait and title modules; accurate strategy/tactical controller mappings | `UI_CONTRAST_TYPOGRAPHY_AUDIT.md`; `title`, `title_controller`, `title_hover`; help/log/modals; 1024 x 768 strategy-control audit | PASS |
| 8 | Capture every supported scene, compare against baseline/report, and fix visual defects | Current 85-scene deterministic harness and capture builders | `FINAL_CAPTURE_INVENTORY.md`, eight inspected contact sheets, `capture_audit.json`, `UI_GRAPHICS_COMPARISON_AUDIT.md`, responsive and grayscale evidence | PASS |
| 9 | Validate meaningful checkpoints with Cargo and `publish.ps1` without treating them as sole completion evidence | Test, strict lint, publisher, package, capture, visual, and browser checks are separate gates | Current: 180 crate tests plus the source-limit target test; strict Clippy; successful Windows/WebGL publish; native capture and published-browser checks | PASS |
| 10 | Work actively for at least four hours and continue until no acceptance item remains unresolved | The fresh goal timer is the authority; the matrix time row may close only after it exceeds 14,400 active seconds | Current fresh-goal timer remains below 14,400 seconds | PENDING |

## Cross-checks

- Research translation: `docs/UI_GRAPHICS_VISUAL_SPEC.md` records the visual grammar derived from `Tactical_RPG_Graphics_Comparative_Report.docx` before implementation.
- Baseline comparison: `UI_GRAPHICS_COMPARISON_AUDIT.md` records starting defects and final outcomes across battlefield, terrain, characters, colony, combat, UI, campaign identity, and missing-art behavior.
- Capture integrity: `capture_audit.json` reports 85 expected/actual images, zero missing/extra names, zero size failures, zero magenta diagnostics, zero duplicates, and zero blank/dark/low-detail frames.
- File integrity: all 85 byte counts and SHA-256 values in `FINAL_CAPTURE_INVENTORY.md` were recomputed against the frozen PNG set and match in manifest order.
- Source constraints: every Rust source and test file passes the enforced 800-physical-line limit; the current largest file is `src/strategy.rs`.
- Shipping integrity: `publish.ps1` creates Windows and WebGL builds with 27 runtime assets (25.85 MB), includes both manifests and all atlas families, and excludes `art_sources`.
- Browser integrity: a fresh final preview load renders the WebGL canvas at 1200 x 675 with no panic, runtime-error, or WASM-load-error overlay.
- Office-suite constraint: no LibreOffice/soffice dependency or invocation exists in the worktree or verification workflow.

## Remaining gate

The fresh-goal four-hour minimum remains open. All substantive implementation, capture, audit, test, lint, publish, and browser rows are resolved.
