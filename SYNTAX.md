# SYNTAX

## Rule Priority
This file's rules override standard Rust conventions. Treat this file as the absolute source of truth for architecture and naming.

## Game Module Mapping
We are building two test games. In these rules, we use `game::` as a placeholder. In actual implementation, replace `game::` with the specific game module:
- `boxes::` (for the boxes game)
- `clickers::` (for the clicker game)

## Component Structure
Components are game-specific and not tied to the player entity. 
- Path structure: `[game]/component/[ComponentName].rs`
- Valid examples: `boxes::component::Position`, `clickers::component::Velocity`, `boxes::component::InputBuffer`
- *Note:* Align with AGENTS.md rules. The filename must exactly match the struct name (e.g., `Position.rs`).

## Contextual Naming (Zero Redundancy)
Items (files, functions, structs) inherently inherit the context of their parent directory and module path. **Never repeat parent folder or module names in the child filename, struct name, or function name.** Write names as if they are meant to be read from the root of their path.
- **Domain Logic Example:**
  - ✗ Wrong: `path::copy::copy_one_path` (Redundant "copy" and "path")
  - ✓ Correct: `path::copy::only_one`
- **Systems Example:**
  - ✗ Wrong: `boxes::systems::input_system` (Redundant "system")
  - ✓ Correct: `boxes::systems::collect_input`
- **Components Example:**
  - ✗ Wrong: `clickers::components::PlayerComponent` (Redundant "component")
  - ✓ Correct: `clickers::components::Player`


## 4. Atomic File Structure & Decomposition (CRITICAL)
Every file must contain exactly **one** primary logic unit (one function or one struct).
- **Functions:** Filename matches the function name (e.g., `detect_click.rs` for `fn detect_click`).
- **Structs:** Filename matches the struct name (e.g., `Position.rs` for `struct Position`).
- **Large Impl Blocks:** If a struct's `impl` has multiple methods, you MUST create a subfolder named after the struct, put each method in its own file, and use the main struct file as a router.

## Module Exporting & Importing Rules (CRITICAL)
Because of our Atomic File Structure (e.g., `detect_click.rs` contains `pub fn detect_click`), Rust will naturally create a redundant path: `system::detect_click::detect_click`. We avoid this by strictly flattening at the `mod.rs` level and importing at the parent level.

### A. Exporting (Inside `mod.rs`)
You MUST flatten single-function and single-struct files in their parent `mod.rs` using `pub use` to prevent stutter.
```rust
// Inside src/clicker/system/mod.rs
pub mod detect_click;
pub use detect_click::detect_click; // ✓ CORRECT: Flattens the path
```

### B. Importing (Inside Consumer Files)
To maintain readability, consumer files must import items based on their type:

- **Structs and Enums:** Import the exact item directly. 
  - ✗ `use crate::clicker::component;` -> `component::Player`
  - ✓ `use crate::clicker::component::Player;`
- **Functions (Systems, Helpers, etc.):** Do NOT import the final function. Import its parent module and call the function through it.
  - ✗ `use crate::clicker::system::detect_click;` (Loses context)
  - ✓ `use crate::clicker::system;` -> Called in code as `system::detect_click()`

## Pure `mod.rs` Files (No Logic Allowed)
A `mod.rs` file exists solely to build the module tree and flatten exports. 
**Rule:** You must NEVER define structs, enums, functions, traits, or constants inside a `mod.rs` file. All logic must live in its own atomic file.

- ✗ Wrong (`mod.rs`):
  ```rust
  pub mod detect_click;
  
  // NEVER do this:
  pub fn tiny_helper() { ... } 
  pub struct LocalConfig { ... }
  ```
- ✓ Correct (`mod.rs`):
  ```rust
  pub mod detect_click;
  pub use detect_click::detect_click;
  
  pub mod tiny_helper;
  pub use tiny_helper::tiny_helper;
  ```


## P2P Separation
P2P logic must remain entirely separated from the game logic. 
- Path structure: `p2p::[module]::[Item].rs`
- Valid examples: `p2p::Swarm`, `p2p::connect`
