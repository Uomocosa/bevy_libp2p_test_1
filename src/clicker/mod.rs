pub mod component;
pub mod plugin;
pub mod system;

pub use component::{ClickCounter, ClickTarget, Owner};
pub use plugin::ClickerGamePlugin;
pub use system::{detect_click, handle_player_join, handle_player_leave, update_counter};