# SYNTAX

## Reasoning and Syntax

1. The uppercase letters are important! If rust dont like them we need to add one or more ignore rules.
2. **Filename case mirrors struct case:**
   - `Position` struct → `Position.rs` file
   - `Player` struct → `Player.rs` file
   - `velocity` struct → `velocity.rs` file (lowercase struct = lowercase file)
3. Components (Position, Velocity, PlayerInput, ...) are game components - not tied to player entity -> game::component::Position, game::component::Velocity, ...
4. Systems tied to game domain, not specific entity -> game::system::input, game::system::jump, game::system::move, ...
5. (TODO!) P2P should be separated by the game: p2p::Swarm, p2p::connect, ...
