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

## Universal Rust Code Style

- **No Comments:** Do not write comments in the code. The code must be self-documenting through clear naming.
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
- **STRICT CASE PRESERVATION:** Do NOT downcase filenames.
- Assume `#![allow(non_snake_case)]` is handled at the crate level.

## Universal Testing Philosophy: Mandatory `test_usage`

Every logic file MUST contain a `#[cfg(test)]` module with a single test named `test_usage`.

**CRITICAL TESTING RULES:**
1. **Test the Primary Item:** The test MUST execute the specific function or struct method that the file is named after. Testing an arbitrary struct initialization instead of the file's primary logic is strictly forbidden.
2. **Context Instantiation:** If the primary item requires an execution context (e.g., a Bevy `App`, a `tokio` runtime, a `libp2p` network state), you MUST construct a minimal, working version of that context inside the test to properly execute the logic. Do not mock the inputs so trivially that the logic is bypassed.

**Example 1: Pure Function**
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
        assert_eq!(result, 12.0);
    }
}
```

**Example 2: Context-Dependent Logic (e.g., Bevy System)**
```rust
pub fn detect_click(
    mut query: Query<&mut ClickCounter>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
) {
    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    }
    for mut counter in &mut query {
        counter.increment();
        tracing::debug!(target: "clicker", "Clicked! New count: {}", counter.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::prelude::*;

    #[test]
    fn test_usage() {
        let mut app = App::new();
        app.world_mut().spawn(ClickCounter(0));
        
        let mut mouse_input = ButtonInput::<MouseButton>::default();
        mouse_input.press(MouseButton::Left);
        app.insert_resource(mouse_input);
        
        app.add_systems(Update, detect_click);
        app.update();
        
        let mut query = app.world_mut().query::<&ClickCounter>();
        let counter = query.single(app.world());
        assert_eq!(counter.0, 1);
    }
}
```

## Standard Build & Verification Routine

Verify changes with:
```bash
cargo build --all-targets
cargo clippy -- -D warnings
cargo fmt -- --check
cargo test --all-targets
```

## Git Workflow
- **Commit messages:** Semantic (`feat:`, `fix:`, `docs:`, `refactor:`, `test:`, `chore:`).
- **Branch naming:** `feature/<desc>`, `fix/<desc>`.
