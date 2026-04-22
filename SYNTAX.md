# SYNTAX

## Rule Priority
This file's rules override standard Rust conventions. Treat this file as the absolute source of truth for architecture and naming.

## 1. Game Module Mapping
We are building two test games. In these rules, we use `game::` as a placeholder. In actual implementation, replace `game::` with the specific game module:
- `boxes::` (for the boxes game)
- `clickers::` (for the clicker game)

## 2. Component Structure
Components are game-specific and not tied to the player entity. 
- Path structure: `[game]/component/[ComponentName].rs`
- Valid examples: `boxes::component::Position`, `clickers::component::Velocity`, `boxes::component::InputBuffer`
- *Note:* Align with AGENTS.md rules. The filename must exactly match the struct name (e.g., `Position.rs`).

## 3. Contextual Naming (Zero Redundancy)
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

## 4. Import Rules (Strict)
To maintain readability, imports are handled strictly based on their type:
- **Structs and Enums:** Import these directly so they are clear in the code.
  - ✓ `use crate::boxes::component::Position;`
- **System Functions:** Do NOT import the final function directly, as it loses context. Import the `system` module and call the function through it.
  - ✗ `use crate::boxes::system::collect_input;`
  - ✓ `use crate::boxes::system;` -> Called in code as `system::collect_input()`

## 5. P2P Separation
P2P logic must remain entirely separated from the game logic. 
- Path structure: `p2p::[module]::[Item].rs`
- Valid examples: `p2p::Swarm`, `p2p::connect`
