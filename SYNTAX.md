# SYNTAX

## Rule Priority
This file's rules override Rust standard conventions.
Treat this file's rules as a source of truth for this project.

### Reasoning and Syntax
1. Components (Position, Velocity, PlayerInput, ...) are game components - not tied to player entity -> game::component::Position, game::component::Velocity, ...
2. Systems tied to game domain, not specific entity -> game::system::input, game::system::jump, game::system::move, ...
3. (TODO!) P2P should be separated by the game: p2p::Swarm, p2p::connect, ...
