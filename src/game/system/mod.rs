pub mod input;
pub mod physics;
pub mod sync_transform;

pub use input::{collect_input, player_input_system};
pub use physics::{apply_input_to_velocity, physics_system};
pub use sync_transform::sync_position_to_transform;
