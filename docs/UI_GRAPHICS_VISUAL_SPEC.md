# Mirexis UI and Graphics Visual Specification

Status: implementation authority  
Source: `docs/research/Tactical_RPG_Graphics_Comparative_Report.docx`  
Baseline evidence: 82 captures in `docs/verification` and 8 prototype rebuild captures in `docs/verification/ui_rebuild`

## 1. Visual thesis

Mirexis is a readable frontier-biotech tactical diorama. The permanent view must explain position, allegiance, facing, elevation, cover, hazards, objectives, and legal actions before it asks the player to read text. At a second glance it must show the colony's progression from human industrial survival into an unstable hybrid of Directorate technology, Brood biology, and Ascendant geometry.

The production target combines:

- Final Fantasy Tactics' compact three-quarter architecture, visible height bands, directional sprites, and muted terrain behind saturated actors.
- GBA and 3DS Fire Emblem's economical split between tactical tokens, illustrated portraits, explicit forecasts, and short authored impact poses.
- Fire Emblem Engage's redundant allegiance and state-change channels: silhouette, base, portrait frame, brackets, and overlays never rely on colour alone.
- Triangle Strategy's authored-diorama material richness, while forbidding persistent bloom, blur, dense particles, or ground noise during planning.

Spectacle is temporary. Planning is always sharp. Resolution may briefly use a push-in, recoil, particles, lighting pulse, or colour grade, then must restore a fully readable board.

## 2. Visual hierarchy

The player must read each frame in this order:

1. Current focus: selected unit, hovered tile, modal decision, or campaign choice.
2. Allegiance and immediate danger: colony cyan, hostile crimson, warning amber, lethal red.
3. Legal movement, attack, support, objective, cover, height, and hazard state.
4. Character identity: sprite silhouette, portrait, class/equipment/mutation cues.
5. Environment and campaign evolution.
6. Atmospheric ornament and narrative texture.

No lower layer may reduce the contrast or legibility of a higher layer.

## 3. Palette and faction channels

### Semantic colours

| Role | Colour | Secondary non-colour channel |
|---|---|---|
| Colony / selected | cyan-teal | round base, white corner brackets, solid fill |
| Directorate | hot vermilion | squared base ticks, scan lines, hard mechanical angles |
| Brood | carmine / acidic coral | barbed base ticks, organic pulse, asymmetrical silhouette |
| Ascendant | violet / ultraviolet | diamond base ticks, concentric geometry, phase shimmer |
| Movement | cool cyan | dotted centre path and chevrons |
| Attack / threat | crimson | diagonal hatch and pointed border |
| Support / heal | mint | cross motif and soft ring |
| Objective | amber | double ring and beacon diamond |
| Hazard | faction-dependent | unique pattern and icon per hazard family |
| Disabled | slate | reduced value, no emissive edge, explicit lock/reason |

Faction accents may decorate panels but may never redefine semantic selection, warning, success, or lethal colours.

### Terrain values

Terrain remains a low-frequency, mid-to-dark value field. Tile centres are clean. Detail belongs on edges, walls, seams, puddles, cabling, rubble, growth, and props. Walkable surfaces, blocked volumes, occupied tiles, and hazards must remain distinguishable in grayscale.

## 4. Typography and layout system

- Logical reference canvas: 1280 x 720, scaled uniformly with letterboxing where necessary.
- Supported desktop browser checks: 1280 x 720, 1366 x 768, 1440 x 900, 1600 x 900, and 1920 x 1080.
- Minimum rendered body size at the smallest supported scale: 14 logical px; captions 12 px only when non-essential.
- Display face: condensed industrial uppercase for headings and tactical labels.
- Reading face: open sans-serif mixed case for descriptions, biographies, help, logs, and event prose.
- Type scale: 12 caption, 14 metadata, 16 body/control, 20 subheading, 28 section title, 64+ title mark.
- Spacing unit: 4 px. Standard gaps: 8, 12, 16, 24, 32. Standard panel padding: 16 or 20.
- Panel hierarchy: backdrop, surface, raised surface, modal. Each step adds value and edge contrast, not arbitrary new colour.
- Focused controls use a white/cyan outer keyline and visible keyboard/controller cue. Hover brightens the leading edge. Pressed offsets content by 1-2 px. Disabled controls retain readable labels and show the blocking reason in a tooltip or adjacent note.
- Navigation position and wording remain stable across related screens. Escape/back and confirm cues appear consistently.

## 5. Asset pipeline

### Required runtime structure

