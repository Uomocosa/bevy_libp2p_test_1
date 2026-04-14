# AGENTS.md

See [OBJECTIVE.md](./OBJECTIVE.md) for project Phase 1/2 goals (source of truth for what to build).

---

## Project Quick Facts

- **Type:** Binary crate (Bevy game engine + libp2p P2P networking)
- **Key deps:** Bevy 0.18.1, libp2p 0.56, tokio, serde
- **Entry point:** `examples/boxes.rs` (run with `cargo run --example boxes`)

---

## Build, Lint, Test

```bash
cargo build              # Build project
cargo run                # Run game
cargo run --example boxes  # Run main P2P example
cargo clippy             # Lint
cargo fmt                # Format
cargo fmt -- --check     # Check without modifying
cargo test --all-targets # Run ALL tests (lib + examples)
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

## Architectural Rules (Target)

**Goal: one function per file** for struct impl blocks. When adding new behavior to an existing struct, create a new file:

```
ModuleName/
├── mod.rs              # Module declarations only
├── ModuleName.rs      # Struct definition + trait derives only
├── fn_name_one.rs     # impl block for fn_name_one ONLY
└── fn_name_two.rs     # impl block for fn_name_two ONLY
```

- DO NOT modify existing files when adding new functions
- Create new `new_function_name.rs`
- Add `pub mod new_function_name;` to mod.rs

---

## Code Style

- Clarity over cleverness
- Early returns to reduce nesting
- Line length: 100 chars max
- 4 spaces indentation

### Testing

**Rule: No `tests/` folder.** Tests live either:
- Inline in the module they test as `#[cfg(test)] mod tests { ... }`
- In `examples/` as example programs with `#[cfg(test)]` modules

**Rule: One function/class per test file.** The test file/module name matches what it tests:
- `src/p2p/swarm.rs` → `mod tests { #[test] fn test_p2p_swarm_...() }`
- `examples/headless_bevy.rs` → `#[cfg(test)] mod tests { #[test] fn test_headless_...() }`

**Rule: Use `assert!` for assertions:**
```rust
// CORRECT: Proper assertion
assert!(player.y > 0.0, "Player should be above ground: y={}", player.y);

// WRONG: Print-based check (creates false positive)
if player.y <= 0.0 {
    tracing::error!("FAIL: Player should be above ground");
}
```

**Rule: Use `tracing!` macros for output:**
- **Use** `tracing::debug!`, `tracing::info!`, `tracing::error!`
- **Do NOT use** `println!`, `print!`, `eprintln!`, `eprint!`

### Examples

Examples go in `examples/`. Two types:
- **Simple examples** - Demonstrations without tests
- **Complex examples** - Integration tests that verify system behavior (contain `#[cfg(test)]` modules)

Run all tests including examples:
```bash
cargo test --all-targets
```

### Occasional Checks

**Flaky test detection** - Run tests multiple times to catch nondeterminism:
```bash
for i in {1..5}; do cargo test --all-targets; done
```

**Single-threaded testing** - Run tests sequentially to catch race conditions:
```bash
cargo test --all-targets -- --test-threads=1
```

**Combined check** (for thorough validation):
```bash
for i in {1..5}; do cargo test --all-targets -- --test-threads=1; done
```

### Naming

| Item | Example |
|------|---------|
| Modules | `snake_case` (my_module) |
| Structs | `PascalCase` (MyStruct) |
| Functions | `snake_case` (my_function) |
| Constants | `SCREAMING_SNAKE_CASE` |

---

## Git Workflow

**Commit messages:**
```
feat: add user authentication
fix: handle null response in API client
docs: update README
refactor: extract validation logic
test: add integration tests
chore: upgrade dependencies
```

**Branch naming:** `feature/<desc>`, `fix/<desc>`, `docs/<desc>`

---

## Before Committing

```bash
cargo fmt && cargo clippy && cargo test --all-targets
```