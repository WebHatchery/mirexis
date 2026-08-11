# Mirexis Asset Pipeline Audit

Audit date: 2026-08-11  
Runtime manifests: `assets/data/texture_manifest.json`, `assets/data/sprite_definitions.json`

## Runtime architecture

- `VisualCatalog` loads stable texture IDs from the manifest and sprite definitions from embedded JSON.
- Startup validation reports texture dimensions, definition counts, and unresolved references.
- Missing required art uses a conspicuous magenta-and-black diagnostic texture; no production character, portrait, terrain, colony, or equipment path silently falls back to geometry.
- Painted atlas textures use linear sampling and the game renders through a 1280 x 720 virtual canvas with uniform scaling and letterboxing.
- Four manifest-defined faction channels combine colour, allegiance-base geometry, silhouette, projected terrain material, and faction-specific effect selection. Tactical units, briefing vignettes, battlefield biomes, and combat impacts all resolve through the same channel records.

## Authored atlas inventory

| Family | File(s) | Grid | Content |
|---|---|---:|---|
| Recruit tactical sprites | `assets/art/atlases/recruit_*.png` | 4 x 6 each | Four facings; idle, move, attack, hit, incapacitated, and faction-power states |
| Enemy tactical sprites | `assets/art/atlases/{brood,directorate,ascendant}_*.png` | 4 x 6 each | Seven enemy records across six silhouette families, four facings, six states |
| Recruit portraits | `assets/art/portraits/recruits.png` | 5 x 1 | Kira, Mara, Ilya, Sol, Nadi illustrated HUD portraits |
| Enemy portraits | `assets/art/portraits/enemies.png` | 6 x 1 | Brood, Directorate, and Ascendant family portraits |
| Terrain | `assets/art/terrain/terrain_atlas.png` | 4 x 3 | Modular ground, elevation, cover, roof, and biome material cells |
| Colony | `assets/art/colony/buildings_atlas.png` | 4 x 4 | Unique settlement buildings and operational-state variants |
| Equipment | `assets/art/equipment/equipment_atlas.png` | 4 x 3 | Weapons, armour, field tools, and faction prototypes |
| Effects | `assets/art/effects/effects_atlas.png` | 4 x 2 | Muzzle, impact, critical, healing, faction, and objective effects |

All declared atlas grids were checked for exact divisibility and non-empty alpha coverage in every cell. The final package contains 27 runtime asset files (25.85 MB) and includes both manifests and all authored atlas families. Moving editable masters out of the runtime tree removed more than 28 MB of authoring-only data from the shipped package.

ImageGen source masters and chroma-key intermediates are retained in root `art_sources/`, outside the runtime `assets/` tree. They remain editable and traceable but are not copied into `assets.zip`.

## Runtime verification

- `cargo test --all-targets`: 213 gameplay tests plus the source-limit target test passed; the source-size audit passed with every Rust file at or below 800 physical lines (current maximum 797).
- `cargo test -p macroquad-toolkit`: 338 library tests and 30 active integration/source tests passed (8 platform-dependent tests ignored).
- `publish.ps1`: Windows and WebGL release builds passed; both asset packages generated and preview deployment completed.
- Published Gamepad API bridge exists at `shared-assets/runtime/macroquad-gamepads-0.1.js` and is referenced by the deployed Mirexis page.
- Live WebGL browser verification exercised title, colony, briefing, deployment, tactical HUD, and help; the accepted fresh load produced zero warning/error console entries.
- Shared runtime bridges now use content-specific cache keys. The gamepad 0.1.7 bridge reports miniquad's encoded version `65543`, and storage exposes a matching named-plugin handshake.
- Package inventory confirms `effects_atlas.png`, the rebuilt enemy portrait atlas, sprite definitions, and texture manifest are present; `art_sources` is absent.
- Final 85-scene capture set contains no visible or pixel-scan magenta missing-asset diagnostics.
- A primitive-rendering source audit confirms remaining circles, polygons, lines, and rectangles are limited to allegiance bases, shadows, projected overlays, UI surfaces, environmental dressing, and effects; character bodies and portraits resolve through authored atlas textures.
