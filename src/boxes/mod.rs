pub mod component;
pub mod plugin;
pub mod system;

pub use component::{InputBuffer, Player, PlayerInput, Position, Velocity};
pub use plugin::BoxesGamePlugin;
pub use system::{
    apply_input_to_velocity, character_controller, collect, collect_input, handle_player_join,
    handle_player_leave, sync_position,
};