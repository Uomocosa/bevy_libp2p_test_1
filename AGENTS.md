# AGENTS.md

Welcome. You are an expert Rust software engineer. 
Before taking any action, you must read:
1. [OBJECTIVE.md](./OBJECTIVE.md) for the project's goals, constraints, and current phase.
2. [SYNTAX.md](./SYNTAX.md) for domain-specific syntax, naming conventions, and file structure rules.

---

## The Prime Directive: Structure & Constraints

**Before writing or modifying any code, you MUST:**
1. Read `SYNTAX.md` to understand how this specific project maps structs/concepts to file names.
2. Look at `src/` to see the existing module hierarchy. Do not invent root modules.
3. If `SYNTAX.md` rules conflict with actual code, the code must be refactored to match `SYNTAX.md`. Do NOT create files that violate the project's structural rules.

## Universal Rust Code Style

- **Clarity over cleverness:** Write readable, maintainable code.
- **Early returns:** Use early returns (`?` operator or explicit `return`) to reduce nesting and rightward drift.
- **Indentation:** 4 spaces.
- **Logging:** Use `tracing!` macros (NOT `println!`). 
  ```rust
  tracing::debug!(target: "module_name", var_name = var.value);
  ```

## Basic Syntax Rules

- **Filename is THE EXACT same as the struct/function/enum/... in it:**
   - `StructNameExample` struct → `StructNameExample.rs` file
   - `SimpleEnum` enum → `SimpleEnum.rs` file
   - `poll_network_system` function → `poll_network_system.rs` file

- **STRICT CASE PRESERVATION (CRITICAL):** - Standard Rust module naming (`snake_case` for files) is **STRICTLY FORBIDDEN** if the primary struct/enum is `PascalCase`.
   - You MUST NOT downcase filenames. If you create a struct named `MyStruct`, you must output `MyStruct.rs`. Do NOT output `my_struct.rs`.
   - Assume `#![allow(non_snake_case)]` is handled at the crate level; do not attempt to format filenames to satisfy standard Rust compiler warnings.

## Universal Testing Philosophy

**No `tests/` folder.** Tests live strictly in:
- The same file as the code they test: `#[cfg(test)] mod tests { ... }`
- Integration tests go in the `examples/` directory.

Use `assert!` with descriptive messages for assertions:
```rust
assert!(player.y > 0.0, "Player should be above ground: y={}", player.y);
```

## Standard Build & Verification Routine

**No unit tests exist.** Run examples to verify the app works:
```bash
cargo build
cargo clippy -- -D warnings
cargo fmt -- --check
```
# Check OBJECTIVE.md or the examples/ directory for the specific run commands for this project.

## Git Workflow

- **Commit messages:** Must be semantic (`feat:`, `fix:`, `docs:`, `refactor:`, `test:`, `chore:`).
- **Branch naming:** `feature/<desc>`, `fix/<desc>`, `docs/<desc>`.
