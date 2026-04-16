pub mod component;
pub mod system;

use bevy::prelude::*;

use crate::p2p::config::P2PEvent;

pub struct BoxesGamePlugin;

impl Plugin for BoxesGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, system::collect)
            .add_systems(FixedUpdate, system::character_controller)
            .add_systems(FixedUpdate, system::sync_position)
            .add_systems(Update, system::handle_player_join)
            .add_systems(Update, system::handle_player_leave)
            .add_event::<P2PEvent>();
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
