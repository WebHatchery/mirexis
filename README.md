# Mirexis

Mirexis is a Rust + Macroquad turn-based tactics game about a persistent frontier
colony caught between the Directorate, the Brood, and the Ascendants. The complete
game concept is in [MIREXIS.md](MIREXIS.md); the implementation architecture and
milestone plan are in [TECHNICAL_DESIGN.md](TECHNICAL_DESIGN.md).

## Phase 0

Phase 0 establishes a development-ready executable rather than production
gameplay. It includes:

- title-to-tactical application state flow
- a deterministic square-grid mission sandbox
- named, data-driven colonists and hostiles
- selection, movement range, action-point spending, and alternating phases
- deterministic placeholder hostile movement
- versioned toolkit persistence with save, load, and delete
- embedded JSON content suitable for native and WASM builds
- virtual-resolution UI and a headless capture hook
- unit tests plus the shared 800-line source gate

Combat, mission completion, colony management, progression, equipment, and
campaign generation are deliberately deferred. Their intended boundaries are
documented so they can be added without replacing the Phase 0 foundation.

## Development

```powershell
cargo run
cargo test
cargo fmt -- --check
cargo clippy --all-targets --all-features -- -D warnings
```

The project-standard end-to-end validation and packaging path is:

```powershell
.\publish.ps1
```

## Controls

- Mouse: select a colonist; click a highlighted tile to move
- Arrow keys: move the tactical cursor
- Enter: end the colony phase and resolve placeholder hostile movement
- S / L: save / load
- Escape: return to the title screen

## Data

Phase 0 content lives under `assets/data/`:

- `game_config.json`: identity, version, grid size, and action-point budget
- `mission.json`: briefing, objective, limit, reward, and blocked tiles
- `roster.json`: named units, faction, roles, mutations, and starting positions
- `texture_manifest.json`: future runtime texture declarations
