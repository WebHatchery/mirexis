# Mirexis TODO

Reviewed against `AGENTS.md`, `CODE_STANDARDS.md`, `MACROQUAD_TOOLKIT.md`, and
`GAME_DEVELOPMENT_GUIDE.md` on 2026-09-12. The local copies of the shared
documents match `rust_management/docs/`.

The current baseline is healthy: `cargo fmt --all -- --check` passes, `cargo
test` passes all 520 tests, the 800-line source gate passes, there are no
`mod.rs` files, and the required publishing files are present. The items below
are follow-up work required to align the implementation with the updated
guidance.

## Architecture and test migration

- [ ] Move the 103 test source files currently under `src/` into the crate's
  root `tests/` directory, remove the 85 `#[cfg(test)] mod tests;` declarations,
  and introduce `src/lib.rs` as the public logic boundary used by `main.rs` and
  integration tests. Preserve private access through intentional public seams
  rather than exposing implementation details. This is the migration required
  by `CODE_STANDARDS.md` §11.4.
- [ ] Split the broad migrated suites by responsibility while moving them:
  `src/campaign/tests/test_part_1.rs` (771 lines),
  `src/strategy/tests.rs` (672 lines),
  `src/persistence/tests/late_migrations.rs` (790 lines),
  `src/colony/tests.rs` (728 lines), and `src/state/tests.rs` (529 lines).
  Review each resulting feature suite against the five-case target in §11.3
  and document any necessary exceptions in the test design.
- [ ] Split the 14 implementation files at or above the 600-line planning
  threshold before extending them: `src/colony_exploration.rs` (786),
  `src/game.rs` (770), `src/data.rs` (762), `src/campaign.rs` (758),
  `src/state.rs` (758), `src/strategy.rs` (754), `src/tactical_map_ui.rs`
  (718), `src/colony.rs` (716), `src/roster_ui.rs` (679),
  `src/colony_story.rs` (665), `src/skills.rs` (646), `src/colony_ui.rs`
  (642), `src/campaign/outsider.rs` (636), and `src/class_actions.rs` (605).
- [ ] Refactor functions beyond the 100-line maximum into cohesive actions or
  render helpers. The clearest current offenders are `Game::apply_action`
  (`src/game.rs`), `draw_operations` (`src/colony_ui.rs`),
  `draw_selected_character` (`src/roster_ui.rs`), `outsider_beat`
  (`src/campaign/outsider.rs`), `GameData::validate_registry` (`src/data.rs`),
  `begin_capture_scene` (`src/game/capture_scenes.rs`), `current_beat`
  (`src/colony_story.rs`), `capture_input` (`src/game/input.rs`), and
  `migrate_save_value` (`src/persistence.rs`).

## State, data, and error boundaries

- [ ] Restore action ownership in the colony UI. `ColonyDrawContext` currently
  passes mutable cameras, explorer state, and overlay flags through
  `src/colony_ui/context.rs`; `src/colony_ui/scene.rs`,
  `src/colony_map_ui/controls.rs`, and `src/colony_map_ui/interaction.rs`
  mutate those values while drawing. Return intents and apply the mutations in
  the game dispatcher as required by `CODE_STANDARDS.md` §§5.1 and 7.
- [ ] Split `src/data.rs` into schema and validation modules, then keep loading
  in `src/data/loader.rs`. The current 762-line module combines all schemas,
  registry validation, and cross-feature validation.
- [ ] Route authored visual data through the toolkit loader. Replace the direct
  `serde_json::from_str` in `src/visual_assets.rs:62` and the texture-manifest
  parsing in `src/visual_assets/tests.rs:136` with labeled toolkit loading;
  review the manifest read in `tests/asset_registry.rs:27` at the same time.
  Make `VisualCatalog` validation explicit and return a useful startup error
  for malformed references instead of relying on later `expect`/`panic!`
  paths.
- [ ] Replace multi-value tuple returns with named result structs, including
  `CampaignState::contact_completion_progress`,
  `CampaignState::adaptation_completion_progress`,
  `ColonyState::repair_building`, and `danger_rating::hostile_profile`.
- [ ] Audit every `#[allow]` in `src/`. Remove avoidable suppressions or add a
  nearby explanation for the intentional ones, covering the three unused
  import allowances in `src/campaign.rs`, the dead-code allowances in
  `src/campaign/derivation.rs`, `src/campaign/modifiers.rs`, and
  `src/strategy.rs`, and the four-argument UI allowances.
- [ ] Add `//!` module documentation to the six implementation modules that
  currently lack it: `src/campaign/story.rs`, `src/colony_ui/context.rs`,
  `src/colony_ui/recruitment.rs`, `src/colony_ui/scene.rs`,
  `src/game/debrief_flow.rs`, and `src/strategy/materialization.rs`.
- [ ] Update the comment in `tests/code_standards.rs`, which still describes
  the source gate as checking “non-test lines”; the current gate counts every
  Rust source line, including tests and comments.

## Browser and content compliance

- [ ] Complete a touch-first browser audit at common desktop and narrow
  viewport sizes. Verify that save/load, end phase, modal dismissal, colony
  interaction, tactical selection, camera pan/zoom, and recovery actions all
  have visible tap targets. `game_page.json` currently advertises `Enter`,
  `S / L`, and `Escape` without naming their visible equivalents, and
  `src/help_ui.rs` contains a keyboard shortcut line that should be updated to
  match the exact touch controls.
- [ ] Restore and verify the `roost_slug` field in `game_page.json`; the
  updated publishing guidance requires the Project Roost slug to remain part
  of the per-game metadata used by the generated WebGL page.
- [ ] Add or refresh representative captures for the touch-critical states in
  `docs/verification/`, using the existing flat directory and replacing
  duplicate screen/state captures as required by §12.
- [ ] Move remaining player-facing narrative and balance literals into the
  authored JSON data under `assets/`. Start with tutorial/help copy in
  `src/first_hour_ui.rs` and `src/help_ui.rs`, and gameplay constants such as
  movement, interaction, and starting-resource values in
  `src/colony_exploration.rs` and `src/colony.rs`; keep purely presentational
  layout constants in Rust.
- [ ] Audit variable shadowing in the implementation, beginning with the
  repeated `name`, `facing`, and `state` bindings in `src/visual_assets.rs`,
  and rename bindings so each value has one clear identity.
