# AGENTS.md

See [OBJECTIVE.md](./OBJECTIVE.md) for project Phase 1/2 goals.
See [SYNTAX.md](./SYNTAX.md) for syntax rules (source of truth for code access patterns).

---

## Syntax Rules (Source of Truth)

**Module path defines file location:**
```
game::component::Position   → game/component/Position.rs
game::component::Velocity   → game/component/Velocity.rs
game::component::Player     → game/component/Player.rs
game::system::input         → game/system/input.rs
game::system::physics      → game/system/physics.rs
game::system::sync         → game/system/sync.rs
```

**Filename case mirrors struct case:**
- `Position` struct → `Position.rs` (uppercase struct, uppercase file)
- `velocity` struct → `velocity.rs` (lowercase struct, lowercase file)

**File name defines content:** The file `Position.rs` MUST contain a `Position` struct/enum. The file `input.rs` MUST contain an `input` function/system.

> **Rule:** Structure (syntax) → File location → File content. Each is bidirectionally bound.

---

## Quick Facts

- **Type:** Library crate (Bevy + libp2p P2P networking)
- **Key deps:** Bevy 0.18.1, libp2p 0.56, tokio, serde
- **Entry point:** `examples/boxes.rs`
- **Build:** `cargo build && cargo run --example boxes`

---

## Build, Lint, Test

```bash
cargo build
cargo run --example boxes
cargo clippy
cargo fmt -- --check
cargo test --all-targets
```

---

## Examples

| Example | Purpose |
|---------|---------|
| `boxes.rs` | Main game demo (mDNS discovery) |
| `test_only_mdns.rs` | mDNS discovery in isolation |
| `test_only_bevy.rs` | Basic Bevy P2P test |
| `test_bevy_dual_window.rs` | Dual player movement test |
| `headless_run_only_bevy.rs` | Headless P2P verification |

---

## Module Overview

| Module | Purpose |
|--------|---------|
| `app/` | Bevy plugin wiring |
| `p2p/` | libp2p swarm, protocol, mDNS discovery |
| `sync/` | Network state, message serialization, tick management |
| `game/` | Player, input, physics components |

---

## Architecture Notes

**Phase 1 (current):** Desktop with mDNS for local peer discovery.
**Phase 2:** Browser/WASM - mDNS won't work in browsers. Architecture must remain modular to swap discovery layers (mDNS → WebRTC signalling).

No authoritative servers - fully peer-to-peer.

---

## Code Style

- Clarity over cleverness
- Early returns to reduce nesting
- Line length: 100 chars max
- 4 spaces indentation
- **Filename case mirrors struct case:**
  - `Position` struct → `Position.rs` (uppercase struct, uppercase file)
  - `Velocity` struct → `Velocity.rs` (uppercase struct, uppercase file)
  - `velocity` struct → `velocity.rs` (lowercase struct, lowercase file)
  - **File naming:** Structure (syntax) defines file names:
    - `Position.rs` MUST contain a `Position` struct/enum
    - `Velocity.rs` MUST contain a `Velocity` struct/enum
    - `input.rs` MUST contain an `input` function/system
- **Use `tracing!` macros (NOT `println!`):**
  ```rust
  tracing::debug!(target: "physics", vel_x = vel.x);
  ```

---

## Testing

**No `tests/` folder.** Tests live in:
- Same file as `#[cfg(test)] mod tests { ... }`
- In `examples/` as integration tests

Use `assert!` for assertions:
```rust
assert!(player.y > 0.0, "Player should be above ground: y={}", player.y);
```

---

## Git Workflow

- **Commit messages:** `feat:`, `fix:`, `docs:`, `refactor:`, `test:`, `chore:`
- **Branch naming:** `feature/<desc>`, `fix/<desc>`, `docs/<desc>`