# AGENTS.md

See [OBJECTIVE.md](./OBJECTIVE.md) for project Phase 1/2 goals.

---

## Project Quick Facts

- **Type:** Binary crate (Bevy game engine + libp2p P2P networking)
- **Key deps:** Bevy 0.18.1, libp2p 0.56, tokio, serde
- **Entry point:** `src/main.rs`

---

## Build, Lint, Test

```bash
cargo build          # Build project
cargo run          # Run game
cargo clippy       # Lint
cargo fmt          # Format
cargo fmt -- --check  # Check without modifying
cargo test        # Run all tests
```

---

## Architectural Rules (CRITICAL)

This repo enforces **one function per file**. For every struct, create a folder:

```
ModuleName/
├── mod.rs              # Module declarations only
├── ModuleName.rs      # Struct definition + trait derives only (NO impl blocks)
├── fn_name_one.rs     # impl block for fn_name_one ONLY
└── fn_name_two.rs     # impl block for fn_name_two ONLY
```

**Enforcement:** When adding a new function to existing struct:
- DO NOT modify existing files
- Create new `new_function_name.rs`
- Add `pub mod new_function_name;` to mod.rs

---

## Code Style

- Clarity over cleverness
- Early returns to reduce nesting
- Line length: 100 chars max
- 4 spaces indentation

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

Run these checks:
```bash
cargo fmt && cargo clippy && cargo test
```