- One manifest-driven asset loader owns texture loading and reports every missing or malformed asset.
- Texture atlases are grouped by use: tactical units, HUD portraits, terrain/structures, colony buildings, and effects/UI marks.
- Sprite definitions map stable IDs to atlas rectangles, pivots, logical size, faction/palette channel, facings, animation state, and frame timing.
- Required facings: south-east, south-west, north-east, north-west. Mirroring is allowed only when equipment and mutation cues remain correct.
- Required unit states: idle, move, attack anticipation, attack release, hit/recoil, incapacitated. Critical and faction-power states may reuse the base state plus effect tracks.
- Texture filtering is linear for the high-resolution painted atlases used by this pass; pixel-native replacements may opt into nearest-neighbour. Integer or half-integer presentation scaling avoids shimmer.
- Palette channels are separate from semantic UI colours. Recruits preserve personal clothing colours while allegiance bases remain colony cyan.
- Missing assets render as an unmistakable magenta-and-black diagnostic tile with the missing asset ID. Silent procedural substitution is forbidden for production-required character, portrait, building, or terrain art.
- Startup logging reports loaded atlas dimensions, definition counts, and missing references.

### Atlas authoring rules

- Tactical cells use a uniform source-cell size and consistent foot pivot.
- Portraits share eye-line, crop, lighting direction, frame safe area, and mutation/equipment attachment zones.
- Terrain tiles include top face, front/side height bands, damaged variants, cover edges, wall segments, stairs/ramps, roof/foreground masks, and biome dressing.
- Buildings include operational, construction, unpowered, damaged, repair, and campaign-stage overlays without requiring unique full images for every combination.

## 6. Battlefield presentation

### Projection and hit testing

- Use a fixed three-quarter/isometric diamond projection. Grid-to-screen and screen-to-grid must be mathematical inverses within tile boundaries.
- Draw order is by projected depth, then elevation, then layer: ground, decals/overlays, structures/cover, units, foreground/roofs, screen-space markers.
- Every tile has an explicit elevation. At least three height bands must be visible in the validation map.
- Elevated tiles draw top faces plus shaded vertical side faces. Stairs/ramps visibly connect levels.
- Walls and cover occupy edges or volumes with visible orientation and height. Cover indicators follow the projected edge rather than floating as unrelated bars.
- Roofs and foreground blockers fade or cut away when they cover the selected unit, hovered tile, legal destination, objective, or target line.
- A flat tactical mode may simplify depth but must preserve the same tile identity, hit testing, and semantic overlays.

### Overlays

- Movement, attack, threat, hazard, objective, targeting, and route overlays conform to projected diamond tiles.
- Overlay alpha never obscures terrain edges or unit feet. Patterns survive grayscale and common colour-vision deficiencies.
- Selected tiles use white corner brackets plus a cyan inset line. Targets use faction-aware brackets and a comparison line.
- Height, cover, hazard, and objective icons occupy stable corners of the projected tile and never collide with vitality/readiness markers.

### Camera and scale

- The complete playable map and command rail remain usable at 1280 x 720.
- Larger windows increase board scale or breathing room without stretching the logical UI.
- Camera framing keeps selected and targeted units visible during forecast and resolution.
- Resolution push-in lasts about one second and returns to the exact prior planning frame.

## 7. Character and portrait system

### Recruits

Every recruit requires a recognisable silhouette at tactical scale and a matching illustrated HUD portrait:

| Recruit | Silhouette anchor | Equipment cue | Mutation cue |
|---|---|---|---|
| Kira Voss | lean forward surveyor stance, asymmetric sensor hood | long frontier rifle, survey harness | luminous neural crown/tendrils |
| Mara Venn | broad planted miner silhouette | Mire LMG, heavy work plates | layered chitin shoulder and forearm shell |
| Ilya Reed | compact clinician coat, upright support stance | pistol and field medkit | pale regenerative vein/marrow glow |
| Sol Cairn | long-limbed gantry runner | toolkit, drone controller, sidearm | extended tendon lines and elastic stance |
| Nadi Vale | narrow xenobiologist silhouette with companion mass | bio-injector/sidearm | visible shoulder/back symbiote |

Class changes alter stance, shoulder line, carried tool, and attachment silhouette. Equipment changes must be visible at least by weapon/tool family. Mutation evolutions intensify or redirect the mutation layer rather than appearing only as text.

### Enemy families

