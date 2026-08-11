# Mirexis Spatial Camera Acceptance Matrix

Updated: 2026-08-12  
Standard target: 1280 x 720  
Capture root: `docs/verification`

| Acceptance item | Implementation evidence | Test / capture evidence | Status |
|---|---|---|---|
| Tactical maps support roughly 30 x 30 to 50 x 50 tiles or larger | `game_config.json` defines a 40 x 40 runtime grid | `grid_ui::tests::forty_tile_world_is_cropped_at_default_zoom`; `ui_gameplay.png` | PASS |
| Normal gameplay zoom shows only part of the tactical map | Fixed 48 x 24 pixel default tactical diamonds are independent of viewport size | Crop-count invariant; `ui_gameplay.png` at 100% | PASS |
| Tactical camera pans and zooms | Persistent `WorldCamera`; thresholded primary/touch drag with click suppression; middle/right drag; visible `< ^ v >` and `-/+` controls; cursor-anchored wheel zoom; 65%-185% bounds | Pan/zoom/primary-gesture/control math tests; live 1280 x 720 published button-pan and button/wheel-zoom exercises; `ui_variant.png` at 78% off-center | PASS |
| Units and tiles are substantially smaller | Tactical tile width is fixed at 48 pixels rather than fit-derived widths near 76 pixels | Fixed-scale invariant; before/after canonical `ui_gameplay.png` | PASS |
| Default tactical view does not fit the whole battlefield | 40 x 40 projected extent exceeds the 884 x 568 viewport at 100% | Crop-count test exposes less than half the tiles; `ui_gameplay.png` | PASS |
| Colony has a large frontier beyond the initial settlement | Colony footprint expanded from 8 x 6 to 20 x 20 | Far-corner construction unit test; `ui_colony.png` | PASS |
| Buildings do not enlarge to consume empty space | Colony tile half-size is fixed at 26 x 13; building sprite footprint is fixed to world zoom | Fixed colony-scale test; `ui_colony.png`, `ui_construction.png` | PASS |
| Primary viewport receives most screen space | World panel is 900 x 608; command rail is 350 pixels wide; header is 52 pixels high | `ui_gameplay.png`, `ui_colony.png` | PASS |
| Empty UI, buttons, headers, and spacing are compact | Header reduced 66→52 pixels; tactical footer controls reduced 44→28 pixels; command rail reduced 406→350 pixels | Canonical tactical and colony captures | PASS |
| Selection remains correct after transforms | Rendering and inverse hit testing share `GridView`; clicks are constrained to the camera viewport | Isometric round-trip tests; live panned-view tile selection; `ui_class_target.png` | PASS |
| Movement and route previews remain correct | Route, movement overlay, and action card use the camera-derived `GridView` | 180 gameplay tests; live transformed move after pan+125% zoom; `ui_movement_route.png` | PASS |
| Targeting remains correct | Equipment, class-action, hostile, and cover targeting use transformed tile positions | Existing targeting tests; `ui_class_target.png`, `ui_valid_shot.png`, `ui_equipment.png` | PASS |
| Colony placement and hover remain correct | `ColonyView` owns projection and hover inversion at every zoom | Multi-camera hover round-trip test; live visible-control pan, 125% zoom, hover, and placement; `ui_construction.png` | PASS |
| Contextual UI and hover states remain correct | World draws are scissored before cards, panels, and footers; overlays share transformed bounds | DPI clip test; full capture audit; inspected tactical/colony context scenes | PASS |
| Map variants stay playable in the larger runtime world | Odd-seed mirroring remains inside the authored 12 x 8 encounter footprint | Large-runtime variant regression test; `ui_variant.png` | PASS |
| Standard-resolution visual validation passes | Full root capture suite rendered at 1280 x 720 | 85 unique captures; no missing, duplicate, magenta, blank, wrong-size, or low-detail frame | PASS |
| Required publisher passes | Windows and WebGL release builds, packaging, preview deployment, and catalog sync completed | `publish.ps1` passed 2026-08-12 | PASS |
| Published WebGL rendering remains stable | UI font atlases are populated before the first frame, preventing mid-frame GPU texture replacement | Fresh published session rendered title, colony pan/zoom/hover/placement, briefing/deployment, tactical pan/zoom/selection/movement at a 1280 x 720 browser viewport; no warning, error, panic, or WASM-load failure appeared | PASS |
| Initial settlement is centered with a readable undeveloped frontier | `SETTLEMENT_CENTER` anchors the command centre at [10, 10]; five supporting buildings occupy spacious positions around it; the 20 x 20 ground pass draws every visible unoccupied plot | `colony::tests::initial_settlement_is_centered_spacious_and_surrounded_by_frontier`; `colony_map_ui::tests::initial_camera_frames_the_centered_settlement_without_fitting_the_colony`; refreshed `ui_colony.png` and centered `ui_construction.png` | PASS |
| Tactical deployment uses the full battlefield | Allied formations enter coherently on the western side; hostile deployment is distributed along an eastern front; authored recipes are projected across the 40 x 40 runtime grid | `campaign::tests::deployments_hold_opposite_ends_with_a_broader_hostile_front`; `map_variants::tests::larger_runtime_world_projects_recipes_across_the_battlefield` and reachability coverage; refreshed `ui_gameplay.png`, `ui_variant.png`, `ui_movement_route.png`, `ui_class_target.png`, and `ui_valid_shot.png` | PASS |
| Tactical terrain has clearly readable elevation | `GridView` supplies distributed raised mesas at levels 1 and 2 plus lowered basins at level -1; exposed neighbor differences generate cliff faces and depth-sorted occlusion without debug height labels | `grid_ui` height-distribution, projection, inverse-hit, and foreground-ownership regressions; refreshed default, close-zoom WebGL, and off-center `ui_variant.png` evidence show stepped silhouettes and cliff faces | PASS |
| Tactical tile artwork uses the correct world origin | Terrain, structures, and canopy art share `GridView::ground_anchor` and pivot-aware art bounds; terrain atlas cells use their measured 4:3 destination aspect and culling uses the same bounds | `grid_ui::tests::terrain_art_bounds_keep_the_declared_pivot_on_the_tile_anchor`; `visual_assets::tests::terrain_atlas_visible_cells_share_the_declared_horizontal_ground_axis`; default, live close-zoom, panned variant, route, target, and full capture evidence | PASS |
| Minimum active work reaches four hours | Goal timer has not yet reached 14,400 active seconds | Goal status audit | PENDING |

Completion is prohibited while any row remains `PENDING` or otherwise unresolved.
