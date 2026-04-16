pub mod component;
pub mod system;

pub use system::detect_click;
pub use system::update_counter;

use bevy::prelude::*;

pub struct ClickerGamePlugin;

impl Plugin for ClickerGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (system::detect_click, system::update_counter));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usage() {
        let _plugin = ClickerGamePlugin;
    }
}
