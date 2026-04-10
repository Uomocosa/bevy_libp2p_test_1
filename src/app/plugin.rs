use bevy::prelude::*;

use crate::game::input::player_input_system;
use crate::game::physics::physics_system;
use crate::game::sync_transform::sync_position_to_transform;
use crate::sync::sync_system::{
    apply_remote_inputs_system, broadcast_input_system, NetworkState, RemoteInputBuffer,
};
use crate::sync::tick::Tick;

pub struct BevyP2PPlugin;

impl Plugin for BevyP2PPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Tick>()
            .init_resource::<NetworkState>()
            .init_resource::<RemoteInputBuffer>()
            .add_systems(
                FixedUpdate,
                (
                    tick_system,
                    player_input_system,
                    physics_system,
                    sync_position_to_transform,
                    broadcast_input_system,
                    apply_remote_inputs_system,
                ),
            );
    }
}

fn tick_system(mut tick: ResMut<Tick>) {
    tick.next();
}
