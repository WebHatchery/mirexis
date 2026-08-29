# Mirexis Concept Sprite Pass

This folder contains an expanded concept-art pass for Mirexis's roster,
colony life, tactical effects, equipment, and terrain language.
These are source/concept assets only; they are intentionally outside
`assets/` and are not registered in the runtime texture manifests.

## Sheets

| File | Grid | Contents | Source treatment |
| --- | --- | --- | --- |
| `mirexis_units_v01_key.png` | 3 x 2 | Six new colony and faction unit silhouettes | Magenta keyed |
| `mirexis_recruits_v01_key.png` | 4 x 2 | Eight human recruit/civilian silhouettes | Magenta keyed |
| `mirexis_salvage_engineer_states_v01_key.png` | 4 x 2 | Eight pose/state cues for one salvage engineer | Magenta keyed |
| `mirexis_elites_v01_key.png` | 3 x 2 | Six elite and commander-scale hostile silhouettes | Magenta keyed |
| `mirexis_colony_support_v01_key.png` | 4 x 2 | Eight adult colony support-role silhouettes | Magenta keyed |
| `mirexis_colony_props_v01_grid.png` | 4 x 2 | Colony structures, defenses, beacon, and faction intrusions | Transparent; normalized grid |
| `mirexis_hazards_v01_grid.png` | 4 x 2 | Mire, Brood, Directorate, Ascendant, overwatch, and recovery effects | Transparent; normalized grid |
| `mirexis_equipment_v01.png` | 4 x 3 | Weapons, tools, armor, drones, and faction devices | Transparent |
| `mirexis_terrain_dressing_v01.png` | 4 x 3 | Terrain, cover, contamination, relic, and objective tiles | Transparent |

The two unsuffixed 4 x 2 renders (`mirexis_colony_props_v01.png` and
`mirexis_hazards_v01.png`) are retained as original masters. Their `_grid`
siblings are a minimal resample to exactly `1776 x 888` so every cell is
mathematically divisible at `444 x 444`.

## Art direction

The pass follows the existing Mirexis vocabulary: a readable three-quarter
tactical diorama, worn human frontier hardware, Brood maroon/coral biology,
Directorate cream/vermilion machinery, and Ascendant violet geometry. Colony
objects stay practical and inhabited; effects use strong silhouettes and
restrained emissive accents so they can survive small tactical display sizes.

## Runtime follow-up

When a concept is approved, cut the selected cells, remove the magenta key
from keyed sheets with the existing source-art helper, preserve alpha on the
transparent sheets, and register only deployable atlases in
`assets/data/texture_manifest.json` and `assets/data/sprite_definitions.json`.
Run the normal `publish.ps1` validation after any runtime integration.
