# Mirexis Art Source Masters

This directory is intentionally outside `assets/`. It preserves ImageGen outputs, reference boards, and chroma-key masters without shipping them in the runtime `assets.zip`.

## Runtime preparation

1. Normalize each atlas to an exactly divisible grid.
2. Remove the `#ff00ff` key with the ImageGen skill's `remove_chroma_key.py` helper, using a soft matte and spill cleanup.
3. Save deployable RGBA PNGs under `assets/art/<family>/`.
4. Register only deployable files in `assets/data/texture_manifest.json` and their grids in `assets/data/sprite_definitions.json`.
5. Run `cargo test visual_assets::tests` to decode every registered atlas and verify exact grid divisibility plus non-empty alpha coverage in every cell.

## Promoted concept atlases

The concept-sprite pass is now available at runtime under
`assets/art/concepts/`. All eighteen concept families are registered and
validated as RGBA atlases. The title tableau uses passage wreckage, flora, and
an Ascendant emplacement; tactical maps use flora, Brood fauna, terrain
dressing, faction emplacements, passage wreckage, and objective markers; and
the colony map uses colony props, flora, and machinery as deterministic
frontier dressing. Unit-facing concept sheets, equipment, salvage, interiors,
hazards, and operative actions are loaded and catalogued for later gameplay
data wiring without replacing the existing directional unit atlases.

## Equipment atlas

- Source: `equipment_atlas_chroma.png`
- Runtime: `assets/art/equipment/equipment_atlas.png`
- ImageGen mode: `stylized-concept`
- Grid: 4 x 3
- Prompt summary: a polished hand-painted 3DS-era tactical RPG equipment atlas on flat magenta, ordered as frontier rifle, Mire LMG, service pistol, breach scattergun; needle carbine, survey harness, chitin plate, field medkit; field toolkit, Directorate smartlink, Brood living plate, Ascendant phase lens. No text, people, shadows, borders, or cell overlap.

## Effects atlas

- Source: `effects_atlas_key.png`
- Runtime: `assets/art/effects/effects_atlas.png`
- ImageGen mode: default generation
- Grid: 4 x 2
- Original retained by Codex: `C:/Users/Kalai/.codex/generated_images/019ff033-397f-7481-9f75-f925247ab357/exec-31976ba9-feab-4d74-9d5c-f31c28294e2f.png`
- Prompt: “Create a production-ready transparent-effects source atlas for MIREXIS, a dark biotech 3DS-era tactical RPG. Exact 4 columns by 2 rows, eight evenly sized square cells, centered effect in each cell, generous transparent-safe margins, flat solid #FF00FF chroma-key background filling every unused pixel, no gradients in the background, no text, no labels, no people, no frame borders, no overlap between cells. Exact cell order left-to-right. Row 1: (1) sharp cyan-white frontier rifle muzzle flash pointing diagonally up-right, (2) directional metal-and-dust bullet impact with amber sparks, (3) large gold-white critical hit starburst with broken angular shards, (4) mint medical bioelectric recovery burst with a clear cross-shaped core. Row 2: (5) Directorate vermilion hard-edged scan shockwave with square circuitry and straight lines, (6) Brood acidic coral spore impact with barbed organic droplets and asymmetrical tendrils, (7) Ascendant violet phase fracture with concentric diamonds and impossible geometric shards, (8) amber objective activation beacon with double diamond rings and rising luminous motes. Hand-painted polished tactical game VFX, crisp readable silhouettes at 48-96 pixel display size, high contrast, restrained particle counts, consistent three-quarter lighting, no bloom haze outside each effect, no typography, no watermark. The atlas must read as exactly 4x2 separate cells.”

The remaining `*_key.png` masters correspond one-to-one with the recruit, enemy, portrait, terrain, and colony atlases registered by the runtime manifests.

## Enemy HUD portrait atlas

- Sources: `enemy_hud_portraits_key.png`, `enemy_hud_portraits_rgba_grid.png`
- Runtime: `assets/art/portraits/enemies.png`
- ImageGen mode: default generation
- Source grid: 3 x 2; runtime strip: 6 x 1
- Original retained by Codex: `C:/Users/Kalai/.codex/generated_images/019ff033-397f-7481-9f75-f925247ab357/exec-888a273d-6974-4ef8-b521-236946ad0d9b.png`
- Prompt summary: six large head-and-shoulders HUD portraits on flat magenta in this exact order: Brood stalker, Brood sporecaster, Directorate rifleman, Directorate suppression drone, Ascendant sentinel, Ascendant rift warden. Consistent upper-left three-quarter light, tight portrait crop, faction-readable silhouettes, no labels, borders, scenery, or full-body poses.
