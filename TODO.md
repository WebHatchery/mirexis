# Mirexis TODO

- [ ] Restore 20 undiscovered nested suites (37 of 520 test cases) under
  `tests/game/`, `tests/colony_ui/`, `tests/colony_exploration/`,
  `tests/colony_map_ui/`, and `tests/tactical_map_ui/`: wire them into
  Cargo-discovered integration-test entry points, fix missing
  imports and stale `include_str!` paths, and update the capture-builder check
  for `src/game/capture_scene_groups.rs`. Confirm these regressions appear in
  `cargo test -- --list` and pass (§11.4).
- [ ] Move `src/lib.rs::test_prelude` into `tests/` and replace blanket public
  exposure of implementation modules/helpers with intentional library seams.
  Remove unused test imports and unnecessary `unused_imports`/`dead_code`
  allowances in campaign and strategy modules (§§1.4, 10.2, 11.4).
- [ ] Complete the colony UI action boundary: replace mutated camera/explorer
  snapshots in `ColonyDrawResult` with explicit intents; move interaction,
  approach updates, build-mode changes, and overlay mutations from
  `colony_map_ui`, its controls, and `colony_ui/operations.rs` into game/input
  handlers. Drawing must read state without advancing it (§§5.1, 7).
- [ ] Refactor remaining functions over 100 physical lines, starting with
  `tactical_map_ui::draw`, `colony_map_ui::draw`, `skills::actions::execute`,
  `colony_exploration::draw_dialogue`, and `combat_feedback::record_for`.
  Replace lettered action/capture groups with cohesive domain handlers;
  `game/action_dispatch.rs` (729 lines) and `game/capture_scene_groups.rs`
  (636 lines) also need planned splits. Check all Rust functions and preserve
  the passing 800-line file gate (§§2.1–2.2, 4.1).
- [ ] Resolve strict Clippy failures: decide whether `ColonyState::new` should
  remain a production constructor and implement `Default` if appropriate;
  replace excessive parameter lists in `colony_ui/operations.rs` and
  `roster_ui/detail.rs` with cohesive context structs. Replace the operations
  helper's `(bool, bool)` return with a named result. Verify
  `cargo clippy --all-targets --all-features -- -D warnings` (§§4.3–4.4, 10.2).
- [ ] Finish moving authored text and balance values into toolkit-loaded JSON:
  first-hour goals/lesson prompts in `first_hour.rs`, help headings/button
  labels, outsider dialogue in `campaign/outsider/beats.rs`, equipment prices
  in `campaign/modifiers.rs`, and duplicate starting resources in
  `colony/state.rs`. Validate the resulting configuration and retain only
  presentation constants in rendering code (§5.3).
- [ ] Complete visual-catalog validation: require concept IDs consumed by
  `world_art.rs` and tactical rendering (including `objectives`) before drawing,
  validate texture references against the manifest, and replace the unchecked
  `concept_atlas` panic path with startup diagnostics or a recoverable lookup.
  Use labeled toolkit loading for `texture_manifest` in `data/loader.rs`;
  cover malformed/missing references with focused regressions (§§5.3, 6).
- [ ] Organize migrated suites by responsibility instead of `test_part_1` and
  `suite_a/b/c` groupings in campaign, strategy, colony, and persistence tests.
  Review feature-level case counts, consolidate related inputs where useful,
  and explain exceptions to the five-case target without deleting distinct
  regression coverage (§§2.1, 11.3–11.4).