| Family | Required archetypes | Shape language | Motion/effect language |
|---|---|---|---|
| Directorate | rifleman, suppression drone | hard armour planes, squared equipment, disciplined upright line | scan sweep, muzzle vector, sparks, directional recoil |
| Brood | stalker, sporecaster | low predatory wedge vs swollen artillery sac, asymmetry, talons | organic pulse, spore bloom, wet impact, spreading edge motion |
| Ascendant | sentinel, rift warden | floating diamonds, split limbs, impossible symmetry, strong negative space | geometric phase step, concentric distortion, violet fracture |

All units require four facings and the six core states. Incapacitated units must remain identifiable while clearly no longer active. Allegiance bases and HUD portrait frames redundantly encode team/faction.

## 8. Colony presentation

The colony is an inhabited settlement, not a construction spreadsheet.

- Projection: the same three-quarter visual grammar as the battlefield, at a calmer camera scale.
- Ground: marsh islands, raised industrial plates, walkways, service paths, water, reeds, floodlights, pipes, crates, cables, vents, and small human activity silhouettes.
- Paths connect occupied plots and communicate settlement growth.
- Buildings: Command Centre, Barracks, Infirmary, Workshop, Barricade, Hydroponics, Power Plant, and Gene Lab each have unique footprints and rooflines.
- Construction: foundation/blueprint, scaffold/partial shell, and near-complete states.
- Powered: warm interior light or faction-appropriate emissive function marks. Unpowered: cold dark windows, stopped machinery, explicit power icon.
- Damaged: broken silhouette, exposed interior, smoke/sparks where readable. Repair: scaffolding, work light, and progress cue.
- Campaign evolution: Directorate cabling and scan hardware, Brood growth and membranes, and Ascendant inlays progressively accumulate according to campaign state while human structure remains legible.
- Interaction: hover isolates the plot, brightens its path, raises a concise status card, and shows valid action/cost. Selection is visually persistent.
- Strategic choices must be readable from the settlement image without opening another panel.

## 9. Combat staging and effects

1. Forecast: attacker portrait/sprite, defender portrait/sprite, hit chance, damage, armour/cover change, range, AP cost, and before/after vitality appear in one comparison composition.
2. Commit: camera eases toward the pair; non-participants dim slightly without disappearing.
3. Anticipation: attacker uses a readable wind-up pose; faction cue begins.
4. Release: projectile/beam/lunge follows the actual direction between projected tiles.
5. Impact: directional recoil, short hit stop, impact sprite/particles, damage numeral, armour/cover response, and critical treatment.
6. State restore: effects clear; incapacitation, hazard, objective, reinforcement, and remaining AP/health are immediately legible.

Critical hits use a materially different flash, portrait crop, and impact frame. Directorate, Brood, and Ascendant attacks remain identifiable in grayscale by geometry and motion. Phase transitions, objective changes, hazards, reinforcements, and invalid commands receive authored banners/markers rather than relying on log text.

## 10. Screen system

- Title: illustrated faction/recruit tableau, readable title mark, primary actions, build/version/footer, and no crude geometric figures.
- Briefing: mission art/map vignette, intelligence hierarchy, faction threat, objective, rewards, squad strip, and deployment readiness.
- Deployment/loadout: portraits, class/equipment silhouettes, legal/locked states, food/readiness budget, and consistent confirm/back controls.
- Colony/construction: inhabited visual settlement is dominant; operations and resources form a secondary command rail.
- Roster/relationships/research/gene lab/equipment: portrait-led identity, stable tabs/navigation, comparable cards, visible prerequisites/costs, tooltips, and disabled reasons.
- Help/log: legible reading typography, grouped topics, key/controller cues, filters/chronology, and a clear return action.
- Tactical HUD/targeting/forecasts/replay/feedback: board remains dominant; decision information sits in predictable edge zones; temporary overlays never leave stale ambiguity.
- Debrief/events/endings: illustrated state, clear outcome/rewards/consequences, portrait reactions, and decisive continuation controls.

## 11. Acceptance-wide visual tests

- Grayscale: units, passable ground, blocked volumes, hazards, selection, and objectives remain distinct.
- Blur/squint: focus, allegiance, danger, and dominant action retain their hierarchy.
- Smallest scale: no required label falls below the defined type floor; no control or tooltip clips.
- Input parity: mouse hover, keyboard focus, controller focus, confirm, cancel, and disabled states use the same design language.
- Asset integrity: no production-required missing-asset diagnostic appears in final captures.
- Sameness: campaign phases, factions, missions, hazards, endings, and colony evolution are visually distinct without breaking semantic colours.
- Capture completeness: every scene in the acceptance matrix has a current post-overhaul PNG and a resolved verdict.
