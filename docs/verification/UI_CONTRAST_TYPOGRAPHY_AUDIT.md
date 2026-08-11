# Mirexis UI Contrast and Typography Audit

Audit date: 2026-08-12
Evidence: final 1280 x 720 capture set plus the tracked `responsive_supported_sizes_contact.jpg` and `responsive_final_critical_scenes.jpg` scale summaries

## Contrast samples

Ratios use WCAG relative-luminance math against the actual shared widget and near-black panel colours. This is a readability benchmark for the rendered game UI, not a claim that canvas text is semantic HTML.

| Foreground / surface | Ratio | Verdict |
|---|---:|---|
| Primary text / default button | 10.13:1 | PASS |
| Primary text / hover button | 5.38:1 | PASS |
| Primary text / focused button | 7.41:1 | PASS |
| Dim text / disabled button | 6.42:1 | PASS |
| Warning / near-black panel | 10.17:1 | PASS |
| Positive / near-black panel | 9.55:1 | PASS |
| Negative / near-black panel | 5.18:1 | PASS |
| Accent / near-black panel | 6.59:1 | PASS |

All sampled functional text exceeds 4.5:1. State is never communicated by colour alone: buttons change border/fill, projected overlays use solid/hatch/double-ring patterns, allegiance uses silhouette plus base shape, and warnings use placement, iconography, and labels.

## Typography tiers

- Display/title: 30-82 logical pixels.
- Screen and panel headings: 15-34 logical pixels.
- Buttons and primary controls: 16 logical pixels.
- Body and operational data: 10-15 logical pixels.
- Captions: 9.5 logical pixels minimum and limited to redundant height/ending metadata.

The final readability pass raised event-kind labels, cover values, armour markers, completed research badges, and height labels. The root capture suite covers the densest battle log, tactical HUD, cover overlay, research state, controller-focused title, and strategy-screen input cues. A current-release 1024 x 768 pass additionally verifies the Mirexis decision, debrief, and equipment/targeting panels. Uniform letterboxing is intentional; no label, footer cue, recovery control, or focus ring clips at the smallest supported desktop size.

## Verdict

PASS. Functional hierarchy remains readable at all supported sizes, disabled/focused/hovered states remain distinct, and the grayscale audit confirms faction and tile-state information does not depend on hue.
