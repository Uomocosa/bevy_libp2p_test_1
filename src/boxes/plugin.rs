use bevy::prelude::*;

use super::system;

pub struct BoxesGamePlugin;

impl Plugin for BoxesGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, system::collect)
            .add_systems(FixedUpdate, system::character_controller)
            .add_systems(FixedUpdate, system::sync_position)
            .add_systems(Update, system::handle_player_join)
            .add_systems(Update, system::handle_player_leave);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usage() {
        let _plugin = BoxesGamePlugin;
    }
}