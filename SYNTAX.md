# SYNTAX

## Rule Priority
This file's rules override Rust standard conventions.
Treat this file's rules as a source of truth for this project.

### Reasoning and Syntax
1. Components (`Position`, `Velocity`, `PlayerInput`, ...) are game components - not tied to player entity -> 
    - `game::component::Position`
    - `game::component::Velocity`
    - `game::component::Player`
    - ...
2. Systems tied to game domain, not specific entity -> 
    - `game::system::input`
    - `game::system::jump`
    - `game::system::move`
    - `game::system::apply_input_to_velocity`
    - ...
3. I dont like repetetions. Even in names ->
    - `game::system::input_system` (✗ Wrong - We are saying two times that this is a 'system')
    - `game::system::collect_input` (✓ Correct)
4. When importing un-clear functions/struct/enum: Since we define well-defined paths, sometimes it may happen that if we import the "final" struct/enum/function, it becomes unclear.
    - `use crate::game::system::collect_input` -> When we call `callect_input` it is unclear that it is a game system. So it must become: `use crate::game` and then we call it as `game::system::callect_input`.
4. When importing clear functions/struct/enum:
    - `use crate::game::components::{...}` -> The components inside the components/ folder are well-defiend struct and names. So it makes sense to directly import them.
5. (TODO!) P2P should be separated by the game: p2p::Swarm, p2p::connect, ...
