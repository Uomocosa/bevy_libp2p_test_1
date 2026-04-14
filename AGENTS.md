# AGENTS.md

See [OBJECTIVE.md](./OBJECTIVE.md) for project Phase 1/2 goals.

See [SYNTAX.md](./SYNTAX.md) for syntax reasoning and rules (source of truth for how code is accessed).

---

## Project Quick Facts

- **Type:** Library crate (Bevy game engine + libp2p P2P networking)
- **Key deps:** Bevy 0.18.1, libp2p 0.56, tokio, serde
- **Entry point:** `examples/boxes.rs`

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

## Module Overview

| Module | Purpose |
|--------|---------|
| `app/` | Bevy plugin wiring |
| `p2p/` | libp2p swarm, protocol, mDNS discovery |
| `sync/` | Network state, message serialization, tick management |
| `game/` | Player, input, physics components |

---

## Structure From Syntax

The syntax rules in `SYNTAX.md` define how code is accessed. The structure must match:

### File Organization Rules

| Syntax Pattern | Structure |
|----------------|------------|
| `domain::ComponentName` | `domain/ComponentName.rs` (struct definition) |
| `domain::function_name` | `domain/function_name.rs` (system/function) |
| `domain::subdomain::Item` | `domain/subdomain/Item.rs` |

### Naming Conventions

| Content | Pattern | Example |
|---------|---------|---------|
| Component (struct) | `Name.rs` | `game/component/Position.rs` |
| System (function) | `name.rs` | `game/input_system.rs` |
| Impl block | `impl_name.rs` | `game/component/impl_position.rs` |
| Submodule folder | `snake_case` | `game/component/`, `p2p/swarm/` |

### One File = One Item

- **ONE struct** per file (e.g., `Position.rs` has `Position` struct only)
- **ONE function** per file (e.g., `input_system.rs` has `input_system` only)
- **ONE impl block** per file (e.g., `impl_position.rs` has `impl Position` only)

### Adding New Code

1. Determine syntax path (e.g., `game::input_system`)
2. Create corresponding file (`game/input_system.rs`)
3. Add `pub mod filename;` to `game/mod.rs`

---

## Code Style

- Clarity over cleverness
- Early returns to reduce nesting
- Line length: 100 chars max
- 4 spaces indentation

---

## Testing

**No `tests/` folder.** Tests live in:
- Same file as `#[cfg(test)] mod tests { ... }`
- In `examples/` as integration tests

**Use `assert!` for assertions:**
```rust
assert!(player.y > 0.0, "Player should be above ground: y={}", player.y);
```

**Use `tracing!` macros** (NOT `println!`):
```rust
tracing::debug!(target: "physics", vel_x = vel.x);
```

---

## Git Workflow

**Commit messages:** `feat:`, `fix:`, `docs:`, `refactor:`, `test:`, `chore:`
**Branch naming:** `feature/<desc>`, `fix/<desc>`, `docs/<desc>`