# Toolkit migration

`CameraTransform` now owns pan, cursor-anchored zoom, viewport projection and
center constraints. Both tactical and colony views obtain their screen origin
through that transform. Mirexis retains isometric/elevation projection, map-art
insets, centering policy for maps smaller than the viewport, selection tracking,
gesture suppression and colony plot confirmation. Existing compatibility tests
compare the viewport clamp against the original independent per-axis formula.

Outsider and colony event descriptions now wrap by measured panel width instead
of byte count. Field notes and colony dialogue use bounded shared text blocks.
Facility upgrade labels use shared truncation with an explicit font style. The
colony's established built-in-font policy is preserved, including measurement,
to avoid the dense panel's historical custom-font atlas exhaustion.

The audit also removes seven strict-Clippy findings: terrain modulo predicates,
a range pattern, excessive upgrade-card arguments and a complex test type. Game
data loading and other generic facilities already use toolkit APIs. Domain
content, progression, isometric math and player-facing decisions remain local.

Validation: 520 game tests, 403 shared toolkit tests, formatting, warning-strict
all-feature Clippy, default Windows/WebGL Preview publisher and tracking. Native
release captures verify the affected text panels and map rendering before commit.
