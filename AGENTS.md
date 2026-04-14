# AGENTS.md

See [OBJECTIVE.md](./OBJECTIVE.md) for project Phase 1/2 goals.
See [SYNTAX.md](./SYNTAX.md) for syntax rules (source of truth for code access patterns).

---

## Syntax Rules - MANDATORY

**The rule: Structure → File location → File content are BIDIRECTIONALLY BOUND.**

```
game::component::Position   → src/game/component/Position.rs
game::component::Velocity → src/game/component/Velocity.rs
game::system::input      → src/game/system/input.rs
game::system::physics   → src/game/system/physics.rs
```

**Filename case mirrors struct case:**
- `Position` struct → `Position.rs` (uppercase struct, uppercase file)
- `velocity` struct → `velocity.rs` (lowercase struct, lowercase file)

**Before writing any code, you MUST:**
1. Look at `src/game/mod.rs` to see what modules exist
2. If creating a new struct `X`, create `src/game/component/X.rs`
3. If creating new impl blocks for `X`, they go in `src/game/component/X.rs`, NOT separate files
4. If creating a system, place in `src/game/system/X.rs`

**If SYNTAX.md rules conflict with actual code:** The code must be refactored to match SYNTAX.md. Do NOT create files that violate these rules.

> ✗ WRONG: `Position` struct in `src/game/player.rs`  
> ✓ CORRECT: `src/game/component/Position.rs` containing `Position` struct

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