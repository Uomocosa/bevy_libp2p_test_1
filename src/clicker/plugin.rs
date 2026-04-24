use bevy::prelude::*;

use super::system;

pub struct ClickerGamePlugin;

impl Plugin for ClickerGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (system::detect_click, system::update_counter))
            .add_systems(Update, system::handle_player_join)
            .add_systems(Update, system::handle_player_leave);
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