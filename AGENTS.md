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
3. If `SYNTAX.md` rules conflict with actual code, the code must be refactored to match `SYNTAX.md`.

## Atomic File Structure (CRITICAL)

To maintain a clean, navigable codebase, we follow a strict **"One Logic Unit Per File"** policy:

### 1. Function Files
- If a file defines a function, it must contain **exactly one** primary public function.
- The filename must match the function name (e.g., `calculate_velocity.rs` contains `pub fn calculate_velocity`).
- Small, private helper functions are permitted only if they are used exclusively by the primary function and do not warrant their own file.

### 2. Struct Files & Decomposition
- Small structs and their `impl` blocks live in a single file named after the struct (e.g., `Position.rs`).
- **Large Impl Decomposition:** If an `impl` block becomes complex or contains multiple methods, you MUST:
    1. Create a subfolder with the same name as the Struct.
    2. Create individual files for each method inside that subfolder.
    3. The main Struct file's `impl` block will then act as a "router," calling these decomposed functions.
- *Example:* `Player` struct in `Player.rs` might have a directory `Player/` containing `jump.rs` and `move.rs`.

## Universal Rust Code Style

- **Clarity over cleverness:** Write readable, maintainable code.
- **Early returns:** Use `?` or `return` to reduce nesting.
- **Indentation:** 4 spaces.
- **Logging:** Use `tracing!` macros.
  ```rust
  tracing::debug!(target: "module_name", var_name = var.value);
  ```

## Filename & Case Rules

- **Filename is THE EXACT same as the primary item:**
   - `StructName` → `StructName.rs`
   - `simple_function` → `simple_function.rs`
- **STRICT CASE PRESERVATION:** Do NOT downcase filenames. If the struct is `PlayerInput`, the file is `PlayerInput.rs`. If the function is `apply_physics`, the file is `apply_physics.rs`.
- Assume `#![allow(non_snake_case)]` is handled at the crate level.

## Universal Testing Philosophy: Mandatory `test_usage`

Every logic file (Function or Struct) MUST contain a demonstration test.
- **The `test_usage` rule:** At the bottom of every file, include a `#[cfg(test)]` module with a test named `test_usage`.
- This test must act as a "living example," showing how to initialize the struct or call the function with realistic data.

**Example for a function file:**
```rust
pub fn add_velocity(pos: f32, vel: f32) -> f32 {
    pos + vel
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usage() {
        let result = add_velocity(10.0, 2.0);
        assert_eq!(result, 12.0, "Velocity addition failed");
    }
}
```

## Standard Build & Verification Routine

Verify changes with:
```bash
cargo build --all-targets
cargo clippy -- -D warnings
cargo fmt -- --check
cargo nextest run --all-targets
```

## Git Workflow
- **Commit messages:** Semantic (`feat:`, `fix:`, `docs:`, `refactor:`, `test:`, `chore:`).
- **Branch naming:** `feature/<desc>`, `fix/<desc>`.
