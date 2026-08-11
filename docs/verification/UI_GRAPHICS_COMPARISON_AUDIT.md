# Mirexis UI Graphics Comparison Audit

Audit date: 2026-08-12
Research authority: `docs/research/Tactical_RPG_Graphics_Comparative_Report.docx`  
Written translation: `docs/UI_GRAPHICS_VISUAL_SPEC.md`

## Evidence sets

- Starting captures: 82 PNGs in `docs/verification`.
- Prototype rebuild captures: `docs/verification/ui_rebuild`.
- Final implementation captures: 85 canonical PNGs directly in `docs/verification`, including controller focus, mouse hover, and explicit damaged and repaired settlement states.
- Immutable file inventory: `FINAL_CAPTURE_INVENTORY.md` records every scene filename, byte count, and SHA-256 digest.
- Final contact sheets: eight temporary, manifest-ordered sheets generated from the canonical root captures and inspected during the 2026-08-12 audit.
- Scale checks: 1024 x 768, 1366 x 768, 1440 x 900, 1600 x 900, and 1920 x 1080 capture sets, summarized by `responsive_supported_sizes_contact.jpg`.
- Final strategy-control scale check: `responsive_final_controls_1024x768` verifies colony, roster, gene lab, briefing, and debrief cues at the smallest supported desktop size.
- Grayscale checks: `docs/verification/grayscale_audit` and the refreshed 14-scene `grayscale_audit_final` set.

## Baseline-to-final findings

| Report criterion | Starting state | Final verified state |
|---|---|---|
| Battlefield projection | Flat rectangular board | Three-quarter diamond projection with inverse hit testing, projected overlays, elevation sidewalls, foreground handling, and depth-sorted sprites |
| Terrain clarity | Repeated checkerboard and shallow blocks | Modular painted terrain, distinct height bands, walls/cover, biome dressing, hazard patterns, and objective beacons |
| Character identity | Circles, rectangles, crude geometric people | Authored recruit and enemy atlases with facings/states, silhouettes, allegiance bases, equipment/mutation cues, and matching portraits |
| Colony | Small procedural island | Painted inhabited settlement with unique buildings, paths, inhabitants, construction, power, damage/repair, and campaign-evolution layers |
| Combat presentation | Small text and numerals | Portrait comparison forecast, directional staging, recoil, impact, critical treatment, faction effects, replay timeline, and readable post-action markers |
| UI consistency | Partial chamfer restyle | Shared typography, surfaces, button states, spacing, portraits, warnings, footer navigation, tooltips, keyboard cues, and controller cues across the full screen set |
| Campaign identity | Phases and endings primarily changed text | Illustrated event portraits, contact/convergence/Mirexis decision dossiers, doctrine/equipment badges, phase dressing, and three materially different ending settlement manifestations |
| Missing art | Procedural substitution | Manifest validation and magenta diagnostic; no missing diagnostic appears in final captures |

## Defect sweep

- Clipping: none observed across the 85 final captures or five supported-size responsive audits.
- Tiny text: required labels remain legible at the supported virtual scale; non-essential metadata alone uses the caption tier.
- Visual sameness: faction maps, hazards, campaign phases, and all three endings have distinct material and effect language.
- Weak hierarchy: title, briefing, forecast, modal, debrief, and event screens now lead with portrait or scene identity before dense detail.
- Placeholder art: none observed in production-required surfaces.
- Unclear terrain: projected diamond edges, stepped top surfaces, exposed cliff faces, cover edges, and pattern-coded overlays remain distinguishable without debug height labels.
- Inconsistent controls: shared button states and stable back/confirm/footer cues are present; Web builds add controller navigation through the shared runtime bridge.
- Input discoverability: title, strategy, briefing, tactical, modal, and debrief surfaces now expose accurate keyboard/controller cues; the final strategy cues were separately checked at 1024 x 768.
- Browser runtime: a fresh published WebGL load is warning/error-free; title-to-colony-to-briefing-to-deployment-to-tactical navigation, colony placement, movement selection, and colony/tactical wheel zoom were exercised interactively.

## Capture verdict

Every scene declared by `scripts/capture_ui.ps1` emitted a current PNG. The eight final contact sheets were inspected as a whole. Controller focus, mouse hover, colony decisions, construction/damage/repair, research, help, combat feedback, targeting, reinforcements, debrief, responsive layouts, and the support atlases were also inspected at original resolution because they carry the highest density or most acceptance-critical state. `capture_audit.json` records 85 images at 1280 x 720, no missing or extra names, no magenta diagnostic pixels, no exact duplicate scenes, and no blank, excessively dark, or low-detail frames. A manifest test also guarantees all 85 names are unique and have explicit scene builders rather than falling through to a generic capture. No visual acceptance defect was found that requires a code or asset correction before the time gate.
