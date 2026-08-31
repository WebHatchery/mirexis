# Mirexis Audio Sources

Mirexis Phase 1 audio is generated at runtime from deterministic synthesis definitions in
`src/audio.rs`. It contains no third-party recordings or music and requires no external
attribution.

The palette covers focus, invalid action, movement, weapon hit, weapon miss, damage,
recovery, ability, objective update, victory, defeat, city ambience, and tactical ambience. Fixed
seeds make every generated WAV repeatable. The quiet ambience loops use separate scenes
and stop cleanly when the game changes between title, city, and tactical contexts.

Any future file-backed sound must be added here with its creator, source URL, licence,
modifications, and the exact shipped asset path before it is registered for publishing.
